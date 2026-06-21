/*Create TempFile struct with a method new that takes a file name as input, it must accept both &str
and String types.
Implement the Drop trait for TempFile to delete the file automatically when the struct goes out of scope.*/

use std::path::PathBuf;

pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    // 1. Define the `new` associated function
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Your code here to delete the file when TempFile is dropped
        fs::remove_file(&self.path)
    }
}

// Example usage
pub fn main() {
    let file_path = PathBuf::from("example_temp_file.tmp");
    let tempfile =
        TempFile::new(file_path.to_str().unwrap()).expect("Failed to create temporary file");

    assert!(tempfile.path.exists(), "File does not exist");

    drop(tempfile);

    assert!(!file_path.exists(), "File was not deleted");

    let tempfile_2 = TempFile::new(&String::from("example_temp_file_2.tmp"))
        .expect("Failed to create temporary file");

    drop(tempfile_2);
}
