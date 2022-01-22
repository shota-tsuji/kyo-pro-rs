use proconio::input;

fn main() {
    input! {
        a: [i32; 5],
    }

    for i in 0..a.len() - 1 {
        if a[i] == a[i + 1] {
            println!("YES");
            return;
        }
    }

    println!("NO");
}