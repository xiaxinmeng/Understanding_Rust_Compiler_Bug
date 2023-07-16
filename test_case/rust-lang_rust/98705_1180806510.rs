plain
    Checking getopts v0.2.21
   Compiling clippy v0.1.64 (/checkout/src/tools/clippy)
    Checking clap_lex v0.2.2
   Compiling libz-sys v1.1.3
error[E0026]: variant `rustc_hir::ExprKind::Closure` does not have fields named `capture_clause`, `body`
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:666:17
666 |                 capture_clause, body, ..
    |                 ^^^^^^^^^^^^^^  ^^^^ variant `rustc_hir::ExprKind::Closure` does not have these fields

    Checking aho-corasick v0.7.18
    Checking aho-corasick v0.7.18
    Checking pulldown-cmark v0.9.1
    Checking bstr v0.2.13
    Checking idna v0.2.0
error[E0026]: variant `rustc_hir::ExprKind::Closure` does not have fields named `fn_decl`, `body`
   --> src/tools/clippy/clippy_utils/src/sugg.rs:793:37
    |
793 |     if let hir::ExprKind::Closure { fn_decl, body, .. } = closure.kind {
    |                                     ^^^^^^^  ^^^^ variant `rustc_hir::ExprKind::Closure` does not have these fields
    Checking getrandom v0.2.0
    Checking dirs-sys-next v0.1.2
    Checking termize v0.1.1
    Checking num_cpus v1.13.1
    Checking num_cpus v1.13.1
    Checking atty v0.2.14
    Checking filetime v0.2.14
error[E0026]: variant `rustc_hir::ExprKind::Closure` does not have a field named `body`
     |
     |
1702 |                 kind: ExprKind::Closure { body, .. },
     |                                           |
     |                                           variant `rustc_hir::ExprKind::Closure` does not have this field
     |                                           variant `rustc_hir::ExprKind::Closure` does not have this field
     |                                           help: `rustc_hir::ExprKind::Closure` has a field named `0`

error[E0026]: variant `rustc_hir::ExprKind::Closure` does not have a field named `body`
     |
     |
1789 |         ExprKind::Closure { body, .. } => is_body_identity_function(cx, cx.tcx.hir().body(body)),
     |                             |
     |                             variant `rustc_hir::ExprKind::Closure` does not have this field
     |                             variant `rustc_hir::ExprKind::Closure` does not have this field
     |                             help: `rustc_hir::ExprKind::Closure` has a field named `0`
For more information about this error, try `rustc --explain E0026`.
error: could not compile `clippy_utils` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:53
