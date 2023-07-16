plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: usage of `ty::TyKind::<kind>`
  --> src/librustdoc/scrape_examples.rs:63:16
   |
63 |         if let TyKind::FnDef(def_id, _) = ty.kind() {
   |                ^^^^^^ help: try using ty::<kind> directly: `ty`
   |
   = note: `-D rustc::usage-of-ty-tykind` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

