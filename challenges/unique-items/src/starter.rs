use std::collections::HashSet;

// 1. Finish the function
pub fn unique_items(iter: impl Iterator<Item = String>) -> Vec<String>  {
    let mut set = HashSet::new();
    let mut results = Vec::new();
    for element in iter {
        let element = element.trim();
        if element.is_empty() {
            continue;
        }
        if !set.contains(&element) {
            set.insert(element);
            results.push(element);
        }
    }
    results
}

/// Example usage
pub fn main() {
    let product_ids = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_ids = unique_items(product_ids.into_iter());
    assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
}
