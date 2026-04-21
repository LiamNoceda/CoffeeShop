use std::io;

pub fn show_menu() {
    println!("\nMenu:");
    println!("1. Latte - 4.20");
    println!("2. Coffee - 3.50");
    println!("3. Tea - 2.80");
}

pub fn get_choice() -> u32 {
    let mut input = String::new();

    println!("\nChoose item");

    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap_or(1)
}
