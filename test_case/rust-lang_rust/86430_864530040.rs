plain
   Compiling lock_api v0.4.1
   Compiling tracing-core v0.1.17
   Compiling thread_local v1.0.1
   Compiling sharded-slab v0.1.1
error: failed to run custom build command for `log v0.4.14`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/log-95ca93bce313e7e3/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit status: 101
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
