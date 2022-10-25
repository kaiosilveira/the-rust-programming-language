fn main() {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value + 1),
        None => None,
    }
}
