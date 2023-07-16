
error: cannot borrow immutable argument `b` as mutable (it's an immutable reference)`.
 --> <anon>:2:18
  |
1 | fn foo(b: &mut u64) {
  |        - use `mut b` here to make mutable (Or see another option below):
2 |     let x = &mut b;
  |                  ^ cannot borrow mutably, but can move it out by removing '&mut'.
