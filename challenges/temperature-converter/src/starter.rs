pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    match (from_unit, to_unit) {
        ("C","F") => value * (9.0 / 5.0) + 32.0,
        ("F","C") => (value - 32.0) * (5.0 / 9.0),
        ("C","K") => value + 273.15,
        ("K","C") => value - 273.15,
        _ => Err(format!("Invalid unit")),
    }
}
