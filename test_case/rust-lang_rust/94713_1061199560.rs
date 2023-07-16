plain
...................i......................i......................i.................................. 3700/3759
...........................................................
failures:

---- src/num/mod.rs - num::u16::is_char_surrogate (line 828) stdout ----
error[E0599]: no method named `is_surrogate` found for type `{integer}` in the current scope
  |
  |
9 | assert!(!low_non_surrogate.is_surrogate());
  |                            ^^^^^^^^^^^^ method not found in `{integer}`

error[E0599]: no method named `is_surrogate` found for type `{integer}` in the current scope
   |
   |
10 | assert!(low_surrogate.is_surrogate());
   |                       ^^^^^^^^^^^^ method not found in `{integer}`

error[E0599]: no method named `is_surrogate` found for type `{integer}` in the current scope
   |
   |
11 | assert!(high_surrogate.is_surrogate());
   |                        ^^^^^^^^^^^^ method not found in `{integer}`

error[E0599]: no method named `is_surrogate` found for type `{integer}` in the current scope
   |
   |
12 | assert!(!high_non_surrogate.is_surrogate());
   |                             ^^^^^^^^^^^^ method not found in `{integer}`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/num/mod.rs - num::u16::is_char_surrogate (line 828)
test result: FAILED. 3726 passed; 1 failed; 32 ignored; 0 measured; 0 filtered out; finished in 50.50s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:16:32
