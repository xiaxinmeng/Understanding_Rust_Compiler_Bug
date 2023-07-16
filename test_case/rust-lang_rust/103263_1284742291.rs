plain
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0004]: non-exhaustive patterns: `rustc_hir::def::DefKind::Trait` and `rustc_hir::def::DefKind::AssocTy` not covered
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1063:11
1063 |     match def_kind {
1063 |     match def_kind {
     |           ^^^^^^^^ patterns `rustc_hir::def::DefKind::Trait` and `rustc_hir::def::DefKind::AssocTy` not covered
note: `rustc_hir::def::DefKind` defined here
     = note: the matched value is of type `rustc_hir::def::DefKind`
     = note: the matched value is of type `rustc_hir::def::DefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
1094 ~         | DefKind::InlineConst => false,
1094 ~         | DefKind::InlineConst => false,
1095 ~         rustc_hir::def::DefKind::Trait | rustc_hir::def::DefKind::AssocTy => todo!(),

For more information about this error, try `rustc --explain E0004`.
[RUSTC-TIMING] rustc_metadata test:false 1.788
error: could not compile `rustc_metadata` due to previous error
