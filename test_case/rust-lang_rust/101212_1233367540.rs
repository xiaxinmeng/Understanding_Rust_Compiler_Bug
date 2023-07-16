plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   --> compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs:181:29
    |
181 |                 ty::Dynamic(data, _) => {
    |
    |
   ::: /checkout/compiler/rustc_type_ir/src/sty.rs:119:13
    |
119 |     Dynamic(I::ListBinderExistentialPredicate, I::Region, DynKind),
    |
help: use `_` to explicitly ignore each field
    |
    |
181 |                 ty::Dynamic(data, _, _) => {

    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_typeck` due to previous error
