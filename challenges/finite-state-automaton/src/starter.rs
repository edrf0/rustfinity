pub fn recognize_pattern(input: &str) -> bool {
    if input.len() < 2 {
        return false;
    }
    if input.chars().nth(0).unwrap() != 'a' || input.chars().nth(input.len()-1).unwrap() != 'c' {
        return false;
    }
    if input == "ac" {
        return true;
    }
    for index in 1..(input.len() - 1) {
        if input.chars().nth(index).unwrap() != 'b' {
            return false;
        }
    }
    true
}
