use proconio::input;

fn main() {
    input! {
        n: i64,
        k: usize,
        mut l: [i64; n],
    }

    l.sort();
    l.reverse();
    let mut sum = 0;
    for i in 0..k {
        sum += l[i];
    }

    println!("{}", sum);
}
