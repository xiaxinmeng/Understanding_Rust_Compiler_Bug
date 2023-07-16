plain
[RUSTC-TIMING] build_script_build test:false 0.512
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.024
   Compiling alloc v0.0.0 (/checkout/library/alloc)
error: unused imports: `MarkerEq`, `is_dangling`
   --> library/alloc/src/rc/mod.rs:257:24
    |
257 | pub(super) use utils::{is_dangling, MarkerEq};
    |                        ^^^^^^^^^^^  ^^^^^^^^
    |
    = note: `-D unused-imports` implied by `-D warnings`
[RUSTC-TIMING] compiler_builtins test:false 1.778
error: aborting due to previous error

[RUSTC-TIMING] alloc test:false 1.313
[RUSTC-TIMING] alloc test:false 1.313
error: could not compile `alloc`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] core test:false 16.403
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "thumbv6m-none-eabi" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "-p" "alloc" "--manifest-path" "/checkout/library/alloc/Cargo.toml" "--features" "compiler-builtins-mem compiler-builtins-c" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
Build completed unsuccessfully in 0:15:51
