fn main() {
    // 空のベクタを新たに作る
    let mut v: Vec<i32> = Vec::new();

    // 要素追加
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // let third: &i32 = &v[2];
    // println!("The third element is {}.", third);
    
    // Option<&T>
    let third = v.get(2);
    println!("The third element is {:?}.", third);

    // get メソッドは存在しない要素の添え字を指定すると None を返す
    match v.get(2) {
        Some(third) => println!("The third element is {}.", third),
        None => println!("There is no third element."),
    }


    // 与えた値を保持する新しいベクタ生成
    // 初期値を与えているので型注釈は不要
    // let v = vec![1, 2, 3];


    // ベクタ内の値を順に処理する
    for i in &v {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        // 全部の要素に 50 足す
        // * は参照外し演算子、i の値にたどり着くために使用
        *i += 50;
    }

    for i in &v2 {
        println!("{}", i);
    }

    // Enum を使って複数の型を保持する
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]
}
