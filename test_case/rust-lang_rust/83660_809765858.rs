plain
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
34 | #![feature(type_alias_impl_trait)]
   |
   |
   = note: `-D incomplete-features` implied by `-D warnings`

error: aborting due to previous error

error: could not compile `rustc_mir`
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:16:41
