plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/wfcheck.rs:1791:5
     |
1791 |     err.emit()
     |     ^^^^^^^^^^ expected `()`, found struct `ErrorReported`
help: consider using a semicolon here
     |
     |
1791 |     err.emit();
help: try adding a return type
     |
1767 | ) -> ErrorReported {
     |   ++++++++++++++++
