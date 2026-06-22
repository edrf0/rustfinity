pub fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    let sqrt_n = (n as f64).sqrt() as u32;
    for divisor in (3..=sqrt_n).step_by(2) {
        if n % divisor == 0 {
            return false;
        }
    }
    true
}
