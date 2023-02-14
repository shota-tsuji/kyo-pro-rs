use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let mut count = 0;
    for i in a..=b {
        if is_palindromic(i) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn is_palindromic(n: i64) -> bool {
    let mut num_digits = 0;
    let mut sum = 0;
    let mut b = n;
    while b > 0 {
        num_digits += 1;
        b /= 10;
    }

    let mut b = n;
    for i in (0..num_digits).rev().step_by(1) {
        sum += (b % 10) * 10i64.pow(i);
        b /= 10;
    }

    if sum == n {
        true
    } else {
        false
    }
}
