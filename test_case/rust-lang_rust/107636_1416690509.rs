plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0004]: non-exhaustive patterns: `&rustc_type_ir::TyKind::GeneratorWitnessMIR(_, _)` not covered
   --> compiler/rustc_hir_analysis/src/coherence/orphan.rs:160:49
    |
160 |         let (local_impl, nonlocal_impl) = match self_ty.kind() {
    |                                                 ^^^^^^^^^^^^^^ pattern `&rustc_type_ir::TyKind::GeneratorWitnessMIR(_, _)` not covered
    |
note: `rustc_type_ir::TyKind<TyCtxt<'_>>` defined here
   --> /checkout/compiler/rustc_type_ir/src/sty.rs:187:5
    |
50  | pub enum TyKind<I: Interner> {
...
...
187 |     GeneratorWitnessMIR(I::DefId, I::SubstsRef),
    |     ^^^^^^^^^^^^^^^^^^^ not covered
    = note: the matched value is of type `&rustc_type_ir::TyKind<TyCtxt<'_>>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
239 ~             ty::Error(..) => (LocalImpl::Allow, NonlocalImpl::Allow),
240 ~             &rustc_type_ir::TyKind::GeneratorWitnessMIR(_, _) => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_hir_analysis` due to previous error
warning: build failed, waiting for other jobs to finish...
