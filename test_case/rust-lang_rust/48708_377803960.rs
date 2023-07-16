plain
Resolving deltas: 100% (609997/609997), completed with 4802 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:04:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:19] expected success, got: exit code: 1
[00:04:19]
[00:04:19]
[00:04:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:19] Build completed unsuccessfully in 0:01:44
[00:04:19] Makefile:79: recipe for target 'tidy' failed
[00:04:19] make: *** [tidy] Error 1
