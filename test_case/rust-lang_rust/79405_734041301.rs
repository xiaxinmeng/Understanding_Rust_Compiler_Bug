
error[E0596]: cannot borrow `*vec` as mutable, as it is behind a `&` reference
 --> src/lib.rs:2:5
  |
1 | pub fn f(vec: &Vec<usize>) {
  |               ----------- help: consider changing this to be a mutable reference: `&mut Vec<usize>`
2 |     vec.push(5);
  |     ^^^ `vec` is a `&` reference, so the data it refers to cannot be borrowed as mutable
