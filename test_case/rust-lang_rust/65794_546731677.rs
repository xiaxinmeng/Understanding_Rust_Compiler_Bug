plain
2019-10-27T19:31:29.8737352Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-27T19:31:29.8902810Z ##[command]git config gc.auto 0
2019-10-27T19:31:29.8966386Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-27T19:31:29.9010342Z ##[command]git config --get-all http.proxy
2019-10-27T19:31:29.9120745Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65794/merge:refs/remotes/pull/65794/merge
---
2019-10-27T20:23:49.7686428Z .................................................................................................... 1600/9253
2019-10-27T20:23:54.9695515Z .................................................................................................... 1700/9253
2019-10-27T20:24:06.1724242Z ..........................................................i...............i......................... 1800/9253
2019-10-27T20:24:13.1558578Z .................................................................................................... 1900/9253
2019-10-27T20:24:26.2818822Z ................................................iiiii............................................... 2000/9253
2019-10-27T20:24:36.1289763Z .................................................................................................... 2200/9253
2019-10-27T20:24:38.4463330Z .................................................................................................... 2300/9253
2019-10-27T20:24:41.8467347Z .................................................................................................... 2400/9253
2019-10-27T20:25:02.5274285Z .................................................................................................... 2500/9253
---
2019-10-27T20:27:37.5546275Z ..................................................i...............i................................. 4800/9253
2019-10-27T20:27:45.6790304Z .................................................................................................... 4900/9253
2019-10-27T20:27:53.2560067Z .................................................................................................... 5000/9253
2019-10-27T20:27:58.8721220Z .................................................................................................... 5100/9253
2019-10-27T20:28:08.2538736Z ...................................................ii.ii...........i................................ 5200/9253
2019-10-27T20:28:16.9978625Z .................................................................................................... 5400/9253
2019-10-27T20:28:25.5381296Z .................................................................................................... 5500/9253
2019-10-27T20:28:32.7966764Z .....................i.............................................................................. 5600/9253
2019-10-27T20:28:37.8840584Z .................................................................................................... 5700/9253
2019-10-27T20:28:37.8840584Z .................................................................................................... 5700/9253
2019-10-27T20:28:48.9541756Z .................................................................................................... 5800/9253
2019-10-27T20:28:59.4373145Z ..................ii...i..ii...........i............................................................ 5900/9253
2019-10-27T20:29:19.7175513Z .................................................................................................... 6100/9253
2019-10-27T20:29:27.1769202Z .................................................................................................... 6200/9253
2019-10-27T20:29:27.1769202Z .................................................................................................... 6200/9253
2019-10-27T20:29:38.6508437Z .........................................i..ii...................................................... 6300/9253
2019-10-27T20:29:58.4058690Z .................................................................................................... 6500/9253
2019-10-27T20:30:00.3601246Z ........i........................................................................................... 6600/9253
2019-10-27T20:30:02.3878267Z ...................................................................................i................ 6700/9253
2019-10-27T20:30:04.7854496Z .................................................................................................... 6800/9253
---
2019-10-27T20:34:02.0893362Z  finished in 5.188
2019-10-27T20:34:02.1060012Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T20:34:02.2704613Z 
2019-10-27T20:34:02.2704830Z running 153 tests
2019-10-27T20:34:05.0344666Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-27T20:34:06.7572095Z i..iiii..............i.........iii.i.........ii......
2019-10-27T20:34:06.7574253Z 
2019-10-27T20:34:06.7578527Z  finished in 4.652
2019-10-27T20:34:06.7728182Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T20:34:06.9116747Z 
---
2019-10-27T20:34:08.6690601Z  finished in 1.896
2019-10-27T20:34:08.6854727Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T20:34:08.8191483Z 
2019-10-27T20:34:08.8191940Z running 9 tests
2019-10-27T20:34:08.8192522Z iiiiiiiii
2019-10-27T20:34:08.8193119Z 
2019-10-27T20:34:08.8200122Z  finished in 0.133
2019-10-27T20:34:08.8340331Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T20:34:08.9977137Z 
2019-10-27T20:34:08.9977137Z 
2019-10-27T20:34:08.9978078Z running 109 tests
2019-10-27T20:34:23.4779957Z ....................................................................F............................... 100/109
2019-10-27T20:34:24.7046723Z .........
2019-10-27T20:34:24.7047154Z failures:
2019-10-27T20:34:24.7050756Z 
2019-10-27T20:34:24.7051278Z ---- [incremental] incremental/issue-59523-on-implemented-is-not-unused.rs stdout ----
2019-10-27T20:34:24.7051314Z 
2019-10-27T20:34:24.7051518Z error in revision `cfail1`: test compilation failed although it shouldn't!
2019-10-27T20:34:24.7051596Z status: exit code: 1
2019-10-27T20:34:24.7052812Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-59523-on-implemented-is-not-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-59523-on-implemented-is-not-unused/issue-59523-on-implemented-is-not-unused.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-59523-on-implemented-is-not-unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-59523-on-implemented-is-not-unused/auxiliary"
2019-10-27T20:34:24.7053399Z ------------------------------------------
2019-10-27T20:34:24.7053431Z 
2019-10-27T20:34:24.7053599Z ------------------------------------------
2019-10-27T20:34:24.7053657Z stderr:
---
2019-10-27T20:34:24.7054474Z 
2019-10-27T20:34:24.7054515Z error[E0658]: this is an internal attribute that will never be stable
2019-10-27T20:34:24.7054796Z   --> /checkout/src/test/incremental/issue-59523-on-implemented-is-not-unused.rs:11:1
2019-10-27T20:34:24.7054963Z    |
2019-10-27T20:34:24.7055048Z LL | #[rustc_on_unimplemented = "invalid"]
2019-10-27T20:34:24.7055141Z    |
2019-10-27T20:34:24.7055141Z    |
2019-10-27T20:34:24.7055713Z    = note: for more information, see ***/issues/29642
2019-10-27T20:34:24.7055963Z    = help: add `#![feature(rustc_attrs)]` to the crate attributes to enable
2019-10-27T20:34:24.7056424Z error[E0658]: this is an internal attribute that will never be stable
2019-10-27T20:34:24.7058318Z   --> /checkout/src/test/incremental/issue-59523-on-implemented-is-not-unused.rs:17:1
2019-10-27T20:34:24.7058649Z    |
2019-10-27T20:34:24.7058649Z    |
2019-10-27T20:34:24.7058728Z LL | #[rustc_on_unimplemented = "a usize is required to index into a slice"]
2019-10-27T20:34:24.7058853Z    |
2019-10-27T20:34:24.7058853Z    |
2019-10-27T20:34:24.7059168Z    = note: for more information, see ***/issues/29642
2019-10-27T20:34:24.7059349Z    = help: add `#![feature(rustc_attrs)]` to the crate attributes to enable
2019-10-27T20:34:24.7059490Z error: aborting due to 3 previous errors
2019-10-27T20:34:24.7059514Z 
2019-10-27T20:34:24.7059574Z Some errors have detailed explanations: E0557, E0658.
2019-10-27T20:34:24.7059859Z For more information about an error, try `rustc --explain E0557`.
---
2019-10-27T20:34:24.7062699Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-27T20:34:24.7062749Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-27T20:34:24.7062775Z 
2019-10-27T20:34:24.7062795Z 
2019-10-27T20:34:24.7064208Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-27T20:34:24.7064620Z 
2019-10-27T20:34:24.7064645Z 
2019-10-27T20:34:24.7068512Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-27T20:34:24.7068840Z Build completed unsuccessfully in 0:56:49
2019-10-27T20:34:24.7068840Z Build completed unsuccessfully in 0:56:49
2019-10-27T20:34:24.7117717Z == clock drift check ==
2019-10-27T20:34:24.7132498Z   local time: Sun Oct 27 20:34:24 UTC 2019
2019-10-27T20:34:24.9774348Z   network time: Sun, 27 Oct 2019 20:34:24 GMT
2019-10-27T20:34:24.9776520Z == end clock drift check ==
2019-10-27T20:34:30.5791716Z 
2019-10-27T20:34:30.5889192Z ##[error]Bash exited with code '1'.
2019-10-27T20:34:30.5922373Z ##[section]Starting: Checkout
2019-10-27T20:34:30.5923905Z ==============================================================================
2019-10-27T20:34:30.5923949Z Task         : Get sources
2019-10-27T20:34:30.5924002Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
