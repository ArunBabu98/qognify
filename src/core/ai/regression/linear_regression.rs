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

impl LinearRegression {
    pub fn new() -> Self {
        Self {
            weights: Array2::zeros((0, 0)),
            bias: 0.0,
        }
    }

    pub fn train(&mut self, x_train: Array2<f64>, y_train: Array2<f64>) {
        let n = x_train.ncols();
        let limit = 1.0 / (n as f64).sqrt();
        let dist = Uniform::new(-limit, limit).unwrap(); // Kaiming/He uniform initialization
        self.weights = Array2::random((n, 1), dist);
    }
}
