
error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)`
process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/release/build/rustc_llvm-32ba2d0adcc10f37/build-script-build` (exit code: 101)
--- stdout
cargo:rerun-if-changed=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config
cargo:rerun-if-env-changed=LLVM_CONFIG

--- stderr
/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config: relocation error: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config: symbol _ZN4llvm6Triple9normalizeB5cxx11ENS_9StringRefE version LLVM_8 not defined in file libLLVM-8svn.so with link time reference
thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config" "--version"
expected success, got: exit code: 127', src/build_helper/lib.rs:113:9
