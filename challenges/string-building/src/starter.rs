use std::fmt::{self, Display, Formatter, Write};

/// Build a greeting message using format!
pub fn build_greeting(name: &str, age: u32) -> String {
    format!("Hello, {name}! You are {age} years old.")
}

/// Build a numbered list from items using write!
pub fn build_list(items: &[&str]) -> String {
    let buffer = String::new();
    for (index,item) in items.iter().enumerate() {
        buffer.push_str(write!("{}. {}{}", index + 1, item, index + 1));
    }
    buffer
}

/// A person with a name and age.
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

// TODO: Implement Display for Person
// Format: "Name (Age years old)"
impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

/// Build a simple text table with headers and data rows.
///
/// The table format uses pipes and dashes:
/// ```text
/// | Name  | Age |
/// |-------|-----|
/// | Alice | 30  |
/// | Bob   | 25  |
/// ```
pub fn build_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    unimplemented!()
}

/// Concatenate strings with a separator without using .join()
pub fn concat_with_separator(parts: &[&str], sep: &str) -> String {
    unimplemented!()
}

pub fn main() {
    // Demonstrate build_greeting
    println!("=== build_greeting ===");
    println!("{}", build_greeting("Alice", 30));

    // Demonstrate build_list
    println!("\n=== build_list ===");
    println!("{}", build_list(&["apple", "banana", "cherry"]));

    // Demonstrate Person Display
    println!("\n=== Person Display ===");
    let person = Person {
        name: "Bob".to_string(),
        age: 25,
    };
    println!("{}", person);

    // Demonstrate build_table
    println!("\n=== build_table ===");
    let headers = &["Name", "Age"];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    println!("{}", build_table(headers, &rows));

    // Demonstrate concat_with_separator
    println!("\n=== concat_with_separator ===");
    println!("{}", concat_with_separator(&["a", "b", "c"], ", "));
}
