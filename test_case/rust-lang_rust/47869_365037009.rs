
error[E0502]: cannot borrow `heap` as immutable because it is also borrowed as mutable
  --> $DIR/issue-47646.rs:25:30
   |
21 |     let borrow = heap.peek_mut();
   |                  ---- mutable borrow occurs here
...
24 |         match (borrow, ()) {
   |               ------------ borrow may end up in a temporary, created here
25 |             println!("{:?}", heap); //~ ERROR cannot borrow `heap` as immutable
   |                              ^^^^ immutable borrow occurs here
...
28 |     };
   |      - temporary later dropped here, potentially using the reference

error: aborting due to previous error
