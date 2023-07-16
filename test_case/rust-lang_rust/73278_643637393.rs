rust
use std::fs::File;

fn main() {
   println!("Hello world!");
   let f = std::fs::File::open("./myfile.txt").unwrap();
}
