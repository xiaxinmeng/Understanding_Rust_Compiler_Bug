plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/traits/chalk.rs:77:5
77  |     type DefId = DefId;
77  |     type DefId = DefId;
    |     ^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
    |
note: required by a bound in `chalk_ir::interner::Interner::DefId`
    |
    |
191 |     type DefId: Debug + Copy + Eq + Ord + Hash;
    |                                     ^^^ required by this bound in `chalk_ir::interner::Interner::DefId`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
