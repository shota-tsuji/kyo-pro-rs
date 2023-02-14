use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i64,
        x: i64,
        m: [i64; n],
    }

    let sum: i64 = m.iter().sum();
    let min = m.iter().min().unwrap();
    let ans = (m.len() as i64) + (x - sum) / min;

    println!("{}", ans);
}
