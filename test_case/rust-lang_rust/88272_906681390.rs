plain
   Compiling chalk-derive v0.55.0
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.25
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0382]: use of moved value: `sparse038`
   --> compiler/rustc_index/src/bit_set/tests.rs:122:20
    |
62  |     let mut sparse038: HybridBitSet<usize> = HybridBitSet::new_empty(256);
    |         ------------- move occurs because `sparse038` has type `bit_set::HybridBitSet<usize>`, which does not implement the `Copy` trait
...
107 |     let mut hybrid = sparse038;
    |                      --------- value moved here
...
122 |     let dense038 = sparse038.to_dense();
    |                    ^^^^^^^^^ value used here after move
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustc_index` due to previous error
warning: build failed, waiting for other jobs to finish...
