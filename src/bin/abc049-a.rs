use proconio::input;

fn main() {
    input! {
        c: char,
    }

    let ans = match c {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        _ => "consonant",
    };
    println!("{}", ans);
}