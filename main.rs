mod app;
mod config;
mod diagnostics;
mod sandbox;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([980.0, 680.0])
            .with_min_inner_size([760.0, 520.0])
            .with_title("NIGHTFALL | Local Client"),
        ..Default::default()
    };

    eframe::run_native(
        "NIGHTFALL",
        options,
        Box::new(|cc| {
            app::configure_style(&cc.egui_ctx);
            Ok(Box::new(app::NightfallApp::default()))
        }),
    )
}
