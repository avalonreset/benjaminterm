use crate::quad::TripleLayerQuadAllocator;
use crate::termwindow::TermWindowNotif;
use crate::utilsprites::RenderMetrics;
use ::window::{ULength, WindowOps};
use config::{ConfigHandle, DimensionContext};
use mux::pane::PaneId;
use mux::Mux;
use std::time::{Duration, Instant};
use wezterm_term::color::ColorPalette;
use window::color::LinearRgba;

// Rankenstein Suite (M3): per-pane attention pulse instead of window-wide.
// When an agent emits a ready signal, only THAT pane's border lights up,
// independently of which pane currently has focus.

/// Pick the most vibrant attention color from a palette. Mirrors the
/// scoring in `TermWindow::theme_attention_color` exactly so per-pane
/// attention picks colors from the pane's OWN palette (set via OSC
/// theme injection — Mandatory M2) and each pane's pulse looks
/// distinct, matching its theme.
pub fn attention_color_from_palette(palette: &ColorPalette) -> LinearRgba {
    [9usize, 10, 11, 12, 13, 14, 1, 2, 3, 4, 5, 6]
        .iter()
        .copied()
        .filter_map(|idx| {
            let color = palette.colors.0[idx];
            let max = color.0.max(color.1).max(color.2);
            let min = color.0.min(color.1).min(color.2);
            let saturation = max - min;

            if saturation < 0.12 || max < 0.25 {
                return None;
            }

            let luminance =
                (0.2126_f32 * color.0) + (0.7152_f32 * color.1) + (0.0722_f32 * color.2);
            let score = saturation * 2.0_f32 + max - (luminance - 0.68_f32).abs() * 0.35_f32;
            Some((score, color))
        })
        .max_by(|(a, _), (b, _)| a.total_cmp(b))
        .map(|(_, color)| color.to_linear())
        .unwrap_or_else(|| palette.selection_bg.to_linear())
}

const TOAST_ATTENTION_PULSE_DURATION: Duration = Duration::from_millis(900);
const TOAST_ATTENTION_PULSE_FRAME: Duration = Duration::from_millis(60);
const IDLE_TEXT_GLOW_FRAME: Duration = Duration::from_millis(50);

impl crate::TermWindow {
    pub fn start_agent_attention_for_pane(&mut self, pane_id: PaneId) {
        self.start_visual_attention_for_pane(pane_id);
    }

    pub fn start_visual_attention_for_pane(&mut self, pane_id: PaneId) {
        let mux = Mux::get();
        if let Some((_domain, window_id, tab_id)) = mux.resolve_pane_id(pane_id) {
            if window_id == self.mux_window_id {
                self.agent_attention_tab_until
                    .borrow_mut()
                    .insert(tab_id, Instant::now() + TOAST_ATTENTION_PULSE_DURATION);
                {
                    let now = Instant::now();
                    let mut pane_state = self.pane_state(pane_id);
                    pane_state.bell_start.replace(now);
                    pane_state.idle_text_glow_start.replace(now);
                    // Rankenstein Suite (M3): dedicated attention timestamp
                    // not cleared by the visual_bell renderer.
                    pane_state.agent_attention_start.replace(now);
                }
                self.start_idle_text_glow_animation_for_pane(pane_id);
                self.invalidate_fancy_tab_bar();
            }
        }

        self.start_toast_attention_pulse();
    }

    fn start_idle_text_glow_animation_for_pane(&mut self, pane_id: PaneId) {
        {
            let mut pane_state = self.pane_state(pane_id);
            if pane_state.idle_text_glow_animation_active {
                return;
            }
            pane_state.idle_text_glow_animation_active = true;
        }

        let Some(window) = self.window.clone() else {
            let mut pane_state = self.pane_state(pane_id);
            pane_state.idle_text_glow_animation_active = false;
            return;
        };

        window.invalidate();
        promise::spawn::spawn(async move {
            loop {
                smol::Timer::after(IDLE_TEXT_GLOW_FRAME).await;

                let win = window.clone();
                window.notify(TermWindowNotif::Apply(Box::new(move |term_window| {
                    let keep_animating = {
                        let mut pane_state = term_window.pane_state(pane_id);
                        let keep_animating = pane_state.idle_text_glow_start.is_some();
                        if !keep_animating {
                            pane_state.idle_text_glow_animation_active = false;
                        }
                        keep_animating
                    };

                    if keep_animating {
                        win.invalidate();
                    }
                })));
            }
        })
        .detach();
    }

