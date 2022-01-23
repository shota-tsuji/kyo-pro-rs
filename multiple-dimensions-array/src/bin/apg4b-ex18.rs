use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        games: [(usize, usize); m],
    }

    let mut results = vec![vec!['-'; n]; n];
    for (a, b) in &games {
        let a = a - 1;
        let b = b - 1;
        results[a][b] = 'o';
        results[b][a] = 'x';
    }

    for result in &results {
        for i in 0..result.len() {
            print!("{}", result[i]); // 現在処理している参加者と i 番目の参加者との結果を表示
            if i == result.len() - 1 { println!(); }
            else { print!(" "); }
        }
    }
}