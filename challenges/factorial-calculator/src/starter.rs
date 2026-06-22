pub fn factorial(n: u32) -> u128 {
    if n == 0 {
        return 1;
    }
    (1..=n).fold(1u128, |acc, x| acc * (x as u128))
}
