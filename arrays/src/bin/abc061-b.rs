use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        path: [(usize, usize); m],
    }

    let mut counts = vec![0; n];
    for &(a, b) in &path {
        counts[a - 1] += 1;
        counts[b - 1] += 1;
    }

    for &v in &counts {
        println!("{}", v);
    }
}