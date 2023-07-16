plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.60 (/checkout/src/tools/clippy/clippy_lints)
error[E0425]: cannot find value `Underscore` in module `sym`
    |
964 |                     let mut ident_bind_name = sym::Underscore;
    |                                                    ^^^^^^^^^^ not found in `sym`
    |
---
1   | use rustc_ast::ExprKind::Underscore;
    |
1   | use rustc_hir::LifetimeName::Underscore;
    |
1   | use rustc_span::symbol::kw::Underscore;

For more information about this error, try `rustc --explain E0425`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:40
