use std::collections::HashMap;

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid] as f32 + numbers[mid - 1] as f32) / 2.0
    } else {
        numbers[mid] as f32
    }
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for number in numbers {
        if !map.contains_key(number) {
            map.insert(*number, 0);
        }
        map.get_mut(number).unwrap().count += 1;
    }
    let mut max_count = 0;
    let mut result = Vec::new();
    for (number, count) in map {
        if count > max_count {
            max_count = count;
            result.clear();
            result.push(number);
        } else if count == max_count {
            result.push(number);
        }
    }
    result
}
