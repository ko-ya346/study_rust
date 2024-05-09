fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// なんらかの型T に関してジェネリックである
// 型Tのスライスを引数に、同じ T型の値を返す
// このままでは 順序付け可能な型のみしか使用できないため、コンパイルエラーが発生
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
// 
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
// 
//     largest
// }

// x と y は同じ型である必要がある
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let number_list = vec![1, 5, 4, 7, 2];

    // 最大値を取り出す
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};

    println!("integer.x = {}", integer.x());
    println!("float.x = {}", float.x());

}
