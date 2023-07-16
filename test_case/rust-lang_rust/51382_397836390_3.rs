
doc tests for: C:\projects\rust\src/doc/reference\src\expressions\array-expr.md
running 2 tests
test C:\projects\rust\src/doc/reference\src\expressions\array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 53) ... FAILED
test C:\projects\rust\src/doc/reference\src\expressions\array-expr.md - Array_and_array_index_expressions::Array_expressions (line 25) ... ok
failures:
---- C:\projects\rust\src/doc/reference\src\expressions\array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 53) stdout ----
error: index out of bounds: the len is 2 but the index is 10
 --> C:\projects\rust\src/doc/reference\src\expressions\array-expr.md:59:9
  |
8 | let x = (["a", "b"])[10]; // warning: const index-expr is out of bounds
  |         ^^^^^^^^^^^^^^^^
  |
  = note: #[deny(const_err)] on by default
error: index out of bounds: the len is 2 but the index is 10
  --> C:\projects\rust\src/doc/reference\src\expressions\array-expr.md:65:1
   |
14 | arr[10];                  // panics
   | ^^^^^^^
thread 'C:\projects\rust\src/doc/reference\src\expressions\array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 53)' panicked at 'couldn't compile the test', librustdoc\test.rs:325:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    C:\projects\rust\src/doc/reference\src\expressions\array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 53)
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
