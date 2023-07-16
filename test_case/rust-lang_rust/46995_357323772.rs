
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
   Compiling rustc_llvm v0.0.0 (file:///home/maglab/artiq-dev/artiq_3.1/rust/src/librustc_llvm)
error: failed to run custom build command for `rustc_llvm v0.0.0 (file:///home/maglab/artiq-dev/artiq_3.1/rust/src/librustc_llvm)`
process didn't exit successfully: `/home/maglab/artiq-dev/artiq_3.1/rust/build/build/x86_64-unknown-linux-gnu/stage2-tools/release/build/rustc_llvm-5f6e96f94cb8b094/build-script-build` (exit code: 1)
--- stdout
cargo:rerun-if-changed=llvm-config
cargo:rerun-if-env-changed=LLVM_CONFIG


**failed to execute command: "llvm-config" "--version"**
error: No such file or directory (os error 2)
