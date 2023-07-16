plain
2020-02-15T12:16:34.5335625Z ========================== Starting Command Output ===========================
2020-02-15T12:16:34.5338686Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b7252f77-0070-43dd-9d70-78aadf5ac20b.sh
2020-02-15T12:16:34.5338917Z 
2020-02-15T12:16:34.5341997Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-15T12:16:34.5348087Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69185/merge to s
2020-02-15T12:16:34.5349767Z Task         : Get sources
2020-02-15T12:16:34.5349804Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T12:16:34.5349840Z Version      : 1.0.0
2020-02-15T12:16:34.5349877Z Author       : Microsoft
---
2020-02-15T12:16:35.4045237Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-15T12:16:35.4161312Z ##[command]git config gc.auto 0
2020-02-15T12:16:35.4228724Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-15T12:16:35.4289114Z ##[command]git config --get-all http.proxy
2020-02-15T12:16:35.4487305Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69185/merge:refs/remotes/pull/69185/merge
---
2020-02-15T13:17:24.0766488Z .................................................................................................... 1700/9661
2020-02-15T13:17:29.0553211Z .................................................................................................... 1800/9661
2020-02-15T13:17:41.9647173Z ...........................................i........................................................ 1900/9661
2020-02-15T13:17:50.7644337Z .................................................................................................... 2000/9661
2020-02-15T13:18:06.0083243Z .................................iiiii.............................................................. 2100/9661
2020-02-15T13:18:16.8391130Z .................................................................................................... 2300/9661
2020-02-15T13:18:19.5467593Z .................................................................................................... 2400/9661
2020-02-15T13:18:24.3807408Z .................................................................................................... 2500/9661
2020-02-15T13:18:47.1977011Z .................................................................................................... 2600/9661
---
2020-02-15T13:21:44.3872139Z ......i............................................................................................. 5000/9661
2020-02-15T13:21:54.3087149Z .................................................................................................... 5100/9661
2020-02-15T13:21:59.4385503Z ................................i................................................................... 5200/9661
2020-02-15T13:22:09.9353019Z .................................................................................................... 5300/9661
2020-02-15T13:22:16.0446875Z ........ii.ii........i...i.......................................................................... 5400/9661
2020-02-15T13:22:25.5231962Z .................................................................................................... 5600/9661
2020-02-15T13:22:37.4016394Z ...................................................................................................i 5700/9661
2020-02-15T13:22:45.4204108Z .................................................................................................... 5800/9661
2020-02-15T13:22:51.3741579Z .................................................................................................i.. 5900/9661
2020-02-15T13:22:51.3741579Z .................................................................................................i.. 5900/9661
2020-02-15T13:23:02.2853129Z ...........................................................................................ii...i..i 6000/9661
2020-02-15T13:23:15.3567408Z i...........i....................................................................................... 6100/9661
2020-02-15T13:23:32.8736328Z .................................................................................................... 6300/9661
2020-02-15T13:23:36.4901065Z .................................................................................................... 6400/9661
2020-02-15T13:23:36.4901065Z .................................................................................................... 6400/9661
2020-02-15T13:23:50.6815939Z ...................i..ii............................................................................ 6500/9661
2020-02-15T13:24:12.7786817Z .................................................................................................... 6700/9661
2020-02-15T13:24:15.2209243Z .......i............................................................................................ 6800/9661
2020-02-15T13:24:17.7011550Z .................................................................................................... 6900/9661
2020-02-15T13:24:20.3542107Z .................i.................................................................................. 7000/9661
---
2020-02-15T13:26:02.8135324Z .................................................................................................... 7600/9661
2020-02-15T13:26:08.5647793Z .................................................................................................... 7700/9661
2020-02-15T13:26:15.0150232Z .................................................................................................... 7800/9661
2020-02-15T13:26:21.5135836Z .................................................................................................... 7900/9661
2020-02-15T13:26:32.6161469Z ...................................................................................................i 8000/9661
2020-02-15T13:26:39.0190384Z iiiiii.i............................................................................................ 8100/9661
2020-02-15T13:26:55.1510031Z .................................................................................................... 8300/9661
2020-02-15T13:27:06.5457629Z .................................................................................................... 8400/9661
2020-02-15T13:27:19.4396195Z .................................................................................................... 8500/9661
2020-02-15T13:27:25.7898842Z .................................................................................................... 8600/9661
---
2020-02-15T13:29:26.1329576Z failures:
2020-02-15T13:29:26.1329726Z 
2020-02-15T13:29:26.1330170Z ---- [compile-fail] compile-fail/consts/const-err3.rs stdout ----
2020-02-15T13:29:26.1330341Z 
2020-02-15T13:29:26.1330823Z error: /checkout/src/test/compile-fail/consts/const-err3.rs:9: unexpected error: '9:13: 9:26: this arithmetic operation will overflow [overflow]'
2020-02-15T13:29:26.1330977Z 
2020-02-15T13:29:26.1331453Z error: /checkout/src/test/compile-fail/consts/const-err3.rs:11: unexpected error: '11:13: 11:22: this arithmetic operation will overflow [overflow]'
2020-02-15T13:29:26.1331602Z 
2020-02-15T13:29:26.1332070Z error: /checkout/src/test/compile-fail/consts/const-err3.rs:13: unexpected error: '13:13: 13:30: this arithmetic operation will overflow [overflow]'
2020-02-15T13:29:26.1332674Z error: /checkout/src/test/compile-fail/consts/const-err3.rs:15: unexpected error: '15:14: 15:22: this operation will panic at runtime [panic]'
2020-02-15T13:29:26.1332840Z 
2020-02-15T13:29:26.1333240Z error: /checkout/src/test/compile-fail/consts/const-err3.rs:9: expected error not found: const_err
2020-02-15T13:29:26.1333396Z 
---
2020-02-15T13:29:26.1334903Z error: /checkout/src/test/compile-fail/consts/const-err3.rs:15: expected error not found: const_err
2020-02-15T13:29:26.1335046Z 
2020-02-15T13:29:26.1335190Z error: 4 unexpected errors found, 4 expected errors not found
2020-02-15T13:29:26.1335352Z status: exit code: 1
2020-02-15T13:29:26.1336489Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/consts/const-err3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/consts/const-err3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/consts/const-err3/auxiliary" "-A" "unused"
2020-02-15T13:29:26.1339449Z     Error {
2020-02-15T13:29:26.1339641Z         line_num: 9,
2020-02-15T13:29:26.1339782Z         kind: Some(
2020-02-15T13:29:26.1339921Z             Error,
2020-02-15T13:29:26.1339921Z             Error,
2020-02-15T13:29:26.1340081Z         ),
2020-02-15T13:29:26.1340251Z         msg: "9:13: 9:26: this arithmetic operation will overflow [overflow]",
2020-02-15T13:29:26.1340554Z     Error {
2020-02-15T13:29:26.1340693Z         line_num: 11,
2020-02-15T13:29:26.1340848Z         kind: Some(
2020-02-15T13:29:26.1341003Z             Error,
2020-02-15T13:29:26.1341003Z             Error,
2020-02-15T13:29:26.1341140Z         ),
2020-02-15T13:29:26.1341305Z         msg: "11:13: 11:22: this arithmetic operation will overflow [overflow]",
2020-02-15T13:29:26.1341595Z     Error {
2020-02-15T13:29:26.1341750Z         line_num: 13,
2020-02-15T13:29:26.1341890Z         kind: Some(
2020-02-15T13:29:26.1342026Z             Error,
2020-02-15T13:29:26.1342026Z             Error,
2020-02-15T13:29:26.1342178Z         ),
2020-02-15T13:29:26.1342326Z         msg: "13:13: 13:30: this arithmetic operation will overflow [overflow]",
2020-02-15T13:29:26.1342616Z     Error {
2020-02-15T13:29:26.1342753Z         line_num: 15,
2020-02-15T13:29:26.1342891Z         kind: Some(
2020-02-15T13:29:26.1343089Z             Error,
---
2020-02-15T13:29:26.1353618Z test result: FAILED. 31 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-15T13:29:26.1353655Z 
2020-02-15T13:29:26.1353830Z 
2020-02-15T13:29:26.1353861Z 
2020-02-15T13:29:26.1355458Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-15T13:29:26.1355788Z 
2020-02-15T13:29:26.1355818Z 
2020-02-15T13:29:26.1356107Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-15T13:29:26.1356190Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-15T13:29:26.1356190Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-15T13:29:26.1356246Z Build completed unsuccessfully in 1:06:23
2020-02-15T13:29:26.1399721Z == clock drift check ==
2020-02-15T13:29:26.1420701Z   local time: Sat Feb 15 13:29:26 UTC 2020
2020-02-15T13:29:26.3102709Z   network time: Sat, 15 Feb 2020 13:29:26 GMT
2020-02-15T13:29:26.3112525Z == end clock drift check ==
2020-02-15T13:29:26.7580637Z 
2020-02-15T13:29:26.7654333Z ##[error]Bash exited with code '1'.
2020-02-15T13:29:26.7667881Z ##[section]Finishing: Run build
2020-02-15T13:29:26.7691046Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69185/merge to s
2020-02-15T13:29:26.7692957Z Task         : Get sources
2020-02-15T13:29:26.7693009Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T13:29:26.7693061Z Version      : 1.0.0
2020-02-15T13:29:26.7693124Z Author       : Microsoft
2020-02-15T13:29:26.7693124Z Author       : Microsoft
2020-02-15T13:29:26.7693175Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-15T13:29:26.7693230Z ==============================================================================
2020-02-15T13:29:27.2208204Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-15T13:29:27.2209686Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69185/merge to s
2020-02-15T13:29:27.2334466Z Cleaning up task key
2020-02-15T13:29:27.2335357Z Start cleaning up orphan processes.
2020-02-15T13:29:27.2472832Z Terminate orphan process: pid (5606) (python)
2020-02-15T13:29:27.2761852Z ##[section]Finishing: Finalize Job
