pub fn weekday_from_number(day: u8) -> &'static str {
    match day {
        1 => "Sunday",
        2 => "Monday",
        3 => "Tuesday",
        4 => "Wednesday",
        5 => "Thursday",
        6 => "Friday",
        7 => "Saturday",
        _ => "Invalid day number",
    }
}
