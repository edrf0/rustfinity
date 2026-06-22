pub fn describe_number(n: i32) -> String {
    match n {
        n if n > 0 && n % 2 == 0 => format!("Positive even"),
        n if n < 0 && n % 2 == 0 => format!("Negative even"),
        n if n > 0 && n % 2 != 0 => format!("Positive odd"),
        n if n < 0 && n % 2 != 0 => format!("Negative odd"),
        0 => format!("Zero"),
        _ => format!("_"),
    }
}
