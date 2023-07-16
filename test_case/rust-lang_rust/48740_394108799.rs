
error[E0119]: conflicting implementations of trait `Reborrow<'_, '_>` for type `&mut _`:
  --> src/main.rs:22:1
   |
11 | / impl<'a, 'b, T> Reborrow<'a, 'b> for &'a mut T
12 | | where
13 | |     'a: 'b,
14 | | {
...  |
19 | |     }
20 | | }
   | |_- first implementation here
21 | 
22 |   impl<'a, 'b, T: Copy> Reborrow<'a, 'b> for T {
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
   |
   = note: downstream crates may implement trait `std::marker::Copy` for type `&mut _`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0119`.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.
