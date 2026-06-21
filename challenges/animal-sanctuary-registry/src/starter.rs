use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    // TODO: implement this function
    registry.entry(String::from(section)).or_insert(Vec::new()).push(String::from(animal));
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    registry.get(&String::from(section)).unwrap_or(Vec::new())
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    registry.keys().sorted().collect::<Vec<String>>()
}
