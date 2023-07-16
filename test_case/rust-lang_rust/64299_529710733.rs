plain
2019-09-09T22:38:43.6187493Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T22:38:43.6416237Z ##[command]git config gc.auto 0
2019-09-09T22:38:43.6507174Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T22:38:43.6564312Z ##[command]git config --get-all http.proxy
2019-09-09T22:38:43.6747041Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64299/merge:refs/remotes/pull/64299/merge
---
2019-09-09T23:41:59.0466570Z .................................................................................................... 1500/9006
2019-09-09T23:42:05.1320798Z .................................................................................................... 1600/9006
2019-09-09T23:42:18.1398914Z .......................................................i...............i............................ 1700/9006
2019-09-09T23:42:26.0407265Z .................................................................................................... 1800/9006
2019-09-09T23:42:40.9467305Z ..............................................iiiii................................................. 1900/9006
2019-09-09T23:42:52.3138357Z .................................................................................................... 2100/9006
2019-09-09T23:42:54.9772052Z .................................................................................................... 2200/9006
2019-09-09T23:42:58.9724755Z .................................................................................................... 2300/9006
2019-09-09T23:43:06.9832823Z .................................................................................................... 2400/9006
---
2019-09-09T23:46:09.9044297Z .................................i...............i.................................................. 4700/9006
2019-09-09T23:46:21.8585394Z .................................................................................................... 4800/9006
2019-09-09T23:46:28.2480012Z .................................................................................................... 4900/9006
2019-09-09T23:46:39.3340154Z .................................................................................................... 5000/9006
2019-09-09T23:46:45.5880893Z ................ii.ii............................................................................... 5100/9006
2019-09-09T23:46:56.6057359Z .................................................................................................... 5300/9006
2019-09-09T23:47:06.9517719Z ...............................................................................i.................... 5400/9006
2019-09-09T23:47:14.9702489Z .................................................................................................... 5500/9006
2019-09-09T23:47:21.2330030Z .................................................................................................... 5600/9006
2019-09-09T23:47:21.2330030Z .................................................................................................... 5600/9006
2019-09-09T23:47:32.1000516Z .........................................................................ii...i..ii...........i..... 5700/9006
2019-09-09T23:47:57.6708165Z .................................................................................................... 5900/9006
2019-09-09T23:48:07.5377322Z .................................................................................................... 6000/9006
2019-09-09T23:48:07.5377322Z .................................................................................................... 6000/9006
2019-09-09T23:48:13.1010021Z ...........................................................................i..ii.................... 6100/9006
2019-09-09T23:48:43.4449925Z .................................................................................................... 6300/9006
2019-09-09T23:48:45.6141180Z ..................................i................................................................. 6400/9006
2019-09-09T23:48:47.8830068Z .................................................................................................... 6500/9006
2019-09-09T23:48:50.5840312Z ......i............................................................................................. 6600/9006
---
2019-09-09T23:52:59.3688372Z failures:
2019-09-09T23:52:59.3708857Z 
2019-09-09T23:52:59.3709442Z ---- [ui] ui/rfc-2126-extern-absolute-paths/non-existent-3.rs stdout ----
2019-09-09T23:52:59.3709522Z 
2019-09-09T23:52:59.3709607Z error: Error: expected failure status (Some(1)) but received status None.
2019-09-09T23:52:59.3709662Z status: signal: 11
2019-09-09T23:52:59.3710611Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2126-extern-absolute-paths/non-existent-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-extern-absolute-paths/non-existent-3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-extern-absolute-paths/non-existent-3/auxiliary" "-A" "unused"
2019-09-09T23:52:59.3711002Z ------------------------------------------
2019-09-09T23:52:59.3711302Z 
2019-09-09T23:52:59.3711607Z ------------------------------------------
2019-09-09T23:52:59.3711659Z stderr:
2019-09-09T23:52:59.3711659Z stderr:
2019-09-09T23:52:59.3711916Z ------------------------------------------
2019-09-09T23:52:59.3711951Z 
2019-09-09T23:52:59.3712190Z ------------------------------------------
2019-09-09T23:52:59.3712224Z 
2019-09-09T23:52:59.3712251Z 
2019-09-09T23:52:59.3712549Z ---- [ui] ui/rfc-2126-extern-absolute-paths/not-whitelisted.rs stdout ----
2019-09-09T23:52:59.3712588Z 
2019-09-09T23:52:59.3712656Z error: Error: expected failure status (Some(1)) but received status None.
2019-09-09T23:52:59.3712726Z status: signal: 11
2019-09-09T23:52:59.3713952Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2126-extern-absolute-paths/not-whitelisted.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-extern-absolute-paths/not-whitelisted" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-extern-absolute-paths/not-whitelisted/auxiliary" "-A" "unused"
2019-09-09T23:52:59.3714422Z ------------------------------------------
2019-09-09T23:52:59.3714459Z 
2019-09-09T23:52:59.3714725Z ------------------------------------------
2019-09-09T23:52:59.3714953Z stderr:
2019-09-09T23:52:59.3714953Z stderr:
2019-09-09T23:52:59.3715247Z ------------------------------------------
2019-09-09T23:52:59.3715282Z 
2019-09-09T23:52:59.3715540Z ------------------------------------------
2019-09-09T23:52:59.3715573Z 
2019-09-09T23:52:59.3715601Z 
2019-09-09T23:52:59.3715837Z ---- [ui] ui/use/use-keyword.rs stdout ----
2019-09-09T23:52:59.3715888Z 
2019-09-09T23:52:59.3715942Z error: Error: expected failure status (Some(1)) but received status None.
2019-09-09T23:52:59.3716008Z status: signal: 11
2019-09-09T23:52:59.3716817Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/use/use-keyword.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-keyword" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-keyword/auxiliary" "-A" "unused"
2019-09-09T23:52:59.3717174Z ------------------------------------------
2019-09-09T23:52:59.3717210Z 
2019-09-09T23:52:59.3717448Z ------------------------------------------
2019-09-09T23:52:59.3717497Z stderr:
2019-09-09T23:52:59.3717497Z stderr:
2019-09-09T23:52:59.3717750Z ------------------------------------------
2019-09-09T23:52:59.3717806Z error[E0429]: `self` imports are only allowed within a { } list
2019-09-09T23:52:59.3718157Z    |
2019-09-09T23:52:59.3718205Z LL |         use self as A;
2019-09-09T23:52:59.3718254Z    |             ^^^^^^^^^
2019-09-09T23:52:59.3718300Z 
2019-09-09T23:52:59.3718300Z 
2019-09-09T23:52:59.3718329Z 
2019-09-09T23:52:59.3718569Z ------------------------------------------
2019-09-09T23:52:59.3718604Z 
2019-09-09T23:52:59.3718632Z 
2019-09-09T23:52:59.3718895Z ---- [ui] ui/use/use-mod/use-mod-2.rs stdout ----
2019-09-09T23:52:59.3718929Z 
2019-09-09T23:52:59.3718991Z error: Error: expected failure status (Some(1)) but received status None.
2019-09-09T23:52:59.3719060Z status: signal: 11
2019-09-09T23:52:59.3719866Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/use/use-mod/use-mod-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-mod/use-mod-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-mod/use-mod-2/auxiliary" "-A" "unused"
2019-09-09T23:52:59.3720394Z ------------------------------------------
2019-09-09T23:52:59.3720430Z 
2019-09-09T23:52:59.3720672Z ------------------------------------------
2019-09-09T23:52:59.3720739Z stderr:
---
2019-09-09T23:52:59.3766980Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-09T23:52:59.3767083Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-09T23:52:59.3783824Z 
2019-09-09T23:52:59.3784131Z 
2019-09-09T23:52:59.3785987Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T23:52:59.3790328Z 
2019-09-09T23:52:59.3790362Z 
2019-09-09T23:52:59.3790459Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T23:52:59.3790551Z Build completed unsuccessfully in 1:07:04
2019-09-09T23:52:59.3790551Z Build completed unsuccessfully in 1:07:04
2019-09-09T23:52:59.3848187Z == clock drift check ==
2019-09-09T23:52:59.3865369Z   local time: Mon Sep  9 23:52:59 UTC 2019
2019-09-09T23:52:59.5233784Z   network time: Mon, 09 Sep 2019 23:52:59 GMT
2019-09-09T23:52:59.5239893Z == end clock drift check ==
2019-09-09T23:53:00.3532355Z ##[error]Bash exited with code '1'.
2019-09-09T23:53:00.3593144Z ##[section]Starting: Checkout
2019-09-09T23:53:00.3595597Z ==============================================================================
2019-09-09T23:53:00.3595678Z Task         : Get sources
2019-09-09T23:53:00.3595730Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
