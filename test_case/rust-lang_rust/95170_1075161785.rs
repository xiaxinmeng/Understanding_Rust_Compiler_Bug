
cargo +beta build --bins
   Compiling bootstrap v0.0.0 (C:\Users\Walther\git\rust\src\bootstrap)
   Compiling same-file v1.0.6
   Compiling lzma-sys v0.1.16
   Compiling tar v0.4.37
   Compiling serde_derive v1.0.125
   Compiling globset v0.4.5
   Compiling tempfile v3.2.0
error: failed to run custom build command for `bootstrap v0.0.0 (C:\Users\Walther\git\rust\src\bootstrap)`

Caused by:
  process didn't exit successfully: `C:\Users\Walther\git\rust\target\debug\build\bootstrap-7757a4777dec0f86\build-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=build.rs
  cargo:rerun-if-env-changed=RUSTC
  cargo:rustc-env=BUILD_TRIPLE=x86_64-pc-windows-msvc
  cargo:rerun-if-env-changed=PATH

  --- stderr
  thread 'main' panicked at 'assertion failed: rustc.is_absolute()', src\bootstrap\build.rs:22:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: build failed
