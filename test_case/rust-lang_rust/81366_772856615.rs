plain
    Checking toml v0.5.7
    Checking url v2.1.1
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.51 (/checkout/src/tools/clippy/clippy_lints)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_lints/src/collapsible_match.rs:89:36
     |
89   |             Some(Guard::If(expr) | Guard::IfLet(_, expr)) => {
     |                                    ^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
     | 
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1206:5
     |
1206 |     IfLet(&'hir Pat<'hir>, &'hir Expr<'hir>, Span),
     |     ---------------------------------------------- tuple variant defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_lints`
