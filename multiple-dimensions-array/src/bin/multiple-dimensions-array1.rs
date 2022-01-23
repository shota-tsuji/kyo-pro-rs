use proconio::input;

fn main() {
    input! {
        n: usize,
        states: [[[char; 3]; 3]; n],
    }

    let mut cnt = 0;
    for state in &states {
        for row in state {
            for &column in row {
                if column == 'o' { cnt += 1; }
            }
        }
    }

    println!("{}", cnt);
}