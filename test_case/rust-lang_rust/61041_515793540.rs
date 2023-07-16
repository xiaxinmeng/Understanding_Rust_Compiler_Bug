plain
2019-07-28T19:26:40.2264469Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-28T19:26:40.2445611Z ##[command]git config gc.auto 0
2019-07-28T19:26:40.2510485Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-28T19:26:40.2568580Z ##[command]git config --get-all http.proxy
2019-07-28T19:26:40.2691242Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61041/merge:refs/remotes/pull/61041/merge
---
2019-07-28T19:27:13.7272230Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-28T19:27:13.7272263Z 
2019-07-28T19:27:13.7272528Z   git checkout -b <new-branch-name>
2019-07-28T19:27:13.7272563Z 
2019-07-28T19:27:13.7272616Z HEAD is now at 46fe5f87a Merge 3b76ac6aa35862d8feaabdbd7aff8657ef718448 into 023525dbda35748a10713471b948974b68a1c2cc
2019-07-28T19:27:13.7434787Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-28T19:27:13.7438460Z ==============================================================================
2019-07-28T19:27:13.7438528Z Task         : Bash
2019-07-28T19:27:13.7438581Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-28T20:28:15.3097510Z .................................................................................................... 1400/8803
2019-07-28T20:28:21.3709535Z .................................................................................................... 1500/8803
2019-07-28T20:28:33.8563378Z .................................................................i...............i.................. 1600/8803
2019-07-28T20:28:41.4699941Z .................................................................................................... 1700/8803
2019-07-28T20:28:56.6921645Z ...................................................iiiii............................................ 1800/8803
2019-07-28T20:29:08.0102398Z .................................................................................................... 2000/8803
2019-07-28T20:29:10.5692944Z .................................................................................................... 2100/8803
2019-07-28T20:29:14.4954474Z .................................................................................................... 2200/8803
2019-07-28T20:29:21.0812505Z .................................................................................................... 2300/8803
---
2019-07-28T20:33:09.7610100Z .................................................................................................... 5200/8803
2019-07-28T20:33:20.4815396Z .................................................................................................... 5300/8803
2019-07-28T20:33:28.2020133Z ..i................................................................................................. 5400/8803
2019-07-28T20:33:33.5735741Z .................................................................................................... 5500/8803
2019-07-28T20:33:45.9777934Z ................................................................................................ii.. 5600/8803
2019-07-28T20:34:00.8316189Z .i..ii...........i.................................................................................. 5700/8803
2019-07-28T20:34:14.4873694Z .................................................................................................... 5900/8803
2019-07-28T20:34:19.3208175Z ................................................................................................i..i 6000/8803
2019-07-28T20:34:33.6487772Z i................................................................................................... 6100/8803
2019-07-28T20:34:50.1817165Z .................................................................................................... 6200/8803
---
2019-07-28T20:39:44.9155512Z  finished in 23.140
2019-07-28T20:39:44.9370033Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-28T20:39:45.1184277Z 
2019-07-28T20:39:45.1184409Z running 146 tests
2019-07-28T20:39:48.4342790Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-07-28T20:39:50.3157997Z iii..............i.........iii.i......ii......
2019-07-28T20:39:50.3158686Z 
2019-07-28T20:39:50.3158832Z  finished in 5.378
2019-07-28T20:39:50.3335175Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-28T20:39:50.4865527Z 
---
2019-07-28T20:39:52.5600382Z  finished in 2.226
2019-07-28T20:39:52.5776244Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-28T20:39:52.7366887Z 
2019-07-28T20:39:52.7367056Z running 9 tests
2019-07-28T20:39:52.7367775Z iiiiiiiii
2019-07-28T20:39:52.7368491Z 
2019-07-28T20:39:52.7377446Z  finished in 0.159
2019-07-28T20:39:52.7548682Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-28T20:39:52.9081666Z 
2019-07-28T20:39:52.9081666Z 
2019-07-28T20:39:52.9082453Z running 105 tests
2019-07-28T20:40:10.7372538Z .........................................................................F.......................... 100/105
2019-07-28T20:40:11.5443461Z .....
2019-07-28T20:40:11.5443905Z failures:
2019-07-28T20:40:11.5444073Z 
2019-07-28T20:40:11.5444582Z ---- [incremental] incremental/no_mangle2.rs stdout ----
2019-07-28T20:40:11.5446800Z thread '[incremental] incremental/no_mangle2.rs' panicked at 'failed to exec `"/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/no_mangle2/a"`: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }', src/libcore/result.rs:999:5
2019-07-28T20:40:11.5447767Z 
2019-07-28T20:40:11.5447942Z 
2019-07-28T20:40:11.5448405Z failures:
2019-07-28T20:40:11.5448616Z     [incremental] incremental/no_mangle2.rs
2019-07-28T20:40:11.5448616Z     [incremental] incremental/no_mangle2.rs
2019-07-28T20:40:11.5448971Z 
2019-07-28T20:40:11.5450225Z test result: FAILED. 104 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-07-28T20:40:11.5450508Z 
2019-07-28T20:40:11.5451929Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:534:22
2019-07-28T20:40:11.5452395Z 
2019-07-28T20:40:11.5452560Z 
2019-07-28T20:40:11.5454513Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-28T20:40:11.5456966Z 
2019-07-28T20:40:11.5457002Z 
2019-07-28T20:40:11.5464114Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-28T20:40:11.5464230Z Build completed unsuccessfully in 1:06:42
2019-07-28T20:40:11.5464230Z Build completed unsuccessfully in 1:06:42
2019-07-28T20:40:15.2473174Z ##[error]Bash exited with code '1'.
2019-07-28T20:40:15.2544564Z ##[section]Starting: Checkout
2019-07-28T20:40:15.2546768Z ==============================================================================
2019-07-28T20:40:15.2546853Z Task         : Get sources
2019-07-28T20:40:15.2546920Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
