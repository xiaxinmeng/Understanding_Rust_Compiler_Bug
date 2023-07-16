plain
2019-08-25T19:28:06.1095832Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-25T19:28:06.1328617Z ##[command]git config gc.auto 0
2019-08-25T19:28:06.1418507Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-25T19:28:06.1479765Z ##[command]git config --get-all http.proxy
2019-08-25T19:28:06.1624685Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63884/merge:refs/remotes/pull/63884/merge
---
2019-08-25T19:28:40.6307866Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-25T19:28:40.6308115Z 
2019-08-25T19:28:40.6308509Z   git checkout -b <new-branch-name>
2019-08-25T19:28:40.6308716Z 
2019-08-25T19:28:40.6308909Z HEAD is now at 398ac402c Merge ea60cf75b7b72cf6ab994b9800d2da802d14c80e into b1522e6023d80faf8ea334ef50ef05528ed806bc
2019-08-25T19:28:40.6449733Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-25T19:28:40.6452665Z ==============================================================================
2019-08-25T19:28:40.6452746Z Task         : Bash
2019-08-25T19:28:40.6452809Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-25T20:26:57.5857387Z .................................................................................................... 1500/8953
2019-08-25T20:27:03.0767221Z .................................................................................................... 1600/8953
2019-08-25T20:27:15.0708587Z .............................................i...............i...................................... 1700/8953
2019-08-25T20:27:22.6785157Z .................................................................................................... 1800/8953
2019-08-25T20:27:36.0352535Z .....................................iiiii.......................................................... 1900/8953
2019-08-25T20:27:46.5307171Z .................................................................................................... 2100/8953
2019-08-25T20:27:49.0689138Z .................................................................................................... 2200/8953
2019-08-25T20:27:53.1242938Z .................................................................................................... 2300/8953
2019-08-25T20:28:00.0909131Z .................................................................................................... 2400/8953
---
2019-08-25T20:30:49.2342751Z .........................i...............i.......................................................... 4700/8953
2019-08-25T20:31:00.1255998Z .................................................................................................... 4800/8953
2019-08-25T20:31:05.9561702Z .................................................................................................... 4900/8953
2019-08-25T20:31:16.2653333Z .................................................................................................... 5000/8953
2019-08-25T20:31:21.4394173Z ......ii.ii......................................................................................... 5100/8953
2019-08-25T20:31:34.9444442Z .................................................................................................... 5300/8953
2019-08-25T20:31:41.5224141Z ..............................................................i..................................... 5400/8953
2019-08-25T20:31:48.2686057Z .................................................................................................... 5500/8953
2019-08-25T20:31:55.5983980Z .................................................................................................... 5600/8953
2019-08-25T20:31:55.5983980Z .................................................................................................... 5600/8953
2019-08-25T20:32:05.8340327Z ........................................................ii...i..ii...........i...................... 5700/8953
2019-08-25T20:32:29.2413182Z .................................................................................................... 5900/8953
2019-08-25T20:32:34.0023008Z .................................................................................................... 6000/8953
2019-08-25T20:32:34.0023008Z .................................................................................................... 6000/8953
2019-08-25T20:32:40.7436542Z .........................................................i..ii...................................... 6100/8953
2019-08-25T20:33:06.3445242Z .................................................................................................... 6300/8953
2019-08-25T20:33:08.4923202Z ...i................................................................................................ 6400/8953
2019-08-25T20:33:10.6367642Z ...........................................................................i........................ 6500/8953
2019-08-25T20:33:13.3151324Z .................................................................................................... 6600/8953
---
2019-08-25T20:37:39.9829468Z  finished in 20.254
2019-08-25T20:37:40.0002094Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-25T20:37:40.1639423Z 
2019-08-25T20:37:40.1641223Z running 149 tests
2019-08-25T20:37:43.3026319Z i....iii......iii..iiii....i.............................i..i..................i....i........ii.i.i. 100/149
2019-08-25T20:37:45.1645681Z .iiii..............i.........iii.i.......ii......
2019-08-25T20:37:45.1646860Z 
2019-08-25T20:37:45.1649946Z  finished in 5.164
2019-08-25T20:37:45.1815852Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-25T20:37:45.9872376Z 
---
2019-08-25T20:37:47.2417152Z  finished in 2.059
2019-08-25T20:37:47.2597124Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-25T20:37:47.4038857Z 
2019-08-25T20:37:47.4039084Z running 9 tests
2019-08-25T20:37:47.4039811Z iiiiiiiii
2019-08-25T20:37:47.4040176Z 
2019-08-25T20:37:47.4040212Z  finished in 0.144
2019-08-25T20:37:47.4200215Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-25T20:37:48.4804760Z 
2019-08-25T20:37:48.4804760Z 
2019-08-25T20:37:48.4866925Z running 104 tests
2019-08-25T20:38:03.9848306Z ..................................................................................................F. 100/104
2019-08-25T20:38:04.5292051Z F...
2019-08-25T20:38:04.5292209Z failures:
2019-08-25T20:38:04.5297477Z 
2019-08-25T20:38:04.5298045Z ---- [incremental] incremental/thinlto/cgu_invalidated_via_import.rs stdout ----
2019-08-25T20:38:04.5298128Z 
2019-08-25T20:38:04.5298391Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-08-25T20:38:04.5298472Z status: exit code: 1
2019-08-25T20:38:04.5299364Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_invalidated_via_import.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/cgu_invalidated_via_import.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/auxiliary"
2019-08-25T20:38:04.5301420Z ------------------------------------------
2019-08-25T20:38:04.5301479Z 
2019-08-25T20:38:04.5301810Z ------------------------------------------
2019-08-25T20:38:04.5301859Z stderr:
2019-08-25T20:38:04.5301859Z stderr:
2019-08-25T20:38:04.5302109Z ------------------------------------------
2019-08-25T20:38:04.5302365Z error: CGU-reuse for `cgu_invalidated_via_import-bar` is `No` but should be `PreLto`
2019-08-25T20:38:04.5302694Z    |
2019-08-25T20:38:04.5302694Z    |
2019-08-25T20:38:04.5302939Z LL | / #![rustc_expected_cgu_reuse(module="cgu_invalidated_via_import-bar",
2019-08-25T20:38:04.5302993Z LL | |                             cfg="cfail2",
2019-08-25T20:38:04.5304666Z LL | |                             kind="pre-lto")]
2019-08-25T20:38:04.5304955Z 
2019-08-25T20:38:04.5305014Z error: aborting due to previous error
2019-08-25T20:38:04.5305039Z 
2019-08-25T20:38:04.5305072Z 
2019-08-25T20:38:04.5305072Z 
2019-08-25T20:38:04.5306118Z ------------------------------------------
2019-08-25T20:38:04.5306153Z 
2019-08-25T20:38:04.5306196Z 
2019-08-25T20:38:04.5306422Z ---- [incremental] incremental/thinlto/independent_cgus_dont_affect_each_other.rs stdout ----
2019-08-25T20:38:04.5306450Z 
2019-08-25T20:38:04.5306637Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-08-25T20:38:04.5307458Z status: exit code: 1
2019-08-25T20:38:04.5308549Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/independent_cgus_dont_affect_each_other.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/independent_cgus_dont_affect_each_other.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/auxiliary"
2019-08-25T20:38:04.5308910Z ------------------------------------------
2019-08-25T20:38:04.5308956Z 
2019-08-25T20:38:04.5309122Z ------------------------------------------
2019-08-25T20:38:04.5309157Z stderr:
2019-08-25T20:38:04.5309157Z stderr:
2019-08-25T20:38:04.5309333Z ------------------------------------------
2019-08-25T20:38:04.5309540Z error: CGU-reuse for `independent_cgus_dont_affect_each_other-bar` is `No` but should be `PreLto`
2019-08-25T20:38:04.5309805Z    |
2019-08-25T20:38:04.5309805Z    |
2019-08-25T20:38:04.5310437Z LL | / #![rustc_expected_cgu_reuse(module="independent_cgus_dont_affect_each_other-bar",
2019-08-25T20:38:04.5310511Z LL | |                             cfg="cfail2",
2019-08-25T20:38:04.5310759Z LL | |                             kind="pre-lto")]
2019-08-25T20:38:04.5310853Z 
2019-08-25T20:38:04.5310896Z error: aborting due to previous error
2019-08-25T20:38:04.5310944Z 
2019-08-25T20:38:04.5310969Z 
---
2019-08-25T20:38:04.5312207Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-25T20:38:04.5312265Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-25T20:38:04.5312352Z 
2019-08-25T20:38:04.5312380Z 
2019-08-25T20:38:04.5313989Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-25T20:38:04.5314186Z 
2019-08-25T20:38:04.5314226Z 
2019-08-25T20:38:04.5317700Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-25T20:38:04.5317758Z Build completed unsuccessfully in 1:02:56
2019-08-25T20:38:04.5317758Z Build completed unsuccessfully in 1:02:56
2019-08-25T20:38:04.5377676Z == clock drift check ==
2019-08-25T20:38:04.5396145Z   local time: Sun Aug 25 20:38:04 UTC 2019
2019-08-25T20:38:04.6815773Z   network time: Sun, 25 Aug 2019 20:38:04 GMT
2019-08-25T20:38:04.6818321Z == end clock drift check ==
2019-08-25T20:38:08.6935538Z ##[error]Bash exited with code '1'.
2019-08-25T20:38:08.6977744Z ##[section]Starting: Checkout
2019-08-25T20:38:08.6979698Z ==============================================================================
2019-08-25T20:38:08.6979756Z Task         : Get sources
2019-08-25T20:38:08.6979874Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
