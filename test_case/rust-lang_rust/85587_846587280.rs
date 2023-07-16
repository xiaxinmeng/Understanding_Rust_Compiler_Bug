plain
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
   --> compiler/rustc_middle/src/hir/map/collector.rs:245:54
    |
245 |             let dk_parent = self.definitions.def_key(item).parent.unwrap();
    |                                                      ^^^^ expected struct `rustc_span::def_id::LocalDefId`, found struct `rustc_hir::HirOwner`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle`
