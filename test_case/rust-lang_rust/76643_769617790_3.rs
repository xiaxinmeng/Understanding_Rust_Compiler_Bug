
warning: variable does not need to be mutable
 --> src/main.rs:8:13
  |
8 |         let mut x = self.clone();
  |             ----^
  |             |
  |             help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

error[E0596]: cannot borrow `x.stuff` as mutable, as it is behind a `&` reference
 --> src/main.rs:9:9
  |
8 |         let mut x = self.clone();
  |             ----- help: consider changing this to be a mutable reference: `&mut MyStruct`
9 |         x.stuff.push(3);
  |         ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable
