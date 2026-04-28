// Rankenstein Suite (M13): in-pane fixed title row.
//
// Renders a 1-cell-tall horizontal strip across the top of every pane
// that overlays the pane's row 0 with the agent identity. Drawn AFTER
// pane content so it stays visible regardless of how the user scrolls.
//
// Visual policy (founder direction 2026-04-25):
//   * NO background fill — the pane's normal background already
//     occupies this row (we shifted only the terminal CONTENT down,
//     the pane bg fills the full pane area). Painting a strip on top
//     would obscure the M3 attention border edge + pane separators.
//   * foreground = a vibrant accent picked from the pane's palette
//     using the same scoring used by M3 attention borders, so the
//     label visually ties to the per-pane theme (and signals "this
//     is chrome" by being visibly tinted).
//   * font       = self.fonts.default_font() — the EXACT same font
//     the terminal uses for its content, with metrics from
//     self.render_metrics so glyph shaping is byte-for-byte identical
//     to terminal text on the row below.
//   * NO separator line — anything that demarcates the strip from the
//     pane content makes it read as a "separate panel," which the
//     founder explicitly does not want.
//
// Title source priority:
//   1) `AGENT_NAME` OSC 1337 user variable (set by agent-launch.ps1
//      — sticky, survives codex's own title overwrites)
//   2) `pane.get_title()` if non-empty
//   3) fallback `pane <id>` so the strip is never blank

use crate::quad::TripleLayerQuadAllocator;
use crate::termwindow::box_model::*;
use crate::termwindow::render::borders::attention_color_from_palette;
use config::{Dimension, DimensionContext};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use window::color::LinearRgba;

const MIN_PANE_WIDTH_COLS: usize = 6;

static FIRST_CALL_LOGGED: AtomicBool = AtomicBool::new(false);

// =====================================================================
// M13 strip render cache
//
// `compute_element(ElementContent::Text(...))` calls `font.shape()` which
// runs HarfBuzz on every invocation — that's the expensive step in the
// strip render path. Without caching, paint_in_pane_titles re-shapes
// every pane's title on every frame (sidebars animate at 500ms, codex
// blinks the cursor, the cockpit repaints freely), pegging ~60% of one
// CPU core at 12 panes.
//
// For our use case the title text is sticky: AGENT_NAME is set once by
// agent-launch.ps1 and never changes for the life of the pane. Same for
// the per-pane theme color (set at pane creation). So we cache the full
// ComputedElement per pane keyed by all the inputs that could matter,
// and on cache hit just feed it to render_element — skipping the entire
// compute_element path including the HarfBuzz shape() call.
//
// Cache keys: pane_id + title + aux + bounds + color + cell metrics.
// Any change (resize, theme cycle, AGENT_NAME update) misses and we
// recompute that one entry. At idle steady-state, the cache hits 100%.
//
// Eviction: not needed in practice — Suite has ~16 panes and pane_ids
// are reused only over very long sessions. If churn becomes real, we'd
// add an LRU cap; for now an unbounded HashMap is fine and simple.
//
// Invalidation gaps (deliberately accepted):
//   * Font reload (rare; user would just rerun the cockpit anyway).
//   * Glyph atlas eviction (CachedGlyph Rc keeps the atlas slot pinned
//     so glyphs we hold won't be repacked under us).
//
// thread_local because all rendering happens on the GUI thread; no
// locking needed and the cache is naturally per-window.

#[derive(Debug)]
struct StripCacheEntry {
    title: String,
    title_computed: ComputedElement,
    aux: Option<String>,
    aux_computed: Option<ComputedElement>,
    x: f32,
    y: f32,
    w: f32,
    fg: LinearRgba,
    cell_width: f32,
    cell_height: f32,
}

thread_local! {
    static STRIP_CACHE: RefCell<HashMap<mux::pane::PaneId, StripCacheEntry>> =
        RefCell::new(HashMap::new());
}

fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}

fn linear_rgba_eq(a: LinearRgba, b: LinearRgba) -> bool {
    approx_eq(a.0, b.0, 1e-4)
        && approx_eq(a.1, b.1, 1e-4)
        && approx_eq(a.2, b.2, 1e-4)
        && approx_eq(a.3, b.3, 1e-4)
}

