plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0053]: method `into_diagnostic` has an incompatible type for trait
   --> compiler/rustc_passes/src/errors.rs:711:15
    |
711 |         sess: &'_ rustc_session::parse::ParseSess,
    |               |
    |               |
    |               expected struct `Handler`, found struct `ParseSess`
    |
    |
    = note: expected fn pointer `fn(InvalidAttrAtCrateLevel, &Handler) -> DiagnosticBuilder<'_, _>`
               found fn pointer `fn(InvalidAttrAtCrateLevel, &ParseSess) -> DiagnosticBuilder<'_, _>`
For more information about this error, try `rustc --explain E0053`.
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_passes` due to previous error
