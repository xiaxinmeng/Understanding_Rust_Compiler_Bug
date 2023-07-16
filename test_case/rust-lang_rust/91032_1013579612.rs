plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 1 field
    --> compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_build.rs:178:29
     |
178  |             | ExprKind::Let(_, _, _)
     |                             ^  ^  ^ expected 1 field, found 3
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1709:9
     |
     |
1709 |     Let(&'hir Let<'hir>),
     |         --------------- tuple variant has 1 field
For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
