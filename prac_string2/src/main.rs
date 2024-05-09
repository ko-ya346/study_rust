fn main() {
    // let mut s = String::new();

    // 文字リテラルを String に変換
    // let data = "initial contents";
    // let s = data.to_string();
    // let s = "initial contents".to_string();
    let mut s = String::from("initial contents");

    println!("{}", s);

    // push_str は文字列スライスを取るため所有権を奪わない
    let s2 = "bar";
    s.push_str(s2);
    // push メソッドは 1文字だけ
    s.push('l');
    println!("s2 is {}.", s2);

    // + 演算子は add メソッドを使用している
    // add メソッドの第二引数は &str だが、参照外し型強制により &String を渡しても大丈夫
    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    // s3 はムーブされる
    let s5 = s3 + &s4;
    println!("s5 is {}.", s5);

    // format! は引数の所有権を奪わない
    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");

    let ss = format!("{}-{}-{}", ss1, ss2, ss3);
    println!("{}", ss);

    // 文字列は Vec<u8> なのでアクセスはそう簡単ではない
    for c in ss.chars() {
        println!("{}", c);
    }

}
