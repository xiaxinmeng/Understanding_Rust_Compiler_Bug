
error[E0502]: cannot borrow `*x` as mutable because it is also borrowed as immutable (Ast)
 --> x.rs:3:12
  |
2 |     let result = &*x;
  |                   -- immutable borrow occurs here
3 |     *(&mut *x) = 1;
  |            ^^ mutable borrow occurs here
4 |     result
5 | }
  | - immutable borrow ends here
