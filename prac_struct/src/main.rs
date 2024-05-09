struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// タプル構造体（名称不要）
struct Color(i32, i32, i32);

fn main() {
    // すべてのインスタンスが不変
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("hoge"),
        active: true,
        sign_in_count: 1,
    };


    // すべてのインスタンスが可変
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("hoge"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("huga@aaa.com");

    let user3 = build_user(String::from("piyo"), String::from("ahaha"));

    println!("{}, {}, {}", user1.email, user2.email, user3.email);

    let black = Color(0, 0, 0);
    println!("{}", black.0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}


