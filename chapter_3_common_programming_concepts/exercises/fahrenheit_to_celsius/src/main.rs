use std::io;

fn main() {
    println!("Fahrenheit to Celsius!");
    println!("Please type your Fahrenheit temperature: ");

    let mut fahrenheit_temperature: String = String::new();
    io::stdin()
        .read_line(&mut fahrenheit_temperature)
        .expect("Failed to read user input!");

    let fahrenheit_temperature: f32 = fahrenheit_temperature
        .trim()
        .parse()
        .expect("Please type a number!");

    let celsius_temperature = fahrenheit_to_celsius(fahrenheit_temperature);

    println!(
        "The equivalent temperature in Celsius is: {}",
        celsius_temperature
    );
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    let multiplier: f32 = 5.0 / 9.0;
    let result: f32 = multiplier * (f - 32.0);
    result
}
