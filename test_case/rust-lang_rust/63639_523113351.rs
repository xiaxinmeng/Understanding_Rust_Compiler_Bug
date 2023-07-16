plain
2019-08-20T15:58:37.0748599Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T15:58:37.0974867Z ##[command]git config gc.auto 0
2019-08-20T15:58:37.1051014Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T15:58:37.1107465Z ##[command]git config --get-all http.proxy
2019-08-20T15:58:37.1248171Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63639/merge:refs/remotes/pull/63639/merge
---
2019-08-20T15:59:13.8070863Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T15:59:13.8071552Z 
2019-08-20T15:59:13.8072331Z   git checkout -b <new-branch-name>
2019-08-20T15:59:13.8072710Z 
2019-08-20T15:59:13.8073086Z HEAD is now at 851590680 Merge 4a24b6022fc2b44a6e8aa1aad6d281741fb2ca7f into 51879c3abaedb926739095d19a2af638ee6a07d8
2019-08-20T15:59:13.8241099Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T15:59:13.8244902Z ==============================================================================
2019-08-20T15:59:13.8244976Z Task         : Bash
2019-08-20T15:59:13.8245028Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T17:02:48.5017827Z .................................................................................................... 1500/8943
2019-08-20T17:02:54.0811490Z .................................................................................................... 1600/8943
2019-08-20T17:03:07.2761499Z .......................................i...............i............................................ 1700/8943
2019-08-20T17:03:15.3432945Z .................................................................................................... 1800/8943
2019-08-20T17:03:30.2041301Z ...............................iiiii................................................................ 1900/8943
2019-08-20T17:03:41.1170911Z .................................................................................................... 2100/8943
2019-08-20T17:03:43.8085484Z .................................................................................................... 2200/8943
2019-08-20T17:03:48.6353866Z .................................................................................................... 2300/8943
2019-08-20T17:03:55.7978354Z .................................................................................................... 2400/8943
---
2019-08-20T17:06:58.3888451Z ...................i...............i................................................................ 4700/8943
2019-08-20T17:07:10.2582697Z .................................................................................................... 4800/8943
2019-08-20T17:07:16.5696396Z .................................................................................................... 4900/8943
2019-08-20T17:07:27.9161179Z .................................................................................................... 5000/8943
2019-08-20T17:07:33.1728440Z ii.ii............................................................................................... 5100/8943
2019-08-20T17:07:48.1563344Z .................................................................................................... 5300/8943
2019-08-20T17:07:55.1762374Z ........................................................i........................................... 5400/8943
2019-08-20T17:08:02.1894198Z .................................................................................................... 5500/8943
2019-08-20T17:08:11.7915337Z .................................................................................................... 5600/8943
2019-08-20T17:08:11.7915337Z .................................................................................................... 5600/8943
2019-08-20T17:08:21.2190519Z .................................................ii...i..ii...........i............................. 5700/8943
2019-08-20T17:08:44.7137772Z .................................................................................................... 5900/8943
2019-08-20T17:08:49.7732397Z .................................................................................................... 6000/8943
2019-08-20T17:08:49.7732397Z .................................................................................................... 6000/8943
2019-08-20T17:09:01.0201317Z ..................................................i..ii............................................. 6100/8943
2019-08-20T17:09:25.5433914Z ................................................................................................i... 6300/8943
2019-08-20T17:09:27.8877588Z .................................................................................................... 6400/8943
2019-08-20T17:09:30.2093385Z ....................................................................i............................... 6500/8943
2019-08-20T17:09:33.3324143Z .................................................................................................... 6600/8943
---
2019-08-20T17:14:18.9763990Z  finished in 21.134
2019-08-20T17:14:18.9967616Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-20T17:14:19.1962377Z 
2019-08-20T17:14:19.1962951Z running 148 tests
2019-08-20T17:14:22.5643921Z i....iii......iii..iiii....i............................i..i..................i....i.........ii.i.i. 100/148
2019-08-20T17:14:24.5892911Z .iiii..............i.........iii.i......ii......
2019-08-20T17:14:24.5893427Z 
2019-08-20T17:14:24.5896217Z  finished in 5.593
2019-08-20T17:14:24.6089757Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-20T17:14:24.7807979Z 
---
2019-08-20T17:14:27.0424629Z  finished in 2.432
2019-08-20T17:14:27.0633956Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-20T17:14:27.2238220Z 
2019-08-20T17:14:27.2239191Z running 9 tests
2019-08-20T17:14:27.2240877Z iiiiiiiii
2019-08-20T17:14:27.2241877Z 
2019-08-20T17:14:27.2247003Z  finished in 0.163
2019-08-20T17:14:27.2449013Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-20T17:14:27.4394651Z 
---
2019-08-20T17:14:46.3644948Z  finished in 19.119
2019-08-20T17:14:46.3906707Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-20T17:14:46.5956389Z 
2019-08-20T17:14:46.5956631Z running 122 tests
2019-08-20T17:15:11.9575920Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-20T17:15:17.0526546Z .i.i......iii.i.....ii
2019-08-20T17:15:17.0527837Z 
2019-08-20T17:15:17.0534013Z  finished in 30.662
2019-08-20T17:15:17.0542825Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-20T17:15:17.0543226Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-20T17:22:59.1217167Z failures:
2019-08-20T17:22:59.1217226Z 
2019-08-20T17:22:59.1217524Z ---- [rustdoc] rustdoc/hidden-impls.rs stdout ----
2019-08-20T17:22:59.1217563Z 
2019-08-20T17:22:59.1217611Z error: htmldocck failed!
2019-08-20T17:22:59.1217677Z status: exit code: 1
2019-08-20T17:22:59.1218082Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-impls" "/checkout/src/test/rustdoc/hidden-impls.rs"
2019-08-20T17:22:59.1218661Z ------------------------------------------
2019-08-20T17:22:59.1218717Z 
2019-08-20T17:22:59.1219006Z ------------------------------------------
2019-08-20T17:22:59.1219058Z stderr:
2019-08-20T17:22:59.1219058Z stderr:
2019-08-20T17:22:59.1219309Z ------------------------------------------
2019-08-20T17:22:59.1219363Z 15: @has check failed
2019-08-20T17:22:59.1219618Z  File does not exist 'implementors/foo/trait.Clone.js'
2019-08-20T17:22:59.1219691Z  // @has implementors/foo/trait.Clone.js
2019-08-20T17:22:59.1219741Z 16: @!has check failed
2019-08-20T17:22:59.1219989Z  File does not exist 'implementors/foo/trait.Clone.js'
2019-08-20T17:22:59.1220218Z  // @!has - 'Foo'
2019-08-20T17:22:59.1220300Z Encountered 2 errors
2019-08-20T17:22:59.1220330Z 
2019-08-20T17:22:59.1220563Z ------------------------------------------
2019-08-20T17:22:59.1220614Z 
2019-08-20T17:22:59.1220614Z 
2019-08-20T17:22:59.1220642Z 
2019-08-20T17:22:59.1220894Z ---- [rustdoc] rustdoc/impl-parts-crosscrate.rs stdout ----
2019-08-20T17:22:59.1221082Z 
2019-08-20T17:22:59.1221147Z error: htmldocck failed!
2019-08-20T17:22:59.1221193Z status: exit code: 1
2019-08-20T17:22:59.1221641Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-parts-crosscrate" "/checkout/src/test/rustdoc/impl-parts-crosscrate.rs"
2019-08-20T17:22:59.1222852Z ------------------------------------------
2019-08-20T17:22:59.1222907Z 
2019-08-20T17:22:59.1223233Z ------------------------------------------
2019-08-20T17:22:59.1223309Z stderr:
2019-08-20T17:22:59.1223309Z stderr:
2019-08-20T17:22:59.1223545Z ------------------------------------------
2019-08-20T17:22:59.1223597Z 15: @has check failed
2019-08-20T17:22:59.1223893Z  File does not exist 'implementors/rustdoc_impl_parts_crosscrate/trait.AnOibit.js'
2019-08-20T17:22:59.1223958Z  // @has implementors/rustdoc_impl_parts_crosscrate/trait.AnOibit.js Bar
2019-08-20T17:22:59.1224010Z 16: @has check failed
2019-08-20T17:22:59.1224334Z  File does not exist 'implementors/rustdoc_impl_parts_crosscrate/trait.AnOibit.js'
2019-08-20T17:22:59.1224555Z  // @has - Send
2019-08-20T17:22:59.1224606Z 17: @has check failed
2019-08-20T17:22:59.1224897Z  File does not exist 'implementors/rustdoc_impl_parts_crosscrate/trait.AnOibit.js'
2019-08-20T17:22:59.1225118Z  // @has - !AnOibit
2019-08-20T17:22:59.1225170Z 18: @has check failed
2019-08-20T17:22:59.1225443Z  File does not exist 'implementors/rustdoc_impl_parts_crosscrate/trait.AnOibit.js'
2019-08-20T17:22:59.1225671Z  // @has - Copy
2019-08-20T17:22:59.1225749Z Encountered 4 errors
2019-08-20T17:22:59.1225780Z 
2019-08-20T17:22:59.1226027Z ------------------------------------------
2019-08-20T17:22:59.1226062Z 
2019-08-20T17:22:59.1226062Z 
2019-08-20T17:22:59.1226089Z 
2019-08-20T17:22:59.1226323Z ---- [rustdoc] rustdoc/tuples.rs stdout ----
2019-08-20T17:22:59.1226374Z 
2019-08-20T17:22:59.1226420Z error: htmldocck failed!
2019-08-20T17:22:59.1226477Z status: exit code: 1
2019-08-20T17:22:59.1226862Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/tuples" "/checkout/src/test/rustdoc/tuples.rs"
2019-08-20T17:22:59.1227177Z ------------------------------------------
2019-08-20T17:22:59.1227212Z 
2019-08-20T17:22:59.1227457Z ------------------------------------------
2019-08-20T17:22:59.1227507Z stderr:
2019-08-20T17:22:59.1227507Z stderr:
2019-08-20T17:22:59.1227736Z ------------------------------------------
2019-08-20T17:22:59.1227787Z 7: @has check failed
2019-08-20T17:22:59.1227851Z  `XPATH PATTERN` did not match
2019-08-20T17:22:59.1228132Z  // @has foo/fn.tuple2.html //pre 'pub fn tuple2(x: (i32, i32)) -> (i32, i32)'
2019-08-20T17:22:59.1228230Z Encountered 1 errors
2019-08-20T17:22:59.1228260Z 
2019-08-20T17:22:59.1228491Z ------------------------------------------
2019-08-20T17:22:59.1228525Z 
---
2019-08-20T17:22:59.1233996Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-20T17:22:59.1234139Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-20T17:22:59.1241641Z 
2019-08-20T17:22:59.1241855Z 
2019-08-20T17:22:59.1243877Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-20T17:22:59.1244358Z 
2019-08-20T17:22:59.1244389Z 
2019-08-20T17:22:59.1253298Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-20T17:22:59.1253371Z Build completed unsuccessfully in 1:17:06
2019-08-20T17:22:59.1253371Z Build completed unsuccessfully in 1:17:06
2019-08-20T17:22:59.1307164Z == clock drift check ==
2019-08-20T17:22:59.1320903Z   local time: Tue Aug 20 17:22:59 UTC 2019
2019-08-20T17:22:59.3106093Z   network time: Tue, 20 Aug 2019 17:22:59 GMT
2019-08-20T17:22:59.3106273Z == end clock drift check ==
2019-08-20T17:23:01.2795138Z ##[error]Bash exited with code '1'.
2019-08-20T17:23:01.2845676Z ##[section]Starting: Checkout
2019-08-20T17:23:01.2847543Z ==============================================================================
2019-08-20T17:23:01.2847618Z Task         : Get sources
2019-08-20T17:23:01.2847666Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
