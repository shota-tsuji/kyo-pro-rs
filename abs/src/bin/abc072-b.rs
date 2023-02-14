use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = String::new();
    for (i, &v) in s.iter().enumerate() {
        // 1始まりのindexに修正
        if (i + 1) % 2 == 1 {
            ans.push(v);
        }
    }

    println!("{}", ans);
}
