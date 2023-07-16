plain
[00:47:34] .................................................................................................... 2000/4313
[00:47:37] .................................................................i.................................. 2100/4313
[00:47:39] .................................................................................................... 2200/4313
[00:47:43] .................................................................................................... 2300/4313
[00:47:45] ..............iiiiiiiii............................................................................. 2400/4313
[00:47:51] .................................................................................................... 2600/4313
[00:47:55] ..................................................................................................i. 2700/4313
[00:47:57] .................................................................................................... 2800/4313
[00:48:00] ..........................................................i.i..ii................................... 2900/4313
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:28] 
[01:00:28] running 106 tests
[01:00:31] i..ii..iii....i...i.........i..iii...........i.....i....ii...i.i.ii..............i...ii..ii.i....iii 100/106
[01:00:31] test result: ok. 76 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:00:31] 
[01:00:31]  finished in 3.205
[01:00:31] travis_fold:end:test_codegen
---
[01:09:57] 
[01:09:57] To learn more, run the command again with --verbose.
[01:09:57] 
[01:09:57] 
[01:09:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:09:57] 
[01:09:57] 
[01:09:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:57] Build completed unsuccessfully in 0:26:35
[01:09:57] Build completed unsuccessfully in 0:26:35
[01:09:57] Makefile:58: recipe for target 'check' failed
[01:09:57] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00b427e3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
