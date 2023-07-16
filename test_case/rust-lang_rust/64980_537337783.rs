plain
2019-10-02T03:49:03.0937013Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-02T03:49:03.1139704Z ##[command]git config gc.auto 0
2019-10-02T03:49:03.1214733Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-02T03:49:03.1267005Z ##[command]git config --get-all http.proxy
2019-10-02T03:49:03.1418872Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64980/merge:refs/remotes/pull/64980/merge
---
2019-10-02T04:56:16.7034172Z .................................................................................................... 1500/9086
2019-10-02T04:56:23.8891480Z .................................................................................................... 1600/9086
2019-10-02T04:56:34.4002358Z ...................................................................................................i 1700/9086
2019-10-02T04:56:43.6439481Z ...............i.................................................................................... 1800/9086
2019-10-02T04:56:51.3111922Z ..........................................................................................iiiii..... 1900/9086
2019-10-02T04:57:15.7237246Z .................................................................................................... 2100/9086
2019-10-02T04:57:18.3999318Z .................................................................................................... 2200/9086
2019-10-02T04:57:21.2422064Z .................................................................................................... 2300/9086
2019-10-02T04:57:28.4121723Z .................................................................................................... 2400/9086
---
2019-10-02T05:00:41.0841271Z .............................................................................i...............i...... 4700/9086
2019-10-02T05:00:50.0628043Z .................................................................................................... 4800/9086
2019-10-02T05:01:00.8026354Z .................................................................................................... 4900/9086
2019-10-02T05:01:07.5454643Z .................................................................................................... 5000/9086
2019-10-02T05:01:19.4684741Z ....................................................................ii.ii........................... 5100/9086
2019-10-02T05:01:30.1141826Z .................................................................................................... 5300/9086
2019-10-02T05:01:40.4218213Z .................................................................................................... 5400/9086
2019-10-02T05:01:48.5888686Z ..................................i................................................................. 5500/9086
2019-10-02T05:01:55.3354844Z .................................................................................................... 5600/9086
2019-10-02T05:01:55.3354844Z .................................................................................................... 5600/9086
2019-10-02T05:02:07.9792004Z ...........................................F........................................................ 5700/9086
2019-10-02T05:02:19.5348496Z ...............................ii...i..ii...........i............................................... 5800/9086
2019-10-02T05:02:43.2274230Z .................................................................................................... 6000/9086
2019-10-02T05:02:53.0991291Z .................................................................................................... 6100/9086
2019-10-02T05:02:53.0991291Z .................................................................................................... 6100/9086
2019-10-02T05:03:11.0481451Z ..................................i..ii............................................................. 6200/9086
2019-10-02T05:03:32.7570315Z ..............................................................................................i..... 6400/9086
2019-10-02T05:03:35.2116855Z .................................................................................................... 6500/9086
2019-10-02T05:03:37.7075449Z ..................................................................i................................. 6600/9086
2019-10-02T05:03:40.9706982Z .................................................................................................... 6700/9086
---
2019-10-02T05:08:06.6217843Z 
2019-10-02T05:08:06.6218501Z ---- [ui] ui/mir-dataflow/indirect-mutation-offset.rs stdout ----
2019-10-02T05:08:06.6218574Z diff of stderr:
2019-10-02T05:08:06.6218612Z 
2019-10-02T05:08:06.6218680Z 1 error: rustc_peek: bit not set
2019-10-02T05:08:06.6219236Z +   --> $DIR/indirect-mutation-offset.rs:35:14
2019-10-02T05:08:06.6219306Z 3    |
2019-10-02T05:08:06.6219306Z 3    |
2019-10-02T05:08:06.6219356Z 4 LL |     unsafe { rustc_peek(x) };
2019-10-02T05:08:06.6219441Z 
2019-10-02T05:08:06.6219471Z 
2019-10-02T05:08:06.6219539Z The actual stderr differed from the expected stderr.
2019-10-02T05:08:06.6219929Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/indirect-mutation-offset.stderr
2019-10-02T05:08:06.6219929Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/indirect-mutation-offset.stderr
2019-10-02T05:08:06.6220221Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T05:08:06.6220569Z To only update this specific test, also pass `--test-args mir-dataflow/indirect-mutation-offset.rs`
2019-10-02T05:08:06.6220662Z error: 1 errors occurred comparing output.
2019-10-02T05:08:06.6220730Z status: exit code: 1
2019-10-02T05:08:06.6220730Z status: exit code: 1
2019-10-02T05:08:06.6221631Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-dataflow/indirect-mutation-offset.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/auxiliary" "-A" "unused"
2019-10-02T05:08:06.6222014Z ------------------------------------------
2019-10-02T05:08:06.6222055Z 
2019-10-02T05:08:06.6222321Z ------------------------------------------
2019-10-02T05:08:06.6222372Z stderr:
2019-10-02T05:08:06.6222372Z stderr:
2019-10-02T05:08:06.6222611Z ------------------------------------------
2019-10-02T05:08:06.6222683Z error: rustc_peek: bit not set
2019-10-02T05:08:06.6222968Z   --> /checkout/src/test/ui/mir-dataflow/indirect-mutation-offset.rs:35:14
2019-10-02T05:08:06.6223025Z    |
2019-10-02T05:08:06.6223100Z LL |     unsafe { rustc_peek(x) }; //~ ERROR rustc_peek: bit not set
2019-10-02T05:08:06.6223185Z 
2019-10-02T05:08:06.6223185Z 
2019-10-02T05:08:06.6223233Z error: stop_after_dataflow ended compilation
2019-10-02T05:08:06.6223547Z error: aborting due to 2 previous errors
2019-10-02T05:08:06.6223598Z 
2019-10-02T05:08:06.6223627Z 
2019-10-02T05:08:06.6223945Z ------------------------------------------
---
2019-10-02T05:08:06.6262932Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-02T05:08:06.6263037Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-02T05:08:06.6290670Z 
2019-10-02T05:08:06.6291119Z 
2019-10-02T05:08:06.6293403Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-02T05:08:06.6294257Z 
2019-10-02T05:08:06.6294414Z 
2019-10-02T05:08:06.6301287Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-02T05:08:06.6301527Z Build completed unsuccessfully in 1:11:38
2019-10-02T05:08:06.6301527Z Build completed unsuccessfully in 1:11:38
2019-10-02T05:08:06.6361195Z == clock drift check ==
2019-10-02T05:08:06.6378391Z   local time: Wed Oct  2 05:08:06 UTC 2019
2019-10-02T05:08:06.7900321Z   network time: Wed, 02 Oct 2019 05:08:06 GMT
2019-10-02T05:08:06.7900449Z == end clock drift check ==
2019-10-02T05:08:07.6430372Z ##[error]Bash exited with code '1'.
2019-10-02T05:08:07.6488522Z ##[section]Starting: Checkout
2019-10-02T05:08:07.6490702Z ==============================================================================
2019-10-02T05:08:07.6490765Z Task         : Get sources
2019-10-02T05:08:07.6490816Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
