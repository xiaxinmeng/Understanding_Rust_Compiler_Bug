plain
[00:00:45] configure: rust.quiet-tests     := True
---
[00:05:14] tidy error: /checkout/src/libsyntax/ext/base.rs:234: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/base.rs:243: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/base.rs:245: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/expand.rs:320: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/expand.rs:321: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/expand.rs:373: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/expand.rs:406: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/expand.rs:465: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/expand.rs:469: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/expand.rs:615: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/expand.rs:632: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/ext/expand.rs:893: TODO is deprecated; use FIXME
[00:05:14] tidy error: /checkout/src/libsyntax/ext/expand.rs:902: line longer than 100 chars
[00:05:15] some tidy checks failed
[00:05:15]
[00:05:15]
[00:05:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:15] expected success, got: exit code: 1
[00:05:15]
[00:05:15]
[00:05:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:15] Build completed unsuccessfully in 0:01:55
[00:05:15] Makefile:79: recipe for target 'tidy' failed
[00:05:15] make: *** [tidy] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot
