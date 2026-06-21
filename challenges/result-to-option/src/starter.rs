pub fn read_file(file_path: &str) -> Option<String> {
    // TODO: Implement this function
    // Hint: Use `File::open` and `.read_to_string()` with `?` to propagate errors.
    let file_handle = File::open(file_path)?;
    let reader = BufReader::new(file_handle);
    let mut file_contents = String::new();
    reader.read_to_string(&mut file_contents)?;
    Some(file_contents)
}

// Example usage
pub fn main() {
    let file_path = "example.txt";

    match read_file(file_path) {
        Some(contents) => println!("File contents:\n{}", contents),
        None => println!("Failed to read the file."),
    }
}
