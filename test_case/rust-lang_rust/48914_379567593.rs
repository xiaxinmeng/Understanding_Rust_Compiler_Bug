plain
Resolving deltas: 100% (612769/612769), completed with 4866 local objects.
---
[00:00:42] configure: rust.quiet-tests     := True
---
[00:05:01] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1578: trailing whitespace
[00:05:01] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1582: trailing whitespace
[00:05:01] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1641: trailing whitespace
[00:05:02] some tidy checks failed
[00:05:02]
[00:05:02]
[00:05:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:02] expected success, got: exit code: 1
[00:05:02]
[00:05:02]
[00:05:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:02] Build completed unsuccessfully in 0:02:00
[00:05:02] Makefile:79: recipe for target 'tidy' failed
[00:05:02] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: canno
