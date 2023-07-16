plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0603]: function `check_unused_or_stable_features` is private
   --> src/librustdoc/core.rs:326:30
    |
326 |     rustc_passes::stability::check_unused_or_stable_features(tcx);
    |
note: the function `check_unused_or_stable_features` is defined here
   --> /checkout/compiler/rustc_passes/src/stability.rs:679:1
    |
    |
679 | fn check_unused_or_stable_features(tcx: TyCtxt<'_>) {

For more information about this error, try `rustc --explain E0603`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:23
