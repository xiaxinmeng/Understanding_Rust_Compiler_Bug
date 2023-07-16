 rust
use core::rand::{ IsaacRng, RngUtil };

fn main() {
  let r = IsaacRng::new();
  io::println(fmt!("%u", r.gen()));
}
