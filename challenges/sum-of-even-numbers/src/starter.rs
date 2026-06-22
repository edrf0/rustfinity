pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    // Your code here...
    (start..=end).filter(|x| x % 2 == 0).sum()
}
