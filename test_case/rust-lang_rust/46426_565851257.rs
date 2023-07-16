
error: unreachable statement
  --> src/main.rs:13:9
   |
8  | /         match e1 {
9  | |             &1 => {
10 | |                 continue 'label;
11 | |             }
12 | |         }
   | |_________- any code following this `match` expression is unreachable, as all arms diverge
13 |           println!("e1: {:?}", e1);
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
   |
note: lint level defined here
  --> src/main.rs:1:9
   |
1  | #![deny(unreachable_code)]
   |         ^^^^^^^^^^^^^^^^
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
