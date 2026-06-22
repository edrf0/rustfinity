use std::str::FromStr;

/// Parse a string into an i32, returning a descriptive error message on failure.
pub fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
}

/// Parse common boolean representations (case-insensitive).
/// Accepts: "true", "false", "1", "0", "yes", "no"
pub fn parse_bool(s: &str) -> Result<bool, String> {
    s.parse::<bool>()
}

/// Parse a "key=value" string into a tuple.
pub fn parse_key_value(s: &str) -> Result<(String, String), String> {
    let s = s.split('=');
    match (s.next(), s.next()) {
        (Some(key), Some(value)) => (key.to_string(), value.to_string()),
        (_, _) => Err(String::from("Expected key and value")),
    }
}

/// A color represented by red, green, and blue components.
#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// TODO: Implement FromStr for Color
// The format is "r,g,b" (e.g., "255,128,0")
impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color_array = s.split(',');
        match (color_array.next(), color_array.next(), color_array.next()) {
            (Some(r), Some(g), Some(b)) => Ok(Color { r, g, b }),
            _ => Err(String::from("Expected color")),
        }
    }
}

/// Parse a delimited list of values into a Vec.
pub fn parse_list<T: FromStr>(s: &str, delimiter: char) -> Result<Vec<T>, String> {
    // TODO: Split by delimiter, parse each part, collect into Result<Vec<T>, String>
    let list_iter = s.split(delimiter);
    let mut result = Vec::new();
    for element in list_iter {
        match element.parse() {
            Ok(element) => result.push(element),
            Err(_) => return Err(format!("Could not parse element {}", element)),
        }
    }
    Ok(result)
}

pub fn main() {
    // Demonstrate parse_int
    println!("Parsing integers:");
    println!("  '42' -> {:?}", parse_int("42"));
    println!("  '-17' -> {:?}", parse_int("-17"));
    println!("  'abc' -> {:?}", parse_int("abc"));

    // Demonstrate parse_bool
    println!("\nParsing booleans:");
    println!("  'true' -> {:?}", parse_bool("true"));
    println!("  'YES' -> {:?}", parse_bool("YES"));
    println!("  '0' -> {:?}", parse_bool("0"));
    println!("  'maybe' -> {:?}", parse_bool("maybe"));

    // Demonstrate parse_key_value
    println!("\nParsing key=value pairs:");
    println!("  'name=Alice' -> {:?}", parse_key_value("name=Alice"));
    println!("  'count=42' -> {:?}", parse_key_value("count=42"));
    println!("  'invalid' -> {:?}", parse_key_value("invalid"));

    // Demonstrate Color parsing
    println!("\nParsing colors:");
    let color: Result<Color, _> = "255,128,0".parse();
    println!("  '255,128,0' -> {:?}", color);

    // Demonstrate parse_list
    println!("\nParsing lists:");
    println!("  '1,2,3' as i32 -> {:?}", parse_list::<i32>("1,2,3", ','));
}
