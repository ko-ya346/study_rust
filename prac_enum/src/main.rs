// enum IpAddrKind {
    // V4,
    // V6
// }
// 
// struct IpAddr {
    // kind: IpAddrKind,
    // address: String
// }

// ここまでの定義は以下の書き方でも可能
// 各列挙子に紐づけるデータの型と量は異なってもよい
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    // println!("{}", home.address);

    let coin = Coin::Quarter(UsState::Alabama);
    let val = value_in_cents(coin);
    println!("{}", val);
}
