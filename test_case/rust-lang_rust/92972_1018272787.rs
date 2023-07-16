plain
[RUSTC-TIMING] build_script_build test:false 0.574
error: field is never read: `0`
  --> library/core/src/iter/sources/empty.rs:28:23
   |
28 | struct FnReturning<T>(fn() -> T);
   |
   |
   = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] core test:false 11.151
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:09:42
