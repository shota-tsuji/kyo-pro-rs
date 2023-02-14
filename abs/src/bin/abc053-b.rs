use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut index_a = 0;
    let mut index_z = 0;
    for (i, &v) in s.iter().enumerate() {
        if v == 'A' {
            index_a = i;
            break;
        }
    }

    let n = s.len();
    for (i, &v) in s.iter().enumerate().rev() {
        if v == 'Z' {
            index_z = i;
            break;
        }
    }

    println!("{}", index_z - index_a + 1);
}
