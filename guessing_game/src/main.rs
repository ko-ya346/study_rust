// 入出力ライブラリ
use std::io;
// スコープに Ordering 型を導入
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Rng トレイト: 乱数生成器が実装すべきメソッドを定義している 
    // rand::thread_rng 関数で乱数生成器を取得 (現在のスレッドに固有)
    // gen_range メソッドで範囲指定
    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // 変数定義 (mutable: 可変)
        // mut がなければ不変
        // String::new 関数 で String型の新しいインスタンスを返す
        // String: 標準ライブラリによって提供される文字列型
        // :: -> new が String型の関連関数であることを示す
        // 関連関数: ある型に対して実装される関数
        let mut guess = String::new();

        // io::stdin()
        // 1行目が use std;  なら、std::io::stdin のように呼び出せば利用可能
        // stdin関数はターミナルの表示入力へのハンドルを表す型である 
        // std::io::Stdin インスタンス を返す

        // .read_line(&mut guess)
        // 標準入力ハンドルの read_line メソッドを呼び出し、
        // ユーザからの入力を得る。read_line の引数として &mut guess を渡し、
        // ユーザが標準入力に入力したものを文字列に追加する
        // 引数の文字列は変更できるよう可変である必要がある
    
        // & はその引数が参照であることを示す。
        // コードの複数部分が同じデータにアクセスしても、
        // そのデータを何度もメモリにコピーしなくて済む
        // 参照もデフォルト不変なので &guess ではなく &mut guess とする必要がある

        // read_line メソッドは処理と同時に値（io::Result）も返す
        // Result型は列挙型 で enum とも呼ばれ、取りうる値として決まった数の列挙子(variant) を持つ
        // Result の列挙子は Ok か Err
        // Result 型の値にもメソッドが定義されており、その一つに expect メソッドがある
        // expect メソッドは 値が Err の場合、プログラムをクラッシュさせて引数のメッセージを表示する
        // expect メソッドを呼び出さないと警告が出る
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 既に guess という変数を作成していたが、新しい値で覆い隠した（shadow）
        // 変数を再利用した（別の型に変換するときによく使われる）
        // parse メソッド: 文字列を様々な数値型へパースできる
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("You guessed: {}", guess);

        // cmp メソッド: 2つの値の比較を行う
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // 正解したら終了する
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
