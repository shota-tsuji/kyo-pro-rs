use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }

    s.sort();
    t.sort_by(|a, b| b.cmp(a));
    let ans = if s < t { "Yes" } else { "No" };
    //println!("{}", t.iter().collect::<String>());
    println!("{}", ans);
}
