plain

error: associated function is never used: `name_cstr`
   --> library/std/src/sys/unix/fs.rs:679:8
    |
679 |     fn name_cstr(&self) -> &CStr {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] std test:false 3.074
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
Build completed unsuccessfully in 0:16:53
