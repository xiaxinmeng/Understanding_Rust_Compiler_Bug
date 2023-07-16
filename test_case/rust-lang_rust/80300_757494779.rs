plain
    Checking rustc-rayon v0.3.0
    Checking tempfile v3.1.0
    Checking regex v1.3.9
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: the item `DefIdTree` is imported redundantly
   --> src/librustdoc/visit_ast.rs:111:21
    |
13  | use rustc_middle::ty::{DefIdTree, TyCtxt};
    |                        --------- the item `DefIdTree` is already imported here
...
111 |                 use rustc_middle::ty::DefIdTree;
    |
    |
    = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

