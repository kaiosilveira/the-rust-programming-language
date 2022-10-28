pub trait Summary {}
pub trait Display {}

// in parameters
pub fn notify(item1: &(impl Summary + Display)) {}

// with generics
pub fn notify<T: Summary + Display>(item1: &T) {}
