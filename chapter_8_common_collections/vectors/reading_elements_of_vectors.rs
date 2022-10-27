fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &vec[2];
    println!("The third element is {}", third);

    let third: Option<i32> = vec.get(2);
    match third {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is not element at the third position"),
    }
}
