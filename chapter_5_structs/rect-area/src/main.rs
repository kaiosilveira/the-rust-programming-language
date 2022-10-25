#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, another_rect: Rectangle) -> bool {
        self.width > another_rect.width && self.height > another_rect.height
    }

    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    dbg!(&rect1);
    println!("rect1 is {:#?}", rect1);

    if rect1.width() {
        println!(
            "rect1 has nonzero width, with an area of {} square pixels",
            rect1.area()
        );
    }

    println!("rect1 can hold rect2? {}", rect1.can_hold(rect2));
    println!("rect1 can hold rect3? {}", rect1.can_hold(rect3));

    let square = Rectangle::square(3);
    println!("square area is {}", square.area());
}
