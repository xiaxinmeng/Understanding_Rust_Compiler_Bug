plain
2019-09-17T15:18:06.5610378Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-17T15:18:06.5794298Z ##[command]git config gc.auto 0
2019-09-17T15:18:06.5888239Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-17T15:18:06.5965031Z ##[command]git config --get-all http.proxy
2019-09-17T15:18:06.6137326Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64324/merge:refs/remotes/pull/64324/merge
---
2019-09-17T16:24:39.5688204Z .................................................................................................... 1500/9023
2019-09-17T16:24:45.9476058Z .................................................................................................... 1600/9023
2019-09-17T16:24:59.3149644Z ...............................................................i...............i.................... 1700/9023
2019-09-17T16:25:07.0279970Z .................................................................................................... 1800/9023
2019-09-17T16:25:23.3415422Z ......................................................iiiii......................................... 1900/9023
2019-09-17T16:25:35.4898360Z .................................................................................................... 2100/9023
2019-09-17T16:25:38.2573662Z .................................................................................................... 2200/9023
2019-09-17T16:25:41.9053257Z .................................................................................................... 2300/9023
2019-09-17T16:25:50.6989715Z .................................................................................................... 2400/9023
---
2019-09-17T16:29:00.5093226Z ...........................................i...............i........................................ 4700/9023
2019-09-17T16:29:11.8999463Z .................................................................................................... 4800/9023
2019-09-17T16:29:19.3020633Z .................................................................................................... 4900/9023
2019-09-17T16:29:29.7012594Z .................................................................................................... 5000/9023
2019-09-17T16:29:37.9314325Z ...........................ii.ii.................................................................... 5100/9023
2019-09-17T16:29:48.5940655Z .................................................................................................... 5300/9023
2019-09-17T16:29:59.6174189Z ...........................................................................................i........ 5400/9023
2019-09-17T16:30:08.4659613Z .................................................................................................... 5500/9023
2019-09-17T16:30:13.7089353Z .................................................................................................... 5600/9023
2019-09-17T16:30:13.7089353Z .................................................................................................... 5600/9023
2019-09-17T16:30:25.8436965Z ......................................................................................ii...i..ii.... 5700/9023
2019-09-17T16:30:51.8776905Z .................................................................................................... 5900/9023
2019-09-17T16:31:02.6303027Z .................................................................................................... 6000/9023
2019-09-17T16:31:02.6303027Z .................................................................................................... 6000/9023
2019-09-17T16:31:10.2850550Z ........................................................................................i..ii....... 6100/9023
2019-09-17T16:31:40.8087325Z .................................................................................................... 6300/9023
2019-09-17T16:31:44.8641154Z ...............................................i.................................................... 6400/9023
2019-09-17T16:31:47.1880442Z .................................................................................................... 6500/9023
2019-09-17T16:31:49.8538812Z ...................i................................................................................ 6600/9023
---
2019-09-17T16:36:11.1519901Z failures:
2019-09-17T16:36:11.1519980Z 
2019-09-17T16:36:11.1520254Z ---- [compile-fail] compile-fail/two-panic-runtimes.rs stdout ----
2019-09-17T16:36:11.1520288Z 
2019-09-17T16:36:11.1520739Z error: error pattern 'cannot link together two panic runtimes: panic_runtime_unwind and panic_runtime_unwind2' not found!
2019-09-17T16:36:11.1520929Z status: exit code: 1
2019-09-17T16:36:11.1523025Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/two-panic-runtimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/two-panic-runtimes" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/two-panic-runtimes/auxiliary" "-A" "unused"
2019-09-17T16:36:11.1523927Z ------------------------------------------
2019-09-17T16:36:11.1523968Z 
2019-09-17T16:36:11.1524370Z ------------------------------------------
2019-09-17T16:36:11.1524577Z stderr:
2019-09-17T16:36:11.1524577Z stderr:
2019-09-17T16:36:11.1525209Z ------------------------------------------
2019-09-17T16:36:11.1525394Z error: requires `start` lang_item
2019-09-17T16:36:11.1525524Z error: aborting due to previous error
2019-09-17T16:36:11.1525551Z 
2019-09-17T16:36:11.1525633Z 
2019-09-17T16:36:11.1525859Z ------------------------------------------
---
2019-09-17T16:36:11.1526772Z test result: FAILED. 30 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-17T16:36:11.1528159Z 
2019-09-17T16:36:11.1528241Z 
2019-09-17T16:36:11.1528315Z 
2019-09-17T16:36:11.1529769Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-17T16:36:11.1530169Z 
2019-09-17T16:36:11.1530213Z 
2019-09-17T16:36:11.1538037Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-17T16:36:11.1538120Z Build completed unsuccessfully in 1:10:40
2019-09-17T16:36:11.1538120Z Build completed unsuccessfully in 1:10:40
2019-09-17T16:36:11.1593337Z == clock drift check ==
2019-09-17T16:36:11.1617066Z   local time: Tue Sep 17 16:36:11 UTC 2019
2019-09-17T16:36:11.3155565Z   network time: Tue, 17 Sep 2019 16:36:11 GMT
2019-09-17T16:36:11.3155639Z == end clock drift check ==
2019-09-17T16:36:12.0476639Z ##[error]Bash exited with code '1'.
2019-09-17T16:36:12.0517562Z ##[section]Starting: Checkout
2019-09-17T16:36:12.0519380Z ==============================================================================
2019-09-17T16:36:12.0519444Z Task         : Get sources
2019-09-17T16:36:12.0519511Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
