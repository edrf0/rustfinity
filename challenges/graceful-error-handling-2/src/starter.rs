/*

Update the parse_percentage function to:

Return Ok(u8) if the input is a valid percentage (between 0 and 100).
Return Err(ParsePercentageError::OutOfRange) if the number is out of range.
Return Err(ParsePercentageError::InvalidInput) if the input is not a valid number.*/

// 1. Finish the definition
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

impl Error for ParsePercentageError {
    fn description(&self) -> &str {
        match *self {
            ParsePercentageError::InvalidInput => "Invalid input",
            ParsePercentageError::OutOfRange => "Out of range",
        }
    }
}
// 2. Implement the `Error` trait

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    // 3. Implement this function
    match input.parse::<u8>() {
        Ok(num @ 0..=100) => Ok(num),
        Err(ParsePercentageError::OutOfRange) => Err(ParsePercentageError::OutOfRange),
        Err(ParsePercentageError::InvalidInput) => Err(ParsePercentageError::InvalidInput),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
