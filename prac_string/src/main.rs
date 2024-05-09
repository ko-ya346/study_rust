fn main() {
    let s1 = "hoge";
    // 文字リテラル
    // 中身はコンパイル時に判明しているので高速
     
    // let mut s2 = "fuga";

    // String型
    // 可変かつ伸長可能なテキスト破片をサポート
    // コンパイル時には不明な量のメモリをヒープに確保して内容保持
    let mut s2 = String::from("fuga");
    s2.push_str("fuga");
    let s3 = "piyo";
    
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);

    // このあと s1, s2, s3 のメモリは自動解放される (drop function)
    // 文字リテラルは問題ない (copy)
    let s4 = s1;
    // s2 はこの操作（move）により使用できなくなる
    // let s5 = s2;

    // clone: deep copy
    let s5 = s2.clone();

    println!("s1: {}, s2: {}, s3: {}, s4: {}, s5: {}", s1, s2, s3, s4, s5);
    // println!("s1: {}, s3: {}, s4: {}, s5: {}", s1, s3, s4, s5);

    let s6 = String::from("hello");
    takes_ownership(s6); // s6 が関数にムーブされ、これ以降このスコープ内では使用できない

    // println!("{}", s6); // エラー
    let x1 = 5;
    makes_copy(x1); // x1 は整数型なのでコピーとして渡されるため、これ以降も使用できる
    println!("{}", x1);


    let s7 = gives_ownership(); // 関数から s7 にムーブ
    let s8 = String::from("hello");

    let s9 = takes_and_gives_back(s8); // s8 が関数にムーブ、関数からs9 にムーブ


    // 参照
    let s10 = String::from("hello");
    let len = calculate_length(&s10);

    println!("The length of '{}' is {}", s10, len);

    // 可変な借用
    // s11 に対して 可変な参照は一つしかもてない
    let mut s11 = String::from("hello");

    change(&mut s11);

    let s12 = String::from("hello, world!");
    println!("{}", first_word(&s12));

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // ここで some_string がスコープを抜けて drop。メモリ解法

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    // 参照として変数を受け取っているため drop されない
    // 所有権をもっていない状態 （借用）
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}