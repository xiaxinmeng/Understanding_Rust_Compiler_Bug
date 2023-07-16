
error[E0502]: cannot borrow `a` as mutable because it is also borrowed as immutable
 --> tmp.rs:4:6
  |
3 |     let borrow = &a;
  |                   - immutable borrow occurs here
4 |     (|| {
  |      ^^ mutable borrow occurs here
5 |         &a;
  |          - borrow occurs due to use of `a` in closure
...
8 | }
  | - immutable borrow ends here
