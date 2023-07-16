plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0004]: non-exhaustive patterns: `Unsized(_)` not covered
   --> src/librustdoc/clean/mod.rs:135:15
135 |         match *self {
135 |         match *self {
    |               ^^^^^ pattern `Unsized(_)` not covered
   ::: /checkout/compiler/rustc_hir/src/hir.rs:404:5
    |
404 |     Unsized(Span),
    |     ------- not covered
