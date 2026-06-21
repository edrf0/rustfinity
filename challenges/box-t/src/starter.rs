/*Implement the create_animal function to return a Box<Animal> containing a new Animal instance.

Define another function, access_animal, that takes a Box<Animal> and returns a tuple (String, u8)
representing the animal's name and age. Use dereferencing to access the fields.*/

pub struct Animal {
    pub name: String,
    pub age: u8,
}

pub fn create_animal(name: &str, age: u8) -> Box<Animal> {
    // Your code here
    Box::new(Animal {
        name: name.to_string(),
        age
    })
}

pub fn access_animal(animal: Box<Animal>) -> (String, u8) {
    // Your code here
    (*animal.name, *animal.age)
}

// Example usage
pub fn main() {
    let animal = create_animal("Leo", 5);
    let (name, age) = access_animal(animal);
    println!("Animal's name: {}, age: {}", name, age);
}
