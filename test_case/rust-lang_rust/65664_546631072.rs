plain
2019-10-26T17:55:56.5542021Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-26T17:55:56.5725294Z ##[command]git config gc.auto 0
2019-10-26T17:55:56.5809005Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-26T17:55:56.5865695Z ##[command]git config --get-all http.proxy
2019-10-26T17:55:56.6005466Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-26T18:56:52.5064862Z .................................................................................................... 1600/9253
2019-10-26T18:56:58.1508542Z .................................................................................................... 1700/9253
2019-10-26T18:57:10.8348917Z .........................................................i...............i.......................... 1800/9253
2019-10-26T18:57:18.7373542Z .................................................................................................... 1900/9253
2019-10-26T18:57:33.3994231Z ...............................................iiiii................................................ 2000/9253
2019-10-26T18:57:44.3828592Z .................................................................................................... 2200/9253
2019-10-26T18:57:47.0250469Z .................................................................................................... 2300/9253
2019-10-26T18:57:50.9987647Z .................................................................................................... 2400/9253
2019-10-26T18:58:14.5574263Z .................................................................................................... 2500/9253
---
2019-10-26T19:01:11.2423284Z ..................................................i...............i................................. 4800/9253
2019-10-26T19:01:20.3999005Z .................................................................................................... 4900/9253
2019-10-26T19:01:29.2215218Z .................................................................................................... 5000/9253
2019-10-26T19:01:35.5843169Z .................................................................................................... 5100/9253
2019-10-26T19:01:46.2467778Z ...................................................ii.ii...........i................................ 5200/9253
2019-10-26T19:01:56.1435144Z .................................................................................................... 5400/9253
2019-10-26T19:02:05.8597731Z .................................................................................................... 5500/9253
2019-10-26T19:02:14.0564448Z .....................i.............................................................................. 5600/9253
2019-10-26T19:02:19.9227745Z .................................................................................................... 5700/9253
2019-10-26T19:02:19.9227745Z .................................................................................................... 5700/9253
2019-10-26T19:02:32.3001814Z .................................................................................................... 5800/9253
2019-10-26T19:02:43.7828376Z ..................ii...i..ii...........i............................................................ 5900/9253
2019-10-26T19:03:06.9627093Z .................................................................................................... 6100/9253
2019-10-26T19:03:13.9981429Z .................................................................................................... 6200/9253
2019-10-26T19:03:13.9981429Z .................................................................................................... 6200/9253
2019-10-26T19:03:27.3857175Z .........................................i..ii...................................................... 6300/9253
2019-10-26T19:03:51.0909543Z .................................................................................................... 6500/9253
2019-10-26T19:03:53.6042000Z .......i............................................................................................ 6600/9253
2019-10-26T19:03:55.8266180Z ..................................................................................i................. 6700/9253
2019-10-26T19:03:58.6678432Z .................................................................................................... 6800/9253
---
2019-10-26T19:08:40.1058783Z  finished in 5.817
2019-10-26T19:08:40.1239809Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-26T19:08:40.2928057Z 
2019-10-26T19:08:40.2928905Z running 153 tests
2019-10-26T19:08:43.4734026Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-26T19:08:45.4803444Z i..iiii..............i.........iii.i.........ii......
2019-10-26T19:08:45.4804039Z 
2019-10-26T19:08:45.4807902Z  finished in 5.357
2019-10-26T19:08:45.5002800Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-26T19:08:45.6612513Z 
---
2019-10-26T19:08:47.7115799Z  finished in 2.210
2019-10-26T19:08:47.7302045Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-26T19:08:47.8898712Z 
2019-10-26T19:08:47.8898984Z running 9 tests
2019-10-26T19:08:47.8900257Z iiiiiiiii
2019-10-26T19:08:47.8901823Z 
2019-10-26T19:08:47.8902194Z  finished in 0.159
2019-10-26T19:08:47.9103906Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-26T19:08:48.0725653Z 
2019-10-26T19:08:48.0725653Z 
2019-10-26T19:08:48.0726506Z running 109 tests
2019-10-26T19:09:05.5517856Z .................................................F.................................................. 100/109
2019-10-26T19:09:07.0427775Z .........
2019-10-26T19:09:07.0428017Z failures:
2019-10-26T19:09:07.0428234Z 
2019-10-26T19:09:07.0428602Z ---- [incremental] incremental/hashes/unary_and_binary_exprs.rs stdout ----
2019-10-26T19:09:07.0428675Z 
2019-10-26T19:09:07.0428942Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-26T19:09:07.0429014Z status: exit code: 101
2019-10-26T19:09:07.0430007Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/unary_and_binary_exprs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs/unary_and_binary_exprs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs/auxiliary"
2019-10-26T19:09:07.0431161Z ------------------------------------------
2019-10-26T19:09:07.0431228Z 
2019-10-26T19:09:07.0431447Z ------------------------------------------
2019-10-26T19:09:07.0431497Z stderr:
2019-10-26T19:09:07.0431497Z stderr:
2019-10-26T19:09:07.0431725Z ------------------------------------------
2019-10-26T19:09:07.0432062Z thread 'rustc' panicked at 'found unstable fingerprints for const_caller_location(7de0a38047d74950-f4f2ced447ab0242)', src/librustc/ty/query/plumbing.rs:490:9
2019-10-26T19:09:07.0432437Z 
2019-10-26T19:09:07.0432488Z error: internal compiler error: unexpected panic
2019-10-26T19:09:07.0432519Z 
2019-10-26T19:09:07.0432566Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-26T19:09:07.0432566Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-26T19:09:07.0432619Z 
2019-10-26T19:09:07.0433117Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-26T19:09:07.0433419Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-26T19:09:07.0433453Z 
2019-10-26T19:09:07.0433453Z 
2019-10-26T19:09:07.0433829Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-26T19:09:07.0434398Z 
2019-10-26T19:09:07.0434621Z ------------------------------------------
2019-10-26T19:09:07.0434654Z 
2019-10-26T19:09:07.0434680Z 
---
2019-10-26T19:09:07.0435582Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-26T19:09:07.0435657Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-26T19:09:07.0435687Z 
2019-10-26T19:09:07.0435712Z 
2019-10-26T19:09:07.0437074Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-26T19:09:07.0437317Z 
2019-10-26T19:09:07.0437344Z 
2019-10-26T19:09:07.0448258Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-26T19:09:07.0448513Z Build completed unsuccessfully in 1:06:33
2019-10-26T19:09:07.0448513Z Build completed unsuccessfully in 1:06:33
2019-10-26T19:09:07.0501552Z == clock drift check ==
2019-10-26T19:09:07.0518509Z   local time: Sat Oct 26 19:09:07 UTC 2019
2019-10-26T19:09:07.2575982Z   network time: Sat, 26 Oct 2019 19:09:07 GMT
2019-10-26T19:09:07.2578980Z == end clock drift check ==
2019-10-26T19:09:12.5633583Z 
2019-10-26T19:09:12.5707434Z ##[error]Bash exited with code '1'.
2019-10-26T19:09:12.5753302Z ##[section]Starting: Checkout
2019-10-26T19:09:12.5755446Z ==============================================================================
2019-10-26T19:09:12.5755491Z Task         : Get sources
2019-10-26T19:09:12.5755545Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
