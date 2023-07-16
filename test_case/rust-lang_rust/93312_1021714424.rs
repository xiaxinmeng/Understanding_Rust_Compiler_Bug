plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0412]: cannot find type `FxIndexMap` in this scope
   --> compiler/rustc_middle/src/ty/mod.rs:138:22
    |
138 |     pub trait_impls: FxIndexMap<DefId, Vec<LocalDefId>>,
    |
help: consider importing this type alias
    |
    |
12  | use rustc_data_structures::fx::FxIndexMap;

    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
For more information about this error, try `rustc --explain E0412`.
error: could not compile `rustc_middle` due to previous error