    pub fn start_toast_attention_pulse(&mut self) {
        let until = Instant::now() + TOAST_ATTENTION_PULSE_DURATION;
        self.toast_attention_pulse_until = Some(until);

        let Some(window) = self.window.clone() else {
            return;
        };

        window.invalidate();
        promise::spawn::spawn(async move {
            loop {
                smol::Timer::after(TOAST_ATTENTION_PULSE_FRAME).await;

                let win = window.clone();
                window.notify(TermWindowNotif::Apply(Box::new(move |term_window| {
                    if term_window
                        .toast_attention_pulse_until
                        .map(|until| Instant::now() >= until)
                        .unwrap_or(false)
                    {
                        term_window.toast_attention_pulse_until.take();
                    }
                    term_window.invalidate_fancy_tab_bar();
                    win.invalidate();
                })));

                if Instant::now() >= until {
                    break;
                }
            }
        })
        .detach();
    }

    fn toast_attention_pulse_alpha(&mut self) -> Option<f32> {
        let until = self.toast_attention_pulse_until?;
        let now = Instant::now();
        if now >= until {
            self.toast_attention_pulse_until.take();
            return None;
        }

        let total = TOAST_ATTENTION_PULSE_DURATION.as_secs_f32();
        let remaining = until.duration_since(now).as_secs_f32();
        let elapsed = total - remaining;
        let fade = remaining / total;
        let wave = (elapsed * std::f32::consts::TAU * 2.4).sin().abs();
        Some((0.18 + 0.62 * wave) * fade)
    }

    fn theme_attention_color(&mut self) -> LinearRgba {
        let palette = self.palette();
        attention_color_from_palette(&palette)
    }

    pub fn paint_window_borders(
        &mut self,
        layers: &mut TripleLayerQuadAllocator,
    ) -> anyhow::Result<()> {
        let border_dimensions = self.get_os_border();

        if border_dimensions.top.get() > 0
            || border_dimensions.bottom.get() > 0
            || border_dimensions.left.get() > 0
            || border_dimensions.right.get() > 0
        {
            let height = self.dimensions.pixel_height as f32;
            let width = self.dimensions.pixel_width as f32;

            let border_top = border_dimensions.top.get() as f32;
            if border_top > 0.0 {
                self.filled_rectangle(
                    layers,
                    1,
                    euclid::rect(0.0, 0.0, width, border_top),
                    self.config
                        .window_frame
                        .border_top_color
                        .map(|c| c.to_linear())
                        .unwrap_or(border_dimensions.color),
                )?;
            }

            let border_left = border_dimensions.left.get() as f32;
            if border_left > 0.0 {
                self.filled_rectangle(
                    layers,
                    1,
                    euclid::rect(0.0, 0.0, border_left, height),
                    self.config
                        .window_frame
                        .border_left_color
                        .map(|c| c.to_linear())
                        .unwrap_or(border_dimensions.color),
                )?;
            }

            let border_bottom = border_dimensions.bottom.get() as f32;
            if border_bottom > 0.0 {
                self.filled_rectangle(
                    layers,
                    1,
                    euclid::rect(0.0, height - border_bottom, width, height),
                    self.config
                        .window_frame
                        .border_bottom_color
                        .map(|c| c.to_linear())
                        .unwrap_or(border_dimensions.color),
                )?;
            }

            let border_right = border_dimensions.right.get() as f32;
            if border_right > 0.0 {
                self.filled_rectangle(
                    layers,
                    1,
                    euclid::rect(width - border_right, 0.0, border_right, height),
                    self.config
                        .window_frame
                        .border_right_color
                        .map(|c| c.to_linear())
                        .unwrap_or(border_dimensions.color),
                )?;
            }
        }

        // Rankenstein Suite (M3): per-pane attention pulse.
        //
        // Replaces the previous behavior that drew the pulse around the
        // entire window. Now we iterate the panes in the active tab and
        // draw the pulse around each pane that has a recent bell_start.
        // This means: when a background pane signals ready, ITS border
        // lights up — not the whole app — and the pulse persists even
        // when the user clicks onto a different pane (until they focus
        // the ready pane, which clears bell_start elsewhere in the code).
        //
        // We still drive the animation timer via toast_attention_pulse_until
        // (set by start_toast_attention_pulse) so the window keeps
        // invalidating each frame; we just don't use the resulting alpha
        // for window-wide drawing anymore.
        let panes = self.get_panes_to_render();
        if !panes.is_empty() {
            // Collect (pane_idx, color) for panes in attention. Color
            // comes from each pane's OWN palette (theme set per-pane via
            // OSC injection — M2) so each pane's pulse looks distinct
            // and matches its theme.
            let mut active_panes: Vec<(usize, LinearRgba)> = Vec::new();
            for (idx, pos) in panes.iter().enumerate() {
                let pane_id = pos.pane.pane_id();
                let attention_start = self.pane_state(pane_id).agent_attention_start;
                if let Some(start) = attention_start {
                    let elapsed = start.elapsed();
                    if elapsed < TOAST_ATTENTION_PULSE_DURATION {
                        let total = TOAST_ATTENTION_PULSE_DURATION.as_secs_f32();
                        let elapsed_f = elapsed.as_secs_f32();
                        let remaining = total - elapsed_f;
                        let fade = remaining / total;
                        let wave =
                            (elapsed_f * std::f32::consts::TAU * 2.4).sin().abs();
                        let alpha = (0.18_f32 + 0.62_f32 * wave) * fade;
                        let palette = pos.pane.palette();
                        let color =
                            attention_color_from_palette(&palette).mul_alpha(alpha);
                        active_panes.push((idx, color));
                    }
                }
            }

            if !active_panes.is_empty() {
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
                let top_pixel_y =
                    top_bar_height + padding_top + inner_border.top.get() as f32;
                let pulse_width = 5.0_f32;
                let win_w = self.dimensions.pixel_width as f32;
                let win_h = self.dimensions.pixel_height as f32;
                let term_cols = self.terminal_size.cols as usize;
                let term_rows = self.terminal_size.rows as usize;

                for (idx, color) in &active_panes {
                    let pos = &panes[*idx];
                    let color = *color;

                    // Match build_pane's background_rect math exactly so
                    // our border sits flush against the pane edge — not
                    // offset by cell_width/2 like the naive computation.
                    let (x, width_delta) = if pos.left == 0 {
                        (0.0, padding_left + inner_border.left.get() as f32 + (cell_width / 2.0))
                    } else {
                        (
                            padding_left + inner_border.left.get() as f32
                                - (cell_width / 2.0)
                                + (pos.left as f32 * cell_width),
                            cell_width,
                        )
                    };
                    let (y, height_delta) = if pos.top == 0 {
                        (top_pixel_y - padding_top, padding_top + (cell_height / 2.0))
                    } else {
                        (
                            top_pixel_y + (pos.top as f32 * cell_height) - (cell_height / 2.0),
                            cell_height,
                        )
                    };
                    let w = if pos.left + pos.width >= term_cols {
                        win_w - x
                    } else {
                        (pos.width as f32 * cell_width) + width_delta
                    };
                    let h = if pos.top + pos.height >= term_rows {
                        win_h - y
                    } else {
                        (pos.height as f32 * cell_height) + height_delta
                    };

                    // Top edge of pane — flush at the pane edge
                    self.filled_rectangle(
                        layers,
                        2,
                        euclid::rect(x, y, w, pulse_width),
                        color,
                    )?;
                    // Bottom edge
                    self.filled_rectangle(
                        layers,
                        2,
                        euclid::rect(x, y + h - pulse_width, w, pulse_width),
                        color,
                    )?;
                    // Left edge
                    self.filled_rectangle(
                        layers,
                        2,
                        euclid::rect(x, y, pulse_width, h),
                        color,
                    )?;
                    // Right edge
                    self.filled_rectangle(
                        layers,
                        2,
                        euclid::rect(x + w - pulse_width, y, pulse_width, h),
                        color,
                    )?;
                }
            }
        }

        Ok(())
    }

