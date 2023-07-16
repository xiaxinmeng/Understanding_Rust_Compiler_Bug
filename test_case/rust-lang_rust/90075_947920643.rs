plain
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0308]: mismatched types
    --> compiler/rustc_resolve/src/diagnostics.rs:1163:85
     |
1163 |                 err.note(&format!("ambiguous because of {}", kind.descr()).push_str(ident))
     |                                                                                     ^^^^^ expected `&str`, found struct `rustc_span::symbol::Ident`
error[E0308]: mismatched types
    --> compiler/rustc_resolve/src/diagnostics.rs:1163:26
     |
     |
1163 |                 err.note(&format!("ambiguous because of {}", kind.descr()).push_str(ident))
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `str`, found `()`
     = note: expected reference `&str`
                found reference `&()`

error[E0308]: mismatched types
error[E0308]: mismatched types
    --> compiler/rustc_resolve/src/diagnostics.rs:1163:17
     |
1161 | /         match kind {
1162 | |             AmbiguityKind::Import => {
1163 | |                 err.note(&format!("ambiguous because of {}", kind.descr()).push_str(ident))
     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&mut DiagnosticBuilder<'_>`
1164 | |             }
1165 | |             _ => err.note(&format!("ambiguous because of {}", kind.descr())),
     | |_________- expected this to be `()`
     |
help: consider using a semicolon here
     |
     |
1163 |                 err.note(&format!("ambiguous because of {}", kind.descr()).push_str(ident));
help: consider using a semicolon here
     |
1166 |         };
     |          +
