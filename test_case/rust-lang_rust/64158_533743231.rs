plain
2019-09-20T22:12:00.8424495Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-20T22:12:00.8646099Z ##[command]git config gc.auto 0
2019-09-20T22:12:00.8735854Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-20T22:12:00.8812474Z ##[command]git config --get-all http.proxy
2019-09-20T22:12:00.8966424Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64158/merge:refs/remotes/pull/64158/merge
---
2019-09-20T23:14:56.4323507Z .................................................................................................... 1500/9031
2019-09-20T23:15:02.4855008Z .................................................................................................... 1600/9031
2019-09-20T23:15:15.3002262Z .....................................................................i...............i.............. 1700/9031
2019-09-20T23:15:22.1415020Z .................................................................................................... 1800/9031
2019-09-20T23:15:37.7395644Z ............................................................iiiii................................... 1900/9031
2019-09-20T23:15:50.5187092Z .................................................................................................... 2100/9031
2019-09-20T23:15:53.2214569Z .................................................................................................... 2200/9031
2019-09-20T23:15:56.5654112Z .................................................................................................... 2300/9031
2019-09-20T23:16:05.3591412Z .................................................................................................... 2400/9031
---
2019-09-20T23:19:08.6759397Z ................................................i...............i................................... 4700/9031
2019-09-20T23:19:18.3761915Z .................................................................................................... 4800/9031
2019-09-20T23:19:26.4708471Z .................................................................................................... 4900/9031
2019-09-20T23:19:36.3806585Z .................................................................................................... 5000/9031
2019-09-20T23:19:44.4928268Z ................................ii.ii............................................................... 5100/9031
2019-09-20T23:19:54.1841978Z .................................................................................................... 5300/9031
2019-09-20T23:20:04.9937326Z ................................................................................................i... 5400/9031
2019-09-20T23:20:13.7266877Z .................................................................................................... 5500/9031
2019-09-20T23:20:18.5447658Z .................................................................................................... 5600/9031
2019-09-20T23:20:18.5447658Z .................................................................................................... 5600/9031
2019-09-20T23:20:29.5196907Z ...........................................................................................ii...i..i 5700/9031
2019-09-20T23:20:44.5133854Z i...........i....................................................................................... 5800/9031
2019-09-20T23:21:05.7969370Z .................................................................................................... 6000/9031
2019-09-20T23:21:05.7969370Z .................................................................................................... 6000/9031
2019-09-20T23:21:12.0666379Z .............................................................................................i..ii.. 6100/9031
2019-09-20T23:21:40.8365879Z .................................................................................................... 6300/9031
2019-09-20T23:21:45.6495875Z ....................................................i............................................... 6400/9031
2019-09-20T23:21:47.9126777Z .................................................................................................... 6500/9031
2019-09-20T23:21:50.4072955Z ........................i........................................................................... 6600/9031
---
2019-09-20T23:26:01.5781210Z 
2019-09-20T23:26:01.5781352Z 18 testing321
2019-09-20T23:26:01.5781775Z 19 thread 'main' panicked at 'assertion failed: `(left == right)`
2019-09-20T23:26:01.5782001Z 20   left: `2`,
2019-09-20T23:26:01.5782385Z -  right: `5`', $DIR/test-panic-abort.rs:27:5
2019-09-20T23:26:01.5782799Z +  right: `5`', $DIR/test-panic-abort.rs:30:5
2019-09-20T23:26:01.5783143Z 23 
2019-09-20T23:26:01.5783315Z 24 
2019-09-20T23:26:01.5783436Z 
2019-09-20T23:26:01.5783719Z 
2019-09-20T23:26:01.5783719Z 
2019-09-20T23:26:01.5783844Z The actual run.stdout differed from the expected run.stdout.
2019-09-20T23:26:01.5784436Z Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/test-panic-abort.run.stdout
2019-09-20T23:26:01.5784747Z error: 1 errors occured comparing run output.
2019-09-20T23:26:01.5784904Z status: exit code: 101
2019-09-20T23:26:01.5784904Z status: exit code: 101
2019-09-20T23:26:01.5785287Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a" "--test-threads=1"
2019-09-20T23:26:01.5785789Z ------------------------------------------
2019-09-20T23:26:01.5785932Z 
2019-09-20T23:26:01.5786079Z running 5 tests
2019-09-20T23:26:01.5786203Z test it_exits ... FAILED
---
2019-09-20T23:26:01.5822473Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-20T23:26:01.5822554Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-20T23:26:01.5836131Z 
2019-09-20T23:26:01.5836211Z 
2019-09-20T23:26:02.1537423Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-20T23:26:02.1538012Z 
2019-09-20T23:26:02.1538057Z 
2019-09-20T23:26:02.1538198Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-20T23:26:02.1538268Z Build completed unsuccessfully in 1:06:58
2019-09-20T23:26:02.1538268Z Build completed unsuccessfully in 1:06:58
2019-09-20T23:26:02.1538310Z == clock drift check ==
2019-09-20T23:26:02.1538606Z   local time: Fri Sep 20 23:26:01 UTC 2019
2019-09-20T23:26:02.1538820Z   network time: Fri, 20 Sep 2019 23:26:01 GMT
2019-09-20T23:26:02.1538893Z == end clock drift check ==
2019-09-20T23:26:02.7785275Z ##[error]Bash exited with code '1'.
2019-09-20T23:26:02.7822756Z ##[section]Starting: Checkout
2019-09-20T23:26:02.7824711Z ==============================================================================
2019-09-20T23:26:02.7824783Z Task         : Get sources
2019-09-20T23:26:02.7824827Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
