plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unexpected `cfg` condition value
   --> library/core/src/ptr/const_ptr.rs:252:17
    |
252 |         if cfg!(target_pointer_width = "128") {
    |
    |
    = note: `-D unexpected-cfgs` implied by `-D warnings`
    = note: expected values for `target_pointer_width` are: 16, 32, 64
error: unexpected `cfg` condition value
   --> library/core/src/ptr/mut_ptr.rs:256:17
    |
    |
256 |         if cfg!(target_pointer_width = "128") {
    |
    = note: expected values for `target_pointer_width` are: 16, 32, 64

error: could not compile `core` due to 2 previous errors
