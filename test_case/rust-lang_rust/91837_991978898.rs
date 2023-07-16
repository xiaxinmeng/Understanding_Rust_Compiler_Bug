plain
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error: unused variable: `to_stable_hash_key`
   --> compiler/rustc_data_structures/src/stable_hasher.rs:556:5
    |
556 |     to_stable_hash_key: F,
    |     ^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_to_stable_hash_key`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_data_structures` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:11
