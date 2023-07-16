plain
2019-10-27T01:16:24.8176848Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-27T01:16:24.8338648Z ##[command]git config gc.auto 0
2019-10-27T01:16:24.8403322Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-27T01:16:24.8457893Z ##[command]git config --get-all http.proxy
2019-10-27T01:16:24.8585111Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-27T02:09:15.9433285Z .................................................................................................... 1600/9253
2019-10-27T02:09:21.3947050Z .................................................................................................... 1700/9253
2019-10-27T02:09:33.1824322Z .........................................................i...............i.......................... 1800/9253
2019-10-27T02:09:40.4943179Z .................................................................................................... 1900/9253
2019-10-27T02:09:53.9245945Z ...............................................iiiii................................................ 2000/9253
2019-10-27T02:10:04.1002189Z .................................................................................................... 2200/9253
2019-10-27T02:10:06.4550509Z .................................................................................................... 2300/9253
2019-10-27T02:10:09.9820255Z .................................................................................................... 2400/9253
2019-10-27T02:10:30.8470684Z .................................................................................................... 2500/9253
---
2019-10-27T02:13:08.1220059Z ..................................................i...............i................................. 4800/9253
2019-10-27T02:13:16.3226213Z .................................................................................................... 4900/9253
2019-10-27T02:13:24.0246040Z .................................................................................................... 5000/9253
2019-10-27T02:13:29.6840069Z .................................................................................................... 5100/9253
2019-10-27T02:13:38.9882325Z ...................................................ii.ii...........i................................ 5200/9253
2019-10-27T02:13:47.8463098Z .................................................................................................... 5400/9253
2019-10-27T02:13:56.0901290Z .................................................................................................... 5500/9253
2019-10-27T02:14:03.5578233Z .....................i.............................................................................. 5600/9253
2019-10-27T02:14:08.5695110Z .................................................................................................... 5700/9253
2019-10-27T02:14:08.5695110Z .................................................................................................... 5700/9253
2019-10-27T02:14:19.4288207Z .................................................................................................... 5800/9253
2019-10-27T02:14:29.5805347Z ..................ii...i..ii...........i............................................................ 5900/9253
2019-10-27T02:14:49.2308099Z .................................................................................................... 6100/9253
2019-10-27T02:14:54.5156801Z .................................................................................................... 6200/9253
2019-10-27T02:14:54.5156801Z .................................................................................................... 6200/9253
2019-10-27T02:15:05.8705405Z .........................................i..ii...................................................... 6300/9253
2019-10-27T02:15:25.3627397Z .................................................................................................... 6500/9253
2019-10-27T02:15:28.0943776Z .......i............................................................................................ 6600/9253
2019-10-27T02:15:29.3302077Z ..................................................................................i................. 6700/9253
2019-10-27T02:15:31.8151985Z .................................................................................................... 6800/9253
---
2019-10-27T02:19:37.7507148Z  finished in 5.276
2019-10-27T02:19:37.7677610Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T02:19:37.9118976Z 
2019-10-27T02:19:37.9119263Z running 153 tests
2019-10-27T02:19:40.7454871Z i....iii......iii..iiii...i.............................i..i..................i....i............iii. 100/153
2019-10-27T02:19:42.5512402Z i..iiii..............i.........iii..i........ii......
2019-10-27T02:19:42.5513484Z 
2019-10-27T02:19:42.5521689Z  finished in 4.784
2019-10-27T02:19:42.5688971Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T02:19:42.7065040Z 
---
2019-10-27T02:19:44.5418909Z  finished in 1.973
2019-10-27T02:19:44.5574953Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T02:19:44.6994203Z 
2019-10-27T02:19:44.6995088Z running 9 tests
2019-10-27T02:19:44.6996204Z iiiiiiiii
2019-10-27T02:19:44.6996963Z 
2019-10-27T02:19:44.7000046Z  finished in 0.142
2019-10-27T02:19:44.7162826Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T02:19:44.8569358Z 
2019-10-27T02:19:44.8569358Z 
2019-10-27T02:19:44.8569992Z running 109 tests
2019-10-27T02:20:00.2508642Z .................................................F.................................................. 100/109
2019-10-27T02:20:01.5907613Z .........
2019-10-27T02:20:01.5907745Z failures:
2019-10-27T02:20:01.5907839Z 
2019-10-27T02:20:01.5911465Z ---- [incremental] incremental/hashes/unary_and_binary_exprs.rs stdout ----
2019-10-27T02:20:01.5911514Z 
2019-10-27T02:20:01.5911814Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-27T02:20:01.5911867Z status: exit code: 101
2019-10-27T02:20:01.5914912Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/unary_and_binary_exprs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs/unary_and_binary_exprs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs/auxiliary"
2019-10-27T02:20:01.5916003Z ------------------------------------------
2019-10-27T02:20:01.5916040Z 
2019-10-27T02:20:01.5916260Z ------------------------------------------
2019-10-27T02:20:01.5916305Z stderr:
2019-10-27T02:20:01.5916305Z stderr:
2019-10-27T02:20:01.5916531Z ------------------------------------------
2019-10-27T02:20:01.5916861Z thread 'rustc' panicked at 'found unstable fingerprints for const_caller_location(7de0a38047d74950-f4f2ced447ab0242)', src/librustc/ty/query/plumbing.rs:490:9
2019-10-27T02:20:01.5917099Z 
2019-10-27T02:20:01.5917143Z error: internal compiler error: unexpected panic
2019-10-27T02:20:01.5917183Z 
2019-10-27T02:20:01.5917227Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-27T02:20:01.5917227Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-27T02:20:01.5917283Z 
2019-10-27T02:20:01.5917718Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-27T02:20:01.5918016Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-27T02:20:01.5918048Z 
2019-10-27T02:20:01.5918048Z 
2019-10-27T02:20:01.5918409Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-27T02:20:01.5918499Z 
2019-10-27T02:20:01.5919932Z ------------------------------------------
2019-10-27T02:20:01.5919970Z 
2019-10-27T02:20:01.5919995Z 
---
2019-10-27T02:20:01.5928508Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-27T02:20:01.5928607Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-27T02:20:01.5934784Z 
2019-10-27T02:20:01.5934845Z 
2019-10-27T02:20:01.5936388Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-27T02:20:01.5936627Z 
2019-10-27T02:20:01.5936670Z 
2019-10-27T02:20:01.5945643Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-27T02:20:01.5946564Z Build completed unsuccessfully in 0:57:27
2019-10-27T02:20:01.5946564Z Build completed unsuccessfully in 0:57:27
2019-10-27T02:20:01.6000676Z == clock drift check ==
2019-10-27T02:20:01.6014767Z   local time: Sun Oct 27 02:20:01 UTC 2019
2019-10-27T02:20:01.6737235Z   network time: Sun, 27 Oct 2019 02:20:01 GMT
2019-10-27T02:20:01.6738034Z == end clock drift check ==
2019-10-27T02:20:07.3273479Z 
2019-10-27T02:20:07.3382403Z ##[error]Bash exited with code '1'.
2019-10-27T02:20:07.3412676Z ##[section]Starting: Checkout
2019-10-27T02:20:07.3414092Z ==============================================================================
2019-10-27T02:20:07.3414156Z Task         : Get sources
2019-10-27T02:20:07.3414195Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
