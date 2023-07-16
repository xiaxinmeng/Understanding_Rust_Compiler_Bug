plain
[00:34:45]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:34:45]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:34:45]    Compiling cc v1.0.15
[00:34:46]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
rror[E0463]: can't find crate for `core` which `std` depends on
[00:35:32] error: aborting due to previous error
[00:35:32] 
[00:35:32] For more information about this error, try `rustc --explain E0463`.
[00:35:32] For more information about this error, try `rustc --explain E0463`.
[00:35:32] error: Could not compile `ucd-util`.
[00:35:32] To learn more, run the command again with --verbose.
[00:35:32] 
[00:35:32] 
[00:35:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml"
[00:35:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml"
[00:35:32] expected success, got: exit code: 101
[00:35:32] 
[00:35:32] 
[00:35:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:35:32] Build completed unsuccessfully in 0:29:45
[00:35:32] Makefile:28: recipe for target 'all' failed
[00:35:32] make: *** [all] Error 1
397172 ./.git/objects
397132 ./.git/objects/pack
315016 ./src/llvm
249872 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
---
128708 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
128704 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
127192 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
123808 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
123804 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1t3fg562a-1u7wzzq-np04vflf21if
91300 ./obj/build/x86_64-unknown-linux-gnu/stage1
91276 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
89812 ./src/llvm/test/CodeGen
83912 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
