// app/myapp.rs
use eframe::{self, egui};
use crate::app::regression::linear_regression_view::LinearRegressionView;

#[derive(Clone, Debug)]
pub struct MenuItem {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct Category {
    pub name: String,
    pub items: Vec<MenuItem>,
    pub is_open: bool,
}

pub struct MyApp {
    search_query: String,
    categories: Vec<Category>,
    filtered_categories: Vec<Category>,
     current_view: Option<String>,  
    lr_view: LinearRegressionView,  
}

impl MyApp {
    pub fn new() -> Self {
        let categories = vec![
            Category {
                name: "AI".to_string(),
                is_open: true,
                items: vec![
                    MenuItem {
                        title: "Linear Regression".to_string(),
                        description: "Visualize Linear Regression".to_string(),
                    },
                    // MenuItem {
                    //     title: "Neural Networks".to_string(),
                    //     description: "Visualize neural network architectures".to_string(),
                    // },
                    // MenuItem {
                    //     title: "Embeddings".to_string(),
                    //     description: "Vector space representations".to_string(),
                    // },
                    // MenuItem {
                    //     title: "Activation Functions".to_string(),
                    //     description: "Explore ReLU, Sigmoid, Tanh".to_string(),
                    // },
                    // MenuItem {
                    //     title: "Gradient Descent".to_string(),
                    //     description: "Optimization visualization".to_string(),
                    // },
                    // MenuItem {
                    //     title: "Attention Mechanisms".to_string(),
                    //     description: "Transformer attention patterns".to_string(),
                    // },
                ],
            },
            Category {
                name: "Quantum".to_string(),
                is_open: true,
                items: vec![
                    MenuItem {
                        title: "Quantum States".to_string(),
                        description: "Superposition and measurement".to_string(),
                    },
                    // MenuItem {
                    //     title: "Quantum Gates".to_string(),
                    //     description: "Pauli, Hadamard, CNOT gates".to_string(),
                    // },
                    // MenuItem {
                    //     title: "Quantum Circuits".to_string(),
                    //     description: "Build and simulate circuits".to_string(),
                    // },
                    // MenuItem {
                    //     title: "Entanglement".to_string(),
                    //     description: "Bell states and correlations".to_string(),
                    // },
                    // MenuItem {
                    //     title: "Bloch Sphere".to_string(),
                    //     description: "Qubit state visualization".to_string(),
                    // },
                ],
            },
            Category {
                name: "Hybrid".to_string(),
                is_open: true,
                items: vec![
                    MenuItem {
                        title: "Quantum Neural Networks".to_string(),
                        description: "QNN architectures".to_string(),
                    },
                    MenuItem {
                        title: "Variational Quantum Eigensolver".to_string(),
                        description: "VQE optimization".to_string(),
                    },
                    MenuItem {
                        title: "Quantum Approximate Optimization".to_string(),
                        description: "QAOA algorithm".to_string(),
                    },
                    MenuItem {
                        title: "Quantum Feature Maps".to_string(),
                        description: "Classical to quantum encoding".to_string(),
                    },
                    MenuItem {
                        title: "Hybrid Inference".to_string(),
                        description: "Classical-Quantum pipelines".to_string(),
                    },
                ],
            },
        ];

        let filtered_categories = categories.clone();

        Self {
            search_query: String::new(),
            categories,
            filtered_categories,
             current_view: None, 
            lr_view: LinearRegressionView::new(),  
        }
    }

