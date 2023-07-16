plain
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error[E0433]: failed to resolve: use of undeclared type `StatementKind`
   --> compiler/rustc_ty_utils/src/ty.rs:355:25
    |
355 |                         StatementKind::StorageLive(_) | StatementKind::StorageDead(_)

error[E0433]: failed to resolve: use of undeclared type `StatementKind`
   --> compiler/rustc_ty_utils/src/ty.rs:355:57
    |
    |
355 |                         StatementKind::StorageLive(_) | StatementKind::StorageDead(_)

[RUSTC-TIMING] rustc_passes test:false 27.788
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
For more information about this error, try `rustc --explain E0433`.
