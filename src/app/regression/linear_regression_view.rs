use crate::core::ai::regression::linear_regression::{LinearRegression, TrainingConfig};
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Points};
use ndarray::Array2;
use tracing_subscriber::registry::Data;

#[derive(Clone)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

pub struct LinearRegressionView {
    // Input data
    data_points: Vec<DataPoint>,
    input_x: String,
    input_y: String,
    // Model
    model: Option<LinearRegression>,
    is_trained: bool,
    // Training config
    learning_rate: f64,
    epochs: usize,
    // Model metrics
    mse: f64,
    rmse: f64,
    r_squared: f64,
    // Prediction
    pred_input: String,
    pred_output: String,
}

impl Default for LinearRegressionView {
    fn default() -> Self {
        Self {
            data_points: vec![
                DataPoint { x: 1.0, y: 2.5 },
                DataPoint { x: 2.0, y: 3.8 },
                DataPoint { x: 3.0, y: 5.2 },
                DataPoint { x: 4.0, y: 6.9 },
                DataPoint { x: 5.0, y: 8.1 },
            ],
            input_x: String::new(),
            input_y: String::new(),
            model: None,
            is_trained: false,
            learning_rate: 0.01,
            epochs: 1000,
            mse: 0.0,
            rmse: 0.0,
            r_squared: 0.0,
            pred_input: String::new(),
            pred_output: String::new(),
        }
    }
}

impl LinearRegressionView {
    pub fn new() -> Self {
        Self::default()
    }
   fn train_model(&mut self) {
        if self.data_points.len() < 2 {
            tracing::warn!("Need at least 2 data points to train");
            return;
        }
        
        let n = self.data_points.len();
        let mut x_train = Array2::zeros((n, 1));
        let mut y_train = Array2::zeros((n, 1));
        
        for (i, point) in self.data_points.iter().enumerate() {
            x_train[[i, 0]] = point.x;
            y_train[[i, 0]] = point.y;
        }
        
        let mut model = LinearRegression::new();
        let config = TrainingConfig {
            learning_rate: self.learning_rate,
            epochs: self.epochs,
            verbose: false,
        };
        
        model.train(x_train.clone(), y_train.clone(), config);
        
        // Calculate metrics
        let predictions = model.predict(x_train.clone());
        self.mse = self.calculate_mse(&y_train, &predictions);
        self.rmse = self.mse.sqrt();
        self.r_squared = self.calculate_r_squared(&y_train, &predictions);
        
        self.model = Some(model);
        self.is_trained = true;
    }
    
    fn calculate_mse(&self, y_true: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
        let n = y_true.nrows() as f64;
        let residuals = y_pred - y_true;
        let squared_residuals = &residuals * &residuals;
        squared_residuals.sum() / n
    }
    
    fn calculate_r_squared(&self, y_true: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
        let y_mean = y_true.sum() / y_true.nrows() as f64;
        let ss_tot: f64 = y_true.iter().map(|&y| (y - y_mean).powi(2)).sum();
        let ss_res: f64 = y_true.iter().zip(y_pred.iter())
            .map(|(&yt, &yp)| (yt - yp).powi(2)).sum();
        
        if ss_tot == 0.0 {
            return 0.0;
        }
        1.0 - (ss_res / ss_tot)
    }
    
    fn make_prediction(&mut self) {
        if let (Some(model), Ok(x_val)) = (&self.model, self.pred_input.parse::<f64>()) {
            let x_test = Array2::from_elem((1, 1), x_val);
            let prediction = model.predict(x_test);
            self.pred_output = format!("{:.4}", prediction[[0, 0]]);
        }
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(16.0);
            
            // Header
            ui.heading(egui::RichText::new("📊 Linear Regression")
                .color(egui::Color32::from_rgb(150, 170, 255))
                .size(24.0));
            
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Predict continuous values using linear relationships")
                .color(egui::Color32::from_rgb(160, 160, 180))
                .size(13.0));
            
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(16.0);
            
