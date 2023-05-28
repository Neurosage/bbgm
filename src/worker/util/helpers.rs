use crate::common::helpers as common_helpers;

pub fn bound(x: f64, min: f64, max: f64) -> f64 {
    return common_helpers::bound(x, min, max);
}

pub fn sigmoid(x: f64, a: f64, b: f64) -> f64 {
    return 1.0 / (1.0 + (-(a * (x-b))).exp());
}