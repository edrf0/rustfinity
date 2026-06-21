/*Define a trait Person:

It should require a method fn name(&self) -> String that returns the name of the person.
Define a trait Student that has Person as a supertrait:

It should require methods:
fn id(&self) -> u32 to return the student ID.
fn field_of_study(&self) -> String to return the student's field of study.
Implement both traits for a struct Undergraduate:

The Undergraduate struct should have fields id, name, and field_of_study.
Use the Student trait to provide the student's ID, name, and field of study.*/

// 1. Finish the trait definition
pub trait Person {
    fn name(&self) -> String;
}

// 2. Finish the trait definition
pub trait Student: Person {
    fn id(&self) -> u32;
    fn field_of_study(&self) -> String;
}

// 3. Finish the struct definition
pub struct Undergraduate {
    // Define fields for id, name, and field_of_study here...
    id: u32,
    name: String,
    field_of_study: String,
}

// 4. Implement the necessary traits for the Undergraduate struct
impl Person for Undergraduate {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Student for Undergraduate {
    fn id(&self) -> u32 {
        self.id
    }
    fn field_of_study(&self) -> String {
        self.field_of_study.clone()
    }
}
// Example usage
pub fn main() {
    let student = Undergraduate {
        id: 101,
        name: "John Doe".to_string(),
        field_of_study: "Computer Science".to_string(),
    };

    assert_eq!(student.name(), "John Doe");
    assert_eq!(student.id(), 101);
    assert_eq!(student.field_of_study(), "Computer Science");
}
