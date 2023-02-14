use proconio::input;

fn main() {
    input! {
        n: usize,
        mut arr: [usize; n],
    }

    let mut res = 0;
    loop {
        let mut exist_odd = false;
        for i in 0..n {
            if arr[i] % 2 != 0 {
                exist_odd = true;
            }
        }

        if exist_odd {
            break;
        }

        for i in 0..n {
            arr[i] /= 2;
        }

        res += 1;
    }

    println!("{}", res);
}
