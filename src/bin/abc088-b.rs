use proconio::input;

fn main() {
    input! {
        n: usize,
        mut cards: [i32; n],
    }

    cards.sort();
    cards.reverse();

    let mut alice = 0;
    let mut bob = 0;
    for i in 0..cards.len() {
        if i % 2 == 0 { alice += cards[i]; }
        else { bob += cards[i]; }
    }

    println!("{}", alice - bob);
}