impl crate::TermWindow {
    pub fn paint_in_pane_titles(
        &mut self,
        _layers: &mut TripleLayerQuadAllocator,
    ) -> anyhow::Result<()> {
        let inset_rows = self.config.pane_top_inset_rows;
        if inset_rows == 0 {
            return Ok(());
        }
        let panes = self.get_panes_to_render();
        if panes.is_empty() {
            return Ok(());
        }

        let cell_width = self.render_metrics.cell_size.width as f32;
        let cell_height = self.render_metrics.cell_size.height as f32;
        let (padding_left, padding_top) = self.padding_left_top();
        let tab_bar_height = if self.show_tab_bar {
            self.tab_bar_pixel_height().unwrap_or(0.0)
        } else {
            0.0
        };
        let top_bar_height = if self.config.tab_bar_at_bottom {
            0.0
        } else {
            tab_bar_height
        };
        let inner_border = self.get_os_border();
        let top_pixel_y = top_bar_height + padding_top + inner_border.top.get() as f32;
        let win_w = self.dimensions.pixel_width as f32;
        let term_cols = self.terminal_size.cols as usize;
        // Strip occupies exactly the inset area at the top of each
        // pane — same number of cell rows the mux subtracted from
        // the terminal size in apply_pane_top_inset.
        let strip_height = (cell_height * inset_rows as f32).round();

        // Snapshot per-pane data so the panes borrow drops before we
        // touch `&mut self` for compute_element / render_element.
        let mut strips: Vec<StripPlan> = Vec::new();
        for pos in &panes {
            if pos.width < MIN_PANE_WIDTH_COLS || pos.height < 2 {
                continue;
            }

            let user_vars = pos.pane.copy_user_vars();
            let raw_title = pos.pane.get_title();
            let display_title =
                derive_display_title(&user_vars, &raw_title, pos.pane.pane_id());
            if display_title.is_empty() {
                continue;
            }

            let (x, width_delta) = if pos.left == 0 {
                (
                    0.0,
                    padding_left + inner_border.left.get() as f32 + (cell_width / 2.0),
                )
            } else {
                (
                    padding_left + inner_border.left.get() as f32 - (cell_width / 2.0)
                        + (pos.left as f32 * cell_width),
                    cell_width,
                )
            };
            let y = if pos.top == 0 {
                top_pixel_y - padding_top
            } else {
                top_pixel_y + (pos.top as f32 * cell_height) - (cell_height / 2.0)
            };
            let w = if pos.left + pos.width >= term_cols {
                win_w - x
            } else {
                (pos.width as f32 * cell_width) + width_delta
            };

            let palette = pos.pane.palette();
            // Title color: most vibrant accent from the pane's
            // palette (same scoring M3 attention borders use). Ties
            // the strip to the per-pane theme and signals "this is
            // chrome" without needing a separator.
            let fg = opaque(attention_color_from_palette(&palette));

            // Optional right-justified aux text (e.g. "up 4m" set
            // by the sidebar). When present we render it at the
            // right edge of the strip in a dimmed variant of the
            // accent so it reads as secondary metadata.
            let aux = user_vars
                .get("AGENT_AUX")
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());

            // Snap to integer pixels so glyph rasterization aligns
            // to the pixel grid — sub-pixel positions cause the same
            // monospace font to render with broken-up strokes.
            strips.push(StripPlan {
                pane_id: pos.pane.pane_id(),
                x: x.round(),
                y: y.round(),
                w: w.round(),
                title: display_title,
                aux,
                fg,
            });
        }

        if !FIRST_CALL_LOGGED.swap(true, Ordering::Relaxed) {
            log::info!(
                "[suite/M13] paint_in_pane_titles: panes={} eligible_strips={}",
                panes.len(),
                strips.len()
            );
            for s in &strips {
                log::info!(
                    "[suite/M13]   strip x={:.0} y={:.0} w={:.0} h={:.0} title={:?}",
                    s.x,
                    s.y,
                    s.w,
                    strip_height,
                    s.title
                );
            }
        }

        if strips.is_empty() {
            return Ok(());
        }

        // No background fill — the pane's normal background already
        // covers this row (we only shifted the terminal CONTENT down,
        // the pane bg fills the full pane area). Painting an opaque
        // strip over it covered the M3 attention border + pane
        // separator lines; the founder caught this. Just render the
        // title text on top of whatever's underneath.

        // Title text in the pane's terminal font, rendered via the
        // box_model Element pipeline. Use self.render_metrics (the
        // window's terminal render metrics — what the per-line text
        // path also uses) so cell_width / cell_height / ascender /
        // descender match the terminal exactly. Deriving fresh
        // metrics from font.metrics() gave the wrong cell sizing and
        // produced visibly crunchy / vertically-cropped glyphs that
        // didn't match the row below.
        let dpi = self.dimensions.dpi as f32;
        let font = self.fonts.default_font()?;
        let gl_state = self.render_state.as_ref().unwrap();
        let transparent = LinearRgba::with_components(0.0, 0.0, 0.0, 0.0);

