use ndarray::{Array1, Array2};
use crate::data_loader::Freelancer;

pub fn analyze_errors(actual: &[f64], predicted: &[f64]) -> Result<(), Box<dyn std::error::Error>> {
    // Calculate Mean Squared Error (MSE)
    let mse = calculate_mse(actual, predicted);
    
    // Calculate Root Mean Squared Error (RMSE)
    let rmse = mse.sqrt();
    
    // Calculate Mean Absolute Error (MAE)
    let mae = calculate_mae(actual, predicted);
    
    // Calculate R-squared
    let r_squared = calculate_r_squared(actual, predicted);
    
    // Print results
    println!("\nError Analysis:");
    println!("Mean Squared Error (MSE): {:.2}", mse);
    println!("Root Mean Squared Error (RMSE): {:.2}", rmse);
    println!("Mean Absolute Error (MAE): {:.2}", mae);
    println!("R-squared: {:.4}", r_squared);
    
    // Print some sample predictions vs actual
    println!("\nSample Predictions vs Actual:");
    for i in 0..std::cmp::min(5, actual.len()) {
        println!("Sample {}: Predicted ${:.2}/hr, Actual ${:.2}/hr, Error: ${:.2}/hr", 
            i + 1, 
            predicted[i], 
            actual[i],
            predicted[i] - actual[i]
        );
    }
    
    Ok(())
}

fn calculate_mse(actual: &[f64], predicted: &[f64]) -> f64 {
    actual.iter()
        .zip(predicted.iter())
        .map(|(a, p)| (p - a).powi(2))
        .sum::<f64>() / actual.len() as f64
}

fn calculate_mae(actual: &[f64], predicted: &[f64]) -> f64 {
    actual.iter()
        .zip(predicted.iter())
        .map(|(a, p)| (p - a).abs())
        .sum::<f64>() / actual.len() as f64
}

fn calculate_r_squared(actual: &[f64], predicted: &[f64]) -> f64 {
    let mean_actual = actual.iter().sum::<f64>() / actual.len() as f64;
    
    let total_sum_squares: f64 = actual.iter()
        .map(|a| (a - mean_actual).powi(2))
        .sum();
    
    let residual_sum_squares: f64 = actual.iter()
        .zip(predicted.iter())
        .map(|(a, p)| (a - p).powi(2))
        .sum();
    
    1.0 - (residual_sum_squares / total_sum_squares)
} 