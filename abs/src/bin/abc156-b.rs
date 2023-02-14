use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }

    println!("{}", find_base_k_digits(n, k));
}

fn find_base_k_digits(n: i64, k: i64) -> i64 {
    let mut digits = 0;
    let mut n = n;
    while n > 0 {
        digits += 1;
        n /= k;
    }
    digits
}
