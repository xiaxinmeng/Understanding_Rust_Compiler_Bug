
error[E0308]: mismatched types
  --> src\lib.rs:18:29
   |
18 |      pub static ref c: [u8; a_b] = [ 1, 2];
   |                             ^^^
   |                             |
   |                             expected usize, found struct `a_b`
   |                             help: consider dereferencing the type: `*a_b`
   |
   = note: expected type `usize`
              found type `a_b`
