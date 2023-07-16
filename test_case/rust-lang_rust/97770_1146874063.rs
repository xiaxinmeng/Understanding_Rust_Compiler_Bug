plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0277]: the size for values of type `(dyn std::ops::Deref<Target = [u8]> + Send + std::marker::Sync + 'static)` cannot be known at compilation time
   --> compiler/rustc_session/src/cstore.rs:170:70
    |
170 |     fn get_rlib_metadata(&self, target: &Target, filename: &Path) -> Result<MetadataRef, String>;
    |
    |
    = help: the trait `std::marker::Sized` is not implemented for `(dyn std::ops::Deref<Target = [u8]> + Send + std::marker::Sync + 'static)`
note: required by a bound in `OwnedSlice`
   --> /checkout/compiler/rustc_data_structures/src/owned_slice.rs:24:23
    |
24  | pub struct OwnedSlice<OwnWrap, T> {
    |                       ^^^^^^^ required by this bound in `OwnedSlice`

error[E0277]: the size for values of type `(dyn std::ops::Deref<Target = [u8]> + Send + std::marker::Sync + 'static)` cannot be known at compilation time
   --> compiler/rustc_session/src/cstore.rs:171:71
    |
171 |     fn get_dylib_metadata(&self, target: &Target, filename: &Path) -> Result<MetadataRef, String>;
    |
    |
    = help: the trait `std::marker::Sized` is not implemented for `(dyn std::ops::Deref<Target = [u8]> + Send + std::marker::Sync + 'static)`
note: required by a bound in `OwnedSlice`
   --> /checkout/compiler/rustc_data_structures/src/owned_slice.rs:24:23
    |
24  | pub struct OwnedSlice<OwnWrap, T> {
    |                       ^^^^^^^ required by this bound in `OwnedSlice`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_session` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_session` due to 2 previous errors
