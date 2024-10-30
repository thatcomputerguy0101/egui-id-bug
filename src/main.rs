use three_d::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    pollster::block_on(run());
}

fn main_panel<'a>(inspector_visible: &'a mut bool) -> impl FnMut(&mut egui::Ui) -> () + 'a {
    move |ui| {
        let response = ui
            .horizontal(|ui| {
                ui.label("This label has the tooltip, but does not show it when hovered over");
            })
            .response;

        if response.contains_pointer() {
            response.show_tooltip_text("Test");
        }

        ui.checkbox(inspector_visible, "Show Inpsector");

        ui.horizontal(|ui| ui.label("Hovering over this label shows the tooltip"));
        // ui.collapsing(
        //     "Using this instead of horizontal also triggers the problem",
        //     |_| (),
        // );
    }
}

async fn run() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let window = Window::new(WindowSettings {
        title: "Egui id bug demo".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut inspector_visible = false;

    let mut gui = GUI::new(&context);

    // Start the main render loop
    window.render_loop(
        move |mut frame_input| // Begin a new frame with an updated frame input
        {
            let mut panel_width = 0.0;
            gui.update(
                &mut frame_input.events,
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |gui_context| {
                    use three_d::egui::*;
                    let panel_response = CentralPanel::default().show(gui_context, main_panel(&mut inspector_visible));
                    panel_width = panel_response.response.rect.width();

                    egui::Window::new("🔍 Inspection")
                        .open(&mut inspector_visible)
                        .vscroll(true)
                        .show(gui_context, |ui| {
                            gui_context.inspection_ui(ui);
                        });
                },
            );

            // Get the screen render target to be able to render something on the screen
            let screen = frame_input.screen();

            // Render the GUI to the screen render target
            screen.write(|| gui.render()).unwrap();

            FrameOutput::default()
        }
    );
}
