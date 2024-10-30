use std::ops::RangeInclusive;

use egui::containers;
use three_d::*;
use lens_rs::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    pollster::block_on(run());
}

fn reset_button<'a, Path: Copy + 'a, S: Lens<Path, impl PartialEq> + Default>(
    settings: &'a mut S,
    path: Path,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response + 'a {
    move |ui| {
        let mut reset_response = ui.add_enabled(
            *settings.view_mut(path) != S::default().view(path),
            egui::Button::new("⟲"),
        );
        if reset_response.clicked() {
            *settings.view_mut(path) = S::default().view(path);
            reset_response.mark_changed();
        }
        reset_response
    }
}

fn number_setting<'a, Path: Copy + 'a, F: egui::emath::Numeric + PartialEq>(
    name: &'a str,
    settings: &'a mut (impl Lens<Path, F> + Default),
    path: Path,
    range: RangeInclusive<F>,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response + 'a {
    move |ui| {
        let mut changed_response = ui.horizontal(|ui| {
            ui.add(reset_button(settings, path)).changed()
                || ui
                    .add(egui::Slider::new(settings.view_mut(path), range).text(name))
                    .changed()
        });
        if changed_response.inner {
            changed_response.response.mark_changed();
        }
        changed_response.response
    }
}

#[derive(Debug, Lens)]
struct Settings {
    #[optic]
    slider_val: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { slider_val: 0.0 }
    }
}

async fn run() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let window = Window::new(WindowSettings {
        title: "Egui bug demo?".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut gui = GUI::new(&context);

    // Our application state:
    let mut inspector_visible = false;
    let mut settings = Settings::default();

    window.render_loop(move |mut frame_input| {
        gui.update(&mut frame_input.events, frame_input.accumulated_time, frame_input.viewport, frame_input.device_pixel_ratio, |ctx| {
            egui::SidePanel::left("Test").show(ctx, |ui| {
                ui.heading("My egui Application");
                let result = ui.add(number_setting("Broken setting", &mut settings, optics!(slider_val), 0.0..=5.0));
                if result.contains_pointer() {
                    containers::show_tooltip_for(&ctx, result.id, &result.rect, |ui| ui.label("Test"));
                }
                ui.collapsing("This triggers the bug", |_ui| {});
                // ui.checkbox(&mut inspector_visible, "Show inspector");
                // ui.label("However, the hover tooltip only shows up when over this label");
            });

            egui::Window::new("🔍 Inspection")
                .open(&mut inspector_visible)
                .vscroll(true)
                .show(ctx, |ui| {
                    ctx.inspection_ui(ui);
                });
        });

        // Get the screen render target to be able to render something on the screen
        let screen = frame_input.screen();

        // Render the GUI to the screen render target
        screen.write(|| gui.render()).unwrap();

        FrameOutput::default()
    });
}
