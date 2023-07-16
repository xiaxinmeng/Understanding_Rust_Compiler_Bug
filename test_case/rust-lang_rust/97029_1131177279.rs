plain
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
    --> compiler/rustc_hir/src/hir.rs:1335:41
     |
1323 |     IfLet(&'hir Let<'hir>),
...
...
1335 |             Guard::If(e) | Guard::IfLet(_, e) => e,
     |                                         ^  ^ expected 1 field, found 2
For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_hir` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_hir` due to previous error
