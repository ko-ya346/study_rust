// データを定義
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッド定義（構造体、enum, トレイトオブジェクトで定義される点が関数と異なる）
// implブロックは複数に分けて記載することもできる（ジェネリック型、トレイトで有用）
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数 (String::from みたいなやつ)
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}


fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    println!("{:?}", rect1);
    println!("{}", rect1.area());

    let rect2 = Rectangle {width: 20, height: 40};
    println!("{}", rect1.can_hold(&rect2));

    let square1 = Rectangle::square(3);
    println!("{}", square1.area());
}
