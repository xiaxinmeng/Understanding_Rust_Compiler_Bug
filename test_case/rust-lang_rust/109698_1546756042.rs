plain

error[E0658]: use of unstable library feature 'os_str_bytes'
  --> src/ffi/os_str.rs:695:14
   |
13 |     unsafe { OsStr::from_os_str_bytes_unchecked(word) }
   |
   = note: see issue #111544 <https://github.com/rust-lang/rust/issues/111544> for more information
   = help: add `#![feature(os_str_bytes)]` to the crate attributes to enable

---
    src/ffi/os_str.rs - ffi::os_str::OsStr::from_os_str_bytes_unchecked (line 685)

test result: FAILED. 1116 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out; finished in 15.66s

error: doctest failed, to rerun pass `-p std --doc`
