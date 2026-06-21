/*Implement the following closures and their respective behaviors:

calculate_total: An Fn closure that calculates the total price of an item, including tax
(price + price * tax_rate).
apply_discount: An FnMut closure that mutates the cart total by subtracting a given discount.
checkout_cart: An FnOnce closure that consumes the cart's details (a String) and
returns a confirmation message.*/

// 1. Based on the `main` function below,
// Find out the types of the closures and define them
pub fn create_typed_closures() {
    // 2. Implement calculate_total closure here
    let calculate_total = |price,tax_rate| {price + price * tax_rate};

    // 3. Implement apply_discount closure here
    let apply_discount = |price,discount_amount| {price -= discount_amount};

    // 4. Implement checkout_cart closure here
    let checkout_cart = |details| {format!("Checkout complete: {}",details);};

    (calculate_total, apply_discount, checkout_cart)
}

// Example usage
pub fn main() {
    let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

    // Example tests
    assert_eq!(calculate_total(100.0, 0.2), 120.0);

    let mut total_price = 120.0;
    apply_discount(&mut total_price, 20.0);
    assert_eq!(total_price, 100.0);

    let cart_details = String::from("Items: Apple, Banana, Orange");
    assert_eq!(
        checkout_cart(cart_details),
        "Checkout complete: Items: Apple, Banana, Orange"
    );
}
