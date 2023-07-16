plain
    Checking tempfile v3.1.0
    Checking regex v1.4.3
    Checking json-types v0.1.0 (/checkout/src/librustdoc/json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: Prefer FxHashMap over HashMap, it has better performance
   --> src/librustdoc/json/mod.rs:219:35
    |
219 |                 std::collections::HashMap::with_capacity(len),
    |                                   ^^^^^^^ help: use: `FxHashMap`
note: the lint level is defined here
   --> src/librustdoc/lib.rs:21:9
    |
    |
21  | #![deny(rustc::internal)]
    |         ^^^^^^^^^^^^^^^
    = note: `#[deny(rustc::default_hash_types)]` implied by `#[deny(rustc::internal)]`
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
error: aborting due to previous error

error: could not compile `rustdoc`

