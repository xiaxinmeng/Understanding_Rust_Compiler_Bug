plain
    Checking pulldown-cmark v0.9.2
    Checking bstr v0.2.17
    Checking tester v0.9.0
    Checking idna v0.2.0
error[E0004]: non-exhaustive patterns: `&ImplTraitInTrait(_)` not covered
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:944:15
944  |         match ty {
944  |         match ty {
     |               ^^ pattern `&ImplTraitInTrait(_)` not covered
note: `rustc_hir::TyKind` defined here
    --> /checkout/compiler/rustc_hir/src/hir.rs:2555:5
     |
     |
2529 | pub enum TyKind<'hir> {
...
...
2555 |     ImplTraitInTrait(ItemId),
     = note: the matched value is of type `&rustc_hir::TyKind`
     = note: the matched value is of type `&rustc_hir::TyKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
     |
991  ~             TyKind::Err | TyKind::Infer | TyKind::Never => {}
992  ~             &ImplTraitInTrait(_) => todo!(),

    Checking regex v1.5.6
For more information about this error, try `rustc --explain E0004`.
error: could not compile `clippy_utils` due to previous error
