plain
2019-12-01T11:13:39.9643168Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T11:13:39.9866927Z ##[command]git config gc.auto 0
2019-12-01T11:13:39.9921062Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T11:13:39.9974409Z ##[command]git config --get-all http.proxy
2019-12-01T11:13:40.0116911Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66927/merge:refs/remotes/pull/66927/merge
---
2019-12-01T12:11:01.8820334Z .................................................................................................... 1600/9316
2019-12-01T12:11:06.6125312Z .................................................................................................... 1700/9316
2019-12-01T12:11:19.0584062Z ........................................i........................................................... 1800/9316
2019-12-01T12:11:26.7422910Z .................................................................................................... 1900/9316
2019-12-01T12:11:40.2772194Z .........................iiiii...................................................................... 2000/9316
2019-12-01T12:11:50.2443972Z .................................................................................................... 2200/9316
2019-12-01T12:11:52.8012039Z .................................................................................................... 2300/9316
2019-12-01T12:11:57.3303580Z .................................................................................................... 2400/9316
2019-12-01T12:12:18.6407672Z .................................................................................................... 2500/9316
---
2019-12-01T12:14:55.8256888Z ...........................i...............i........................................................ 4800/9316
2019-12-01T12:15:06.1382420Z .................................................................................................... 4900/9316
2019-12-01T12:15:12.0374714Z .................................................................................................... 5000/9316
2019-12-01T12:15:19.7958390Z .................................................................................................... 5100/9316
2019-12-01T12:15:27.2991412Z .................................ii.ii...........i.................................................. 5200/9316
2019-12-01T12:15:36.6930482Z .................................................................................................... 5400/9316
2019-12-01T12:15:46.3130343Z .................................................................................................... 5500/9316
2019-12-01T12:15:53.3989698Z ...............i.................................................................................... 5600/9316
2019-12-01T12:15:59.3614141Z .................................................................................................... 5700/9316
2019-12-01T12:15:59.3614141Z .................................................................................................... 5700/9316
2019-12-01T12:16:10.6201086Z .................................................................................................... 5800/9316
2019-12-01T12:16:22.5534680Z .ii...i..ii...........i............................................................................. 5900/9316
2019-12-01T12:16:40.6832513Z .................................................................................................... 6100/9316
2019-12-01T12:16:44.6725651Z .................................................................................................... 6200/9316
2019-12-01T12:16:44.6725651Z .................................................................................................... 6200/9316
2019-12-01T12:16:57.9727072Z ........................i..ii....................................................................... 6300/9316
2019-12-01T12:17:17.3011293Z ...............................................................................................i.... 6500/9316
2019-12-01T12:17:19.5281868Z .................................................................................................... 6600/9316
2019-12-01T12:17:21.8755160Z ......................................................................................i............. 6700/9316
2019-12-01T12:17:24.7074170Z .................................................................................................... 6800/9316
---
2019-12-01T12:22:06.9505328Z failures:
2019-12-01T12:22:06.9505353Z 
2019-12-01T12:22:06.9505596Z ---- [compile-fail] compile-fail/consts/const-err3.rs stdout ----
2019-12-01T12:22:06.9505645Z 
2019-12-01T12:22:06.9505908Z error: /checkout/src/test/compile-fail/consts/const-err3.rs:15: expected error not found: this expression will panic at runtime
2019-12-01T12:22:06.9505994Z error: 0 unexpected errors found, 1 expected errors not found
2019-12-01T12:22:06.9506032Z status: exit code: 1
2019-12-01T12:22:06.9506032Z status: exit code: 1
2019-12-01T12:22:06.9507222Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/consts/const-err3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/consts/const-err3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/consts/const-err3/auxiliary" "-A" "unused"
2019-12-01T12:22:06.9507363Z not found errors (from test file): [
2019-12-01T12:22:06.9507475Z         line_num: 15,
2019-12-01T12:22:06.9507519Z         kind: Some(
2019-12-01T12:22:06.9507562Z             Error,
2019-12-01T12:22:06.9507605Z         ),
2019-12-01T12:22:06.9507605Z         ),
2019-12-01T12:22:06.9507669Z         msg: "this expression will panic at runtime",
2019-12-01T12:22:06.9507756Z ]
2019-12-01T12:22:06.9507802Z 
2019-12-01T12:22:06.9508155Z thread '[compile-fail] compile-fail/consts/const-err3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-01T12:22:06.9508221Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
2019-12-01T12:22:06.9508992Z 
2019-12-01T12:22:06.9509266Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-01T12:22:06.9513471Z 
2019-12-01T12:22:06.9513624Z 
2019-12-01T12:22:06.9517227Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-01T12:22:06.9518245Z 
2019-12-01T12:22:06.9518334Z 
2019-12-01T12:22:06.9525775Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T12:22:06.9525841Z Build completed unsuccessfully in 1:02:44
2019-12-01T12:22:06.9525841Z Build completed unsuccessfully in 1:02:44
2019-12-01T12:22:06.9579285Z == clock drift check ==
2019-12-01T12:22:06.9593825Z   local time: Sun Dec  1 12:22:06 UTC 2019
2019-12-01T12:22:07.0491197Z   network time: Sun, 01 Dec 2019 12:22:07 GMT
2019-12-01T12:22:07.0494374Z == end clock drift check ==
2019-12-01T12:22:07.8812721Z 
2019-12-01T12:22:07.8911414Z ##[error]Bash exited with code '1'.
2019-12-01T12:22:07.9018598Z ##[section]Starting: Checkout
2019-12-01T12:22:07.9020656Z ==============================================================================
2019-12-01T12:22:07.9020706Z Task         : Get sources
2019-12-01T12:22:07.9020764Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
