pub fn create_order(name: &str, price: f64) {
    println!("Preparing {}..", name);
    println!("Total: ${:.2}", price);
    println!("Order ready");
}
