plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
   --> compiler/rustc_middle/src/traits/chalk.rs:210:18
    |
210 |         Box::new(ty)
    |                  ^^ expected struct `TyData`, found enum `chalk_ir::TyKind`
    |
    = note: expected struct `TyData<RustInterner<'_>>`
                 found enum `chalk_ir::TyKind<RustInterner<'tcx>>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
