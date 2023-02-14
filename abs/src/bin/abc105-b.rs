use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let max_cakes = 100 / 4;
    let max_doughnuts = 100 / 7;
    let mut can_buy = false;
    for i in 0..=max_cakes {
        for j in 0..=max_doughnuts {
            if (4 * i + 7 * j) == n {
                can_buy = true;
                break;
            }
        }
    }
    let ans = if can_buy { "Yes" } else { "No" };
    println!("{}", ans);
}
