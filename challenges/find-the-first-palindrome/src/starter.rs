pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
    if start > end {
        let temp = start;
        start = end;
        end = temp;
    }
    if end < 0 {
        return None;
    }
    let is_palindrome;
    for number in start..=end {
        is_palindrome = true;
        let number_str: String = number.to_string();
        for index in 0..(number_str.len() / 2) {
            if number_str.chars().nth(index).unwrap() !=
                number_str.chars().nth(number_str.len() - 1 - index).unwrap() {
                is_palindrome = false;
                break;
            }
        }
        if is_palindrome {
            return Some(number);
        }
    }
    None
}
