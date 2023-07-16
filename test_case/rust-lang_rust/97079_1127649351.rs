plain
    Checking clippy_lints v0.1.62 (/checkout/src/tools/clippy/clippy_lints)
error[E0614]: type `BasicBlock` cannot be dereferenced
   --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:117:48
    |
117 |             if terminator.successors().any(|s| *s == bb) {

error[E0614]: type `BasicBlock` cannot be dereferenced
   --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:443:57
    |
    |
443 |                 tdata.terminator().successors().any(|s| *s == bb)

For more information about this error, try `rustc --explain E0614`.
error: could not compile `clippy_lints` due to 2 previous errors
Build completed unsuccessfully in 0:04:09
