
use std::io::BufRead;

fn main() {
    let line = std::io::stdin().lock().lines().next().unwrap().unwrap();
    println!("{}", line);
}
