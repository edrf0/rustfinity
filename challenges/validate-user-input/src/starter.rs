pub fn validate_user(age: i32, email: &str) -> Result<(), String> {
    if age < 0 || age > 120 {
        return Err(String::from("Invalid age"));
    }
    if !email.contains("@") {
        return Err(String::from("Invalid email"));
    }
    Ok(())
}
/*If the age is less than 0 or greater than 120, return an error with the message "Invalid age".
If the email does not contain an '@' symbol, return an error with the message "Invalid email".
If both the age and email are valid, return Ok(()).*/