// constant variables
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // mutable variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing rules
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // changing a variable type using shadowing
    let spaces = "    ";
    let spaces = spaces.len();

    let mut spaces = "    ";
    spaces = spaces.len(); // will cause an error, because we can't change a mut variable's type
}
