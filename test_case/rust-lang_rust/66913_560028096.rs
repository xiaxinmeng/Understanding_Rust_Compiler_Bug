plain
2019-11-30T21:44:20.4608754Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T21:44:20.4660661Z ##[command]git config gc.auto 0
2019-11-30T21:44:20.4664253Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T21:44:20.4673047Z ##[command]git config --get-all http.proxy
2019-11-30T21:44:20.4677472Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66913/merge:refs/remotes/pull/66913/merge
---
2019-11-30T22:47:34.6875138Z .................................................................................................... 1600/9313
2019-11-30T22:47:39.7636008Z .................................................................................................... 1700/9313
2019-11-30T22:47:53.1212133Z ........................................i........................................................... 1800/9313
2019-11-30T22:48:01.5595684Z .................................................................................................... 1900/9313
2019-11-30T22:48:16.2024605Z .........................iiiii...................................................................... 2000/9313
2019-11-30T22:48:27.0897179Z .................................................................................................... 2200/9313
2019-11-30T22:48:29.9225921Z .................................................................................................... 2300/9313
2019-11-30T22:48:34.7759581Z .................................................................................................... 2400/9313
2019-11-30T22:48:57.7854542Z .................................................................................................... 2500/9313
---
2019-11-30T22:51:48.4438517Z ...........................i...............i........................................................ 4800/9313
2019-11-30T22:51:59.6855923Z .................................................................................................... 4900/9313
2019-11-30T22:52:06.2214599Z .................................................................................................... 5000/9313
2019-11-30T22:52:14.8258194Z .................................................................................................... 5100/9313
2019-11-30T22:52:23.0330189Z ................................ii.ii...........i................................................... 5200/9313
2019-11-30T22:52:33.1285737Z .................................................................................................... 5400/9313
2019-11-30T22:52:43.8420861Z .................................................................................................... 5500/9313
2019-11-30T22:52:51.4042647Z ..............i..................................................................................... 5600/9313
2019-11-30T22:52:58.1482153Z .................................................................................................... 5700/9313
2019-11-30T22:52:58.1482153Z .................................................................................................... 5700/9313
2019-11-30T22:53:09.9724709Z .................................................................................................... 5800/9313
2019-11-30T22:53:22.6502902Z ii...i..ii...........i.............................................................................. 5900/9313
2019-11-30T22:53:41.9133690Z .................................................................................................... 6100/9313
2019-11-30T22:53:49.9013511Z .................................................................................................... 6200/9313
2019-11-30T22:53:49.9013511Z .................................................................................................... 6200/9313
2019-11-30T22:54:08.5019417Z .......................i..ii........................................................................ 6300/9313
2019-11-30T22:54:29.5682620Z ...........................................................................................i........ 6500/9313
2019-11-30T22:54:32.0570968Z .................................................................................................... 6600/9313
2019-11-30T22:54:34.4751011Z ..................................................................................i................. 6700/9313
2019-11-30T22:54:37.3589964Z .................................................................................................... 6800/9313
---
2019-11-30T22:59:55.9032033Z diff of stderr:
2019-11-30T22:59:55.9033524Z 
2019-11-30T22:59:55.9034291Z 2   --> $DIR/suggest-self-2.rs:5:9
2019-11-30T22:59:55.9034416Z 3    |
2019-11-30T22:59:55.9034618Z 4 LL |         bar(self);
2019-11-30T22:59:55.9035275Z -    |         ^^^ help: try calling method instead of passing `self` as parameter: `self.bar`
2019-11-30T22:59:55.9036007Z +    |
2019-11-30T22:59:55.9036260Z + help: try calling a method instead of passing `self` as parameter
2019-11-30T22:59:55.9036885Z +    |
2019-11-30T22:59:55.9037259Z + LL |         self.bar(self);
2019-11-30T22:59:55.9037259Z + LL |         self.bar(self);
2019-11-30T22:59:55.9037511Z +    |         ^^^^^^^^
2019-11-30T22:59:55.9037766Z 6 
2019-11-30T22:59:55.9038018Z 7 error[E0425]: cannot find function `bar` in this scope
2019-11-30T22:59:55.9038611Z 8   --> $DIR/suggest-self-2.rs:10:9
2019-11-30T22:59:55.9038927Z 
2019-11-30T22:59:55.9039211Z 9    |
2019-11-30T22:59:55.9039453Z 10 LL |         bar(&self);
2019-11-30T22:59:55.9040061Z -    |         ^^^ help: try calling method instead of passing `self` as parameter: `self.bar`
2019-11-30T22:59:55.9040688Z +    |
2019-11-30T22:59:55.9040953Z + help: try calling a method instead of passing `self` as parameter
2019-11-30T22:59:55.9041211Z +    |
2019-11-30T22:59:55.9041211Z +    |
2019-11-30T22:59:55.9041455Z + LL |         self.bar(&self);
2019-11-30T22:59:55.9041956Z 12 
2019-11-30T22:59:55.9042220Z 13 error[E0425]: cannot find function `bar` in this scope
2019-11-30T22:59:55.9042771Z 14   --> $DIR/suggest-self-2.rs:14:9
2019-11-30T22:59:55.9043070Z 
2019-11-30T22:59:55.9043070Z 
2019-11-30T22:59:55.9043289Z 
2019-11-30T22:59:55.9043555Z The actual stderr differed from the expected stderr.
2019-11-30T22:59:55.9044177Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/suggest-self-2/suggest-self-2.stderr
2019-11-30T22:59:55.9044800Z To update references, rerun the tests and pass the `--bless` flag
2019-11-30T22:59:55.9045469Z To only update this specific test, also pass `--test-args self/suggest-self-2.rs`
2019-11-30T22:59:55.9046070Z error: 1 errors occurred comparing output.
2019-11-30T22:59:55.9047150Z status: exit code: 1
2019-11-30T22:59:55.9047150Z status: exit code: 1
2019-11-30T22:59:55.9048479Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/suggest-self-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/suggest-self-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/suggest-self-2/auxiliary" "-A" "unused"
2019-11-30T22:59:55.9049428Z ------------------------------------------
2019-11-30T22:59:55.9049733Z 
2019-11-30T22:59:55.9050294Z ------------------------------------------
2019-11-30T22:59:55.9050615Z stderr:
---
2019-11-30T22:59:55.9059239Z    |
2019-11-30T22:59:55.9059400Z LL |         bar();
2019-11-30T22:59:55.9059558Z    |         ^^^ not found in this scope
2019-11-30T22:59:55.9059728Z 
2019-11-30T22:59:55.9059891Z error[E0599]: no method named `bar` found for type `&Foo` in the current scope
2019-11-30T22:59:55.9060332Z   --> /checkout/src/test/ui/self/suggest-self-2.rs:17:14
2019-11-30T22:59:55.9060717Z LL |         self.bar();
2019-11-30T22:59:55.9060875Z    |              ^^^ method not found in `&Foo`
2019-11-30T22:59:55.9061027Z 
2019-11-30T22:59:55.9061178Z error: aborting due to 4 previous errors
---
2019-11-30T22:59:55.9074697Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-30T22:59:55.9074796Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-30T22:59:55.9099074Z 
2019-11-30T22:59:55.9099618Z 
2019-11-30T22:59:55.9101868Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-30T22:59:55.9106075Z 
2019-11-30T22:59:55.9106104Z 
2019-11-30T22:59:55.9106182Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-30T22:59:55.9106235Z Build completed unsuccessfully in 1:09:32
2019-11-30T22:59:55.9106235Z Build completed unsuccessfully in 1:09:32
2019-11-30T22:59:55.9172871Z == clock drift check ==
2019-11-30T22:59:55.9190505Z   local time: Sat Nov 30 22:59:55 UTC 2019
2019-11-30T22:59:56.1985908Z   network time: Sat, 30 Nov 2019 22:59:56 GMT
2019-11-30T22:59:56.1991077Z == end clock drift check ==
2019-11-30T22:59:57.0058226Z 
2019-11-30T22:59:57.0191512Z ##[error]Bash exited with code '1'.
2019-11-30T22:59:57.0225537Z ##[section]Starting: Checkout
2019-11-30T22:59:57.0227745Z ==============================================================================
2019-11-30T22:59:57.0227821Z Task         : Get sources
2019-11-30T22:59:57.0227868Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
