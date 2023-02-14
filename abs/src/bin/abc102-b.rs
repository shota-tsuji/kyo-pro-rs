use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    }

    let mut min = 99999999999999i64;
    let mut max = -99999999999999i64;
    for v in a {
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
    }

    println!("{}", (max - min));
}
