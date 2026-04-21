pub fn find_drink(id: u32) -> (&'static str, f64) {
    match id {
        1 => ("Latte", 4.20),
        2 => ("Coffee", 3.50),
        3 => ("Tea", 2.80),
        _ => ("Tea", 2.80),
    }
}
