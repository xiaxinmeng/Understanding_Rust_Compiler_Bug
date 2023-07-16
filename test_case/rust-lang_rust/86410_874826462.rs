plain
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0720]: cannot resolve opaque type
   --> compiler/rustc_middle/src/ty/sty.rs:656:31
    |
656 |     ) -> impl Iterator<Item = impl Iterator<Item = Ty<'tcx>> + Captures<'tcx>> {
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot resolve opaque type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0720`.
error: could not compile `rustc_middle`
