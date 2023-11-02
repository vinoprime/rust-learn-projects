pub fn calculate_parallel_resistance(resistors: &[f64]) -> f64 {
    let mut total_resistance: f64 = 0.0;
    let mut product_of_reciprocals: f64 = 0.0;

    for r in resistors {
        product_of_reciprocals += 1.0 / *r;
    }

    total_resistance = 1.0 / product_of_reciprocals;

    total_resistance
}
