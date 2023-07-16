plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0004]: non-exhaustive patterns: `rustc_type_ir::sty::TyKind::Alias(AliasKind::Weak, _)` not covered
    --> src/librustdoc/clean/mod.rs:1707:11
     |
1707 |     match *bound_ty.skip_binder().kind() {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `rustc_type_ir::sty::TyKind::Alias(AliasKind::Weak, _)` not covered
     |
note: `rustc_type_ir::sty::TyKind<rustc_middle::ty::TyCtxt<'_>>` defined here
    --> /checkout/compiler/rustc_type_ir/src/sty.rs:202:5
     |
56   | pub enum TyKind<I: Interner> {
...
...
202  |     Alias(AliasKind, I::AliasTy),
     |     ^^^^^ not covered
     = note: the matched value is of type `rustc_type_ir::sty::TyKind<rustc_middle::ty::TyCtxt<'_>>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
     |
1863 ~         ty::Error(_) => rustc_errors::FatalError.raise(),
1864 ~         rustc_type_ir::sty::TyKind::Alias(AliasKind::Weak, _) => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:01:56
