 rust
use std::comm;

fn main() {
  let (output_port, _) = comm::stream::<int>();
  let ports = ~[output_port];
  for ports.each |&p| {}
}
