plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:05:53] tidy error: /checkout/src/libsyntax_pos/symbol.rs:399: line longer than 100 chars
[00:05:54] some tidy checks failed
[00:05:54]
[00:05:54]
[00:05:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:54] expected success, got: exit code: 1
[00:05:54]
[00:05:54]
[00:05:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:54] Build completed unsuccessfully in 0:02:30
[00:05:54] Makefile:79: recipe for target 'tidy' failed
[00:05:54] make: *** [tidy] Error 1
