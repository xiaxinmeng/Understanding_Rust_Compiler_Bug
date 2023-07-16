plain
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
[RUSTC-TIMING] build_script_build test:false 0.159
[RUSTC-TIMING] build_script_build test:false 0.199
[RUSTC-TIMING] build_script_build test:false 0.432
error[E0599]: no function or associated item named `wrap_mut_2` found for struct `NeverShortCircuit` in the current scope
    |
    |
251 | / macro_rules! zip_impl_general_defaults {
252 | |     () => {
253 | |         default fn new(a: A, b: B) -> Self {
254 | |             Zip {
...   |
298 | |             ZipImpl::try_fold(&mut self, init, NeverShortCircuit::wrap_mut_2(f)).0
    | |                                                                   |
    | |                                                                   |
    | |                                                                   function or associated item not found in `NeverShortCircuit<_>`
    | |                                                                   help: there is an associated function with a similar name: `wrap_mut_2_imp`
332 | |     };
333 | | }
333 | | }
    | |_- in this expansion of `zip_impl_general_defaults!`
...
344 |       zip_impl_general_defaults! {}
    |
   ::: library/core/src/ops/try_trait.rs:379:1
    |
    |
379 |   pub(crate) struct NeverShortCircuit<T>(pub T);
    |   -------------------------------------- function or associated item `wrap_mut_2` not found for this struct

error[E0599]: no function or associated item named `wrap_mut_2` found for struct `NeverShortCircuit` in the current scope
    |
    |
251 | / macro_rules! zip_impl_general_defaults {
252 | |     () => {
253 | |         default fn new(a: A, b: B) -> Self {
254 | |             Zip {
...   |
298 | |             ZipImpl::try_fold(&mut self, init, NeverShortCircuit::wrap_mut_2(f)).0
    | |                                                                   |
    | |                                                                   |
    | |                                                                   function or associated item not found in `NeverShortCircuit<_>`
    | |                                                                   help: there is an associated function with a similar name: `wrap_mut_2_imp`
332 | |     };
333 | | }
333 | | }
    | |_- in this expansion of `zip_impl_general_defaults!`
...
377 |       zip_impl_general_defaults! {}
    |
   ::: library/core/src/ops/try_trait.rs:379:1
    |
    |
379 |   pub(crate) struct NeverShortCircuit<T>(pub T);
    |   -------------------------------------- function or associated item `wrap_mut_2` not found for this struct
For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] core test:false 7.338
error: could not compile `core` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
