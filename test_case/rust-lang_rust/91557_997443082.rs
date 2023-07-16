plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: unused import: `Symbol`
  --> compiler/rustc_resolve/src/late/lifetimes.rs:25:42
   |
25 | use rustc_span::symbol::{kw, sym, Ident, Symbol};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: could not compile `rustc_resolve` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
