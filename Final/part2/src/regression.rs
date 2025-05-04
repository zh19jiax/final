/// Module for performing linear regression analysis on freelancer data.
/// Implements a simple linear regression model to predict hourly rates based on various features.

use linfa::Dataset;
use linfa::traits::Fit;
use ndarray::{Array1, Array2, array};
use linfa_linear::LinearRegression;
use crate::data_loader::Freelancer;

/// Performs linear regression on freelancer data to predict hourly rates.
/// 
/// # Arguments: `freelancers` - Slice of Freelancer structs containing the training data
/// 
/// # Returns: `Result<(Array1<f64>, f64), Box<dyn Error>>` - Tuple containing:
///   - Coefficients for each feature
///   - Intercept term
/// 
/// # Features Used
/// 1. Job Success Rate (normalized to 0-1 range)
/// 2. Job Category (encoded as 1-5)
/// 3. Experience Level (encoded as 1-3)
pub fn perform_regression(freelancers: &[Freelancer]) -> Result<(Array1<f64>, f64), Box<dyn std::error::Error>> {
    // Prepare data structures for features and target
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    
    // Process each freelancer's data
    for freelancer in freelancers {
        // Convert job success rate from percentage (50-100) to 0-1 range
        let normalized_success = (freelancer.job_success_rate as f64) / 100.0;
        
        // Convert categorical variables to numerical values
        let job_category_value = match freelancer.job_category.as_str() {
            "Web Development" => 1.0,
            "Mobile Development" => 2.0,
            "Design" => 3.0,
            "Writing" => 4.0,
            "Data Science" => 5.0,
            _ => 0.0,
        };
        
        let experience_value = match freelancer.experience_level.as_str() {
            "Entry Level" => 1.0,
            "Intermediate" => 2.0,
            "Expert" => 3.0,
            _ => 0.0,
        };
        
        // Combine features into a single vector
        x_data.push(vec![
            normalized_success,
            job_category_value,
            experience_value,
        ]);
        y_data.push(freelancer.hourly_rate as f64);
    }
    
    // Convert data to ndarray format for the regression model
    let x: Array2<f64> = Array2::from_shape_vec((x_data.len(), 3), x_data.into_iter().flatten().collect())?;
    let y: Array1<f64> = Array1::from_vec(y_data);
    
    // Create and fit the regression model
    let dataset = Dataset::new(x.clone(), y.clone());
    let lin_reg = LinearRegression::new();
    let model = lin_reg.fit(&dataset)?;
    
    // Return only model parameters
    Ok((model.params().clone(), model.intercept()))
}


/// Creates a simple test dataset with two freelancers
fn create_test_freelancers() -> Vec<Freelancer> {
    vec![
        Freelancer {
            id: 1,
            job_category: "Web Development".to_string(),
            platform: "Upwork".to_string(),
            experience_level: "Expert".to_string(),
            client_region: "North America".to_string(),
            earnings_usd: 5000.0,
            hourly_rate: 50.0,
            job_success_rate: 95.0,
        },
        Freelancer {
            id: 2,
            job_category: "Design".to_string(),
            platform: "Fiverr".to_string(),
            experience_level: "Entry Level".to_string(),
            client_region: "Europe".to_string(),
            earnings_usd: 1000.0,
            hourly_rate: 20.0,
            job_success_rate: 75.0,
        },
    ]
}

/// Tests basic regression functionality
#[test]
fn test_basic_regression() {
    let freelancers = create_test_freelancers();
    let result = perform_regression(&freelancers);
    
    // Verify regression runs without error
    assert!(result.is_ok());
    
    let (coefficients, intercept) = result.unwrap();
    
    // Verify we get the expected number of coefficients
    assert_eq!(coefficients.len(), 3);
    
    // Verify coefficients and intercept are valid numbers
    assert!(intercept.is_finite());
    for &coef in coefficients.iter() {
        assert!(coef.is_finite());
    }
}