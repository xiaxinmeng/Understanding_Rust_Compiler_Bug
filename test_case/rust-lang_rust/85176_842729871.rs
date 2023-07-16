plain
* highest error code: E0783
Found 517 error codes
Found 0 error codes with no tests
Done!
thread '<unnamed>' panicked at 'cmd.exec() failed with Error during execution of `cargo metadata`:     Updating git repository `https://github.com/bjorn3/rust-ar.git`
    Updating git submodule `https://github.com/WebAssembly/WASI`
warning: spurious network error (2 tries remaining): failed to connect to github.com: Connection refused; class=Os (2)
warning: spurious network error (1 tries remaining): failed to connect to github.com: Connection refused; class=Os (2)
warning: spurious network error (1 tries remaining): failed to connect to github.com: Connection refused; class=Os (2)
error: failed to get `cranelift-codegen` as a dependency of package `rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)`
Caused by:
  failed to load source for dependency `cranelift-codegen`

Caused by:
Caused by:
  Unable to update https://github.com/bytecodealliance/wasmtime/?branch=main#45bee40f

Caused by:
  failed to update submodule `WASI`

Caused by:
  failed to fetch submodule `WASI` from https://github.com/WebAssembly/WASI
Caused by:
  failed to connect to github.com: Connection refused; class=Os (2)
', src/tools/tidy/src/deps.rs:306:20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any', src/tools/tidy/src/main.rs:88:9


command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "14"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:05:25
