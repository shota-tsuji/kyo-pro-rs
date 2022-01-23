use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [String; h],
    }

    for _ in 0..w + 2 { print!("#"); }
    println!();

    for s in &a {
        println!("#{}#", s);
    }

    for _ in 0..w + 2 { print!("#"); }
    println!();
}