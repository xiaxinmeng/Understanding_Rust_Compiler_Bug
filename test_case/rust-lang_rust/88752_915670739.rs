plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: the feature `bindings_after_at` has been stable since 1.54.0 and no longer requires an attribute to enable
  |
  |
1 | #![cfg_attr(bootstrap, feature(bindings_after_at))]
  |
  |
  = note: `-D stable-features` implied by `-D warnings`
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:51
