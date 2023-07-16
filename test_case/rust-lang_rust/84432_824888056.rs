plain
   Compiling addr2line v0.14.0
[RUSTC-TIMING] addr2line test:false 0.377
[RUSTC-TIMING] core test:false 16.004
[RUSTC-TIMING] gimli test:false 4.436
rustc exited with signal: 11 (core dumped)
error: could not compile `gimli`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name gimli --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/gimli-0.23.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 --cfg 'feature="alloc"' --cfg 'feature="compiler_builtins"' --cfg 'feature="core"' --cfg 'feature="read"' --cfg 'feature="rustc-dep-of-std"' -C metadata=2cd1eab9998ac7a3 -C extra-filename=-2cd1eab9998ac7a3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/riscv64gc-unknown-linux-gnu/release/deps --target riscv64gc-unknown-linux-gnu -C linker=riscv64-unknown-linux-gnu-gcc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/riscv64gc-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/riscv64gc-unknown-linux-gnu/release/deps/libcompiler_builtins-22e557f04ace6b03.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/riscv64gc-unknown-linux-gnu/release/deps/librustc_std_workspace_alloc-ebe86155c9f35ea7.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/riscv64gc-unknown-linux-gnu/release/deps/librustc_std_workspace_core-72d74fb9181ca8a7.rmeta --cap-lints allow -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zsave-analysis -Cprefer-dynamic -Cembed-bitcode=yes -Cforce-unwind-tables=yes -Z binary-dep-depinfo` (exit code: 254)
[RUSTC-TIMING] object test:false 9.065
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "riscv64gc-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
