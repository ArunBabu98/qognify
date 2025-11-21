pub mod linear_regression;

use linear_regression::{LinearRegression, TrainingConfig};
#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_linear_regression_train_and_predict() {
        // Example dataset: y = 2 * x1 + 3 * x2 + 5
        let x_train = array![[1.0, 2.0], [2.0, 0.5], [3.0, 1.0], [4.0, 2.0]];
        let y_train = array![
            [13.0], // 2*1 + 3*2 + 5 = 13
            [12.0], // 2*2 + 3*0.5 + 5 = 12
            [14.0], // 2*3 + 3*1 + 5 = 14
            [19.0]  // 2*4 + 3*2 + 5 = 19
        ];

        // Create LinearRegression model and config
        let mut model = LinearRegression::new();
        let config = TrainingConfig {
            learning_rate: 0.001,
            epochs: 100000,
            verbose: true,
        };

        // Train model
        model.train(x_train.clone(), y_train.clone(), config);

        // Test prediction on training data
        let y_pred = model.predict(x_train);

        // Check that prediction is close to true y (within a tolerance)
        for (predicted, actual) in y_pred.iter().zip(y_train.iter()) {
            let diff = (predicted - actual).abs();
            assert!(
                diff < 1e-0,
                "Prediction {} differs from actual {}",
                predicted,
                actual
            );
        }
    }
}
