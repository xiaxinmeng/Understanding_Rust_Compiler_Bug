 rust
use std::io::stdio::{stdin};

fn main() {
  let contents = stdin().read_to_str();
  std::io::println(contents.clone().unwrap());
  let contents: ~[&str] = contents.unwrap().lines().collect();
  for row in contents.iter() {
    println!("Row: {:?}",*row);
  }
}
