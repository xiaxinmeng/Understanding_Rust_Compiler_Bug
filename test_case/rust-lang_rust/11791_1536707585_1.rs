rust
let x: u64 = ...;
match x {
   XX::A as u64 => XX::A,
  XX::B as u64 => XX::B,
   XX::C as u64 => XX::C,
  _=> panic!()
}
