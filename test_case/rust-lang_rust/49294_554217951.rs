
error[E0308]: mismatched types
  --> src/lib.rs:10:13
   |
5  | pub extern "C" fn main(_argc: isize, _arg: *const *const u8) -> isize {
   |                                                                 ----- expected `isize` because of return type
...
10 |             break;
   |             ^^^^^
   |             |
   |             expected isize, found ()
   |             help: give it a value of the expected type: `break 42`
   |
   = note: expected type `isize`
              found type `()`
