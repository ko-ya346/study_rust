// モジュールと関数のシグネチャを定義
// 関連する定義を一つにまとめることができる
// 関数以外（構造体、enum、定数、トレイトなど）も置ける

// モジュールは標準で非公開
mod front_of_house {
    // eat_at_resutaurant() で add_to_waitlist 関数を使いたいので公開
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    // 構造体の要素ごとに公開設定が可能
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    // enum はすべて公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use でパスをスコープに持ち込む
// 関数まで持ち込むと、どこで定義されたかわからなくなる
use crate::front_of_house::hosting;
// use self::front_of_house::hosting;

// 構造体や enum、その他の要素を use で持ち込むときはフルパスを書くのが慣例的
// as でリネームできる
// pub use で、外部のコードが hosting::add_to_waitlist を使って呼び出せるようになった

//
// use std::cmp::Ordering;
// use std::io;
// と
// use std::{cmp::Ordering, io}; 
// は一緒
// use std::io::{self, Write}; -> std::io, std::io::Write
// use std::collections::*;


pub fn eat_at_restaurant() {
    // use で指定したので一部パスを省略出来る
    // crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    // 季節のフルーツを知ることも修正することも許されていないので下はエラー
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}