plain
2019-11-15T16:18:01.9065887Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-15T16:18:01.9268851Z ##[command]git config gc.auto 0
2019-11-15T16:18:01.9351695Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-15T16:18:01.9408997Z ##[command]git config --get-all http.proxy
2019-11-15T16:18:01.9557993Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66389/merge:refs/remotes/pull/66389/merge
---
2019-11-15T17:15:14.1183115Z .................................................................................................... 1500/9243
2019-11-15T17:15:20.3819490Z .................................................................................................... 1600/9243
2019-11-15T17:15:29.6795527Z .................................................................................................... 1700/9243
2019-11-15T17:15:38.4871785Z .....i.............................................................................................. 1800/9243
2019-11-15T17:15:45.2715059Z .........................................................................................iiiii...... 1900/9243
2019-11-15T17:16:06.7153339Z .................................................................................................... 2100/9243
2019-11-15T17:16:09.1251794Z .................................................................................................... 2200/9243
2019-11-15T17:16:11.6750538Z .................................................................................................... 2300/9243
2019-11-15T17:16:18.0531443Z .................................................................................................... 2400/9243
---
2019-11-15T17:19:14.6081630Z ........................................................................................i........... 4700/9243
2019-11-15T17:19:21.2832715Z ....i............................................................................................... 4800/9243
2019-11-15T17:19:30.4309779Z .................................................................................................... 4900/9243
2019-11-15T17:19:35.7605945Z .................................................................................................... 5000/9243
2019-11-15T17:19:46.4301259Z ............................................................................................ii.ii... 5100/9243
2019-11-15T17:19:55.4297145Z ............................i....................................................................... 5300/9243
2019-11-15T17:20:03.6852649Z .................................................................................................... 5400/9243
2019-11-15T17:20:12.1213781Z ..........................................................................i......................... 5500/9243
2019-11-15T17:20:19.7478398Z .................................................................................................... 5600/9243
2019-11-15T17:20:19.7478398Z .................................................................................................... 5600/9243
2019-11-15T17:20:26.4726864Z .................................................................................................... 5700/9243
2019-11-15T17:20:36.2161778Z ............................................................ii...i..ii...........i.................. 5800/9243
2019-11-15T17:20:58.4493155Z .................................................................................................... 6000/9243
2019-11-15T17:21:06.8552813Z .................................................................................................... 6100/9243
2019-11-15T17:21:06.8552813Z .................................................................................................... 6100/9243
2019-11-15T17:21:11.6401581Z ...............................................................................i..ii................ 6200/9243
2019-11-15T17:21:40.4023063Z .................................................................................................... 6400/9243
2019-11-15T17:21:43.6786698Z ...............................................i.................................................... 6500/9243
2019-11-15T17:21:45.9060855Z .................................................................................................... 6600/9243
2019-11-15T17:21:48.3729364Z ..................................i................................................................. 6700/9243
---
2019-11-15T17:26:30.7525453Z 13 
2019-11-15T17:26:30.7525482Z 
2019-11-15T17:26:30.7525524Z 
2019-11-15T17:26:30.7525568Z The actual stderr differed from the expected stderr.
2019-11-15T17:26:30.7526100Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-66387-if-without-else/issue-66387-if-without-else.stderr
2019-11-15T17:26:30.7526817Z To update references, rerun the tests and pass the `--bless` flag
2019-11-15T17:26:30.7527203Z To only update this specific test, also pass `--test-args async-await/issue-66387-if-without-else.rs`
2019-11-15T17:26:30.7527304Z error: 1 errors occurred comparing output.
2019-11-15T17:26:30.7527375Z status: exit code: 1
2019-11-15T17:26:30.7527375Z status: exit code: 1
2019-11-15T17:26:30.7528269Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-66387-if-without-else.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-66387-if-without-else" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-66387-if-without-else/auxiliary" "-A" "unused"
2019-11-15T17:26:30.7528672Z ------------------------------------------
2019-11-15T17:26:30.7528710Z 
2019-11-15T17:26:30.7528992Z ------------------------------------------
2019-11-15T17:26:30.7529042Z stderr:
---
2019-11-15T17:26:30.7571177Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-15T17:26:30.7571298Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-15T17:26:30.7592884Z 
2019-11-15T17:26:30.7594255Z 
2019-11-15T17:26:30.7595959Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-15T17:26:30.7596679Z 
2019-11-15T17:26:30.7596710Z 
2019-11-15T17:26:30.7615745Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-15T17:26:30.7615826Z Build completed unsuccessfully in 1:02:07
2019-11-15T17:26:30.7615826Z Build completed unsuccessfully in 1:02:07
2019-11-15T17:26:30.7660686Z == clock drift check ==
2019-11-15T17:26:30.7675495Z   local time: Fri Nov 15 17:26:30 UTC 2019
2019-11-15T17:26:31.0473958Z   network time: Fri, 15 Nov 2019 17:26:31 GMT
2019-11-15T17:26:31.0478806Z == end clock drift check ==
2019-11-15T17:26:31.8830609Z 
2019-11-15T17:26:31.8935211Z ##[error]Bash exited with code '1'.
2019-11-15T17:26:31.8980280Z ##[section]Starting: Checkout
2019-11-15T17:26:31.8983195Z ==============================================================================
2019-11-15T17:26:31.8983244Z Task         : Get sources
2019-11-15T17:26:31.8983304Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
