 rust
extern mod extra;
use extra::bitv::Bitv;

fn main() {
  let mut bits: ~Bitv = ~Bitv::new(5, false);
  bits[0] = true;
  //bits.set(0, true); // <-- What the above should be.
}
