use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let mut count = 0;
    for i in 1..=n {
        let sum = find_sum_of_digits(i);
        if sum >= a && sum <= b {
            total += i;
        }
    }

    println!("{}", count);
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
