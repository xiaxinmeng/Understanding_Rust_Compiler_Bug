plain
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:33] 
[00:52:33] running 2966 tests
[00:52:50] ................F...................................................................................
[00:53:27] ....................................................................................................
[00:53:43] ....................................................................................................
[00:53:59] ....................................................................................................
[00:54:23] ....................................................................................................
---
[01:03:54] ....................................................................................................
[01:04:13] ..................................................................
[01:04:13] failures:
[01:04:13] 
[01:04:13] ---- [run-pass] run-pass/arbitrary_self_types_pointers_and_wrappers.rs stdout ----
[01:04:13] error: compilation failed!
[01:04:13] status: exit code: 101
[01:04:13] status: exit code: 101
[01:04:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/arbitrary_self_types_pointers_and_wrappers.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/arbitrary_self_types_pointers_and_wrappers.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/arbitrary_self_types_po3990968 .
2727276 ./obj/build
1967992 ./obj/build/x86_64-unknown-linux-gnu
722092 ./src
569160 ./obj/build/bootstrap
---
149124 ./src/llvm-emscripten/test
148948 ./.git/modules
148944 ./.git/modules/src
122708 ./obj/build/bootstrap/debug/incremental/bootstrap-1sil8jgb030ka
122704 ./obj/build/bootstrap/debug/incremental/bootstrap-1sil8jgb030ka/s-f0dlsc2ho6-9mypsc-2f3cmz1knheyt
110008 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
110004 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
106104 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102808 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
102808 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
101924 ./obj/build/bootstrap/debug/incremental/bootstrap-2s8ik7x786gic
101920 ./obj/build/bootstrap/debug/incremental/bootstrap-2s8ik7x786gic/s-f0dn2jz4ts-ewpepo-8chh8umqc096
89224 ./obj/build/x86_64-unknown-linux-gnu/stage1
89200 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
88080 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
88080 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
88076 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0dmznt0f1-1fxfs2a-3by8jin3pp2v3
84852 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
81100 ./obj/build/x86_64-unknown-linux-gnu/doc/std
78760 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/release
78700 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
