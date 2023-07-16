
error[E0502]: cannot borrow `v` as immutable because it is also borrowed as mutable
 --> borrow.rs:5:20
  |
4 |     { r = &mut v; }
  |                - mutable borrow occurs here
5 |     println!("{}", v);
  |                    ^ immutable borrow occurs here
6 | }
  | - mutable borrow ends here

error: aborting due to previous error
