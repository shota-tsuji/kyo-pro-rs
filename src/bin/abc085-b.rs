use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mochi: [i32; n],
    }

    let mochi_set: HashSet<i32> = mochi.clone().into_iter().collect();
    println!("{}", mochi_set.len());
}