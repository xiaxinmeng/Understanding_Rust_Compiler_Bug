
error[E0596]: cannot borrow `*x` as mutable, as it is behind a `&` reference
 --> src/main.rs:3:5
  |
2 |     let x: &String = &mut String::new();
  |                      ------------------ help: consider changing this to be a mutable reference: `&mut mut String::new()`
3 |     x.push('a');
  |     ^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable
