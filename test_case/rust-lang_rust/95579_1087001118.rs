plain
.......................i............................................................................ 3800/3818
..................
failures:

---- src/slice/mod.rs - slice::[[T;N]]::flatten (line 4010) stdout ----
error[E0599]: `&[{integer}]` is not an iterator
   |
   |
19 |     assert_eq!([[1, 2], [3, 4], [5, 6]].flatten().sum(), 21);
   |                                                   ^^^ `&[{integer}]` is not an iterator; try calling `.iter()`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `&[{integer}]: Iterator`
           which is required by `&mut &[{integer}]: Iterator`
           `[{integer}]: Iterator`
           which is required by `&mut [{integer}]: Iterator`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/slice/mod.rs - slice::[[T;N]]::flatten (line 4010)
test result: FAILED. 3785 passed; 1 failed; 32 ignored; 0 measured; 0 filtered out; finished in 52.35s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:17:38
