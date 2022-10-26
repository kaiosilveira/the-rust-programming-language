mod front_of_house;

pub use front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist("Kaio");
}
