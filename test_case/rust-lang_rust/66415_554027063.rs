plain
2019-11-14T17:48:28.1800500Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-14T17:48:28.1996757Z ##[command]git config gc.auto 0
2019-11-14T17:48:28.2065263Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-14T17:48:28.2124135Z ##[command]git config --get-all http.proxy
2019-11-14T17:48:28.2257804Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66415/merge:refs/remotes/pull/66415/merge
---
2019-11-14T18:41:11.6290225Z .................................................................................................... 1500/9240
2019-11-14T18:41:17.0655831Z .................................................................................................... 1600/9240
2019-11-14T18:41:25.3115653Z .................................................................................................... 1700/9240
2019-11-14T18:41:32.9446553Z ....i............................................................................................... 1800/9240
2019-11-14T18:41:38.9008229Z ........................................................................................iiiii....... 1900/9240
2019-11-14T18:41:58.7057750Z .................................................................................................... 2100/9240
2019-11-14T18:42:00.7566438Z .................................................................................................... 2200/9240
2019-11-14T18:42:03.0554514Z .................................................................................................... 2300/9240
2019-11-14T18:42:09.6045087Z .................................................................................................... 2400/9240
---
2019-11-14T18:44:49.2261796Z .......................................................................................i............ 4700/9240
2019-11-14T18:44:55.2335512Z ...i................................................................................................ 4800/9240
2019-11-14T18:45:03.7884263Z .................................................................................................... 4900/9240
2019-11-14T18:45:08.6221955Z .................................................................................................... 5000/9240
2019-11-14T18:45:18.5321241Z ..........................................................................................ii.ii..... 5100/9240
2019-11-14T18:45:26.0437401Z .........................i.......................................................................... 5300/9240
2019-11-14T18:45:33.6710609Z .................................................................................................... 5400/9240
2019-11-14T18:45:41.3868154Z .......................................................................i............................ 5500/9240
2019-11-14T18:45:47.9372053Z .................................................................................................... 5600/9240
2019-11-14T18:45:47.9372053Z .................................................................................................... 5600/9240
2019-11-14T18:45:54.3991404Z .................................................................................................... 5700/9240
2019-11-14T18:46:03.0966703Z .........................................................ii...i..ii...........i..................... 5800/9240
2019-11-14T18:46:23.3108505Z .................................................................................................... 6000/9240
2019-11-14T18:46:30.6134689Z .................................................................................................... 6100/9240
2019-11-14T18:46:30.6134689Z .................................................................................................... 6100/9240
2019-11-14T18:46:36.0478277Z ............................................................................i..ii................... 6200/9240
2019-11-14T18:47:02.0397728Z .................................................................................................... 6400/9240
2019-11-14T18:47:04.0876882Z ............................................i....................................................... 6500/9240
2019-11-14T18:47:05.9980343Z .................................................................................................... 6600/9240
2019-11-14T18:47:08.0508049Z ..............................i..................................................................... 6700/9240
---
2019-11-14T18:51:28.6105087Z ---- [ui] ui/test-panic-abort-disabled.rs stdout ----
2019-11-14T18:51:28.6105161Z 
2019-11-14T18:51:28.6105400Z error: error pattern 'building tests with panic=abort is not yet supported' not found!
2019-11-14T18:51:28.6105470Z status: exit code: 1
2019-11-14T18:51:28.6106340Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-panic-abort-disabled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort-disabled" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-Cpanic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort-disabled/auxiliary" "-A" "unused"
2019-11-14T18:51:28.6106650Z ------------------------------------------
2019-11-14T18:51:28.6106678Z 
2019-11-14T18:51:28.6106876Z ------------------------------------------
2019-11-14T18:51:28.6107072Z stderr:
---
2019-11-14T18:51:28.6137606Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-14T18:51:28.6137696Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-14T18:51:28.6152403Z 
2019-11-14T18:51:28.6152470Z 
2019-11-14T18:51:28.6153873Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-14T18:51:28.6154255Z 
2019-11-14T18:51:28.6154279Z 
2019-11-14T18:51:28.6160764Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-14T18:51:28.6160848Z Build completed unsuccessfully in 0:57:19
2019-11-14T18:51:28.6160848Z Build completed unsuccessfully in 0:57:19
2019-11-14T18:51:28.6206583Z == clock drift check ==
2019-11-14T18:51:28.6219760Z   local time: Thu Nov 14 18:51:28 UTC 2019
2019-11-14T18:51:28.6770798Z   network time: Thu, 14 Nov 2019 18:51:28 GMT
2019-11-14T18:51:28.6773944Z == end clock drift check ==
2019-11-14T18:51:29.4792387Z 
2019-11-14T18:51:29.4889336Z ##[error]Bash exited with code '1'.
2019-11-14T18:51:29.4922343Z ##[section]Starting: Checkout
2019-11-14T18:51:29.4923882Z ==============================================================================
2019-11-14T18:51:29.4923944Z Task         : Get sources
2019-11-14T18:51:29.4923982Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
