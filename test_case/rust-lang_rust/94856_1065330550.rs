plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error: missing documentation for the crate
   --> src/lib.rs:1:1
    |
1   | / #![feature(rustc_private, decl_macro)]
2   | | #![cfg_attr(feature = "jit", feature(never_type, vec_into_raw_parts, once_cell))]
3   | | #![deny(missing_docs)]
...   |
...   |
311 | |     Box::new(CraneliftCodegenBackend { config: None })
    | |_^
    |
note: the lint level is defined here
   --> src/lib.rs:3:9
