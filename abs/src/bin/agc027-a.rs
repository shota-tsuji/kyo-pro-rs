use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: i64,
        mut a: [i64; n],
    }

    a.sort();
    let mut sum_a = Vec::new();
    let mut sum = 0;
    for &v in a.iter() {
        sum += v;
        sum_a.push(sum);
    }
    let mut ans = 0;
    let mut i = 0;
    loop {
        if sum_a[i] <= x {
            // まだ配れる場合
            ans += 1;
        } else if sum_a[i] >= x {
            // 全員の希望分のおかしが足りない場合
            // あまりの分は次の子に押し付ける
            break;
        }

        if i == n - 1 {
            // おかしが余る場合
            if sum_a[i] != x {
                ans -= 1;
            }
            break;
        }
        i += 1;
    }

    println!("{}", ans);
}
