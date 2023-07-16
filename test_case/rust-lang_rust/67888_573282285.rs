plain
2020-01-11T03:49:33.9945857Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T03:49:33.9957286Z ##[command]git config gc.auto 0
2020-01-11T03:49:33.9962936Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T03:49:34.0033999Z ##[command]git config --get-all http.proxy
2020-01-11T03:49:34.0201328Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67888/merge:refs/remotes/pull/67888/merge
---
2020-01-11T04:54:56.2502854Z .......................................i...............i............................................ 4900/9511
2020-01-11T04:55:06.6297272Z .................................................................................................... 5000/9511
2020-01-11T04:55:14.0573918Z ....................................................................................i............... 5100/9511
2020-01-11T04:55:20.4259552Z .................................................................................................... 5200/9511
2020-01-11T04:55:31.8838992Z ...................................................ii.ii...........i................................ 5300/9511
2020-01-11T04:55:42.1583807Z .................................................................................................... 5500/9511
2020-01-11T04:55:53.4336821Z .................................................................................................... 5600/9511
2020-01-11T04:56:01.2367053Z ...................................i................................................................ 5700/9511
2020-01-11T04:56:08.7032067Z .................................................................................................... 5800/9511
2020-01-11T04:56:08.7032067Z .................................................................................................... 5800/9511
2020-01-11T04:56:21.1492390Z .................................................................................................... 5900/9511
2020-01-11T04:56:32.7515650Z ..........................ii...i..ii...........i.................................................... 6000/9511
2020-01-11T04:56:52.3918150Z .................................................................................................... 6200/9511
2020-01-11T04:57:01.5059482Z .................................................................................................... 6300/9511
2020-01-11T04:57:01.5059482Z .................................................................................................... 6300/9511
2020-01-11T04:57:15.8132694Z .....................................................i..ii.......................................... 6400/9511
2020-01-11T04:57:45.4438306Z .................................................................................................... 6600/9511
2020-01-11T04:57:47.7651317Z ............................i....................................................................... 6700/9511
2020-01-11T04:57:50.4050664Z .................................................................................................... 6800/9511
2020-01-11T04:57:53.2355908Z ............................i....................................................................... 6900/9511
---
2020-01-11T04:59:42.8387103Z .................................................................................................... 7500/9511
2020-01-11T04:59:47.2011652Z .................................................................................................... 7600/9511
2020-01-11T04:59:54.0655624Z .................................................................................................... 7700/9511
2020-01-11T05:00:02.8234229Z .................................................................................................... 7800/9511
2020-01-11T05:00:13.6713014Z .............................................................................iiii................... 7900/9511
2020-01-11T05:00:32.2302170Z ...........i......i................................................................................. 8100/9511
2020-01-11T05:00:38.0681483Z .................................................................................................... 8200/9511
2020-01-11T05:00:53.5878421Z .................................................................................................... 8300/9511
2020-01-11T05:01:04.5289938Z .................................................................................................... 8400/9511
---
2020-01-11T05:03:46.4085668Z  finished in 7.692
2020-01-11T05:03:46.4322678Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-11T05:03:46.6567586Z 
2020-01-11T05:03:46.6567840Z running 166 tests
2020-01-11T05:03:50.0029666Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-11T05:03:52.5215260Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-11T05:03:52.5218719Z 
2020-01-11T05:03:52.5226331Z  finished in 6.090
2020-01-11T05:03:52.5493885Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-11T05:03:52.7449800Z 
---
2020-01-11T05:03:54.9273722Z  finished in 2.379
2020-01-11T05:03:54.9486366Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-11T05:03:55.1242355Z 
2020-01-11T05:03:55.1244790Z running 9 tests
2020-01-11T05:03:55.1250824Z iiiiiiiii
2020-01-11T05:03:55.1251331Z 
2020-01-11T05:03:55.1251411Z  finished in 0.176
2020-01-11T05:03:55.1461171Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-11T05:03:55.3553651Z 
2020-01-11T05:03:55.3553651Z 
2020-01-11T05:03:55.3555451Z running 114 tests
2020-01-11T05:04:15.3013828Z .........................................F.......................................................... 100/114
2020-01-11T05:04:18.3793007Z ..............
2020-01-11T05:04:18.3793200Z failures:
2020-01-11T05:04:18.3793240Z 
2020-01-11T05:04:18.3793553Z ---- [incremental] incremental/hashes/let_expressions.rs stdout ----
2020-01-11T05:04:18.3793589Z 
2020-01-11T05:04:18.3793870Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-01-11T05:04:18.3793924Z status: exit code: 1
2020-01-11T05:04:18.3801788Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
2020-01-11T05:04:18.3802361Z ------------------------------------------
2020-01-11T05:04:18.3802712Z 
2020-01-11T05:04:18.3803032Z ------------------------------------------
2020-01-11T05:04:18.3803102Z stderr:
2020-01-11T05:04:18.3803102Z stderr:
2020-01-11T05:04:18.3803328Z ------------------------------------------
2020-01-11T05:04:18.3803382Z error: `promoted_mir(add_ref_in_pattern)` should be clean but is not
2020-01-11T05:04:18.3803713Z    |
2020-01-11T05:04:18.3803755Z LL | / pub fn add_ref_in_pattern() {
2020-01-11T05:04:18.3803755Z LL | / pub fn add_ref_in_pattern() {
2020-01-11T05:04:18.3803986Z LL | |     let (ref _a, _b) = (1u8, 'y');
2020-01-11T05:04:18.3804090Z    | |_^
2020-01-11T05:04:18.3804117Z 
2020-01-11T05:04:18.3804282Z error: aborting due to previous error
2020-01-11T05:04:18.3804339Z 
---
2020-01-11T05:04:18.3805466Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:386:22
2020-01-11T05:04:18.3805540Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-11T05:04:18.3805655Z 
2020-01-11T05:04:18.3805683Z 
2020-01-11T05:04:18.3807355Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-11T05:04:18.3807615Z 
2020-01-11T05:04:18.3807650Z 
2020-01-11T05:04:18.3814812Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-11T05:04:18.3814886Z Build completed unsuccessfully in 1:08:42
2020-01-11T05:04:18.3814886Z Build completed unsuccessfully in 1:08:42
2020-01-11T05:04:18.3886400Z == clock drift check ==
2020-01-11T05:04:18.3909241Z   local time: Sat Jan 11 05:04:18 UTC 2020
2020-01-11T05:04:18.6818685Z   network time: Sat, 11 Jan 2020 05:04:18 GMT
2020-01-11T05:04:18.6819947Z == end clock drift check ==
2020-01-11T05:04:20.1972059Z 
2020-01-11T05:04:20.2075656Z ##[error]Bash exited with code '1'.
2020-01-11T05:04:20.2134150Z ##[section]Starting: Checkout
2020-01-11T05:04:20.2136136Z ==============================================================================
2020-01-11T05:04:20.2136216Z Task         : Get sources
2020-01-11T05:04:20.2136273Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
