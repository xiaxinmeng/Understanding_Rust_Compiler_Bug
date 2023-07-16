plain
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0308]: mismatched types
   --> compiler/rustc_index/src/bit_set/tests.rs:123:40
    |
123 |     assert!(!sparse01358.clone().union(&dense038));
    |                                        ^^^^^^^^^ expected enum `bit_set::HybridBitSet`, found struct `bit_set::BitSet`
    |
    = note: expected reference `&bit_set::HybridBitSet<usize>`
               found reference `&bit_set::BitSet<usize>`
error[E0308]: mismatched types
   --> compiler/rustc_index/src/bit_set/tests.rs:125:43
    |
    |
125 |     assert!(sparse01358.clone().intersect(&dense038));
    |                                           ^^^^^^^^^ expected enum `bit_set::HybridBitSet`, found struct `bit_set::BitSet`
    |
    = note: expected reference `&bit_set::HybridBitSet<usize>`
               found reference `&bit_set::BitSet<usize>`
error[E0308]: mismatched types
   --> compiler/rustc_index/src/bit_set/tests.rs:127:42
    |
    |
127 |     assert!(sparse01358.clone().subtract(&dense038));
    |                                          ^^^^^^^^^ expected enum `bit_set::HybridBitSet`, found struct `bit_set::BitSet`
    |
    = note: expected reference `&bit_set::HybridBitSet<usize>`
               found reference `&bit_set::BitSet<usize>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_index` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
