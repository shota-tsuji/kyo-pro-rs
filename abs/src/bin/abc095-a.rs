use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans = 700;
    if s.chars().nth(0).unwrap() == 'o' {
        ans += 100;
    }
    if s.chars().nth(1).unwrap() == 'o' {
        ans += 100;
    }
    if s.chars().nth(2).unwrap() == 'o' {
        ans += 100;
    }
    println!("{}", ans);
}
