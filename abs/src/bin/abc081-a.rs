use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans = 0;
    if s.chars().nth(0).unwrap() == '1' {
        ans += 1;
    }
    if s.chars().nth(1).unwrap() == '1' {
        ans += 1;
    }
    if s.chars().nth(2).unwrap() == '1' {
        ans += 1;
    }
    println!("{}", ans);
}
