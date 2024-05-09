use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // エラー発生時は panic! が呼び出される
    // エラーメッセージの最後2行目に表示される
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    // panic! 発生
    // RUST_BACKTRACE=1 cargo run で詳細を追うことができる
    //   6: prac_erryr::main
    //         at ./src/main.rs:8:6
    // v[99];

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    // unwrap: panic! を呼び出してくれる
    // expect: panic! 呼び出し + エラーメッセージ指定できる
    // let f2 = File::open("hello.txt").unwrap();
    // let f2 = File::open("hello.txt").expect("Not found some file.");

}

fn read_username_from_file2() -> Result<String, io::Error> {
    // ?演算子によりエラーの型をfrom関数を通す
    // from関数を通ると、関数の戻り値型で定義されているエラー型に変換する
    // エラー発生時は早期リターンされる
    // ?演算子は Result を返す関数でしか使用できない
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // さらに短くできる
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        // 問題があった場合、エラーを早期リターン
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
