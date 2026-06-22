pub fn fizz_buzz(num: u32) -> String {
    match num {
        n if n % 3 == 0 => format!("Fizz"),
        n if n % 5 == 0 => format!("Buzz"),
        n if n % 15 == 0 => format!("FizzBuzz"),
        n => format!("{}",n),
    }
}