        // compute_element subtracts `bounds.min_x()` from
        // `width.pixel_max` to derive the max text width. So
        // pixel_max must be the ABSOLUTE right edge of the window,
        // not the strip's local width — otherwise panes whose x > 0
        // (everything except the leftmost) end up with a negative
        // max width and render no text.
        let abs_right = self.dimensions.pixel_width as f32;

        for s in strips {
            // Aux geometry (right-justified, reserves chars × cell_w
            // at right edge). Computed up-front so the cache key /
            // miss path can both reference the same numbers.
            let aux_geom: Option<(String, f32, f32)> = s.aux.as_ref().map(|aux| {
                let aux_chars = aux.chars().count() as f32;
                let aux_w = aux_chars * cell_width;
                let aux_x = (s.x + s.w - aux_w).max(s.x);
                (aux.clone(), aux_x, aux_w)
            });

            // Cache hit? Compare the full input fingerprint. On hit we
            // skip compute_element entirely — including font.shape() —
            // and just re-emit the cached quads via render_element.
            // That's the optimization: at idle steady-state every
            // frame is a hit and the strip costs ~nothing.
            let cache_hit = STRIP_CACHE.with(|cache| {
                let cache = cache.borrow();
                cache.get(&s.pane_id).and_then(|entry| {
                    let aux_text_match = match (&entry.aux, &aux_geom) {
                        (None, None) => true,
                        (Some(a), Some((b, _, _))) => a == b,
                        _ => false,
                    };
                    if entry.title == s.title
                        && aux_text_match
                        && approx_eq(entry.x, s.x, 0.5)
                        && approx_eq(entry.y, s.y, 0.5)
                        && approx_eq(entry.w, s.w, 0.5)
                        && linear_rgba_eq(entry.fg, s.fg)
                        && approx_eq(entry.cell_width, cell_width, 0.01)
                        && approx_eq(entry.cell_height, cell_height, 0.01)
                    {
                        Some((
                            entry.title_computed.clone(),
                            entry.aux_computed.clone(),
                        ))
                    } else {
                        None
                    }
                })
            });

            if let Some((title_computed, aux_computed)) = cache_hit {
                self.render_element(&title_computed, gl_state, None)?;
                if let Some(ac) = aux_computed {
                    self.render_element(&ac, gl_state, None)?;
                }
                continue;
            }

            // Cache miss: compute fresh, render, then store.
            let title_computed = self.compute_strip_element(
                &font,
                &s.title,
                s.x,
                s.y,
                s.w,
                s.fg,
                strip_height,
                cell_width,
                cell_height,
                dpi,
                abs_right,
                transparent,
                gl_state,
            )?;
            self.render_element(&title_computed, gl_state, None)?;

            let aux_computed = if let Some((aux_text, aux_x, aux_w)) = aux_geom.as_ref() {
                let computed = self.compute_strip_element(
                    &font,
                    aux_text,
                    *aux_x,
                    s.y,
                    *aux_w,
                    s.fg,
                    strip_height,
                    cell_width,
                    cell_height,
                    dpi,
                    abs_right,
                    transparent,
                    gl_state,
                )?;
                self.render_element(&computed, gl_state, None)?;
                Some(computed)
            } else {
                None
            };

            STRIP_CACHE.with(|cache| {
                cache.borrow_mut().insert(
                    s.pane_id,
                    StripCacheEntry {
                        title: s.title.clone(),
                        title_computed,
                        aux: aux_geom.as_ref().map(|(t, _, _)| t.clone()),
                        aux_computed,
                        x: s.x,
                        y: s.y,
                        w: s.w,
                        fg: s.fg,
                        cell_width,
                        cell_height,
                    },
                );
            });
        }

        Ok(())
    }
}

