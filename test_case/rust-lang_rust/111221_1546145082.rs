plain

error[E0433]: failed to resolve: use of undeclared type `ConstVariableValue`
   --> compiler/rustc_infer/src/infer/nll_relate/mod.rs:915:34
    |
915 | ...                   val: ConstVariableValue::Unknown { universe: self.universe },
    |                            ^^^^^^^^^^^^^^^^^^ use of undeclared type `ConstVariableValue`
error[E0422]: cannot find struct, variant or union type `ConstVarValue` in this scope
   --> compiler/rustc_infer/src/infer/nll_relate/mod.rs:913:65
    |
    |
913 |                         let new_var_id = variable_table.new_key(ConstVarValue {
    |
help: consider importing one of these items
    |
24  + use crate::infer::ConstVarValue;
24  + use crate::infer::ConstVarValue;
    |
24  + use rustc_middle::infer::unify_key::ConstVarValue;

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0433]: failed to resolve: use of undeclared crate or module `relate`
