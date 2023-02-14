use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[f64; d]; n],
    }

    let mut is_integer = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut distance = 0.0;
            for k in 0..d {
                distance += (x[i][k] - x[j][k]).powi(2);
            }
            if distance.sqrt().fract() == 0.0 {
                is_integer += 1;
            }
        }
    }
    println!("{}", is_integer);
}
