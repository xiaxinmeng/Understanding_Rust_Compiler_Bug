plain
   Compiling polonius-engine v0.13.0
error: failed to run custom build command for `rustc_llvm v0.0.0 (D:\a\rust\rust\compiler\rustc_llvm)`

Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\i686-pc-windows-gnu\stage1-rustc\release\build\rustc_llvm-c966c495581f7f1a\build-script-build` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
  --- stdout
  cargo:rerun-if-env-changed=RUST_CHECK
  cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
  cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
  cargo:rerun-if-env-changed=LLVM_CONFIG
  cargo:rerun-if-changed=D:\a\rust\rust\build\i686-pc-windows-gnu\llvm\build\bin\llvm-config.exe
[RUSTC-TIMING] polonius_engine test:false 0.954
[RUSTC-TIMING] cc test:false 14.813
[RUSTC-TIMING] winapi test:false 5.496
[RUSTC-TIMING] regex_automata test:false 10.776
[RUSTC-TIMING] regex_automata test:false 10.776
[RUSTC-TIMING] regex_syntax test:false 18.309
error: build failed
Build completed unsuccessfully in 0:17:50
make: *** [Makefile:80: ci-mingw-subset-1] Error 1
