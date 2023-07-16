
   Compiling playground v0.0.1 (/playground)
warning: value assigned to `a` is never read
 --> src/main.rs:6:5
  |
6 |     a += 1;
  |     ^
  |
  = note: `#[warn(unused_assignments)]` on by default
  = help: maybe it is overwritten before being read?

error[E0506]: cannot assign to `a` because it is borrowed
 --> src/main.rs:6:5
  |
4 |     let closure = || a;
  |                   -- - borrow occurs due to use in closure
  |                   |
  |                   borrow of `a` occurs here
5 |     
6 |     a += 1;
  |     ^^^^^^ assignment to borrowed `a` occurs here
7 |     
8 |     closure();
  |     ------- borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.

