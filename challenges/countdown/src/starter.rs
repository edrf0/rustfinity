pub fn countdown(n: u32) -> Vec<u32> {
    let vec = Vec::with_capacity(n+1);
    loop {
        vec.push(n);
        if n == 0 {break}
        n -= 1;
    }
    vec
}
