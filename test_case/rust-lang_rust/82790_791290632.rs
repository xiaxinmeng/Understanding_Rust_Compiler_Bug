plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMN8oPavJ/Pw

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
   Compiling rustc-main v0.0.0 (/Users/runner/work/rust/rust/compiler/rustc)
error[E0308]: mismatched types
  --> compiler/rustc/src/main.rs:39:50
   |
39 |             static _F7: unsafe extern "C" fn() = _rjem_je_zone_register();
   |
   |
   = note: expected fn pointer `unsafe extern "C" fn()`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] rustc_main test:false 0.533
error: could not compile `rustc-main`

To learn more, run the command again with --verbose.
command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-Zbinary-dep-depinfo" "-j" "3" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/Users/runner/work/rust/rust/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:42:35
