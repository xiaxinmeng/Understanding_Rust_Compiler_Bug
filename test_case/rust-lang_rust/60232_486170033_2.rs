
error[E0502]: cannot borrow `*a` as mutable because it is also borrowed as immutable
  --> src/main.rs:22:5
   |
21 |     let ref y = a; // a is borrowed as immutable.
   |         ----- immutable borrow occurs here
22 |     bar(a); // error: cannot borrow `*a` as mutable because `a` is also borrowed
   |     ^^^^^^ mutable borrow occurs here
23 |             //        as immutable
24 |     println!("y = {}", y);
   |                        - immutable borrow later used here