    pub fn get_os_border_impl(
        os_parameters: &Option<window::parameters::Parameters>,
        config: &ConfigHandle,
        dimensions: &crate::Dimensions,
        render_metrics: &RenderMetrics,
    ) -> window::parameters::Border {
        let mut border = os_parameters
            .as_ref()
            .and_then(|p| p.border_dimensions.clone())
            .unwrap_or_default();

        border.left += ULength::new(
            config
                .window_frame
                .border_left_width
                .evaluate_as_pixels(DimensionContext {
                    dpi: dimensions.dpi as f32,
                    pixel_max: dimensions.pixel_width as f32,
                    pixel_cell: render_metrics.cell_size.width as f32,
                })
                .ceil() as usize,
        );
        border.right += ULength::new(
            config
                .window_frame
                .border_right_width
                .evaluate_as_pixels(DimensionContext {
                    dpi: dimensions.dpi as f32,
                    pixel_max: dimensions.pixel_width as f32,
                    pixel_cell: render_metrics.cell_size.width as f32,
                })
                .ceil() as usize,
        );
        border.top += ULength::new(
            config
                .window_frame
                .border_top_height
                .evaluate_as_pixels(DimensionContext {
                    dpi: dimensions.dpi as f32,
                    pixel_max: dimensions.pixel_height as f32,
                    pixel_cell: render_metrics.cell_size.height as f32,
                })
                .ceil() as usize,
        );
        border.bottom += ULength::new(
            config
                .window_frame
                .border_bottom_height
                .evaluate_as_pixels(DimensionContext {
                    dpi: dimensions.dpi as f32,
                    pixel_max: dimensions.pixel_height as f32,
                    pixel_cell: render_metrics.cell_size.height as f32,
                })
                .ceil() as usize,
        );

        border
    }

    pub fn get_os_border(&self) -> window::parameters::Border {
        Self::get_os_border_impl(
            &self.os_parameters,
            &self.config,
            &self.dimensions,
            &self.render_metrics,
        )
    }
}
