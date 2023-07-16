plain
Resolving deltas: 100% (611484/611484), completed with 4863 local objects.
---
[00:00:47] configure: rust.quiet-tests     := True
---
[00:04:38] tidy error: /checkout/src/test/run-pass/edition-keywords-2018.rs:24: trailing whitespace
[00:04:40] some tidy checks failed
[00:04:40]
[00:04:40]
[00:04:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:40] expected success, got: exit code: 1
[00:04:40]
[00:04:40]
[00:04:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:40] Build completed unsuccessfully in 0:01:54
[00:04:40] Makefile:79: recipe for target 'tidy' failed
[00:04:40] make: *** [tidy] Error 1
