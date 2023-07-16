
error[[E0502]](https://doc.rust-lang.org/nightly/error-index.html#E0502): cannot borrow `*collection` as mutable because it is also borrowed as immutable
 --> src/lib.rs:5:32
  |
5 |             if collection[j] < collection[least_element] {
  |                ----------------^^^^^^^^^^---------------
  |                |               |
  |                |               mutable borrow occurs here
  |                immutable borrow occurs here
  |                immutable borrow later used here
  |
help: try adding a local storing this...
 --> src/lib.rs:5:32
  |
5 |             if collection[j] < collection[least_element] {
  |                                ^^^^^^^^^^^^^^^^^^^^^^^^^
help: ...and then using that local here
 --> src/lib.rs:5:16
  |
5 |             if collection[j] < collection[least_element] {
  |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0502`.
error: could not compile `playground` due to previous error
