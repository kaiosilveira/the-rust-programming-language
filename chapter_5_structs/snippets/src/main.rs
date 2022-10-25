struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i32,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let p1 = Point(1, 1, 1);

    let user = build_user(
        String::from("silveira_kaio"),
        String::from("silveira.kaio@icloud.com"),
    );

    println!(
        "Welcome back, {}! Are you active? {}",
        user.username, user.active
    );

    println!("Your registered e-mail is {}", user.email);
    println!("Your sign_in_count is: {}", user.sign_in_count);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
