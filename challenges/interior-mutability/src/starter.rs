use std::cell::RefCell;
use std::rc::Rc;

pub fn push<T>(data: Rc<RefCell<Vec<T>>>, element: T) {
    // 1. Finish the function
    /*
    Accept an Rc<RefCell<Vec<T>>> as input.
    Append an element to the shared vector inside the RefCell.
    */
    data.borrow_mut().push(element);
}

pub fn iterate_and_print_shared_data<T>(data: Rc<RefCell<Vec<T>>>) {
    // 2. Borrow the data and print each item
    /*
    Take an Rc<RefCell<Vec<T>>> as input.
    Iterate through the vector and print each element.
    */
    for element in data.borrow().iter() {
        println!("{}", element);
    }
}

pub fn plus_one(data: Rc<RefCell<i32>>) {
    // 3. Finish the function
    /*
    Accept an Rc<RefCell<i32> as input.
    Increment the value inside the RefCell by one.
    */
    data.borrow_mut() += 1;
}

// Example usage
pub fn main() {
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    // Updating shared data
    push(Rc::clone(&shared_data), 4);
    push(Rc::clone(&shared_data), 5);

    // Iterating and printing shared data
    println!("Shared Data:");
    iterate_and_print_shared_data(Rc::clone(&shared_data));

    let my_num = Rc::new(RefCell::new(5));
    plus_one(Rc::clone(&my_num));
    assert_eq!(*my_num.borrow(), 6, "Value was not incremented correctly.");
}
