use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    let ans = s.replacen("7", "8", 1);
    println!("{}", ans);
}
