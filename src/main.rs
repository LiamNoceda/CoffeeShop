mod front_of_house;
mod back_of_house;

fn main() {
    println!("Coffee Shop");

    front_of_house::menu::show_menu();

    let choice = front_of_house::menu::get_choice();
    let (name, price) = back_of_house::drinks::find_drink(choice);

    println!("\nYou chose {}.", name);

    let tax_rate = 0.25_f64;
    let taxes = price * tax_rate;
    let total = price + taxes;

    println!("Taxes: ${:.2}", taxes);
    println!("Total: ${:.2}", total);

    back_of_house::order::create_order(name, total);
}
