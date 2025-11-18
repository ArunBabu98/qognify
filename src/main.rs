// main.rs
mod app;

use tracing::info;
use tracing_subscriber;

use eframe;

use app::myapp::MyApp;

fn main() -> eframe::Result<()> {
    // Initializing tracing for logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    info!("Starting qognify...");

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_min_inner_size([1200.0, 700.0]),
        ..Default::default()
    };

    // Launch
    eframe::run_native(
        "Qognify - AI & Quantum Simulator",
        native_options,
        Box::new(|cc| {
            // Set custom theme on startup
            cc.egui_ctx.set_visuals(MyApp::custom_theme());
            Ok(Box::new(MyApp::new()))
        }),
    )
}
