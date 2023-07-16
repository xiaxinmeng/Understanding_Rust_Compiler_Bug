plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.6.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.65 (/checkout/src/tools/clippy/clippy_lints)
error[E0277]: `Cell<bool>` cannot be shared between threads safely
    |
    |
865 |     store.register_late_pass(|| Box::new(only_used_in_recursion::OnlyUsedInRecursion::default()));
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Cell<bool>` cannot be shared between threads safely
    |
    = help: within `only_used_in_recursion::Param`, the trait `std::marker::Sync` is not implemented for `Cell<bool>`
note: required because it appears within the type `only_used_in_recursion::Param`
   --> src/tools/clippy/clippy_lints/src/only_used_in_recursion.rs:98:8
98  | struct Param {
    |        ^^^^^
    |        ^^^^^
    = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<only_used_in_recursion::Param>`
    = note: required because it appears within the type `alloc::raw_vec::RawVec<only_used_in_recursion::Param>`
    = note: required because it appears within the type `std::vec::Vec<only_used_in_recursion::Param>`
note: required because it appears within the type `Params`
   --> src/tools/clippy/clippy_lints/src/only_used_in_recursion.rs:137:8
137 | struct Params {
    |        ^^^^^^
    |        ^^^^^^
note: required because it appears within the type `OnlyUsedInRecursion`
   --> src/tools/clippy/clippy_lints/src/only_used_in_recursion.rs:215:12
    |
215 | pub struct OnlyUsedInRecursion {
    |            ^^^^^^^^^^^^^^^^^^^
    = note: required for the cast from `OnlyUsedInRecursion` to the object type `dyn for<'tcx> LateLintPass<'tcx> + Send + std::marker::Sync`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:04:07
