/*The MutableTextFinder struct should provide the following functionality:

new: Creates a new instance of MutableTextFinder with the given content.
find_first: Searches for the first line containing a given keyword and returns it as an
immutable reference (Option<&str>).
replace_lines: Replaces all lines containing a given keyword with a replacement string.
get_text: Returns the reference to the content.
Searches should be case-sensitive.*/

// 1. Finish the struct definition
pub struct MutableTextFinder {
    text: String,
}

// 2. Implement the methods for the struct
impl MutableTextFinder {
    pub fn new(s: &'a str) -> Self {
        Self {
            text: s.to_string(),
        }
    }
    pub fn find_first(&self, keyword: &str) -> Option<&String> {
        match self.text.find(keyword) {
            Some(idx) => Some(&self.text[idx..]),
            None => None,
        }
    }
    pub fn replace_lines(&mut self, keyword: &'a str, line: &'a str) {
        loop {
            match self.text.find(keyword) {
                Some(idx) => self.text.replace_range(idx.., line),
                None => break,
            }
        }
    }
    pub fn get_text(&self) -> &String {
        &self.text
    }
}
// Example usage
pub fn main() {
    let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
    let mut finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is awesome")

    finder.replace_lines("Rust", "Programming in Rust");
    println!("{}", finder.get_text()); // Should print the modified text
}
