plain
    Checking tempfile v3.1.0
    Checking regex v1.4.3
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: Prefer FxHashSet over HashSet, it has better performance
  --> src/librustdoc/json/conversions.rs:17:23
17 | use std::collections::HashSet;
17 | use std::collections::HashSet;
   |                       ^^^^^^^ help: use: `FxHashSet`
note: the lint level is defined here
  --> src/librustdoc/lib.rs:20:9
   |
   |
20 | #![deny(rustc::internal)]
   |         ^^^^^^^^^^^^^^^
   = note: `#[deny(rustc::default_hash_types)]` implied by `#[deny(rustc::internal)]`
   = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
   --> src/librustdoc/json/conversions.rs:229:58
    |
229 | crate fn from_fn_header(header: &rustc_hir::FnHeader) -> HashSet<Modifiers> {
    |                                                          ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
   --> src/librustdoc/json/conversions.rs:230:17
    |
230 |     let mut v = HashSet::new();
    |                 ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
   --> src/librustdoc/json/conversions.rs:376:30
    |
376 |                 let mut hs = HashSet::new();
    |                              ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
   --> src/librustdoc/json/conversions.rs:380:17
380 |                 HashSet::new()
380 |                 HashSet::new()
    |                 ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
error: aborting due to 5 previous errors

error: could not compile `rustdoc`

