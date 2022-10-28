pub trait Summary {
    fn summarize_author(&self) -> String; // implementor's responsibility to implement this

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    // requires an item that implements the Summary trait
    println!("Breaking news! {}", item.summarize());
}
