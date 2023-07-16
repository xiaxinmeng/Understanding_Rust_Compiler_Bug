
   Compiling playground v0.0.1 (/playground)
error[E0503]: cannot use `*current` because it was mutably borrowed
  --> src/lib.rs:18:13
   |
18 |             None => break,
   |             ^^^^
   |             |
   |             use of borrowed `current.0`
   |             borrow later used here
19 |             Some(node) => {
   |                  ---- borrow of `current.0` occurs here

error[E0499]: cannot borrow `current.0` as mutable more than once at a time
  --> src/lib.rs:19:18
   |
19 |             Some(node) => {
   |                  ^^^^ mutable borrow starts here in previous iteration of loop

error[E0506]: cannot assign to `*current` because it is borrowed
  --> src/lib.rs:21:21
   |
19 |             Some(node) => {
   |                  ---- borrow of `*current` occurs here
20 |                 if node.val == val {
21 |                     *current = node.next.take();
   |                     ^^^^^^^^
   |                     |
   |                     assignment to borrowed `*current` occurs here
   |                     borrow later used here

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0499, E0503, E0506.
For more information about an error, try `rustc --explain E0499`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
