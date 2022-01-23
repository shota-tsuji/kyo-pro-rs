use std::cmp::max;
use proconio::input;

fn main() {
    input! {
        n: usize,
        blue: [String; n],
        m: usize,
        red: [String; m],
    }

    let mut ans = 0;
    for s in &blue {
        let mut sum = 0;

        for v in &blue {
            if *v == *s { sum += 1; }
        }

        for v in &red {
            if *v == *s { sum -= 1; }
        }

        ans = max(ans, sum);
    }

    println!("{}", ans);
}