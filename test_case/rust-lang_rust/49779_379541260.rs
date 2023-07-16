plain
Resolving deltas: 100% (613251/613251), completed with 4883 local objects.
---
[00:00:50] configure: rust.quiet-tests     := True
---
[00:05:17] tidy error: /checkout/src/test/ui/type-dependent-def-issue-49241.rs:14: line longer than 100 chars
[00:05:18] some tidy checks failed
[00:05:18]
[00:05:18]
[00:05:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:18] expected success, got: exit code: 1
[00:05:18]
[00:05:18]
[00:05:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:18] Build completed unsuccessfully in 0:02:03
[00:05:18] Makefile:79: recipe for target 'tidy' failed
[00:05:18] make: *** [tidy] Error 1
