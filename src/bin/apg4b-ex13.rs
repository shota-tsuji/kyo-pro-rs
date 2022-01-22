use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let ave = a.iter().sum::<i32>() / n as i32;
    for &v in &a {
        println!("{}", (v - ave).abs());
    }
}