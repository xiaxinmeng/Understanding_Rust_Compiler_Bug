plain
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0433]: failed to resolve: use of undeclared type `SymbolMangling`
    --> compiler/rustc_session/src/config.rs:2031:15
     |
2031 |         (Some(SymbolMangling::V0), _) => {}
     |               ^^^^^^^^^^^^^^ use of undeclared type `SymbolMangling`
    Checking chalk-engine v0.55.0
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_session` due to previous error
warning: build failed, waiting for other jobs to finish...
