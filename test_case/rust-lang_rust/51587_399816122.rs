plain
[00:55:52] ---- [run-pass] run-pass/macro-at-most-once-rep.rs stdout ----
[00:55:52] 
[00:55:52] error: compilation failed!
[00:55:52] status: exit code: 101
[00:55:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/macro-at-most-once-rep.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-at-most-once-rep/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-at-most-once-rep/auxiliary"
[00:55:52] ------------------------------------------
[00:55:52] 
[00:55:52] ------------------------------------------
[00:55:52] stderr:
[00:55:52] stderr:
[00:55:52] ------------------------------------------
[00:55:52] error: expected `*` or `+`
[00:55:52]    |
[00:55:52]    |
[00:55:52] 24 |     ($($a:ident)? ; $num:expr) => { {
[00:55:52]    |
[00:55:52]    |
[00:55:52]    = note: `?` is not a macro repetition operator
[00:55:52] 
[00:55:52] error: expected `*` or `+`
[00:55:52]    |
[00:55:52] 29 |          )?
[00:55:52]    |           ^
[00:55:52]    |
[00:55:52]    |
[00:55:52]    = note: `?` is not a macro repetition operator
[00:55:52] error: aborting due to 2 previous errors
[00:55:52] 
[00:55:52] 
[00:55:52] ------------------------------------------
---
[00:55:52] 
[00:55:52] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:52] 
[00:55:52] 
[00:55:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/File3380992 .
2490436 ./obj/build
1886572 ./obj/build/x86_64-unknown-linux-gnu
729176 ./src
549328 ./obj/build/x86_64-unknown-linux-gnu/stage0
---
143784 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
143780 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
138744 ./obj/build/bootstrap/debug/incremental
124172 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
124168 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f2aum0tdwn-13fia35-183djf9a4im2w
107672 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
103612 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/ruibcompiler_builtins/modules
32180 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
31752 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
