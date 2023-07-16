plain
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0277]: `&&[ImplItemRef]` is not an iterator
   --> compiler/rustc_typeck/src/coherence/inherent_impls.rs:160:30
160 |             for impl_item in items {
160 |             for impl_item in items {
    |                              ^^^^^ `&&[ImplItemRef]` is not an iterator
    |
    = help: the trait `Iterator` is not implemented for `&&[ImplItemRef]`
    = note: required because of the requirements on the impl of `IntoIterator` for `&&[ImplItemRef]`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:24
