use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut inspector_visible = false;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.label("This label has a hover tooltip.")
                .on_hover_text("Hello World!");
            ui.checkbox(&mut inspector_visible, "Show inspector");
            ui.label("However, the hover tooltip only shows up when over this label");
        });

        egui::Window::new("🔍 Inspection")
            .open(&mut inspector_visible)
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.inspection_ui(ui);
            });
    })
}
