plain
2019-10-24T10:14:34.1508301Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T10:14:34.1714162Z ##[command]git config gc.auto 0
2019-10-24T10:14:34.1799969Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T10:14:34.1865605Z ##[command]git config --get-all http.proxy
2019-10-24T10:14:34.2007056Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65759/merge:refs/remotes/pull/65759/merge
---
2019-10-24T11:15:27.7610069Z .................................................................................................... 1600/9241
2019-10-24T11:15:33.0941228Z .................................................................................................... 1700/9241
2019-10-24T11:15:45.2442199Z .....................................................i...............i.............................. 1800/9241
2019-10-24T11:15:52.8202404Z .................................................................................................... 1900/9241
2019-10-24T11:16:06.5061096Z ...........................................iiiii.................................................... 2000/9241
2019-10-24T11:16:16.5075536Z .................................................................................................... 2200/9241
2019-10-24T11:16:18.8708227Z .................................................................................................... 2300/9241
2019-10-24T11:16:22.7135150Z .................................................................................................... 2400/9241
2019-10-24T11:16:44.8919515Z .................................................................................................... 2500/9241
---
2019-10-24T11:19:32.1833010Z ...............................................i...............i.................................... 4800/9241
2019-10-24T11:19:40.9436480Z .................................................................................................... 4900/9241
2019-10-24T11:19:49.1880050Z .................................................................................................... 5000/9241
2019-10-24T11:19:55.6908218Z .................................................................................................... 5100/9241
2019-10-24T11:20:05.2175158Z ...............................................ii.ii................................................ 5200/9241
2019-10-24T11:20:14.6198095Z .................................................................................................... 5400/9241
2019-10-24T11:20:23.7765368Z .................................................................................................... 5500/9241
2019-10-24T11:20:30.8559203Z ..............i..................................................................................... 5600/9241
2019-10-24T11:20:36.2993491Z .................................................................................................... 5700/9241
2019-10-24T11:20:36.2993491Z .................................................................................................... 5700/9241
2019-10-24T11:20:47.9270213Z .................................................................................................... 5800/9241
2019-10-24T11:20:59.2568730Z ...........ii...i..ii...........i................................................................... 5900/9241
2019-10-24T11:21:19.9692710Z .................................................................................................... 6100/9241
2019-10-24T11:21:26.1216468Z .................................................................................................... 6200/9241
2019-10-24T11:21:26.1216468Z .................................................................................................... 6200/9241
2019-10-24T11:21:39.3324862Z .................................i..ii.............................................................. 6300/9241
2019-10-24T11:21:59.4659398Z ...................................................................................................i 6500/9241
2019-10-24T11:22:01.6050647Z .................................................................................................... 6600/9241
2019-10-24T11:22:03.8299623Z ..........................................................................i......................... 6700/9241
2019-10-24T11:22:06.5286979Z .................................................................................................... 6800/9241
---
2019-10-24T11:26:27.8725589Z  finished in 5.515
2019-10-24T11:26:27.8907834Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-24T11:26:28.0490774Z 
2019-10-24T11:26:28.0491817Z running 153 tests
2019-10-24T11:26:31.0424192Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-24T11:26:32.9075430Z i..iiii..............i.........iii.i.........ii......
2019-10-24T11:26:32.9076215Z 
2019-10-24T11:26:32.9076299Z  finished in 5.016
2019-10-24T11:26:32.9286769Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-24T11:26:33.0866774Z 
---
2019-10-24T11:26:35.0588318Z  finished in 2.130
2019-10-24T11:26:35.0764738Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-24T11:26:35.2305712Z 
2019-10-24T11:26:35.2306661Z running 9 tests
2019-10-24T11:26:35.2308062Z iiiiiiiii
2019-10-24T11:26:35.2308862Z 
2019-10-24T11:26:35.2312860Z  finished in 0.154
2019-10-24T11:26:35.2515477Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-24T11:26:35.4294679Z 
---
2019-10-24T11:26:52.9642446Z  finished in 17.712
2019-10-24T11:26:52.9825151Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-24T11:26:53.1450774Z 
2019-10-24T11:26:53.1451110Z running 123 tests
2019-10-24T11:27:16.1769787Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-24T11:27:20.6121695Z i.i.i......iii.i.....ii
2019-10-24T11:27:20.6124291Z 
2019-10-24T11:27:20.6124725Z  finished in 27.629
2019-10-24T11:27:20.6133438Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-24T11:27:20.6133832Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-24T11:28:21.0971193Z failures:
2019-10-24T11:28:21.0971220Z 
2019-10-24T11:28:21.0971465Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2019-10-24T11:28:21.0971493Z 
2019-10-24T11:28:21.0971993Z error: /checkout/src/test/ui-fulldeps/lint-group-plugin.rs:6: unexpected warning: '6:1: 6:35: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-10-24T11:28:21.0972086Z error: 1 unexpected errors found, 0 expected errors not found
2019-10-24T11:28:21.0972139Z status: exit code: 0
2019-10-24T11:28:21.0972139Z status: exit code: 0
2019-10-24T11:28:21.0973436Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2019-10-24T11:28:21.0973566Z unexpected errors (from JSON output): [
2019-10-24T11:28:21.0973677Z         line_num: 6,
2019-10-24T11:28:21.0973720Z         kind: Some(
2019-10-24T11:28:21.0973762Z             Warning,
2019-10-24T11:28:21.0973803Z         ),
2019-10-24T11:28:21.0973803Z         ),
2019-10-24T11:28:21.0974468Z         msg: "6:1: 6:35: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-10-24T11:28:21.0974620Z ]
2019-10-24T11:28:21.0974648Z 
2019-10-24T11:28:21.0975010Z thread '[ui] ui-fulldeps/lint-group-plugin.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-10-24T11:28:21.0975089Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-24T11:28:21.0975089Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-24T11:28:21.0975123Z 
2019-10-24T11:28:21.0975350Z ---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
2019-10-24T11:28:21.0975383Z 
2019-10-24T11:28:21.0975825Z error: /checkout/src/test/ui-fulldeps/lint-plugin.rs:5: unexpected warning: '5:1: 5:29: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-10-24T11:28:21.0975936Z error: 1 unexpected errors found, 0 expected errors not found
2019-10-24T11:28:21.0975984Z status: exit code: 0
2019-10-24T11:28:21.0975984Z status: exit code: 0
2019-10-24T11:28:21.0979028Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
2019-10-24T11:28:21.0979323Z unexpected errors (from JSON output): [
2019-10-24T11:28:21.0979412Z         line_num: 5,
2019-10-24T11:28:21.0979444Z         kind: Some(
2019-10-24T11:28:21.0979476Z             Warning,
2019-10-24T11:28:21.0979507Z         ),
2019-10-24T11:28:21.0979507Z         ),
2019-10-24T11:28:21.0979874Z         msg: "5:1: 5:29: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-10-24T11:28:21.0979972Z ]
2019-10-24T11:28:21.0979999Z 
2019-10-24T11:28:21.0980231Z thread '[ui] ui-fulldeps/lint-plugin.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-10-24T11:28:21.0980260Z 
---
2019-10-24T11:28:21.0982484Z 
2019-10-24T11:28:21.0983445Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-24T11:28:21.0983576Z 
2019-10-24T11:28:21.0983604Z 
2019-10-24T11:28:21.0985548Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-24T11:28:21.0985859Z 
2019-10-24T11:28:21.0985890Z 
2019-10-24T11:28:21.0985935Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-24T11:28:21.0986003Z Build completed unsuccessfully in 1:07:08
2019-10-24T11:28:21.0986003Z Build completed unsuccessfully in 1:07:08
2019-10-24T11:28:21.1033686Z == clock drift check ==
2019-10-24T11:28:21.1055571Z   local time: Thu Oct 24 11:28:21 UTC 2019
2019-10-24T11:28:21.3970840Z   network time: Thu, 24 Oct 2019 11:28:21 GMT
2019-10-24T11:28:21.3973167Z == end clock drift check ==
2019-10-24T11:28:22.0368835Z 
2019-10-24T11:28:22.0483846Z ##[error]Bash exited with code '1'.
2019-10-24T11:28:22.0570563Z ##[section]Starting: Checkout
2019-10-24T11:28:22.0573153Z ==============================================================================
2019-10-24T11:28:22.0573214Z Task         : Get sources
2019-10-24T11:28:22.0573428Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
