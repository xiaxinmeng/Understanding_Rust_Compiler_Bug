plain
Resolving deltas: 100% (613145/613145), completed with 4855 local objects.
---
[00:00:45] configure: rust.quiet-tests     := True
---
[00:05:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:47] expected success, got: exit code: 1
[00:05:47]
[00:05:47]
[00:05:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:47] Build completed unsuccessfully in 0:02:40
[00:05:47] Makefile:79: recipe for target 'tidy' failed
[00:05:47] make: *** [tidy] Error 1
