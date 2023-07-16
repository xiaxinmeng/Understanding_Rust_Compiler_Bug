plain
....i......................i......................i......................i......................i... 3700/3790
..........................................................................................
failures:

---- src/num/mod.rs - num::u16::is_utf16_surrogate (line 828) stdout ----
error[E0689]: can't call method `is_utf16_surrogate` on ambiguous numeric type `{integer}`
  |
  |
4 | let low_non_surrogate = 0xA000;
  |     ----------------- you must specify a type for this binding, like `i32`
...
9 | assert!(!low_non_surrogate.is_utf16_surrogate());


error[E0689]: can't call method `is_utf16_surrogate` on ambiguous numeric type `{integer}`
   |
   |
5  | let low_surrogate = 0xD800;
   |     ------------- you must specify a type for this binding, like `i32`
...
10 | assert!(low_surrogate.is_utf16_surrogate());


error[E0689]: can't call method `is_utf16_surrogate` on ambiguous numeric type `{integer}`
   |
   |
6  | let high_surrogate = 0xDC00;
   |     -------------- you must specify a type for this binding, like `i32`
...
11 | assert!(high_surrogate.is_utf16_surrogate());


error[E0689]: can't call method `is_utf16_surrogate` on ambiguous numeric type `{integer}`
   |
   |
7  | let high_non_surrogate = 0xE000;
   |     ------------------ you must specify a type for this binding, like `i32`
...
12 | assert!(!high_non_surrogate.is_utf16_surrogate());

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0689`.
For more information about this error, try `rustc --explain E0689`.
Couldn't compile the test.

failures:
    src/num/mod.rs - num::u16::is_utf16_surrogate (line 828)
test result: FAILED. 3757 passed; 1 failed; 32 ignored; 0 measured; 0 filtered out; finished in 60.99s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:21:10
