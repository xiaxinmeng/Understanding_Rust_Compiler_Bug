plain
[00:54:38] ....i......................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:54:43] .........
[00:55:15] ....................................................................................................
[00:55:44] ......................................................................ii............................
[00:56:37] .................................i....................................................i.ii.......test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:57:20] ..............................................................................................iiiiii
[00:57:48] i...................................................................................................
[00:58:18] ....................................................................................................
[00:58:46] ....................................................................................................
---
[01:31:22] travis_fold:end:stage0-linkchecker

[01:31:22] travis_time:end:stage0-linkchecker:start=1524427076559403483,finish=1524427079421650858,duration=2862247375

[01:31:33] book/first-edition/print.html:12010: broken link fragment `#tymethod.description` pointing to `std/error/trait.Error.html`
[01:31:33] book/first-edition/error-handling.html:1372: broken link fragment `#tymethod.description` pointing to `std/error/trait.Error.html`
[01:31:41] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:31:41] 
[01:31:41] 
[01:31:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:31:41] expected success, got: exit code: 101
[01:31:41] expected success, got: exit code: 101
[01:31:41] 
[01:31:41] 
[01:31:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:41] Build completed unsuccessfully in 0:47:47
[01:31:41] make: *** [check] Error 1
[01:31:41] Makefile:58: recipe for target 'check' failed
