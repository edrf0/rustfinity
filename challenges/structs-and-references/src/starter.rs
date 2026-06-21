/*Create a struct called TextFinder that holds a reference to a string slice.
The struct should have a constructor new() that takes a string slice and returns a TextFinder instance.
The struct should have a method called find_first that returns the first line containing the keyword,
or None if no match is found.
The struct should have a method called find_many that returns a vector of all lines containing the keyword.
The search functionality should be case-sensitive.
Ensure you return references to the original string slice rather than creating new owned strings.*/

// 1. Define the struct
pub struct TextFinder<'a> {
    text: &'a str,
}

// 2. Implement the struct and define the methods
impl<'a> TextFinder<'a> {
    pub fn new(text: &'a str) -> Self<'a> {
        Self { text }
    }
    pub fn find_first(&self,text: &'a str) -> Option<&'a str> {
        match self.text.find(text) {
            None => None,
            Some(i) => Some(self.text[i..].trim()),
        }
    }
    pub fn find_many(&self,text: Vec<&'a str>) -> Vec<&'a str> {
        let mut result = Vec::new();
        loop {
            match self.text.find(text) {
                None => break,
                Some(i) => result.push(self.text[i..].trim()),
            }
        }
        result
    }
}
// Example usage
pub fn main() {
    let text = "Rust is fast and memory-efficient.\nOwnership is key to Rust's safety.\nRustaceans love the borrow checker.";
    let finder = TextFinder::new(text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is fast and memory-efficient.")

    let matches = finder.find_many("Rust");
    println!("{:?}", matches); // Should print: ["Rust is fast and memory-efficient.", "Ownership is key to Rust's safety."]
}
