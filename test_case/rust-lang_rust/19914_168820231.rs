
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => println!("{:?}", line.as_bytes()),
            Err(e) => panic!("{}", e),
        }
    }
}
