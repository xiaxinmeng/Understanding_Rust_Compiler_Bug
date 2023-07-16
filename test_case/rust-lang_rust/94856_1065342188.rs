plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error: missing documentation for the crate
   --> src/lib.rs:8:1
    |
8   | / #![feature(rustc_private, decl_macro, associated_type_bounds, never_type, trusted_len)]
9   | | #![allow(broken_intra_doc_links)]
10  | | #![deny(missing_docs)]
11  | | #![recursion_limit="256"]
285 | |         .collect()
286 | | }
    | |_^
    |
---

error: missing documentation for a struct
  --> src/lib.rs:70:1
   |
70 | pub struct PrintOnPanic<F: Fn() -> String>(pub F);

error: missing documentation for a struct
  --> src/lib.rs:81:1
   |
---

error: missing documentation for a function
   --> src/lib.rs:267:1
    |
267 | pub fn target_cpu(sess: &Session) -> &str {

error: missing documentation for a function
   --> src/lib.rs:272:1
    |
    |
272 | pub fn target_features(sess: &Session) -> Vec<Symbol> {

error: could not compile `rustc_codegen_gcc` due to 8 previous errors
Build completed unsuccessfully in 0:02:47
