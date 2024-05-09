use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // キー, 値はすべて同じ型でなければならない
    // insert でムーブが発生する
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 以下の方法でも作成できる
    // zip メソッドでタプルのベクタを作り、collect メソッドで ハッシュマップに変換
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // 型注釈必要、ベクタのデータ型に基づいて推論
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 値をとりだす
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); 
    println!("{:?}", score);

    for (key, value) in &scores {
        println!{"{}: {}", key, value};
    }

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);
    println!("{:?}", scores2);

    // キーに値がなかった時のみ値を挿入
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores2);

    // 古い値に基づいて値を更新
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert 関数は、このキーに対する値への可変参照 (&mut V) を返す 
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
