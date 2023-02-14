use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let a = vec![1, 2, 4, 8, 16, 32, 64];
    let mut ans = 1;
    for &value in a.iter() {
        if value <= n {
            ans = value;
        }
    }

    println!("{}", ans);
}
