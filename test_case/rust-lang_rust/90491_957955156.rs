plain
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.28
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0599]: no method named `last_set_before` found for reference `&bit_set::BitSet<usize>` in the current scope
   --> compiler/rustc_index/src/bit_set/tests.rs:423:24
    |
423 |         assert_eq!(set.last_set_before(needle), easy(set, needle));
    |                        ^^^^^^^^^^^^^^^ method not found in `&bit_set::BitSet<usize>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_index` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
