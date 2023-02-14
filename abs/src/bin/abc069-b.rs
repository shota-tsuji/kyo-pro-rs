use proconio::input;

fn main() {
    input! {
        s: String,
    }

    print!("{}", s.chars().nth(0).unwrap());
    print!("{}", s.len() - 2);
    println!("{}", s.chars().nth(s.len() - 1).unwrap());
}
