plain
[RUSTC-TIMING] rustc_expand test:false 16.153
error: field is never read: `up_to`
   --> compiler/rustc_infer/src/traits/mod.rs:103:5
    |
103 |     up_to: usize,
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] rustc_infer test:false 3.605
error: could not compile `rustc_infer` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_symbol_mangling test:false 5.236
