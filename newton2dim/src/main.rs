use ndarray::prelude::*;
use ndarray_linalg::*;


fn f(x: f32) -> f32 {
    x.powi(3) - 5.0 * x + 1.0
}

fn df(x: f32) -> f32 {
    3.0 * x.powi(2) - 5.0
}

struct Newton1dim {
    eps: f32, 
    max_iter: i32,
}

impl Newton1dim {
    fn solve(&self, x0: f32) -> f32{
        let mut x = x0.clone();
        let mut iter = 0;
        let mut x_new: f32;

        loop {
            x_new = x - f(x) / df(x);
            if (x - x_new).powi(2) < self.eps.powi(2) {
                break
            }

            x = x_new.clone();
            iter += 1;
            if iter >= self.max_iter {
                break
            };
        }
        println!("iteration: {}", iter);
        x 
    }
}

fn f1(x: &f32, y: &f32) -> f32 {
    x.powi(3) - 2.0 * y
}

fn f2(x: &f32, y: &f32) -> f32 {
    x.powi(2) + y.powi(2) - 1.0
}

fn ff(xx: &Array2<f32>) -> Array2<f32> {
    let x = xx[[0, 0]];
    let y = xx[[1, 0]];

    array![[f1(&x, &y)], [f2(&x, &y)]]
}

fn dff(xx: &Array2<f32>) -> Array2<f32> { 
    let x = xx[[0, 0]];
    let y = xx[[1, 0]];

    array![[3.0 * x.powi(2), -2.0], [2.0 * x, 2.0 * y]]
}

// 多次元のニュートン法
struct Newton {
    eps: f32,
    max_iter: u32,
}

impl Newton {
    fn solve(&self, x0: &Array2<f32>) -> Array2<f32> {
        let mut x = x0.clone();
        let mut iter = 0;
        let mut x_new: Array2<f32>;
        let mut a: Array2<f32>;
        let mut b: Array2<f32>;

        loop {
            // ヤコビアンの逆行列
            a = dff(&x).inv_into().unwrap();

            b = ff(&x);

            x_new = x.clone() - &a.dot(&b);
            if (x[[0, 0]] - x_new[[0, 0]]).powi(2) + (x[[1, 0]] - x_new[[1, 0]]).powi(2) < self.eps.powi(2) {
                break
            }

            x = x_new.clone();
            iter += 1;
            if iter >= self.max_iter {
                break
            }
        }
        x
    }
}

fn main() {
    // ニュートン法を実装する
    let newton1dim = Newton1dim {
        eps: 0.0001,
        max_iter: 1000,
    };

    let result = newton1dim.solve(2.0);
    println!("result: {}", result);

    // let xx = array![1.0, 1.0];
    // println!("{:?}", ff(&xx));
    // println!("{:?}", dff(&xx));

    // ndarray の計算の練習
    // let a: Array1<f32> = array![1.0, 2.0];
    // println!("{:?}", a);
    // println!("{}", a[0]);

    // let b: Array2<f32> = array![[3.0, -1.0], [2.0, 2.0]];
    // let bb: Array2<f32> = array![[3.0, -1.0], [2.0, 1.0]];
    // println!("{:?}", b.inv_into().unwrap());
    // println!("{:?}", b - bb);

    let newton = Newton {
        eps: 0.001,
        max_iter: 1000,
    };

    let init = array![[1.0], [1.0]];
    let result = newton.solve(&init);
    println!("{:?}", result);
}
