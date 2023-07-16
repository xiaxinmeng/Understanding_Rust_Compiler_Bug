rust
error[E0204]: the trait `Copy` may not be implemented for this type
  --> src/lib.rs:8:17
   |
8  | #[derive(Clone, Copy)]
   |                 ^^^^
9  | union U {
10 |     a: NoCopy,
   |     --------- this field does not implement `Copy`
