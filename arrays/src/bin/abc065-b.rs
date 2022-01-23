use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut trained = vec![false; n];
    a = a.iter().map(|x| x - 1).collect();
    let mut next  = 0;
    let mut count = 0;
    loop {
        trained[next] = true;
        next = a[next];
        count += 1;

        if trained[next] {
            println!("-1");
            return;
        }

        if next == 1 {
            println!("{}", count);
            return;
        }
    }
}