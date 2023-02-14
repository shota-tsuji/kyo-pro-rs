use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut min = std::i64::MAX;
    for a in 1..n {
        let b = n - a;
        let sum = find_sum_of_digits(a) + find_sum_of_digits(b);
        if sum < min {
            min = sum;
        }
    }

    println!("{}", min);
}

fn find_sum_of_digits(n: i64) -> i64 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
