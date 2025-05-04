use ndarray::{Array1, Array2, stack, Axis};
use ndarray_linalg::LeastSquaresSvd;
use crate::data_loader::Freelancer;

pub struct RegressionModel {
    pub coefficients: Array1<f64>,
    pub feature_names: Vec<String>,
}