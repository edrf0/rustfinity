pub struct Book {
    // 1. Define the fields of the struct
    // Make all of them public with `pub`
    // Read the description for the fields
    pub title: String,
    pub author: String,
    pub year: i32,
    pub likes: i32,
}

impl Book {
    // 2. Define the `new` associated function
    pub fn new(title: String, author: String, year: i32, likes: i32) -> Self {
        Self {
            title,
            author,
            year,
            likes,
        }
    }
}
