plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.35
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
rustc exited with signal: 9
error: could not compile `core`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2018 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C metadata=2675a9a46b5cec89 -C extra-filename=-2675a9a46b5cec89 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zsave-analysis -Cprefer-dynamic -Cembed-bitcode=yes -Z binary-dep-depinfo` (exit code: 254)
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest
Build completed unsuccessfully in 0:21:47
== clock drift check ==
  local time: Thu Oct 22 05:51:05 UTC 2020
  local time: Thu Oct 22 05:51:05 UTC 2020
  network time: Thu, 22 Oct 2020 01:02:47 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4598) (python)
