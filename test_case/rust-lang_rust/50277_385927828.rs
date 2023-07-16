plain
[00:57:22] .............................................................................ii.....................
[00:58:16] .........................................i....................................................i.ii..
[00:58:19] .....test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:59:02] ...............................................................................................
[00:59:20] ..iiiiiii...........................................................................................
[01:00:04] ....................................................................................................
[01:00:23] ...........................................................................
[01:00:23] test result: ok. 2956 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
[01:00:23] 
---
[01:01:48] ....................................................................................................
[01:01:58] ....................................................................................................
[01:02:10] ....................................................................................................
[01:02:19] ...i................................................................................................
[01:02:25] .i..ii................................................test [compile-fail] compile-fail/issue-22638.rs has been running for over 60 seconds
[01:02:38] ....................................................................................................
[01:02:46] ....................................................................................................
[01:02:54] ...................................................................i................................
[01:03:04] ............i.......................................................................................
---
Testing rustc_borrowck stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:29:24]     Finished release [optimized] target(s) in 0.0 secs
[01:29:24] 
[01:29:24] 
[01:29:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_borrowck" "--" "--quiet"
[01:29:24] expected success, got: signal: 9
[01:29:24] 
[01:29:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:24] Build completed unsuccessfully in 0:43:43
[01:29:24] Build completed unsuccessfully in 0:43:43
[01:29:24] make: *** [check] Error 1
[01:29:24] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:037b2c88
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.4
travis_time:start:0079fa64
$ dmesg | grep -i kill
[   11.137842] init: failsafe main process (1093) killed by TERM signal
[ 5435.594053]  [<ffffffff8108c79b>] ? do_tkill+0x8b/0xa0
travis_fold:end:after_failure.4

Done. Your build exited with 1.
