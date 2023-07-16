plain
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
   Compiling rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: this trait bound has already been specified
    |
    |
222 |         F: Float + Into<Scalar<M::PointerTag>> + FloatConvert<Single> + FloatConvert<Double>,
    |
    |
    = note: `-D duplicate-bounds` implied by `-D warnings`
    = help: consider removing this trait bound
error: could not compile `rustc_mir` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:08:50
