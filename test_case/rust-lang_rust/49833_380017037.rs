plain
Resolving deltas: 100% (613484/613484), completed with 4872 local objects.
---
[00:00:47] configure: rust.quiet-tests     := True
---
[00:04:31] tidy error: /checkout/src/librustc/ty/maps/on_disk_cache.rs:633: line longer than 100 chars
[00:04:32] some tidy checks failed
[00:04:32]
[00:04:32]
[00:04:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:32] expected success, got: exit code: 1
[00:04:32]
[00:04:32]
[00:04:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:32] Build completed unsuccessfully in 0:01:46
[00:04:32] make: *** [tidy] Error 1
[00:04:32] Makefile:79: recipe for target 'tidy' failed
