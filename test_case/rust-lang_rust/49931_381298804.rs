plain
Resolving deltas: 100% (614320/614320), completed with 4875 local objects.
---
[00:00:44] configure: rust.quiet-tests     := True
---
[00:04:39] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1663: line longer than 100 chars
[00:04:41] some tidy checks failed
[00:04:41]
[00:04:41]
[00:04:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:41] expected success, got: exit code: 1
[00:04:41]
[00:04:41]
[00:04:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:41] Build completed unsuccessfully in 0:01:53
[00:04:41] make: *** [tidy] Error 1
[00:04:41] Makefile:79: recipe for target 'tidy' failed
