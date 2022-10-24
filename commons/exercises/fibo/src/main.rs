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
    let result: i32 = if t <= 2 { 1 } else { fibo(t - 1) + fibo(t - 2) };
    result
}
