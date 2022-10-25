fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let new_number = if condition { 5 } else { 6 };

    println!("The value of new_number is: {new_number}");
}
