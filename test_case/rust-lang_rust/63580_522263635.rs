plain
2019-08-17T18:03:41.8739256Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-17T18:03:41.8901222Z ##[command]git config gc.auto 0
2019-08-17T18:03:41.8979085Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-17T18:03:41.9038862Z ##[command]git config --get-all http.proxy
2019-08-17T18:03:41.9186142Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63580/merge:refs/remotes/pull/63580/merge
---
2019-08-17T18:04:16.5317964Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-17T18:04:16.5317998Z 
2019-08-17T18:04:16.5318222Z   git checkout -b <new-branch-name>
2019-08-17T18:04:16.5318253Z 
2019-08-17T18:04:16.5318320Z HEAD is now at 8d09e8886 Merge da1f6792034c6b6deffcfbef49d8010647e6b6e7 into d65e272a9fe3e61aa5f229c5358e35a909435575
2019-08-17T18:04:16.5481108Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-17T18:04:16.5484129Z ==============================================================================
2019-08-17T18:04:16.5484194Z Task         : Bash
2019-08-17T18:04:16.5484261Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-17T19:07:14.7691414Z .................................................................................................... 1500/8926
2019-08-17T19:07:20.3777071Z .................................................................................................... 1600/8926
2019-08-17T19:07:33.5717555Z ................................i...............i................................................... 1700/8926
2019-08-17T19:07:41.3556374Z .................................................................................................... 1800/8926
2019-08-17T19:07:56.0152644Z ........................iiiii....................................................................... 1900/8926
2019-08-17T19:08:06.8101220Z .................................................................................................... 2100/8926
2019-08-17T19:08:09.4008660Z .................................................................................................... 2200/8926
2019-08-17T19:08:14.3371006Z .................................................................................................... 2300/8926
2019-08-17T19:08:21.3502434Z .................................................................................................... 2400/8926
---
2019-08-17T19:11:17.5107005Z .................................................................................................... 4600/8926
2019-08-17T19:11:24.7169177Z .......i...............i............................................................................ 4700/8926
2019-08-17T19:11:36.2223223Z .................................................................................................... 4800/8926
2019-08-17T19:11:42.1556277Z .................................................................................................... 4900/8926
2019-08-17T19:11:54.2513557Z ........................................................................................ii.ii....... 5000/8926
2019-08-17T19:12:03.7983907Z .................................................................................................... 5200/8926
2019-08-17T19:12:13.5925473Z .................................................................................................... 5300/8926
2019-08-17T19:12:20.5764433Z ............................................i....................................................... 5400/8926
2019-08-17T19:12:27.3840307Z .................................................................................................... 5500/8926
2019-08-17T19:12:27.3840307Z .................................................................................................... 5500/8926
2019-08-17T19:12:38.2860628Z .................................................................................................... 5600/8926
2019-08-17T19:12:49.7930233Z .....................................ii...i..ii...........i......................................... 5700/8926
2019-08-17T19:13:06.3286308Z .................................................................................................... 5900/8926
2019-08-17T19:13:11.2863530Z .................................................................................................... 6000/8926
2019-08-17T19:13:11.2863530Z .................................................................................................... 6000/8926
2019-08-17T19:13:24.7659301Z ......................................i..ii......................................................... 6100/8926
2019-08-17T19:13:46.0977946Z .................................................................................i.................. 6300/8926
2019-08-17T19:13:48.4308145Z .................................................................................................... 6400/8926
2019-08-17T19:13:50.7628722Z ....................................................i............................................... 6500/8926
2019-08-17T19:13:54.1607096Z .................................................................................................... 6600/8926
---
2019-08-17T19:17:54.6767465Z 11 error[E0080]: evaluation of constant expression failed
2019-08-17T19:17:54.6768001Z -   --> $DIR/issue-50814.rs:18:5
2019-08-17T19:17:54.6768447Z +   --> $DIR/issue-50814.rs:17:5
2019-08-17T19:17:54.6768677Z 13    |
2019-08-17T19:17:54.6768844Z 14 LL |     &Sum::<U8,U8>::MAX
2019-08-17T19:17:54.6769239Z 15    |     ^-----------------
2019-08-17T19:17:54.6769597Z 
2019-08-17T19:17:54.6769767Z The actual stderr differed from the expected stderr.
2019-08-17T19:17:54.6770296Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/issue-50814.stderr
2019-08-17T19:17:54.6770296Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/issue-50814.stderr
2019-08-17T19:17:54.6770815Z To update references, rerun the tests and pass the `--bless` flag
2019-08-17T19:17:54.6771312Z To only update this specific test, also pass `--test-args consts/const-eval/issue-50814.rs`
2019-08-17T19:17:54.6771696Z error: 1 errors occurred comparing output.
2019-08-17T19:17:54.6771879Z status: exit code: 1
2019-08-17T19:17:54.6771879Z status: exit code: 1
2019-08-17T19:17:54.6773125Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-50814.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/auxiliary" "-A" "unused"
2019-08-17T19:17:54.6773921Z ------------------------------------------
2019-08-17T19:17:54.6774125Z 
2019-08-17T19:17:54.6774539Z ------------------------------------------
2019-08-17T19:17:54.6776171Z stderr:
2019-08-17T19:17:54.6776171Z stderr:
2019-08-17T19:17:54.6776883Z ------------------------------------------
2019-08-17T19:17:54.6777688Z error: any use of this value will cause an error
2019-08-17T19:17:54.6778171Z   --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:13:21
2019-08-17T19:17:54.6778234Z    |
2019-08-17T19:17:54.6778294Z LL |     const MAX: u8 = A::MAX + B::MAX; //~ ERROR any use of this value will cause an error
2019-08-17T19:17:54.6778574Z    |     ----------------^^^^^^^^^^^^^^^-
2019-08-17T19:17:54.6778681Z    |                     attempt to add with overflow
2019-08-17T19:17:54.6778747Z    |
2019-08-17T19:17:54.6778796Z    = note: `#[deny(const_err)]` on by default
2019-08-17T19:17:54.6778842Z 
2019-08-17T19:17:54.6778842Z 
2019-08-17T19:17:54.6778913Z error[E0080]: evaluation of constant expression failed
2019-08-17T19:17:54.6779193Z   --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:17:5
2019-08-17T19:17:54.6779246Z    |
2019-08-17T19:17:54.6779312Z LL |     &Sum::<U8,U8>::MAX //~ ERROR E0080
2019-08-17T19:17:54.6779544Z    |     ^-----------------
2019-08-17T19:17:54.6779812Z    |      referenced constant has errors
2019-08-17T19:17:54.6779865Z 
2019-08-17T19:17:54.6779913Z error: aborting due to 2 previous errors
2019-08-17T19:17:54.6779944Z 
---
2019-08-17T19:17:54.6804222Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-17T19:17:54.6804311Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-17T19:17:54.6822774Z 
2019-08-17T19:17:54.6822932Z 
2019-08-17T19:17:54.6825216Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-17T19:17:55.5658575Z 
2019-08-17T19:17:55.5658667Z 
2019-08-17T19:17:55.5658774Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-17T19:17:55.5659107Z Build completed unsuccessfully in 1:07:13
2019-08-17T19:17:55.5659107Z Build completed unsuccessfully in 1:07:13
2019-08-17T19:17:55.5659173Z == clock drift check ==
2019-08-17T19:17:55.5659241Z   local time: Sat Aug 17 19:17:54 UTC 2019
2019-08-17T19:17:55.5659293Z   network time: Sat, 17 Aug 2019 19:17:54 GMT
2019-08-17T19:17:55.5659343Z == end clock drift check ==
2019-08-17T19:17:55.9753534Z ##[error]Bash exited with code '1'.
2019-08-17T19:17:55.9820210Z ##[section]Starting: Checkout
2019-08-17T19:17:55.9822202Z ==============================================================================
2019-08-17T19:17:55.9822276Z Task         : Get sources
2019-08-17T19:17:55.9822450Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
