use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut h_flag = vec![false; h];
    let mut w_flag = vec![false; w];

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' { h_flag[i] = true; }
        }
    }

    for i in 0..w {
        for j in 0..h {
            if a[j][i] == '#' { w_flag[i] = true; }
        }
    }

    for i in 0..h {
        if h_flag[i] {
            for j in 0..w {
                if w_flag[j] { print!("{}", a[i][j]); }
            }
            println!();
        }
    }
}