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
        viewport: eframe::egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    // Launch
    eframe::run_native(
        "Qognify",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}
