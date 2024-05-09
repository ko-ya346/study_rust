fn main() {
    // 数値計算の練習
    // +-*/
    let mut n: i32 = 10;
    n += 1;
    println!("n: {}", n);
    
    n -= 2;
    println!("n: {}", n);

    n *= 3; 
    println!("n: {}", n);
    
    n /= 3;
    println!("n: {}", n);

    // 累乗
    // 底が float -> べき指数が float -> .powf
    // 底が float -> べき指数が int -> .powi
    // 底が int -> べき指数が int -> .pow

    n = n.pow(2);
    println!("n: {}", n);

}
