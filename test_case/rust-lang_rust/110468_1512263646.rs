plain
   Compiling rustc_abi v0.0.0 (/checkout/compiler/rustc_abi)
[RUSTC-TIMING] derive_more test:false 6.395
   Compiling rustc_baked_icu_data v0.0.0 (/checkout/compiler/rustc_baked_icu_data)
[RUSTC-TIMING] fluent_bundle test:false 1.801
error: prefer `FxHashSet` over `HashSet`, it has better performance
  --> compiler/rustc_fluent_macro/src/fluent.rs:86:30
86 |     let mut previous_attrs = HashSet::new();
   |                              ^^^^^^^
   |
   |
   = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
   = note: `-D rustc::default-hash-types` implied by `-D warnings`

error: prefer `FxHashMap` over `HashMap`, it has better performance
   --> compiler/rustc_fluent_macro/src/fluent.rs:179:30
179 |     let mut previous_defns = HashMap::new();
    |                              ^^^^^^^
    |
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[RUSTC-TIMING] rustc_fluent_macro test:false 0.205
error: could not compile `rustc_fluent_macro` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_baked_icu_data test:false 0.671
