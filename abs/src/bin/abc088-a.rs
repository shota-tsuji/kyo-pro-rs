use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
    }
    let ans = if (n % 500) <= a { "YES" } else { "NO" };
    print!("{}", ans);
}
