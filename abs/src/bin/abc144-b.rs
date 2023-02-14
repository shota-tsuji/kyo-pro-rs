use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut can_multiply = false;
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j == n {
                can_multiply = true;
                break;
            }
        }
    }
    let ans = if can_multiply { "Yes" } else { "No" };
    println!("{}", ans);
}
