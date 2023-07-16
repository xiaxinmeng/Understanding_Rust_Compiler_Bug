 rust
fn ret_old_memory() -> Box<Iterator<u8>> {
  box [0u8].iter().map(|n| *n) as Box<Iterator<u8>>
}

fn main() {
  println!("{:}", ret_old_memory().next());
}
