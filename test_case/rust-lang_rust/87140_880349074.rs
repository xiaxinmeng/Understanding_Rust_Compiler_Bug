plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error[E0433]: failed to resolve: use of undeclared type `ConstKind`
   --> compiler/rustc_mir/src/const_eval/mod.rs:166:26
    |
166 |         ty::Const { val: ConstKind::Value(val), ty: field_op.layout.ty }
    |
help: consider importing one of these items
    |
3   | use crate::transform::required_consts::ConstKind;
