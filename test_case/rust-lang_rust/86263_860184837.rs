plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: usage of `ty::TyKind::<kind>`
    --> src/librustdoc/html/render/print_item.rs:1584:44
     |
1584 |                         let ident = if let TyKind::Adt(adt, _) = ty_layout.ty.kind() {
     |                                            ^^^^^^ help: try using ty::<kind> directly: `ty`
     |
     = note: `-D rustc::usage-of-ty-tykind` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

