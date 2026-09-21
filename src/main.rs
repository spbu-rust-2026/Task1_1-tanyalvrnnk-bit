use std::io;
fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    let words: Vec<i32> = inp.split_whitespace().map(|s| s.parse().unwrap()).collect();
    println!("{}", words.iter().sum::<i32>());
}
