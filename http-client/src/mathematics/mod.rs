pub fn derivative(f: fn(f64) -> f64, x: f64, eps: f64) -> f64 {
    // Estimate derivative using central difference
    let dy = (f(x + eps) - f(x - eps)) / (2.0 * eps);

    dy
}
