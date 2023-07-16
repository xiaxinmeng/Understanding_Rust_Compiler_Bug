plain
[RUSTC-TIMING] snapbox test:false 4.298
[RUSTC-TIMING] cargo_test_support test:false 6.713
warning: `cargo-test-support` (lib) generated 1 warning
   Compiling cargo v0.65.0 (/checkout/src/tools/cargo)
error: custom attribute panicked
 --> src/tools/cargo/tests/testsuite/init/simple_hg/mod.rs:7:1
  |
7 | #[cargo_test(requires_hg)]
  |
  |
  = help: message: expected command `hg` to be somewhere in PATH: No such file or directory (os error 2)
[RUSTC-TIMING] internal test:true 1.487
warning: unused import: `cargo_test_support::compare::assert_ui`
 --> src/tools/cargo/tests/testsuite/init/simple_hg/mod.rs:1:5
  |
---
error: could not compile `cargo` due to previous error; 4 warnings emitted
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] cargo test:true 24.393
Build completed unsuccessfully in 0:27:05
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
