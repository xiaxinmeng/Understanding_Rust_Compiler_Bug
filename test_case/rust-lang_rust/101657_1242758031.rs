plain
   Compiling rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
   Compiling rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
   Compiling rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0599]: no variant or associated item named `Type` found for enum `ParamKindOrd` in the current scope
    |
    |
173 |             ast::ParamKindOrd::Type => "type",
    |                                ^^^^ variant or associated item not found in `ParamKindOrd`

error[E0599]: no variant or associated item named `Const` found for enum `ParamKindOrd` in the current scope
    |
    |
174 |             ast::ParamKindOrd::Const => "const",
    |                                ^^^^^ variant or associated item not found in `ParamKindOrd`

error[E0599]: no variant or associated item named `Infer` found for enum `ParamKindOrd` in the current scope
    |
    |
175 |             ast::ParamKindOrd::Infer => "infer",
    |                                ^^^^^ variant or associated item not found in `ParamKindOrd`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_errors` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:08
