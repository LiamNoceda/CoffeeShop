mod front_of_house;
mod back_of_house;

fn main() {
    println!("Coffe Shop");

    front_of_house::menu::show_menu();

    let choise = front_of_house::menu::get_choice();
    let (name, price) = back_of_house::drinks::find_drink(choise);

    println!("\nYou choosed {}.", name);
    println!("Price: ${:.2}", price);

    back_of_house::order::create_order(name, price);
}
