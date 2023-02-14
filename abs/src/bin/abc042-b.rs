use proconio::input;

fn main() {
    input! {
        n: i64,
        _: i64,
        mut strings: [String; n],
    }

    strings.sort();
    for i in 0..strings.len() {
        print!("{}", strings[i]);
    }

    println!("");
}
