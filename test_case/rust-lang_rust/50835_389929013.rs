plain
[00:59:21] ........................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:59:32] ............................................
[00:59:46] ...................................................................................................i
[01:00:32] i...............................................................i...................................
[01:00:51] ................i.ii...............................................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:01:32] ........................iiiiiii.....................................................................
[01:01:49] ....................................................................................................
[01:02:04] ....................................................................................................
[01:02:21] .................................................................................................
---
[01:33:37] travis_fold:end:stage0-linkchecker

[01:33:37] travis_time:end:stage0-linkchecker:start=1526574718333820815,finish=1526574721503254092,duration=3169433277

[01:33:53] reference/tokens.html:586: broken link fragment `#symbols` pointing to `grammar.html`
[01:33:53] reference/types.html:326: broken link fragment `#type-definitions` pointing to `grammar.html`
[01:33:53] reference/notation.html:150: broken link fragment `#special-unicode-productions` pointing to `grammar.html`
[01:33:53] reference/print.html:182: broken link fragment `#special-unicode-productions` pointing to `grammar.html`
[01:33:53] reference/print.html:906: broken link fragment `#symbols` pointing to `grammar.html`
[01:33:53] reference/print.html:5408: broken link fragment `#type-definitions` pointing to `grammar.html`
[01:33:55] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:33:55] 
[01:33:55] 
[01:33:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:33:55] expected success, got: exit code: 101
[01:33:55] expected success, got: exit code: 101
[01:33:55] 
[01:33:55] 
[01:33:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:55] Build completed unsuccessfully in 0:44:16
[01:33:55] make: *** [check] Error 1
[01:33:55] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:004a6358
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
