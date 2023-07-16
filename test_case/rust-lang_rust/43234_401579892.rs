rust
error[E0499]: cannot borrow `self.bar` as mutable more than once at a time
  --> src/bin/nll.rs:25:23
   |
25 |             let baz = self.bar.get_baz_mut();
   |                       ^^^^^^^^ mutable borrow starts here in previous iteration of loop
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the method body at 23:5...
  --> src/bin/nll.rs:23:5
   |
23 | /     fn foo(&mut self) -> Option<&mut Baz> {
24 | |         for _i in 0..4 {
25 | |             let baz = self.bar.get_baz_mut();
26 | |             if baz.x == 0 {
...  |
30 | |         None
31 | |     }
   | |_____^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.
error: Could not compile `rust-playground`.

To learn more, run the command again with --verbose.
