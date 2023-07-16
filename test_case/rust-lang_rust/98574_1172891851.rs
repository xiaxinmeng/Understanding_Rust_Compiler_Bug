plain
    Checking bstr v0.2.13
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:120:54
    |
120 |                     && both(&le, &re, |l, r| if let (Some(l), Some(r)) = (l, r) {
    |                                                      ^^^^^^^             ------ this expression has type `(&&rustc_hir::Block<'_>, &&rustc_hir::Block<'_>)`
    |                                                      expected struct `rustc_hir::Block`, found enum `Option`
    |
    = note: expected struct `rustc_hir::Block<'_>`
                 found enum `Option<_>`
                 found enum `Option<_>`

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:120:63
    |
120 |                     && both(&le, &re, |l, r| if let (Some(l), Some(r)) = (l, r) {
    |                                                               ^^^^^^^    ------ this expression has type `(&&rustc_hir::Block<'_>, &&rustc_hir::Block<'_>)`
    |                                                               expected struct `rustc_hir::Block`, found enum `Option`
    |
    = note: expected struct `rustc_hir::Block<'_>`
                 found enum `Option<_>`
