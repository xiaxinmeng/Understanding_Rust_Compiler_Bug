plain
...................i................................................................................ 3800/3814
..............
failures:

---- src/num/mod.rs - num::u16::is_utf16_surrogate (line 828) stdout ----
error[E0658]: use of unstable library feature 'utf16_extra'
  |
  |
9 | assert!(!low_non_surrogate.is_utf16_surrogate());
  |
  = note: see issue #94919 <https://github.com/rust-lang/rust/issues/94919> for more information
  = note: see issue #94919 <https://github.com/rust-lang/rust/issues/94919> for more information
  = help: add `#![feature(utf16_extra)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'utf16_extra'
   |
   |
10 | assert!(low_surrogate.is_utf16_surrogate());
   |
   = note: see issue #94919 <https://github.com/rust-lang/rust/issues/94919> for more information
   = note: see issue #94919 <https://github.com/rust-lang/rust/issues/94919> for more information
   = help: add `#![feature(utf16_extra)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'utf16_extra'
   |
   |
11 | assert!(high_surrogate.is_utf16_surrogate());
   |
   = note: see issue #94919 <https://github.com/rust-lang/rust/issues/94919> for more information
   = note: see issue #94919 <https://github.com/rust-lang/rust/issues/94919> for more information
   = help: add `#![feature(utf16_extra)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'utf16_extra'
   |
   |
12 | assert!(!high_non_surrogate.is_utf16_surrogate());
   |
   = note: see issue #94919 <https://github.com/rust-lang/rust/issues/94919> for more information
   = note: see issue #94919 <https://github.com/rust-lang/rust/issues/94919> for more information
   = help: add `#![feature(utf16_extra)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/num/mod.rs - num::u16::is_utf16_surrogate (line 828)
test result: FAILED. 3781 passed; 1 failed; 32 ignored; 0 measured; 0 filtered out; finished in 60.94s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:19:44
