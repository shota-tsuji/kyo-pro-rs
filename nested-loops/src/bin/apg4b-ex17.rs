use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i32,
        apples: [i32; n],
        pains: [i32; n],
    }

    let mut cnt = 0;
    for &a in &apples {
        for &p in &pains {
            if a + p == s { cnt += 1; }
        }
    }
    println!("{}", cnt);
}