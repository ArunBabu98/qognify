use eframe;

pub struct MyApp;

impl MyApp {
    pub fn new() -> Self {
        Self
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.label("Qognify");
        });
    }
}
