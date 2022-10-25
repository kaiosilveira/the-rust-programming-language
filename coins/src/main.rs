#[derive(Debug)]
enum USState {
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn main() {
    println!("Penny: {}", value_in_cents(Coin::Penny));
    println!("Nickel: {}", value_in_cents(Coin::Nickel));
    println!("Dime: {}", value_in_cents(Coin::Dime));
    println!(
        "Quarter: {}",
        value_in_cents(Coin::Quarter(USState::Alabama))
    );
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("New Quarter from {:?}", state);
            25
        }
    }
}
