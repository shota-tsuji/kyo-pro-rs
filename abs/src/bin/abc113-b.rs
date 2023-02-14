use proconio::input;

fn main() {
    input! {
        n: i64,
        t: f64,
        a: f64,
        h: [f64; n],
    }

    let mut min = 99999999999999.9;
    let mut index = 0;
    for (i, v) in h.iter().enumerate() {
        // A度との差分
        let sub = f64::abs(a - (t - v * 0.006));
        if sub < min {
            min = sub;
            index = i;
        }
    }

    println!("{}", index + 1);
}
