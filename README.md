# CoffeeShop

CoffeeShop is a small educational Rust CLI demo that simulates a simple coffee shop menu and ordering flow. It demonstrates basic Rust module structure, console input/output, and simple calculations.

## Purpose

This project is intended as a learning example for organizing code into modules (`front_of_house` and `back_of_house`), handling user input, and basic program flow in Rust.

## Project structure

- `Cargo.toml` - Cargo manifest
- `src/main.rs` - Entry point: shows menu, reads user choice, computes taxes/total and creates an order
- `src/front_of_house/menu.rs` - Menu display and input handling
- `src/front_of_house/mod.rs` - Front-of-house module declaration
- `src/back_of_house/drinks.rs` - Drink lookup (name and base price)
- `src/back_of_house/order.rs` - Order creation/printing
- `src/back_of_house/mod.rs` - Back-of-house module declaration

## Requirements

- Rust (stable) and Cargo. Tested on a recent stable toolchain.

## Build and run

Build the project:

```bash
cargo build
```

Run the app:

```bash
cargo run
```

The program will display a menu and prompt for a choice. It then calculates taxes (configurable in `main.rs`) and prints the total before creating the order.

## Example session

```
Menu:
1. Latte - 4.20
2. Coffee - 3.50
3. Tea - 2.80

Choose item
1

You chose Latte.
Taxes: $1.05
Total: $5.25
Preparing Latte..
Total: $5.25
Order ready
```

## Notes & next steps

- The tax rate is currently a hard-coded value in `src/main.rs` — consider making it configurable or passing it as a CLI argument.
- Add unit tests and error handling for user input to make the program more robust.

## License

This repository does not currently specify a license. Add a `LICENSE` file if you want to make the code open-source.
