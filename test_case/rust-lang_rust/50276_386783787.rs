plain
[00:48:10]   --> tools/compiletest/src/main.rs:14:9
[00:48:10]    |
[00:48:10] 14 | #![deny(warnings)]
[00:48:10]    |         ^^^^^^^^
[00:48:10]    = note: #[deny(unused_imports)] implied by #[deny(warnings)]
unknown-linux-gnu/release/deps/libdiff-ea92613c7599e347.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/liblibc-9f809ae84e8276de.rlib --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f668a40d2fd37628.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libenv_logger-cc9002545f326a3e.rlib --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-3ed5916ccf0904d0.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/liblog-a2eefc5a1ecdfa18.rlib --extern serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps/libserde_derive-473cada37922c0a7.so` (exit code: 101)
[00:48:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/compiletest/Cargo.toml" "--features" "" "--message-format" "json"
[00:48:11] expected success, got: exit code: 101
[00:48:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:11] Build completed unsuccessfully in 0:01:02
[00:48:11] Makefile:58: recipe for target 'check' failed
[00:48:11] make: *** [check] Error 1
75436 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/release
71232 ./.git/modules/src/tools
70976 ./obj/build/x86_64-unknown-linux-gnu/native
70596 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
70596 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
70300 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
68772 ./src/llvm/lib
65424 ./src/llvm-emscripten/test/CodeGen
60840 ./src/llvm-emscripten/lib
56092 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
55380 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53660 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33ta18b3panbi
53656 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33ta18b3panbi/s-f0qtwax8q2-1lsc0lx-wp0t9vh6a0ar
48604 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
47892 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
47052 ./src/test
46720 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
