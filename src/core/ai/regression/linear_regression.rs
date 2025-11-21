/*
--------------------------------------------------------------------
                        Linear Regression
                        -----------------
Notes
-----

- 0 weights are fine for linear regression but not NN
- uses Kaiming/He uniform initialization for small random number initialization

--------------------------------------------------------------------
*/

use ndarray::Array2;
use ndarray_rand::RandomExt;
use rand::distr::Uniform;

pub struct LinearRegression {
    pub weights: Array2<f64>, // Values that the model learns
    pub bias: f64,
}

pub struct TrainingConfig {
    pub learning_rate: f64,
    pub epochs: usize,
    pub verbose: bool,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            epochs: 1000,
            verbose: false,
        }
    }
}

pub fn vprint(text: String, verbose: &bool) {
    if *verbose == true {
        tracing::debug!(text);
    }
}

impl LinearRegression {
    pub fn new() -> Self {
        Self {
            weights: Array2::zeros((0, 0)),
            bias: 0.0,
        }
    }

    pub fn train(&mut self, x_train: Array2<f64>, y_train: Array2<f64>, config: TrainingConfig) {
        let n = x_train.ncols();
        let limit = 1.0 / (n as f64).sqrt();
        let dist = Uniform::new(-limit, limit).unwrap(); // Kaiming/He uniform initialization
        self.weights = Array2::random((n, 1), dist);

        // Training loop
        for epoch in 0..config.epochs {
            vprint("Epoch: ".to_string() + &epoch.to_string(), &config.verbose);
            // Forward pass
            let y_hat = x_train.dot(&self.weights) + self.bias;
            // Loss calculation [MSE]
            if y_hat.shape() != y_train.shape() {
                tracing::error!("Shape mismatched!");
                return;
            };
            let rows = y_train.nrows();
            let residuals = &y_hat - &y_train;
            let squared_residuals = &residuals * &residuals;
            let mse = squared_residuals.sum() / (rows as f64);
            vprint("MSE: ".to_string() + &mse.to_string(), &config.verbose);
            // Gradient computation
            // partial derivative on weights = 1/m * X(transpose) * error
            // partial derivative on bias = 1/m * sum(error)
            let grad_w = x_train.t().dot(&residuals) / rows as f64;
            let grad_b = residuals.sum() / rows as f64;
            // update parameters
            self.weights = &self.weights - &(config.learning_rate * grad_w);
            self.bias = self.bias - config.learning_rate * grad_b;
        }
    }

    pub fn predict(&self, x_test: Array2<f64>) -> Array2<f64> {
        x_test.dot(&self.weights) + self.bias
    }
}
