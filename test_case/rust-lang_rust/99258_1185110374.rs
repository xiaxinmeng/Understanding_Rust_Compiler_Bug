plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: the item `Visitor` is imported redundantly
     |
     |
10   | use rustc_hir::intravisit::{walk_expr, Visitor};
     |                                        ------- the item `Visitor` is already imported here
1507 |                 use rustc_hir::intravisit::Visitor;
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     |
     = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_borrowck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to previous error
Build completed unsuccessfully in 0:02:37
