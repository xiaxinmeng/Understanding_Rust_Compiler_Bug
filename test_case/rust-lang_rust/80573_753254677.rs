plain
    Checking rustc-rayon v0.3.0
    Checking tempfile v3.1.0
    Checking regex v1.3.9
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: usage of qualified `ty::TyCtxt<'_>`
  --> src/librustdoc/passes/collect_intra_doc_links.rs:87:24
   |
87 |     fn name(self, tcx: ty::TyCtxt<'_>) -> String {
   |                        ^^^^^^^^^^^^^^ help: try using it unqualified: `TyCtxt<'_>`
   |
   = note: `-D rustc::usage-of-qualified-ty` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

