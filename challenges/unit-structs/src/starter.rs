// Define a struct named `Logger`
// Implement an associated function `log_message`
// That accepts a `&str` and prints the output.
struct Logger;

impl Logger {
    fn log_message(s: &str) {
        println!("{}", s);
    }
}

// Example usage:
pub fn main() {
    Logger::log_message("Hello, World!");
}
