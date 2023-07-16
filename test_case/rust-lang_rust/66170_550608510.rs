plain
2019-11-07T01:58:10.7072880Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-07T01:58:10.7252657Z ##[command]git config gc.auto 0
2019-11-07T01:58:10.7323414Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-07T01:58:10.7389726Z ##[command]git config --get-all http.proxy
2019-11-07T01:58:10.7542383Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66170/merge:refs/remotes/pull/66170/merge
---
2019-11-07T02:56:29.6005592Z ..................................................................F................................. 1600/9282
2019-11-07T02:56:35.2802528Z .................................................................................................... 1700/9282
2019-11-07T02:56:47.3087612Z ................................................................i................................... 1800/9282
2019-11-07T02:56:54.8982165Z .................................................................................................... 1900/9282
2019-11-07T02:57:08.9857025Z ................................................iiiii............................................... 2000/9282
2019-11-07T02:57:19.2301255Z .................................................................................................... 2200/9282
2019-11-07T02:57:21.6362452Z .................................................................................................... 2300/9282
2019-11-07T02:57:25.1594883Z .................................................................................................... 2400/9282
2019-11-07T02:57:47.4924687Z .................................................................................................... 2500/9282
---
2019-11-07T03:00:33.9867022Z ..............................................i...............i..................................... 4800/9282
2019-11-07T03:00:42.7655208Z .................................................................................................... 4900/9282
2019-11-07T03:00:50.8604856Z .................................................................................................... 5000/9282
2019-11-07T03:00:57.2954073Z .................................................................................................... 5100/9282
2019-11-07T03:01:06.8944236Z ...............................................ii.ii...........i.................................... 5200/9282
2019-11-07T03:01:16.2191398Z .................................................................................................... 5400/9282
2019-11-07T03:01:26.1953481Z .................................................................................................... 5500/9282
2019-11-07T03:01:33.3359472Z .............................i...................................................................... 5600/9282
2019-11-07T03:01:39.6870032Z .................................................................................................... 5700/9282
2019-11-07T03:01:39.6870032Z .................................................................................................... 5700/9282
2019-11-07T03:01:51.6648239Z .................................................................................................... 5800/9282
2019-11-07T03:02:02.7296790Z ..............ii...i..ii...........i................................................................ 5900/9282
2019-11-07T03:02:22.8103059Z .................................................................................................... 6100/9282
2019-11-07T03:02:30.0581489Z .................................................................................................... 6200/9282
2019-11-07T03:02:30.0581489Z .................................................................................................... 6200/9282
2019-11-07T03:02:43.6788884Z .................................i..ii.............................................................. 6300/9282
2019-11-07T03:03:03.9259293Z .................................................................................................... 6500/9282
2019-11-07T03:03:05.9799041Z i................................................................................................... 6600/9282
2019-11-07T03:03:08.0418250Z ...............................................................................i.................... 6700/9282
2019-11-07T03:03:10.5403929Z .................................................................................................... 6800/9282
---
2019-11-07T03:04:54.7444660Z .................................................................................................... 7500/9282
2019-11-07T03:05:02.7756317Z .................................................................................................... 7600/9282
2019-11-07T03:05:13.5675593Z .................................................................................................... 7700/9282
2019-11-07T03:05:22.4379674Z .................................................................................................... 7800/9282
2019-11-07T03:05:28.9500735Z ..ii......i......................................................................................... 7900/9282
2019-11-07T03:05:49.9393150Z .................................................................................................... 8100/9282
2019-11-07T03:05:58.4516736Z .................................................................................................... 8200/9282
2019-11-07T03:06:06.4779014Z .................................................................................................... 8300/9282
2019-11-07T03:06:44.8752280Z .................................................................................................... 8400/9282
---
2019-11-07T03:07:54.1930430Z 1 error[E0744]: `loop` is not allowed in a `const`
2019-11-07T03:07:54.1931440Z -   --> $DIR/issue-62272.rs:3:17
2019-11-07T03:07:54.1931996Z +   --> $DIR/issue-62272.rs:7:17
2019-11-07T03:07:54.1932374Z 3    |
2019-11-07T03:07:54.1932699Z 4 LL | const FOO: () = loop { break; };
2019-11-07T03:07:54.1933367Z 
2019-11-07T03:07:54.1933492Z 
2019-11-07T03:07:54.1933955Z The actual stderr differed from the expected stderr.
2019-11-07T03:07:54.1934350Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-62272/issue-62272.stderr
2019-11-07T03:07:54.1934350Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-62272/issue-62272.stderr
2019-11-07T03:07:54.1935079Z To update references, rerun the tests and pass the `--bless` flag
2019-11-07T03:07:54.1935789Z To only update this specific test, also pass `--test-args consts/const-eval/issue-62272.rs`
2019-11-07T03:07:54.1936118Z error: 1 errors occurred comparing output.
2019-11-07T03:07:54.1936235Z status: exit code: 1
2019-11-07T03:07:54.1936235Z status: exit code: 1
2019-11-07T03:07:54.1937864Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-62272.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-62272" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-62272/auxiliary" "-A" "unused"
2019-11-07T03:07:54.1938588Z ------------------------------------------
2019-11-07T03:07:54.1938760Z 
2019-11-07T03:07:54.1939128Z ------------------------------------------
2019-11-07T03:07:54.1939350Z stderr:
2019-11-07T03:07:54.1939350Z stderr:
2019-11-07T03:07:54.1939718Z ------------------------------------------
2019-11-07T03:07:54.1939910Z error[E0744]: `loop` is not allowed in a `const`
2019-11-07T03:07:54.1940322Z   --> /checkout/src/test/ui/consts/const-eval/issue-62272.rs:7:17
2019-11-07T03:07:54.1940515Z    |
2019-11-07T03:07:54.1940673Z LL | const FOO: () = loop { break; }; //~ ERROR `loop` is not allowed in a `const`
2019-11-07T03:07:54.1941304Z 
2019-11-07T03:07:54.1941410Z error: aborting due to previous error
2019-11-07T03:07:54.1941521Z 
2019-11-07T03:07:54.1941817Z For more information about this error, try `rustc --explain E0744`.
---
2019-11-07T03:07:54.1943313Z 1 error[E0744]: `while` is not allowed in a `const`
2019-11-07T03:07:54.1943609Z -   --> $DIR/const-labeled-break.rs:6:19
2019-11-07T03:07:54.1943909Z +   --> $DIR/const-labeled-break.rs:10:19
2019-11-07T03:07:54.1944072Z 3    |
2019-11-07T03:07:54.1944367Z 4 LL | const CRASH: () = 'a: while break 'a {};
2019-11-07T03:07:54.1944606Z 
2019-11-07T03:07:54.1944716Z 
2019-11-07T03:07:54.1944822Z The actual stderr differed from the expected stderr.
2019-11-07T03:07:54.1945157Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-labeled-break/const-labeled-break.stderr
2019-11-07T03:07:54.1945157Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-labeled-break/const-labeled-break.stderr
2019-11-07T03:07:54.1945502Z To update references, rerun the tests and pass the `--bless` flag
2019-11-07T03:07:54.1945898Z To only update this specific test, also pass `--test-args consts/const-labeled-break.rs`
2019-11-07T03:07:54.1946184Z error: 1 errors occurred comparing output.
2019-11-07T03:07:54.1946304Z status: exit code: 1
2019-11-07T03:07:54.1946304Z status: exit code: 1
2019-11-07T03:07:54.1947286Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-labeled-break.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-labeled-break" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-labeled-break/auxiliary" "-A" "unused"
2019-11-07T03:07:54.1948327Z ------------------------------------------
2019-11-07T03:07:54.1948519Z 
2019-11-07T03:07:54.1948890Z ------------------------------------------
2019-11-07T03:07:54.1949093Z stderr:
2019-11-07T03:07:54.1949093Z stderr:
2019-11-07T03:07:54.1949448Z ------------------------------------------
2019-11-07T03:07:54.1949749Z error[E0744]: `while` is not allowed in a `const`
2019-11-07T03:07:54.1950233Z   --> /checkout/src/test/ui/consts/const-labeled-break.rs:10:19
2019-11-07T03:07:54.1950431Z    |
2019-11-07T03:07:54.1950865Z LL | const CRASH: () = 'a: while break 'a {}; //~ ERROR `while` is not allowed in a `const`
2019-11-07T03:07:54.1951314Z 
2019-11-07T03:07:54.1951441Z error: aborting due to previous error
2019-11-07T03:07:54.1951537Z 
2019-11-07T03:07:54.1951832Z For more information about this error, try `rustc --explain E0744`.
---
2019-11-07T03:07:54.1958415Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-07T03:07:54.1959447Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-07T03:07:54.1976706Z 
2019-11-07T03:07:54.1976791Z 
2019-11-07T03:07:54.1980371Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-07T03:07:54.1980859Z 
2019-11-07T03:07:54.1981244Z 
2019-11-07T03:07:54.1982426Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-07T03:07:54.1982477Z Build completed unsuccessfully in 1:03:18
2019-11-07T03:07:54.1982477Z Build completed unsuccessfully in 1:03:18
2019-11-07T03:07:54.2031985Z == clock drift check ==
2019-11-07T03:07:54.8788095Z   local time: Thu Nov  7 03:07:54 UTC 2019
2019-11-07T03:07:54.8799979Z   network time: Thu, 07 Nov 2019 03:07:54 GMT
2019-11-07T03:07:54.8800367Z == end clock drift check ==
2019-11-07T03:07:55.7372544Z 
2019-11-07T03:07:55.7482170Z ##[error]Bash exited with code '1'.
2019-11-07T03:07:55.7509965Z ##[section]Starting: Checkout
2019-11-07T03:07:55.7511651Z ==============================================================================
2019-11-07T03:07:55.7511693Z Task         : Get sources
2019-11-07T03:07:55.7511745Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
