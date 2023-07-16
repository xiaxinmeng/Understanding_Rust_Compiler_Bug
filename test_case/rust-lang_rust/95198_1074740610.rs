plain
....................i............................................................................... 3800/3815
...............
failures:

---- src/slice/mod.rs - slice::[T]::get_chunk_mut (line 1211) stdout ----
error[E0596]: cannot borrow data in a `&` reference as mutable
  |
  |
6 | let slice = &mut *b"hello world";
  |             ^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
