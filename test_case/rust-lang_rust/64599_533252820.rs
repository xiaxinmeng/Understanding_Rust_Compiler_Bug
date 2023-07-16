plain
2019-09-19T17:00:20.6686225Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-19T17:00:20.6869099Z ##[command]git config gc.auto 0
2019-09-19T17:00:20.6954854Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-19T17:00:20.7008565Z ##[command]git config --get-all http.proxy
2019-09-19T17:00:20.7166927Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64599/merge:refs/remotes/pull/64599/merge
---
2019-09-19T18:05:40.4875885Z .................................................................................................... 1500/9024
2019-09-19T18:05:46.7829724Z .................................................................................................... 1600/9024
2019-09-19T18:06:00.0341243Z .................................................................i...............i.................. 1700/9024
2019-09-19T18:06:07.5482975Z .................................................................................................... 1800/9024
2019-09-19T18:06:23.6540508Z ........................................................iiiii....................................... 1900/9024
2019-09-19T18:06:35.9450412Z .................................................................................................... 2100/9024
2019-09-19T18:06:38.7004644Z .................................................................................................... 2200/9024
2019-09-19T18:06:42.3034143Z .................................................................................................... 2300/9024
2019-09-19T18:06:51.1740262Z .................................................................................................... 2400/9024
---
2019-09-19T18:09:58.6506688Z ............................................i...............i....................................... 4700/9024
2019-09-19T18:10:09.7308837Z .................................................................................................... 4800/9024
2019-09-19T18:10:17.3581684Z .................................................................................................... 4900/9024
2019-09-19T18:10:27.5920054Z .................................................................................................... 5000/9024
2019-09-19T18:10:35.9336022Z ............................ii.ii................................................................... 5100/9024
2019-09-19T18:10:46.6638985Z .................................................................................................... 5300/9024
2019-09-19T18:10:57.6554360Z ............................................................................................i....... 5400/9024
2019-09-19T18:11:06.3990625Z .................................................................................................... 5500/9024
2019-09-19T18:11:11.5757931Z .................................................................................................... 5600/9024
2019-09-19T18:11:11.5757931Z .................................................................................................... 5600/9024
2019-09-19T18:11:22.8826459Z .......................................................................................ii...i...ii.. 5700/9024
2019-09-19T18:11:49.5556929Z .................................................................................................... 5900/9024
2019-09-19T18:12:00.0098328Z .................................................................................................... 6000/9024
2019-09-19T18:12:00.0098328Z .................................................................................................... 6000/9024
2019-09-19T18:12:07.0061540Z .........................................................................................i..ii...... 6100/9024
2019-09-19T18:12:36.8041642Z .................................................................................................... 6300/9024
2019-09-19T18:12:41.5010611Z ................................................i................................................... 6400/9024
2019-09-19T18:12:43.9404857Z .................................................................................................... 6500/9024
2019-09-19T18:12:46.6416941Z ....................i............................................................................... 6600/9024
---
2019-09-19T18:17:33.2303332Z  finished in 5.430
2019-09-19T18:17:33.2537769Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T18:17:33.4385553Z 
2019-09-19T18:17:33.4386977Z running 150 tests
2019-09-19T18:17:36.9182107Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-19T18:17:39.0123928Z ..iiii..............i.........iii.i.......ii......
2019-09-19T18:17:39.0124559Z 
2019-09-19T18:17:39.0128925Z  finished in 5.759
2019-09-19T18:17:39.0329944Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T18:17:39.2070065Z 
---
2019-09-19T18:17:41.3800857Z  finished in 2.347
2019-09-19T18:17:41.4013066Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T18:17:41.5631246Z 
2019-09-19T18:17:41.5632325Z running 9 tests
2019-09-19T18:17:41.5633463Z iiiiiiiii
2019-09-19T18:17:41.5634238Z 
2019-09-19T18:17:41.5634478Z  finished in 0.162
2019-09-19T18:17:41.5828998Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T18:17:41.7674052Z 
---
2019-09-19T18:18:00.9279956Z  finished in 19.345
2019-09-19T18:18:00.9517914Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T18:18:01.1586461Z 
2019-09-19T18:18:01.1586783Z running 123 tests
2019-09-19T18:18:27.2574343Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-19T18:18:32.2241841Z i.i.i......iii.i.....ii
2019-09-19T18:18:32.2242569Z 
2019-09-19T18:18:32.2248167Z  finished in 31.273
2019-09-19T18:18:32.2259597Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T18:18:32.2260037Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-19T18:26:34.0775395Z ---- [rustdoc] rustdoc/issue-29503.rs stdout ----
2019-09-19T18:26:34.0775458Z 
2019-09-19T18:26:34.0775511Z error: rustdoc failed!
2019-09-19T18:26:34.0775585Z status: exit code: 1
2019-09-19T18:26:34.0776273Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503" "/checkout/src/test/rustdoc/issue-29503.rs"
2019-09-19T18:26:34.0776668Z ------------------------------------------
2019-09-19T18:26:34.0776705Z 
2019-09-19T18:26:34.0776950Z ------------------------------------------
2019-09-19T18:26:34.0777020Z stderr:
2019-09-19T18:26:34.0777020Z stderr:
2019-09-19T18:26:34.0777261Z ------------------------------------------
2019-09-19T18:26:34.0777331Z error: internal compiler error: src/librustc/ty/query/mod.rs:101: `tcx.is_async_fn(DefId(0:17 ~ issue_29503[8787]::{{impl}}[0]::my_string[0]))` unsupported by its crate
2019-09-19T18:26:34.0777679Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-09-19T18:26:34.0777740Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-19T18:26:34.0777813Z error: aborting due to previous error
2019-09-19T18:26:34.0777846Z 
---
2019-09-19T18:26:34.0779506Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-19T18:26:34.0779587Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-19T18:26:34.0779622Z 
2019-09-19T18:26:34.0779651Z 
2019-09-19T18:26:34.0781571Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-19T18:26:34.0781880Z 
2019-09-19T18:26:34.0781912Z 
2019-09-19T18:26:34.0814629Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-19T18:26:34.0814730Z Build completed unsuccessfully in 1:18:34
2019-09-19T18:26:34.0814730Z Build completed unsuccessfully in 1:18:34
2019-09-19T18:26:34.0847400Z == clock drift check ==
2019-09-19T18:26:34.0878075Z   local time: Thu Sep 19 18:26:34 UTC 2019
2019-09-19T18:26:34.2446869Z   network time: Thu, 19 Sep 2019 18:26:34 GMT
2019-09-19T18:26:34.2447392Z == end clock drift check ==
2019-09-19T18:26:36.0735628Z ##[error]Bash exited with code '1'.
2019-09-19T18:26:36.0899261Z ##[section]Starting: Checkout
2019-09-19T18:26:36.0901427Z ==============================================================================
2019-09-19T18:26:36.0901490Z Task         : Get sources
2019-09-19T18:26:36.0901560Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
