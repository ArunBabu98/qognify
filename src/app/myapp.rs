use eframe::{self, egui::widgets};

pub struct MyApp {
    search_query: String,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            search_query: "Search".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::SidePanel::left("left_panel")
            .exact_width(150.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label("Search");
                ui.add_space(8.0);
                ui.text_edit_singleline(&mut self.search_query);
            });
        eframe::egui::CentralPanel::default().show(ctx, |ui| ui.label("Main Panel"));
    }
}
