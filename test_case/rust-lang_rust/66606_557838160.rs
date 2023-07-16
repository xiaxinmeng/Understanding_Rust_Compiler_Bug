plain
2019-11-23T21:16:40.5345903Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T21:16:40.5508193Z ##[command]git config gc.auto 0
2019-11-23T21:16:40.5623175Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T21:16:40.5629390Z ##[command]git config --get-all http.proxy
2019-11-23T21:16:40.5749163Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66606/merge:refs/remotes/pull/66606/merge
---
2019-11-23T22:11:05.8828584Z .....................F.............................................................................. 1600/9282
2019-11-23T22:11:10.3258713Z .................................................................................................... 1700/9282
2019-11-23T22:11:22.4013941Z .............................i...................................................................... 1800/9282
2019-11-23T22:11:28.6162044Z .................................................................................................... 1900/9282
2019-11-23T22:11:41.4425952Z ..............iiiii................................................................................. 2000/9282
2019-11-23T22:11:50.5452087Z .................................................................................................... 2200/9282
2019-11-23T22:11:52.8808684Z .................................................................................................... 2300/9282
2019-11-23T22:11:57.7944935Z .................................................................................................... 2400/9282
2019-11-23T22:12:17.1231587Z .................................................................................................... 2500/9282
---
2019-11-23T22:14:47.8110306Z ..............i...............i..................................................................... 4800/9282
2019-11-23T22:14:57.4691214Z .................................................................................................... 4900/9282
2019-11-23T22:15:02.8567159Z .................................................................................................... 5000/9282
2019-11-23T22:15:11.9169326Z .................................................................................................... 5100/9282
2019-11-23T22:15:17.6891265Z ...................ii.ii...........i................................................................ 5200/9282
2019-11-23T22:15:27.0837719Z .................................................................................................... 5400/9282
2019-11-23T22:15:36.7786822Z .................................................................................................... 5500/9282
2019-11-23T22:15:44.5873096Z .i.................................................................................................. 5600/9282
2019-11-23T22:15:50.0884141Z .................................................................................................... 5700/9282
2019-11-23T22:15:50.0884141Z .................................................................................................... 5700/9282
2019-11-23T22:15:59.0657211Z .......................................................................................ii...i..ii... 5800/9282
2019-11-23T22:16:20.7796785Z .................................................................................................... 6000/9282
2019-11-23T22:16:28.4515600Z .................................................................................................... 6100/9282
2019-11-23T22:16:32.6362821Z .................................................................................................... 6200/9282
2019-11-23T22:16:32.6362821Z .................................................................................................... 6200/9282
2019-11-23T22:16:45.2135071Z ..........i..ii..................................................................................... 6300/9282
2019-11-23T22:17:02.8423758Z ..............................................................................i..................... 6500/9282
2019-11-23T22:17:04.9849840Z .................................................................................................... 6600/9282
2019-11-23T22:17:07.1268599Z .....................................................................i.............................. 6700/9282
2019-11-23T22:17:09.8857209Z .................................................................................................... 6800/9282
---
2019-11-23T22:21:37.4687012Z 9 
2019-11-23T22:21:37.4687039Z 
2019-11-23T22:21:37.4687080Z 
2019-11-23T22:21:37.4687120Z The actual stderr differed from the expected stderr.
2019-11-23T22:21:37.4687447Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mutable_references/mutable_references.stderr
2019-11-23T22:21:37.4687707Z To update references, rerun the tests and pass the `--bless` flag
2019-11-23T22:21:37.4687965Z To only update this specific test, also pass `--test-args consts/const-mut-refs/mutable_references.rs`
2019-11-23T22:21:37.4688033Z error: 1 errors occurred comparing output.
2019-11-23T22:21:37.4688090Z status: exit code: 1
2019-11-23T22:21:37.4688090Z status: exit code: 1
2019-11-23T22:21:37.4688790Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-mut-refs/mutable_references.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mutable_references" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mutable_references/auxiliary" "-A" "unused"
2019-11-23T22:21:37.4689226Z ------------------------------------------
2019-11-23T22:21:37.4689254Z 
2019-11-23T22:21:37.4689466Z ------------------------------------------
2019-11-23T22:21:37.4689504Z stderr:
2019-11-23T22:21:37.4689504Z stderr:
2019-11-23T22:21:37.4689691Z ------------------------------------------
2019-11-23T22:21:37.4689755Z error[E0594]: cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2019-11-23T22:21:37.4689986Z   --> /checkout/src/test/ui/consts/const-mut-refs/mutable_references.rs:32:5
2019-11-23T22:21:37.4690029Z    |
2019-11-23T22:21:37.4690099Z LL |     *OH_YES = 99; //~ ERROR cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2019-11-23T22:21:37.4690168Z 
2019-11-23T22:21:37.4690222Z error: aborting due to previous error
2019-11-23T22:21:37.4690247Z 
2019-11-23T22:21:37.4690466Z For more information about this error, try `rustc --explain E0594`.
---
2019-11-23T22:21:37.4725612Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-23T22:21:37.4725691Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-23T22:21:37.4748769Z 
2019-11-23T22:21:37.4750534Z 
2019-11-23T22:21:37.4753840Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-23T22:21:37.4754471Z 
2019-11-23T22:21:37.4754508Z 
2019-11-23T22:21:37.4764029Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-23T22:21:37.4764295Z Build completed unsuccessfully in 0:59:06
2019-11-23T22:21:37.4764295Z Build completed unsuccessfully in 0:59:06
2019-11-23T22:21:37.4819540Z == clock drift check ==
2019-11-23T22:21:37.4831617Z   local time: Sat Nov 23 22:21:37 UTC 2019
2019-11-23T22:21:38.0049798Z   network time: Sat, 23 Nov 2019 22:21:38 GMT
2019-11-23T22:21:38.0050887Z == end clock drift check ==
2019-11-23T22:21:38.8714258Z 
2019-11-23T22:21:38.8810914Z ##[error]Bash exited with code '1'.
2019-11-23T22:21:38.8843155Z ##[section]Starting: Checkout
2019-11-23T22:21:38.8844653Z ==============================================================================
2019-11-23T22:21:38.8844699Z Task         : Get sources
2019-11-23T22:21:38.8844868Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
