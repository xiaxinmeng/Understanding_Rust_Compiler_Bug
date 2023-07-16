plain
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0308]: mismatched types
   --> compiler/rustc_errors/src/diagnostic_builder.rs:230:41
    |
230 |                 handler.emit_diagnostic(&db.inner.diagnostic);
    |                                         ^^^^^^^^^^^^^^^^^^^^ types differ in mutability
    = note: expected mutable reference `&mut diagnostic::Diagnostic`
    = note: expected mutable reference `&mut diagnostic::Diagnostic`
                       found reference `&Box<diagnostic::Diagnostic>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_errors` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
