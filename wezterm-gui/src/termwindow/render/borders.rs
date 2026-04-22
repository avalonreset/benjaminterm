use crate::quad::TripleLayerQuadAllocator;
use crate::termwindow::TermWindowNotif;
use crate::utilsprites::RenderMetrics;
use ::window::{ULength, WindowOps};
use config::{ConfigHandle, DimensionContext};
use mux::pane::PaneId;
use mux::Mux;
use std::time::{Duration, Instant};
use window::color::LinearRgba;

const TOAST_ATTENTION_PULSE_DURATION: Duration = Duration::from_millis(900);
const TOAST_ATTENTION_PULSE_FRAME: Duration = Duration::from_millis(60);

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
                self.pane_state(pane_id).bell_start.replace(Instant::now());
                self.invalidate_fancy_tab_bar();
            }
        }

        self.start_toast_attention_pulse();
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

        if let Some(alpha) = self.toast_attention_pulse_alpha() {
            let height = self.dimensions.pixel_height as f32;
            let width = self.dimensions.pixel_width as f32;
            let pulse_width = 5.0;
            let color = self.theme_attention_color().mul_alpha(alpha);

            self.filled_rectangle(layers, 2, euclid::rect(0.0, 0.0, width, pulse_width), color)?;
            self.filled_rectangle(
                layers,
                2,
                euclid::rect(0.0, height - pulse_width, width, pulse_width),
                color,
            )?;
            self.filled_rectangle(
                layers,
                2,
                euclid::rect(0.0, 0.0, pulse_width, height),
                color,
            )?;
            self.filled_rectangle(
                layers,
                2,
                euclid::rect(width - pulse_width, 0.0, pulse_width, height),
                color,
            )?;
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
