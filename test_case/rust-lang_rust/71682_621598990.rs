
[RUSTC-TIMING] graphviz test:false 0.343
rustc exited with signal: 11
error: could not compile `graphviz`.

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name graphviz --edition=2018 src/libgraphviz/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C debuginfo=0 -C metadata=c7b3392305f79bf9 -C extra-filename=-c7b3392305f79bf9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/mips64el-unknown-linux-gnuabi64/release/deps --target mips64el-unknown-linux-gnuabi64 -C linker=mips64el-linux-gnuabi64-gcc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/mips64el-unknown-linux-gnuabi64/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps -Zdual-proc-macros -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Zbinary-dep-depinfo` (exit code: 254)
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] build_script_build test:false 0.164
{"reason":"build-finished","success":false}
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "mips64el-unknown-linux-gnuabi64" "-Zdual-proc-macros" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host mips64el-unknown-linux-gnuabi64 --target mips64el-unknown-linux-gnuabi64
