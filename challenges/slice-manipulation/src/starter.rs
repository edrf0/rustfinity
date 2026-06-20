pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    // Implement your logic here
    for index in indices {
        slice[*index] = value;
    }
}