            // Two column layout
            ui.horizontal(|ui| {
                // Left panel - Controls
                ui.vertical(|ui| {
                    ui.set_width(350.0);
                    self.render_controls(ui);
                });
                
                ui.add_space(16.0);
                
                // Right panel - Visualization
                ui.vertical(|ui| {
                    self.render_plot(ui);
                });
            });
            
            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);
            
            // Model Information Section
            if self.is_trained {
                self.render_model_info(ui);
            }
        });
    }
    
    fn render_controls(&mut self, ui: &mut egui::Ui) {
        // Data Input Section
        ui.label(egui::RichText::new("📝 Data Points")
            .color(egui::Color32::from_rgb(120, 140, 180))
            .size(14.0)
            .strong());
        
        ui.add_space(8.0);
        
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(25, 25, 35))
            .rounding(6.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(egui::TextEdit::singleline(&mut self.input_x)
                        .desired_width(80.0)
                        .hint_text("e.g., 6.0"));
                    
                    ui.add_space(8.0);
                    
                    ui.label("Y:");
                    ui.add(egui::TextEdit::singleline(&mut self.input_y)
                        .desired_width(80.0)
                        .hint_text("e.g., 9.5"));
                });
                
                ui.add_space(8.0);
                
                if ui.button("➕ Add Point").clicked() {
                    if let (Ok(x), Ok(y)) = (self.input_x.parse::<f64>(), self.input_y.parse::<f64>()) {
                        self.data_points.push(DataPoint { x, y });
                        self.input_x.clear();
                        self.input_y.clear();
                        self.is_trained = false;
                    }
                }
            });
        
        ui.add_space(12.0);
        
        // Display current data points
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(25, 25, 35))
            .rounding(6.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Current Data:")
                    .color(egui::Color32::from_rgb(140, 160, 200))
                    .size(12.0));
                
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        let mut to_remove = None;
                        for (i, point) in self.data_points.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("({:.2}, {:.2})", point.x, point.y));
                                if ui.small_button("❌").clicked() {
                                    to_remove = Some(i);
                                }
                            });
                        }
                        if let Some(idx) = to_remove {
                            self.data_points.remove(idx);
                            self.is_trained = false;
                        }
                    });
                
                ui.add_space(8.0);
                
                if ui.button("🗑 Clear All").clicked() {
                    self.data_points.clear();
                    self.is_trained = false;
                }
            });
        
        ui.add_space(20.0);
        
        // Training Parameters
        ui.label(egui::RichText::new("⚙️ Training Parameters")
            .color(egui::Color32::from_rgb(120, 140, 180))
            .size(14.0)
            .strong());
        
        ui.add_space(8.0);
        
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(25, 25, 35))
            .rounding(6.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Learning Rate:");
                    ui.add(egui::DragValue::new(&mut self.learning_rate)
                        .speed(0.001)
                        .clamp_range(0.0001..=1.0));
                });
                
                ui.add_space(4.0);
                
                ui.horizontal(|ui| {
                    ui.label("Epochs:");
                    ui.add(egui::DragValue::new(&mut self.epochs)
                        .speed(100)
                        .clamp_range(100..=100000));
                });
            });
        
        ui.add_space(12.0);
        
        // Train Button
        let train_button = egui::Button::new(
            egui::RichText::new("🚀 Train Model")
                .size(14.0)
        )
        .min_size(egui::vec2(ui.available_width(), 36.0));
        
        if ui.add(train_button).clicked() {
            self.train_model();
        }
        
        ui.add_space(20.0);
        
        // Prediction Section
        if self.is_trained {
            ui.label(egui::RichText::new("🔮 Make Prediction")
                .color(egui::Color32::from_rgb(120, 140, 180))
                .size(14.0)
                .strong());
            
            ui.add_space(8.0);
            
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(25, 25, 35))
                .rounding(6.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(egui::TextEdit::singleline(&mut self.pred_input)
                            .desired_width(100.0));
                    });
                    
                    ui.add_space(8.0);
                    
                    if ui.button("Predict").clicked() {
                        self.make_prediction();
                    }
                    
                    if !self.pred_output.is_empty() {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(format!("Predicted Y: {}", self.pred_output))
                            .color(egui::Color32::from_rgb(100, 255, 150))
                            .size(13.0)
                            .strong());
                    }
                });
        }
    }
    
 fn render_plot(&mut self, ui: &mut egui::Ui) {
    let plot = Plot::new("linear_regression_plot")
        .view_aspect(1.5)
        .height(500.0)
        .legend(egui_plot::Legend::default());
    
    plot.show(ui, |plot_ui| {
        // Plot data points
        let points: PlotPoints = self.data_points
            .iter()
            .map(|p| [p.x, p.y])
            .collect();
        
        plot_ui.points(
            Points::new("Data Points", points)  // Changed: Added name as first argument
                .radius(5.0)
                .color(egui::Color32::from_rgb(100, 200, 255))
        );
        
        // Plot regression line if trained
        if let Some(model) = &self.model {
            if !self.data_points.is_empty() {
                let x_min = self.data_points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
                let x_max = self.data_points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
                let margin = (x_max - x_min) * 0.2;
                
                let line_points: PlotPoints = (0..100)
                    .map(|i| {
                        let x = x_min - margin + (x_max - x_min + 2.0 * margin) * i as f64 / 99.0;
                        let x_array = Array2::from_elem((1, 1), x);
                        let y = model.predict(x_array)[[0, 0]];
                        [x, y]
                    })
                    .collect();
                
                plot_ui.line(
                    Line::new("Regression Line", line_points)  // Changed: Added name as first argument
                        .color(egui::Color32::from_rgb(255, 100, 150))
                        .width(2.0)
                );
            }
        }
    });
}

    
    fn render_model_info(&self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("📈 Model Information")
            .color(egui::Color32::from_rgb(120, 140, 180))
            .size(16.0)
            .strong());
        
        ui.add_space(12.0);
        
        // Model equation
        if let Some(model) = &self.model {
            let weight = model.weights[[0, 0]];
            let bias = model.bias;
            
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(25, 25, 35))
                .rounding(6.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Learned Equation:")
                        .color(egui::Color32::from_rgb(140, 160, 200))
                        .size(13.0));
                    
                    ui.add_space(4.0);
                    
                    ui.label(egui::RichText::new(format!("y = {:.4}x + {:.4}", weight, bias))
                        .color(egui::Color32::from_rgb(255, 200, 100))
                        .size(16.0)
                        .strong()
                        .code());
                });
            
            ui.add_space(12.0);
        }
        
        // Metrics in grid layout
        ui.horizontal(|ui| {
            self.render_metric_card(ui, "MSE", self.mse, "Mean Squared Error");
            ui.add_space(12.0);
            self.render_metric_card(ui, "RMSE", self.rmse, "Root Mean Squared Error");
            ui.add_space(12.0);
            self.render_metric_card(ui, "R²", self.r_squared, "Coefficient of Determination");
        });
        
        ui.add_space(16.0);
        
        // Interpretation
        self.render_interpretation(ui);
    }
    
    fn render_metric_card(&self, ui: &mut egui::Ui, name: &str, value: f64, tooltip: &str) {
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(30, 30, 42))
            .rounding(6.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.set_min_width(150.0);
                
                ui.label(egui::RichText::new(name)
                    .color(egui::Color32::from_rgb(140, 160, 200))
                    .size(12.0));
                
                ui.add_space(4.0);
                
                let color = if name == "R²" {
                    if value > 0.8 { egui::Color32::from_rgb(100, 255, 150) }
                    else if value > 0.5 { egui::Color32::from_rgb(255, 200, 100) }
                    else { egui::Color32::from_rgb(255, 100, 100) }
                } else {
                    egui::Color32::from_rgb(100, 200, 255)
                };
                
                ui.label(egui::RichText::new(format!("{:.4}", value))
                    .color(color)
                    .size(18.0)
                    .strong());
            })
            .response.on_hover_text(tooltip);
    }
    
    fn render_interpretation(&self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(25, 25, 35))
            .rounding(6.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("📖 Understanding the Metrics")
                    .color(egui::Color32::from_rgb(140, 160, 200))
                    .size(13.0)
                    .strong());
                
                ui.add_space(8.0);
                
                ui.label(format!("• MSE (Mean Squared Error): {:.4} - Average squared difference between predicted and actual values. Lower is better.", self.mse));
                ui.add_space(4.0);
                
                ui.label(format!("• RMSE (Root Mean Squared Error): {:.4} - Standard deviation of prediction errors. Same units as target variable.", self.rmse));
                ui.add_space(4.0);
                
                let r2_interpretation = if self.r_squared > 0.8 {
                    "Excellent fit! Model explains >80% of variance."
                } else if self.r_squared > 0.5 {
                    "Moderate fit. Model explains >50% of variance."
                } else if self.r_squared > 0.0 {
                    "Weak fit. Model explains <50% of variance."
                } else {
                    "Poor fit. Model performs worse than baseline."
                };
                
                ui.label(format!("• R² Score: {:.4} - {} Ranges from 0 to 1 (higher is better).", self.r_squared, r2_interpretation));
            });
    }
}
