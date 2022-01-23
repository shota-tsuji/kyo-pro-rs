use proconio::input;

fn main() {
    input! {
        heights: [i32; 3],
    }

    let h_max = heights.iter().max().unwrap();
    let h_min = heights.iter().min().unwrap();
    println!("{}", h_max - h_min);
}