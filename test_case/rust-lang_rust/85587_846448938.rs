plain
    Checking clippy_utils v0.1.54 (/checkout/src/tools/clippy/clippy_utils)
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_utils/src/usage.rs:25:13
   |
25 |             expr.hir_id.owner,
   |             ^^^^^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirOwner`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_utils`
