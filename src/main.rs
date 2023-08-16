mod math;
mod nn;
use crate::math::MathOps;

fn main() {
    /*
    let a = Matrix {
        data: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
    };
    let b = Matrix {
        data: vec![vec![5.0, 6.0], vec![7.0, 8.0]],
    };

    let c = a.add(&b);
    let d = a.sub(&b);
    let e = a.mul(&b);

    let f = Matrix {
        data: vec![vec![0.0;5], vec![0.0;5]],
    };



    let h = f.randomize();

    println!("Matrix - Random:\n{}", h);
    println!("Matrix A + B:\n{}", c);
    println!("Matrix A - B:\n{}", d);
    println!("Matrix A * B:\n{}", e);

    let identity = Matrix::identity(3);
    println!("Identity Matrix:\n{}", identity);

    let z = math::Matrix {
        data: vec![vec![0.0;2], vec![0.0;2], vec![0.0;2]],
    }.randomize();

    let y = math::Matrix {
        data: vec![vec![0.0;3], vec![0.0;3]],
    }.randomize();

    println!("Matrix Z:\n{}",z);
    println!("Matrix Y:\n{}",y);
    let res = z.dot_product(&y);
    println!("Dot Product Z * Y:\n{}", res);

    println!("Matrix Y:\n{}",y);
    println!("Matrix Z:\n{}",z);
    let res = y.dot_product(&z);
    println!("Dot Product Y * Z:\n{}", res);
    */

    let nn = nn::new();
}
