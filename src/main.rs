use std::{num, ops::RangeInclusive};

use egui::containers;
use three_d::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    pollster::block_on(run());
}

fn reset_button<'a>(
    setting: &'a mut f32,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response + 'a {
    move |ui| {
        let mut reset_response = ui.add_enabled(
            *setting != 0.0,
            egui::Button::new("⟲"),
        );
        if reset_response.clicked() {
            *setting = 0.0;
            reset_response.mark_changed();
        }
        reset_response
    }
}

fn number_setting<'a>(
    name: &'a str,
    setting: &'a mut f32,
    range: RangeInclusive<f32>,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response + 'a {
    move |ui| {
        let mut changed_response = ui.horizontal(|ui| {
            ui.add(reset_button(setting)).changed()
                || ui
                    .add(egui::Slider::new(setting, range).text(name))
                    .changed()
        });
        if changed_response.inner {
            changed_response.response.mark_changed();
        }
        changed_response.response
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
    let mut slider_val = 0.0;

    window.render_loop(move |mut frame_input| {
        gui.update(&mut frame_input.events, frame_input.accumulated_time, frame_input.viewport, frame_input.device_pixel_ratio, |ctx| {
            egui::SidePanel::left("Main_left_panel").show(ctx, |ui| {
                ui.heading("My egui Application");
                let result = ui.add(number_setting("Broken setting", &mut slider_val, 0.0..=5.0));
                if result.contains_pointer() {
                    containers::show_tooltip_for(&ctx, result.id, &result.rect, |ui| ui.label("Test"));
                }
                ui.collapsing("This triggers the bug", |ui| {});
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
