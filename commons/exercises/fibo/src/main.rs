use std::cmp::Ordering;

fn main() {
    println!("1st Fibonacci number is: {}", fibo(1));
    println!("2nd Fibonacci number is: {}", fibo(2));
    println!("3rd Fibonacci number is: {}", fibo(3));
    println!("4th Fibonacci number is: {}", fibo(4));
    println!("5th Fibonacci number is: {}", fibo(5));
    println!("5th Fibonacci number is: {}", fibo(6));
    println!("7th Fibonacci number is: {}", fibo(7));
}

fn fibo(t: i32) -> i32 {
    match t.cmp(&2) {
        Ordering::Greater => fibo(t - 1) + fibo(t - 2),
        _ => 1,
    }
}
