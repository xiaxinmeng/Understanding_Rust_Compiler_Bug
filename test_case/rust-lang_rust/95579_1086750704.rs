plain
.......................i............................................................................ 3800/3818
..................
failures:

---- src/slice/mod.rs - slice::[[T;N]]::flatten (line 4004) stdout ----
error[E0405]: cannot find trait `Display` in this scope
  |
  |
5 | fn print_all<T: Display>(slice: &[T]) {
  |
help: consider importing one of these items
  |
5 | use core::fmt::Display;
---
For more information about this error, try `rustc --explain E0405`.
Couldn't compile the test.

failures:
    src/slice/mod.rs - slice::[[T;N]]::flatten (line 4004)
test result: FAILED. 3785 passed; 1 failed; 32 ignored; 0 measured; 0 filtered out; finished in 50.69s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:16:33
