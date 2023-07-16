plain
   Compiling tracing-attributes v0.1.13
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.55.0
[RUSTC-TIMING] syn test:false 7.649
error: prefer `FxHashMap` over `HashMap`, it has better performance
   --> compiler/rustc_macros/src/session_diagnostic.rs:138:30
138 |         let mut fields_map = HashMap::new();
    |                              ^^^^^^^
    |
    |
    = note: `-D rustc::default-hash-types` implied by `-D warnings`
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: prefer `FxHashMap` over `HashMap`, it has better performance
   --> compiler/rustc_macros/src/session_diagnostic.rs:262:13
    |
262 |     fields: HashMap<String, &'a syn::Field>,
    |
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: prefer `FxHashMap` over `HashMap`, it has better performance
   --> compiler/rustc_macros/src/symbols.rs:132:9
    |
132 |         HashMap::<String, Span>::with_capacity(input.keywords.len() + input.symbols.len() + 10);
    |
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[RUSTC-TIMING] chalk_derive test:false 0.970
error: aborting due to 3 previous errors

[RUSTC-TIMING] rustc_macros test:false 0.665
