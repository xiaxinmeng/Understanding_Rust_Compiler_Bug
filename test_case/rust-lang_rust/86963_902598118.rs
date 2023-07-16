plain
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.027
[RUSTC-TIMING] compiler_builtins test:false 1.730
[RUSTC-TIMING] cfg_if test:false 0.027
[RUSTC-TIMING] build_script_build test:false 0.228
error: target is not supported, for more information see: https://docs.rs/getrandom/#unsupported-targets
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.2.0/src/lib.rs:224:9
    |
224 | /         compile_error!("target is not supported, for more information see: \
225 | |                         https://docs.rs/getrandom/#unsupported-targets");

error[E0433]: failed to resolve: use of undeclared crate or module `imp`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.2.0/src/lib.rs:246:5
    |
---
[RUSTC-TIMING] ppv_lite86 test:false 0.352
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "wasm32-unknown-unknown" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--"


Build completed unsuccessfully in 0:15:34
