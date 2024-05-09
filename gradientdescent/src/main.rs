#[derive(Debug)]
struct GradientDescent {
    alpha: f32,
    eps: f32,
}

impl GradientDescent {
    // テキストではインスタンス生成時に関数を引数で渡していた
    // f, df を引数として渡す方法が分からなかったので関連関数として実装
    fn f(xx: &Vec<f32>) -> f32 {
        let x = xx[0];
        let y = xx[1];
        5.0 * x.powi(2) - 6.0 * x * y + 3.0 * y.powi(2) + 6.0 * x - 6.0 * y 
    }

    fn df(xx: &Vec<f32>) -> Vec<f32> {
        let x = xx[0];
        let y = xx[1];
        let result_x = 10.0 * x - 6.0 * y + 6.0;
        let result_y = -6.0 * x + 6.0 * y - 6.0; 
        vec![result_x, result_y]
    }

    fn solve(&self, initial: &Vec<f32>) -> (Vec<f32>, Vec<Vec<f32>>) {
        // 雰囲気でclone を付けている
        let mut x = initial.clone();
        let mut path: Vec<_> = vec![];
        path.push(x.clone());
        
        let mut grad = GradientDescent::df(&x);
        loop {
            // 勾配が閾値以下になったら終了
            if grad[0].powi(2) + grad[1].powi(2) < self.eps.powi(2) {
                break
            }
            // パラメータ更新
            x[0] -= self.alpha * grad[0];
            x[1] -= self.alpha * grad[1];
            grad = GradientDescent::df(&x);

            path.push(x.clone());
        }

        (x, path)
    }
}

fn main() {
    // 最急降下法を実装


    let algo = GradientDescent {
        alpha: 0.01,
        eps: 0.00001,
    };

    // 初期値
    let initial = vec![1.0, 1.0];
    
    // f, df を計算してみる
    let result_f = GradientDescent::f(initial);
    println!("result_f: {}", result_f);
    let result_df = GradientDescent::df(&initial);
    println!("result_df: x {}, y {}", result_df[0], result_df[1]);

    // 最適化実行
    let (x, path) = algo.solve(&initial);

    println!("initialize: {:?}", initial);
    println!("opt: {:?}", x);
    println!("iteration: {}", path.len());

}
