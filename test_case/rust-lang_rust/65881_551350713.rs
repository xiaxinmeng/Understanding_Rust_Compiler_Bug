plain
2019-11-08T00:54:34.3437581Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T00:54:34.3614811Z ##[command]git config gc.auto 0
2019-11-08T00:54:34.3694903Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T00:54:34.3754923Z ##[command]git config --get-all http.proxy
2019-11-08T00:54:34.3910365Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65881/merge:refs/remotes/pull/65881/merge
---
2019-11-08T01:51:52.1633642Z .................................................................................................... 1600/9290
2019-11-08T01:51:57.4291322Z .................................................................................................... 1700/9290
2019-11-08T01:52:09.1596333Z ...............................................................i.................................... 1800/9290
2019-11-08T01:52:16.5286493Z .................................................................................................... 1900/9290
2019-11-08T01:52:30.2802035Z ...............................................iiiii................................................ 2000/9290
2019-11-08T01:52:40.2994639Z .................................................................................................... 2200/9290
2019-11-08T01:52:42.7470184Z .................................................................................................... 2300/9290
2019-11-08T01:52:46.3030378Z .................................................................................................... 2400/9290
2019-11-08T01:53:08.1955324Z .................................................................................................... 2500/9290
---
2019-11-08T01:55:47.5471211Z ............................................i...............i....................................... 4800/9290
2019-11-08T01:55:56.5215125Z .................................................................................................... 4900/9290
2019-11-08T01:56:03.9246318Z .................................................................................................... 5000/9290
2019-11-08T01:56:10.2570858Z .................................................................................................... 5100/9290
2019-11-08T01:56:19.5559597Z ..............................................ii.ii...........i..................................... 5200/9290
2019-11-08T01:56:28.8088200Z .................................................................................................... 5400/9290
2019-11-08T01:56:38.4155407Z .................................................................................................... 5500/9290
2019-11-08T01:56:45.3151406Z ............................i....................................................................... 5600/9290
2019-11-08T01:56:52.2119473Z .................................................................................................... 5700/9290
2019-11-08T01:56:52.2119473Z .................................................................................................... 5700/9290
2019-11-08T01:57:02.4609940Z .................................................................................................... 5800/9290
2019-11-08T01:57:13.4995669Z .............ii...i..ii............i................................................................ 5900/9290
2019-11-08T01:57:32.4660029Z .................................................................................................... 6100/9290
2019-11-08T01:57:37.3811317Z .................................................................................................... 6200/9290
2019-11-08T01:57:37.3811317Z .................................................................................................... 6200/9290
2019-11-08T01:57:50.3350694Z ................................i..ii............................................................... 6300/9290
2019-11-08T01:58:09.9389343Z .................................................................................................... 6500/9290
2019-11-08T01:58:12.0212365Z i................................................................................................... 6600/9290
2019-11-08T01:58:14.1722476Z ....................................................................................i............... 6700/9290
2019-11-08T01:58:16.7102653Z .................................................................................................... 6800/9290
---
2019-11-08T02:02:50.9441242Z ---- [ui] ui/codegen-object-shim.rs stdout ----
2019-11-08T02:02:50.9441285Z 
2019-11-08T02:02:50.9441541Z error: test compilation failed although it shouldn't!
2019-11-08T02:02:50.9441593Z status: exit code: 101
2019-11-08T02:02:50.9442343Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codegen-object-shim.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen-object-shim/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen-object-shim/auxiliary"
2019-11-08T02:02:50.9442702Z ------------------------------------------
2019-11-08T02:02:50.9442746Z 
2019-11-08T02:02:50.9443196Z ------------------------------------------
2019-11-08T02:02:50.9443269Z stderr:
2019-11-08T02:02:50.9443269Z stderr:
2019-11-08T02:02:50.9443529Z ------------------------------------------
2019-11-08T02:02:50.9443599Z error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:749: Instance { def: Virtual(DefId(5:3701 ~ alloc[b012]::string[0]::ToString[0]::to_string[0]), 0), substs: [dyn std::string::ToString] } being reified
2019-11-08T02:02:50.9443924Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:931:9
2019-11-08T02:02:50.9443981Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-08T02:02:50.9444045Z 
2019-11-08T02:02:50.9444092Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-08T02:02:50.9444092Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-08T02:02:50.9444122Z 
2019-11-08T02:02:50.9444891Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-11-08T02:02:50.9445211Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-11-08T02:02:50.9445246Z 
2019-11-08T02:02:50.9445246Z 
2019-11-08T02:02:50.9445549Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-11-08T02:02:50.9445632Z error: aborting due to previous error
2019-11-08T02:02:50.9445660Z 
2019-11-08T02:02:50.9445685Z 
2019-11-08T02:02:50.9445923Z ------------------------------------------
---
2019-11-08T02:02:50.9477601Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-08T02:02:50.9477690Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-08T02:02:50.9493942Z 
2019-11-08T02:02:50.9494670Z 
2019-11-08T02:02:50.9499191Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-08T02:02:50.9499511Z 
2019-11-08T02:02:50.9509668Z 
2019-11-08T02:02:50.9509768Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-08T02:02:50.9509968Z Build completed unsuccessfully in 1:01:48
2019-11-08T02:02:50.9509968Z Build completed unsuccessfully in 1:01:48
2019-11-08T02:02:50.9560804Z == clock drift check ==
2019-11-08T02:02:50.9575066Z   local time: Fri Nov  8 02:02:50 UTC 2019
2019-11-08T02:02:51.1142938Z   network time: Fri, 08 Nov 2019 02:02:51 GMT
2019-11-08T02:02:51.1146971Z == end clock drift check ==
2019-11-08T02:02:51.9424375Z 
2019-11-08T02:02:51.9495563Z ##[error]Bash exited with code '1'.
2019-11-08T02:02:51.9543444Z ##[section]Starting: Checkout
2019-11-08T02:02:51.9545587Z ==============================================================================
2019-11-08T02:02:51.9545666Z Task         : Get sources
2019-11-08T02:02:51.9545713Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
