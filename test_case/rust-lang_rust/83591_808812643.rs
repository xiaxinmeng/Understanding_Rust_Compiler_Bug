plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: Prefer FxHashMap over HashMap, it has better performance
  --> src/librustdoc/passes/calculate_doc_coverage.rs:14:23
14 | use std::collections::HashMap;
14 | use std::collections::HashMap;
   |                       ^^^^^^^ help: use: `FxHashMap`
note: the lint level is defined here
  --> src/librustdoc/lib.rs:19:9
   |
   |
19 | #![deny(rustc::internal)]
   |         ^^^^^^^^^^^^^^^
   = note: `#[deny(rustc::default_hash_types)]` implied by `#[deny(rustc::internal)]`
   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: Prefer FxHashMap over HashMap, it has better performance
   --> src/librustdoc/passes/calculate_doc_coverage.rs:103:12
    |
103 |     items: HashMap<FileName, ItemCount>,
    |            ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: Prefer FxHashMap over HashMap, it has better performance
   --> src/librustdoc/passes/calculate_doc_coverage.rs:124:28
    |
124 |                 .collect::<HashMap<String, &ItemCount>>(),
    |                            ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
error: aborting due to 3 previous errors

error: could not compile `rustdoc`

