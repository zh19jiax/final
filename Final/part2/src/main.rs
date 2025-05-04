/// Main module for the freelancer hourly rate prediction system.

mod data_loader;
mod regression;

use data_loader::{Freelancer, load_freelancers};
use regression::perform_regression;
use ndarray::array;

/// Main function that demonstrates the data loading, model training, and prediction demonstration.
/// 1. Loads freelancer data from CSV
/// 2. Trains a linear regression model
/// 3. Displays model parameters and example predictions
/// 
/// # Features Used
/// - Job Success Rate (normalized to 0-1 range)
/// - Job Category (encoded as 1-5)
/// - Experience Level (encoded as 1-3)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the freelancer data
    let freelancers = load_freelancers("freelancer_data.csv")?;
    
    // Perform regression analysis
    let (coefficients, intercept) = perform_regression(&freelancers)?;
    
    // Print model results
    println!("Model Results:");
    println!("Intercept: {:.2}", intercept);
    println!("\nCoefficients:");
    println!("Job Success Rate (0-1): {:.2}", coefficients[0]);
    println!("Job Category (1-5): {:.2}", coefficients[1]);
    println!("Experience Level (1-3): {:.2}", coefficients[2]);
    
    // Simple example predictions
    println!("\nExample Predictions:");
    
    // Example 1: Expert Web Developer
    let expert = array![[0.95, 1.0, 3.0]];  // 95% success, Web Dev, Expert
    let pred_expert = intercept + expert.dot(&coefficients);
    println!("Expert Web Developer: ${:.2}/hr", pred_expert);

    // Example 2: Entry Level Designer
    let entry = array![[0.75, 3.0, 1.0]];  // 75% success, Design, Entry Level
    let pred_entry = intercept + entry.dot(&coefficients);
    println!("Entry Level Designer: ${:.2}/hr", pred_entry);

    Ok(())
}