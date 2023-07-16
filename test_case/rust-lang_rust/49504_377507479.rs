plain
[00:00:48] configure: rust.quiet-tests     := True
---
[00:04:26] tidy error: /checkout/src/librustdoc/html/render.rs:1463: line longer than 100 chars
[00:04:28] some tidy checks failed
[00:04:28]
[00:04:28]
[00:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:28] expected success, got: exit code: 1
[00:04:28]
[00:04:28]
[00:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:28] Build completed unsuccessfully in 0:01:52
[00:04:28] make: *** [tidy] Error 1
[00:04:28] Makefile:79: recipe for target 'tidy' failed
