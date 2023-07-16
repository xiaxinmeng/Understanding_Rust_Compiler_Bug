plain
[00:42:57] .....................................................i..............................................
[00:43:01] .........................................................................ii.........................
[00:43:07] ....................................................................................................
[00:43:13] ...................................................................................i................
[00:43:15] .iiiiiiiii...................................................
[00:43:15] 
[00:43:15] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:44:01] .....................................................i..............................................
[00:44:05] .........................................................................ii.........................
[00:44:10] ....................................................................................................
[00:44:16] ...................................................................................i................
[00:44:18] .iiiiiiiii...................................................
[00:44:18] 
[00:44:18]  finished in 62.980
[00:44:18] travis_fold:end:test_ui_nll

---
[01:17:20] 
[01:17:20] To learn more, run the command again with --verbose.
[01:17:20] 
[01:17:20] 
[01:17:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:17:20] 
[01:17:20] 
[01:17:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:20] Build completed unsuccessfully in 0:36:30
[01:17:20] Build completed unsuccessfully in 0:36:30
[01:17:20] Makefile:58: recipe for target 'check' failed
[01:17:20] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:072c2d5a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
