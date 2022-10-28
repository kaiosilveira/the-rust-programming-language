pub trait Summary {}
pub trait Display {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    t: Summary,
    u: Display,
{
    1
}
