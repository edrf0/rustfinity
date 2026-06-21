/*Define the Speakable trait with a method speak that returns a String.
Define a struct Dog with two fields: name and breed, both of type String.
Implement the Speakable trait for Dog to return a string Woof.
Define a struct Robot with two fields: model and purpose, both of type String.
Implement the Speakable trait for Robot to return a string Beep boop.
Finish the function get_speaker that takes a &str parameter and returns either a Dog
or a Robot based on the parameter.
The parameter can either be dog or robot.*/
trait Speakable {
    fn speak(&self) -> String;
}

struct Dog {
    name: String,
    breed: String
}

impl Speakable for Dog {
    fn speak(&self) -> String {
        String::from("Woof")
    }
}

struct Robot {
    model: String,
    purpose: String,
}

impl Speakable for Robot {
    fn speak(&self) -> String {
        String::from("Beep boop")
    }
}
pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => {
            // Return a Dog instance here
            Box::new(Dog {})
        }
        "robot" => {
            // Return a Robot instance here
            Box::new(Robot {})
        }
        _ => panic!("Unknown speaker type"),
    }
}

// Example usage
pub fn main() {
    let dog_speaker = get_speaker("dog");
    println!("{}", dog_speaker.speak()); // Expected output: Woof

    let robot_speaker = get_speaker("robot");
    println!("{}", robot_speaker.speak()); // Expected output: Beep boop
}
