plain
2019-09-27T21:51:17.3168809Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T21:51:17.3380469Z ##[command]git config gc.auto 0
2019-09-27T21:51:17.3452223Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T21:51:17.3526501Z ##[command]git config --get-all http.proxy
2019-09-27T21:51:17.3661091Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64856/merge:refs/remotes/pull/64856/merge
---
2019-09-27T22:55:46.7034151Z .................................................................................................... 1500/9047
2019-09-27T22:55:52.9767431Z .................................................................................................... 1600/9047
2019-09-27T22:56:05.6844016Z .........................................................................i...............i.......... 1700/9047
2019-09-27T22:56:12.9983631Z .................................................................................................... 1800/9047
2019-09-27T22:56:22.0116588Z ................................................................iiiii............................... 1900/9047
2019-09-27T22:56:41.8989910Z .................................................................................................... 2100/9047
2019-09-27T22:56:44.5167621Z .................................................................................................... 2200/9047
2019-09-27T22:56:47.8979223Z .................................................................................................... 2300/9047
2019-09-27T22:56:56.5347848Z .................................................................................................... 2400/9047
---
2019-09-27T23:00:03.7611204Z .....................................................i...............i.............................. 4700/9047
2019-09-27T23:00:13.3034917Z .................................................................................................... 4800/9047
2019-09-27T23:00:22.0979048Z .................................................................................................... 4900/9047
2019-09-27T23:00:29.9461624Z .................................................................................................... 5000/9047
2019-09-27T23:00:39.9951799Z .........................................ii.ii...................................................... 5100/9047
2019-09-27T23:00:50.4455972Z .................................................................................................... 5300/9047
2019-09-27T23:01:01.1503641Z .................................................................................................... 5400/9047
2019-09-27T23:01:08.9648116Z ......i............................................................................................. 5500/9047
2019-09-27T23:01:14.6753102Z .................................................................................................... 5600/9047
2019-09-27T23:01:14.6753102Z .................................................................................................... 5600/9047
2019-09-27T23:01:26.9088010Z .................................................................................................... 5700/9047
2019-09-27T23:01:40.5306113Z .ii...i..ii...........i............................................................................. 5800/9047
2019-09-27T23:02:02.9098668Z .................................................................................................... 6000/9047
2019-09-27T23:02:08.5872489Z .................................................................................................... 6100/9047
2019-09-27T23:02:08.5872489Z .................................................................................................... 6100/9047
2019-09-27T23:02:22.8029304Z ....i..ii........................................................................................... 6200/9047
2019-09-27T23:02:42.3567851Z ................................................................i................................... 6400/9047
2019-09-27T23:02:44.6965056Z .................................................................................................... 6500/9047
2019-09-27T23:02:47.3392269Z ....................................i............................................................... 6600/9047
2019-09-27T23:02:51.5035911Z .................................................................................................... 6700/9047
---
2019-09-27T23:07:01.8461498Z failures:
2019-09-27T23:07:01.8494339Z 
2019-09-27T23:07:01.8496065Z ---- [ui] ui/fmt/issue-64477.rs stdout ----
2019-09-27T23:07:01.8499581Z normalized stderr:
2019-09-27T23:07:01.8499640Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-09-27T23:07:01.8499977Z   --> $DIR/issue-64477.rs:6:1
2019-09-27T23:07:01.8500021Z    |
2019-09-27T23:07:01.8500222Z LL | async fn foo(_: String) {}
2019-09-27T23:07:01.8500317Z 
2019-09-27T23:07:01.8500317Z 
2019-09-27T23:07:01.8500362Z error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `move`
2019-09-27T23:07:01.8500608Z   --> $DIR/issue-64477.rs:9:11
2019-09-27T23:07:01.8500703Z LL |     async move {
2019-09-27T23:07:01.8500744Z    |           ^^^^ expected one of 8 possible tokens here
2019-09-27T23:07:01.8500770Z 
2019-09-27T23:07:01.8500823Z error: aborting due to 2 previous errors
2019-09-27T23:07:01.8500823Z error: aborting due to 2 previous errors
2019-09-27T23:07:01.8500847Z 
2019-09-27T23:07:01.8501065Z For more information about this error, try `rustc --explain E0670`.
2019-09-27T23:07:01.8501093Z 
2019-09-27T23:07:01.8501131Z 
2019-09-27T23:07:01.8501152Z 
2019-09-27T23:07:01.8501191Z The actual stderr differed from the expected stderr.
2019-09-27T23:07:01.8501443Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/issue-64477/issue-64477.stderr
2019-09-27T23:07:01.8501678Z To update references, rerun the tests and pass the `--bless` flag
2019-09-27T23:07:01.8501898Z To only update this specific test, also pass `--test-args fmt/issue-64477.rs`
2019-09-27T23:07:01.8501988Z error: 1 errors occurred comparing output.
2019-09-27T23:07:01.8502027Z status: exit code: 1
2019-09-27T23:07:01.8502027Z status: exit code: 1
2019-09-27T23:07:01.8502625Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/issue-64477.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/issue-64477" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/issue-64477/auxiliary" "-A" "unused"
2019-09-27T23:07:01.8503500Z ------------------------------------------
2019-09-27T23:07:01.8503569Z 
2019-09-27T23:07:01.8503798Z ------------------------------------------
2019-09-27T23:07:01.8503844Z stderr:
2019-09-27T23:07:01.8503844Z stderr:
2019-09-27T23:07:01.8504070Z ------------------------------------------
2019-09-27T23:07:01.8504141Z error[E0670]: `async fn` is not permitted in the 2015 edition
2019-09-27T23:07:01.8504375Z   --> /checkout/src/test/ui/fmt/issue-64477.rs:6:1
2019-09-27T23:07:01.8504423Z    |
2019-09-27T23:07:01.8504485Z LL | async fn foo(_: String) {}
2019-09-27T23:07:01.8504556Z 
2019-09-27T23:07:01.8504556Z 
2019-09-27T23:07:01.8504606Z error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `move`
2019-09-27T23:07:01.8504862Z   --> /checkout/src/test/ui/fmt/issue-64477.rs:9:11
2019-09-27T23:07:01.8504952Z LL |     async move {
2019-09-27T23:07:01.8505017Z    |           ^^^^ expected one of 8 possible tokens here
2019-09-27T23:07:01.8505047Z 
2019-09-27T23:07:01.8505090Z error: aborting due to 2 previous errors
---
2019-09-27T23:07:01.8537372Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-27T23:07:01.8537476Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-27T23:07:01.8556689Z 
2019-09-27T23:07:01.8556765Z 
2019-09-27T23:07:01.8558433Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-27T23:07:01.8558694Z 
2019-09-27T23:07:01.8558722Z 
2019-09-27T23:07:01.8567463Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-27T23:07:01.8567526Z Build completed unsuccessfully in 1:08:10
2019-09-27T23:07:01.8567526Z Build completed unsuccessfully in 1:08:10
2019-09-27T23:07:01.8620217Z == clock drift check ==
2019-09-27T23:07:01.8639948Z   local time: Fri Sep 27 23:07:01 UTC 2019
2019-09-27T23:07:02.0160637Z   network time: Fri, 27 Sep 2019 23:07:02 GMT
2019-09-27T23:07:02.0165859Z == end clock drift check ==
2019-09-27T23:07:03.4607752Z ##[error]Bash exited with code '1'.
2019-09-27T23:07:03.4693207Z ##[section]Starting: Checkout
2019-09-27T23:07:03.4695088Z ==============================================================================
2019-09-27T23:07:03.4695145Z Task         : Get sources
2019-09-27T23:07:03.4695215Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
