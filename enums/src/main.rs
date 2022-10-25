#[derive(Debug)]
enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    // ip example
    let home = IPAddrKind::V4(127, 0, 0, 1);
    let loopback = IPAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);

    // message example
    let message = Message::Write(String::from("Hello there"));
    message.call();
}

fn route(ip_kind: IPAddrKind) {
    println!("ip version is: {:?}", ip_kind);
}
