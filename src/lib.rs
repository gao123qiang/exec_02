mod vector;
mod matrix;
mod metrics;
mod ametics;

pub use vector::{Vector, dot_product};
pub use matrix::{multiply, Matrix};
pub use metrics::Metrics;
pub use ametics::AmapMetrics;