plain
.......................i............................................................................ 3800/3818
..................
failures:

---- src/slice/mod.rs - slice::[[T;N]]::flatten (line 4010) stdout ----
    --> src/slice/mod.rs:4027:58
     |
     |
19   |     assert_eq!([[1, 2], [3, 4], [5, 6]].flatten().iter().sum(), 21);
     |                                                          ^^^ cannot infer type for type parameter `S` declared on the associated function `sum`
     |
     = note: cannot satisfy `_: Sum<&i32>`
note: required by a bound in `std::iter::Iterator::sum`
     |
     |
3293 |         S: Sum<Self::Item>,
     |            ^^^^^^^^^^^^^^^ required by this bound in `std::iter::Iterator::sum`
help: consider specifying the type argument in the method call
     |
19   |     assert_eq!([[1, 2], [3, 4], [5, 6]].flatten().iter().sum::<S>(), 21);
help: consider specifying the type argument in the function call
     |
     |
19   |     assert_eq!([[1, 2], [3, 4], [5, 6]].flatten().iter().sum::<S>(), 21);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
For more information about this error, try `rustc --explain E0283`.
Couldn't compile the test.

failures:
    src/slice/mod.rs - slice::[[T;N]]::flatten (line 4010)
test result: FAILED. 3785 passed; 1 failed; 32 ignored; 0 measured; 0 filtered out; finished in 51.96s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:17:28
