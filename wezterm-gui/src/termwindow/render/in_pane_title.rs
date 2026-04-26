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
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use window::color::LinearRgba;

const MIN_PANE_WIDTH_COLS: usize = 6;

static FIRST_CALL_LOGGED: AtomicBool = AtomicBool::new(false);

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
            // Left-aligned title (project / agent name).
            self.render_strip_text(
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

            // Right-justified aux text (e.g. uptime). Reserve
            // approximately `aux.chars() * cell_width` pixels at the
            // right edge — assumes the user's terminal font is
            // monospace, which it always is for our use case.
            if let Some(aux) = s.aux {
                let aux_chars = aux.chars().count() as f32;
                let aux_w = aux_chars * cell_width;
                let aux_x = (s.x + s.w - aux_w).max(s.x);
                self.render_strip_text(
                    &font,
                    &aux,
                    aux_x,
                    s.y,
                    aux_w,
                    s.fg,
                    strip_height,
                    cell_width,
                    cell_height,
                    dpi,
                    abs_right,
                    transparent,
                    gl_state,
                )?;
            }
        }

        Ok(())
    }
}

impl crate::TermWindow {
    /// Render a single text run inside a strip rectangle. Used for
    /// both the left-aligned title and the optional right-justified
    /// aux text — same rendering path so font / color / metrics stay
    /// identical between the two.
    #[allow(clippy::too_many_arguments)]
    fn render_strip_text(
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
    ) -> anyhow::Result<()> {
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

        let computed = self.compute_element(
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
        )?;

        self.render_element(&computed, gl_state, None)
    }
}

struct StripPlan {
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
