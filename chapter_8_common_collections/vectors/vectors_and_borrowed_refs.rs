fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let first = vec[0];
    v.push(6);
    println!("The first element is {}", first); // will cause the program to panic because
                                                // there is a immutable ref at line 3 and a
                                                // mutable ref in line 4
}
