rust
error[E0596]: cannot borrow `*me` as mutable, as it is behind a `&` reference
 --> src/main.rs:3:5
  |
2 | fn not_using_mut(me: &Vec<u8>) {
  |                      -------- help: consider changing this to be a mutable reference: `&mut Vec<u8>`
3 |     me.push(4);
  |     ^^ `me` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error: aborting due to previous error