    pub fn custom_theme() -> egui::Visuals {
        let mut visuals = egui::Visuals::dark();
        
        // Modern simulator color scheme - deep space theme
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(18, 18, 24);
        visuals.widgets.noninteractive.weak_bg_fill = egui::Color32::from_rgb(25, 25, 35);
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 45, 60));
        
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 30, 42);
        visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(35, 35, 48);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 50, 70));
        
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 65);
        visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(50, 50, 75);
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.5, egui::Color32::from_rgb(100, 120, 255));
        
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(55, 55, 80);
        visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(60, 60, 90);
        visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(120, 140, 255));
        
        // Text colors
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 220));
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 200));
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 220, 255));
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(240, 240, 255));
        
        // Window and panel styling
        visuals.window_fill = egui::Color32::from_rgb(20, 20, 28);
        visuals.panel_fill = egui::Color32::from_rgb(18, 18, 24);
        visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 50, 70));
        
        // Selection and highlight colors - quantum-inspired cyan/purple
        visuals.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(100, 120, 255, 80);
        visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(120, 140, 255));
        
        // Extreme background (behind panels)
        visuals.extreme_bg_color = egui::Color32::from_rgb(12, 12, 16);
        
        // Hyperlink color - bright cyan
        visuals.hyperlink_color = egui::Color32::from_rgb(100, 200, 255);
        
        // Window rounding for modern look (use window_rounding field)
        visuals.window_corner_radius = egui::CornerRadius::same(8);
        
        // Widget rounding (these are methods that return new WidgetVisuals, not fields)
        visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(6);
        visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(6);
        visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(6);
        visuals.widgets.active.corner_radius = egui::CornerRadius::same(6);
        
        visuals
    }

    fn filter_categories(&mut self) {
        let query = self.search_query.trim().to_lowercase();
        
        if query.is_empty() {
            self.filtered_categories = self.categories.clone();
        } else {
            self.filtered_categories = self.categories
                .iter()
                .filter_map(|category| {
                    let filtered_items: Vec<MenuItem> = category
                        .items
                        .iter()
                        .filter(|item| item.title.to_lowercase().contains(&query))
                        .cloned()
                        .collect();
                    
                    if !filtered_items.is_empty() {
                        Some(Category {
                            name: category.name.clone(),
                            items: filtered_items,
                            is_open: true, // Auto-expand when filtering
                        })
                    } else {
                        None
                    }
                })
                .collect();
        }
    }

    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(12.0);
        
        // Title
        ui.vertical_centered(|ui| {
            ui.heading(egui::RichText::new("🔮 QOGNIFY")
                .color(egui::Color32::from_rgb(150, 170, 255))
                .size(20.0)
                .strong());
        });
        
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(12.0);
        
        // Search section
        ui.label(egui::RichText::new("🔍 SEARCH")
            .color(egui::Color32::from_rgb(120, 140, 180))
            .size(11.0)
            .strong());
        
        ui.add_space(6.0);
        
        let search_response = ui.add_sized(
            [ui.available_width(), 32.0],
            egui::TextEdit::singleline(&mut self.search_query)
                .hint_text("Type to search...")
                .font(egui::TextStyle::Body)
        );
        
        if search_response.changed() {
            self.filter_categories();
        }
        
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(12.0);
        
        // Categories list
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Clone to avoid borrow issues
                let categories_clone = self.filtered_categories.clone();
                
                for (category_idx, category) in categories_clone.iter().enumerate() {
                    self.render_category(ui, category_idx, category);
                    ui.add_space(8.0);
                }
                
                if self.filtered_categories.is_empty() {
                    ui.add_space(20.0);
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("No results found")
                            .color(egui::Color32::from_rgb(120, 120, 140))
                            .italics());
                    });
                }
            });
    }

    fn render_category(&mut self, ui: &mut egui::Ui, category_idx: usize, category: &Category) {
        let category_name = category.name.clone();
        let is_open = category.is_open;
        
        let header_response = ui.horizontal(|ui| {
            let icon = if is_open { "▼" } else { "▶" };
            
            let emoji = match category_name.as_str() {
                "AI" => "🤖",
                "Quantum" => "⚛️",
                "Hybrid" => "🔮",
                _ => "📁",
            };
            
            let header_text = format!("{} {} {}", icon, emoji, category_name);
            
            let label = egui::Label::new(
                egui::RichText::new(header_text)
                    .color(egui::Color32::from_rgb(180, 190, 230))
                    .size(14.0)
                    .strong()
            ).sense(egui::Sense::click());
            
            let response = ui.add(label);
            
            if response.clicked() {
                self.filtered_categories[category_idx].is_open = !is_open;
            }
            
            response
        });
        
        if header_response.inner.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        
        if is_open {
            ui.add_space(4.0);
            
            for item in &category.items {
                self.render_menu_item(ui, item);
            }
        }
    }

    fn render_menu_item(&mut self, ui: &mut egui::Ui, item: &MenuItem) {
        let item_response = ui.horizontal(|ui| {
            ui.add_space(12.0);
            
            let label = egui::Label::new(
                egui::RichText::new(&item.title)
                    .color(egui::Color32::from_rgb(200, 200, 220))
                    .size(13.0)
            ).sense(egui::Sense::click());
            
            ui.add(label)
        });
        
        let response = item_response.inner;
        
        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        
        // Clone response for hover text (consumes self)
        let response_for_hover = response.clone();
        response_for_hover.on_hover_text(&item.description);
        
        if response.clicked() {
             self.current_view = Some(item.title.clone());
            tracing::info!("Clicked on: {}", item.title);
        }
        
        ui.add_space(2.0);
    }
    
    fn render_main_panel(&mut self, ui: &mut egui::Ui) {
        match &self.current_view {
            Some(view) if view == "Linear Regression" => {
                self.lr_view.render(ui);
            },
            _ => {
                // Existing welcome screen code
                self.render_welcome_screen(ui);
            }
        }
    }

    fn render_welcome_screen(&mut self, ui: &mut egui::Ui) {
        // Center content
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 3.0);
            
            // Logo/Icon area
            ui.label(
                egui::RichText::new("🔮⚛️🤖")
                    .size(80.0)
            );
            
            ui.add_space(24.0);
            
            // Title
            ui.label(
                egui::RichText::new("Qognify Simulator")
                    .color(egui::Color32::from_rgb(150, 170, 255))
                    .size(32.0)
                    .strong()
            );
            
            ui.add_space(12.0);
            
            // Subtitle
            ui.label(
                egui::RichText::new("AI + Quantum Computing Visualization")
                    .color(egui::Color32::from_rgb(140, 160, 200))
                    .size(16.0)
            );
            
            ui.add_space(32.0);
            
            // Description
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 500.0) / 2.0);
                ui.vertical(|ui| {
                    ui.set_max_width(500.0);
                    ui.label(
                        egui::RichText::new("Select a visualization from the sidebar to begin exploring AI and Quantum Computing concepts in real-time.")
                            .color(egui::Color32::from_rgb(160, 160, 180))
                            .size(14.0)
                    );
                });
            });
            
            ui.add_space(40.0);
            
            // Status indicator
            ui.horizontal(|ui| {
                let circle = egui::Shape::circle_filled(
                    ui.cursor().center(),
                    4.0,
                    egui::Color32::from_rgb(100, 255, 150),
                );
                ui.painter().add(circle);
                
                ui.add_space(8.0);
                
                ui.label(
                    egui::RichText::new("System Ready")
                        .color(egui::Color32::from_rgb(100, 255, 150))
                        .size(12.0)
                );
            });
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Left sidebar panel
        eframe::egui::SidePanel::left("left_panel")
            .exact_width(280.0)
            .resizable(false)
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });
        
        // Main central panel
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_panel(ui);
        });
    }
}
