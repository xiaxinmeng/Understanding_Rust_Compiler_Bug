rust
macro_rules! blowup {
  // (the brackets work around the performance bug)
  ([$($t:tt)+]) => { blowup!{[$($t:tt)+]};
}

fn main() {
  blowup!([a]);
}
