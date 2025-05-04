use linfa::Dataset;
use linfa::traits::Fit;
use ndarray::{Array1, Array2, array};
use linfa_linear::LinearRegression;
use linfa_linear::FittedLinearRegression;
use linfa::prelude::*;


fn main() {
    // Example data: 4 samples with 3 features each
    let x: Array2<f64> = array![[1.0, 2.0, 3.0],
                                //[2.0, 3.0, 1.0],
                                [2.0, 3.0, 4.0],
                                [3.0, 4.0, 5.0],
                                [4.0, 5.0, 6.0],
                                //[6.0, 11.0, 12.0]
                                ];
    // Target values
    let y: Array1<f64> = array![6.0, 
                                //6.0, 
                                9.0, 
                                12.0, 
                                15.0, 
                                //29.0,
        ];

    // Create dataset
    let dataset = Dataset::new(x.clone(), y.clone());

    // Fit linear regression model
    let lin_reg = LinearRegression::new();
    let model = lin_reg.fit(&dataset).unwrap();

    // Print coefficients
    println!("Coefficients: {:?}", model.params());
    println!("Intercept: {:?}", model.intercept());

    // Predict using the fitted model
    let predictions = model.predict(&x);
    println!("Predictions: {:?}", predictions);
}