impl crate::TermWindow {
    /// Build an Element + run compute_element to produce the
    /// ComputedElement for a single strip text run. Caller is
    /// responsible for calling `render_element` on the result and (if
    /// caching) storing it in STRIP_CACHE. compute_element is the part
    /// that calls `font.shape()` and walks the box-model pipeline,
    /// which is what we want to skip on cached frames.
    #[allow(clippy::too_many_arguments)]
    fn compute_strip_element(
        &self,
        font: &std::rc::Rc<wezterm_font::LoadedFont>,
        text: &str,
        x: f32,
        y: f32,
        w: f32,
        fg: LinearRgba,
        strip_height: f32,
        cell_width: f32,
        cell_height: f32,
        dpi: f32,
        abs_right: f32,
        transparent: LinearRgba,
        gl_state: &crate::termwindow::RenderState,
    ) -> anyhow::Result<ComputedElement> {
        let element = Element::new(font, ElementContent::Text(text.to_string()))
            .colors(ElementColors {
                border: BorderColor::default(),
                bg: transparent.into(),
                text: fg.into(),
            })
            // Zero padding so the first glyph aligns to the cell
            // boundary — sub-cell padding put glyphs at sub-pixel x
            // positions and visibly distorted the monospace font.
            .padding(BoxDimension {
                left: Dimension::Pixels(0.0),
                right: Dimension::Pixels(0.0),
                top: Dimension::Pixels(0.0),
                bottom: Dimension::Pixels(0.0),
            })
            .display(DisplayType::Inline);

        self.compute_element(
            &LayoutContext {
                height: DimensionContext {
                    dpi,
                    pixel_max: strip_height,
                    pixel_cell: cell_height,
                },
                width: DimensionContext {
                    dpi,
                    pixel_max: abs_right,
                    pixel_cell: cell_width,
                },
                bounds: euclid::rect(x, y, w, strip_height),
                metrics: &self.render_metrics,
                gl_state,
                zindex: 12,
            },
            &element,
        )
    }
}

struct StripPlan {
    pane_id: mux::pane::PaneId,
    x: f32,
    y: f32,
    w: f32,
    title: String,
    /// Optional right-justified aux text (e.g. uptime).
    aux: Option<String>,
    fg: LinearRgba,
}

fn opaque(c: LinearRgba) -> LinearRgba {
    LinearRgba::with_components(c.0, c.1, c.2, 1.0)
}

fn derive_display_title(
    user_vars: &HashMap<String, String>,
    raw_title: &str,
    pane_id: mux::pane::PaneId,
) -> String {
    if let Some(name) = user_vars.get("AGENT_NAME") {
        let trimmed = name.trim();
        if !trimmed.is_empty() {
            // Plain name, matches terminal styling. Leader/worker
            // distinction is conveyed by the sidebar, not duplicated
            // here — keeps the strip uncluttered.
            return trimmed.to_string();
        }
    }
    let trimmed_title = raw_title.trim();
    if !trimmed_title.is_empty() {
        return strip_redundant_leader_prefix(trimmed_title);
    }
    format!("pane {}", pane_id)
}

fn strip_redundant_leader_prefix(title: &str) -> String {
    let separator = " · ";
    let mut parts: Vec<&str> = title.split(separator).collect();

    if parts.len() >= 2
        && parts[0].eq_ignore_ascii_case("leader")
        && parts[1].eq_ignore_ascii_case("leader")
    {
        parts.remove(0);
    }

    if parts.len() >= 2 {
        let last_lower = parts[parts.len() - 1].to_ascii_lowercase();
        if last_lower == "codex" || last_lower == "claude" || last_lower == "pwsh" {
            parts.pop();
        }
    }

    parts.join(separator)
}

#[cfg(test)]
mod tests {
    use super::{derive_display_title, strip_redundant_leader_prefix};
    use std::collections::HashMap;

    #[test]
    fn collapses_doubled_leader_prefix() {
        assert_eq!(
            strip_redundant_leader_prefix("LEADER · Leader · codex"),
            "Leader"
        );
        assert_eq!(strip_redundant_leader_prefix("LEADER · Leader"), "Leader");
    }

    #[test]
    fn drops_trailing_command() {
        assert_eq!(
            strip_redundant_leader_prefix("wiki-keeper · codex"),
            "wiki-keeper"
        );
    }

    #[test]
    fn user_var_wins_over_title() {
        let mut vars = HashMap::new();
        vars.insert("AGENT_NAME".to_string(), "wiki-keeper".to_string());
        assert_eq!(
            derive_display_title(&vars, "python.exe", 5),
            "wiki-keeper"
        );
    }

    #[test]
    fn leader_flag_does_not_duplicate_name() {
        let mut vars = HashMap::new();
        vars.insert("AGENT_NAME".to_string(), "Leader".to_string());
        vars.insert("AGENT_IS_LEADER".to_string(), "1".to_string());
        // Sidebar shows leader status; strip stays uncluttered.
        assert_eq!(derive_display_title(&vars, "python.exe", 5), "Leader");
    }

    #[test]
    fn falls_back_to_pane_id_when_empty() {
        let vars = HashMap::new();
        assert_eq!(derive_display_title(&vars, "", 7), "pane 7");
        assert_eq!(derive_display_title(&vars, "   ", 9), "pane 9");
    }
}
