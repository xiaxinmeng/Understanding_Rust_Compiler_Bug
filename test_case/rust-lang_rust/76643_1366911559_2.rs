
error[E0596]: cannot borrow `x.stuff` as mutable, as it is behind a `&` reference
 --> src/main.rs:9:9
  |
8 |         let mut x = self.clone();
  |             ----- consider changing this binding's type to be: `&mut MyStruct`
9 |         x.stuff.push(3);
  |         ^^^^^^^^^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable
