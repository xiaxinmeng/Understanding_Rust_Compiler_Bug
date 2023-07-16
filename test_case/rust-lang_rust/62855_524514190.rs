plain
2019-08-24T02:07:47.4351564Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T02:07:47.4518912Z ##[command]git config gc.auto 0
2019-08-24T02:07:47.4585379Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T02:07:47.4637585Z ##[command]git config --get-all http.proxy
2019-08-24T02:07:47.4765173Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62855/merge:refs/remotes/pull/62855/merge
---
2019-08-24T02:08:22.1344779Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T02:08:22.1344805Z 
2019-08-24T02:08:22.1344990Z   git checkout -b <new-branch-name>
2019-08-24T02:08:22.1345013Z 
2019-08-24T02:08:22.1345052Z HEAD is now at 079778052 Merge 8977f4358621ba61f4d3ea8049d3f4972d938fc8 into 49935246829165b2c4c5c69f981a300bcaa83d7f
2019-08-24T02:08:22.1511599Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T02:08:22.1514910Z ==============================================================================
2019-08-24T02:08:22.1514954Z Task         : Bash
2019-08-24T02:08:22.1514988Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T03:08:51.2901523Z .................................................................................................... 1500/8950
2019-08-24T03:08:56.5414594Z .................................................................................................... 1600/8950
2019-08-24T03:09:08.7688814Z ...........................................i...............i........................................ 1700/8950
2019-08-24T03:09:16.6308071Z .................................................................................................... 1800/8950
2019-08-24T03:09:30.2797849Z ...................................iiiii............................................................ 1900/8950
2019-08-24T03:09:40.4369070Z .................................................................................................... 2100/8950
2019-08-24T03:09:42.8557131Z .................................................................................................... 2200/8950
2019-08-24T03:09:47.0767214Z .................................................................................................... 2300/8950
2019-08-24T03:09:54.4947769Z .................................................................................................... 2400/8950
---
2019-08-24T03:12:46.3933928Z .......................i...............i............................................................ 4700/8950
2019-08-24T03:12:57.6401922Z .................................................................................................... 4800/8950
2019-08-24T03:13:03.5657395Z .................................................................................................... 4900/8950
2019-08-24T03:13:14.2876376Z .................................................................................................... 5000/8950
2019-08-24T03:13:19.3849194Z ....ii.ii........................................................................................... 5100/8950
2019-08-24T03:13:33.3156614Z .................................................................................................... 5300/8950
2019-08-24T03:13:39.8340751Z ............................................................i....................................... 5400/8950
2019-08-24T03:13:46.6200994Z .................................................................................................... 5500/8950
2019-08-24T03:13:54.1426391Z .................................................................................................... 5600/8950
2019-08-24T03:13:54.1426391Z .................................................................................................... 5600/8950
2019-08-24T03:14:04.6745015Z ......................................................ii...i..ii...........i........................ 5700/8950
2019-08-24T03:14:27.0124524Z .................................................................................................... 5900/8950
2019-08-24T03:14:31.7357235Z .................................................................................................... 6000/8950
2019-08-24T03:14:31.7357235Z .................................................................................................... 6000/8950
2019-08-24T03:14:38.9245814Z .......................................................i..ii........................................ 6100/8950
2019-08-24T03:15:05.1415395Z .................................................................................................... 6300/8950
2019-08-24T03:15:07.1453785Z .i.................................................................................................. 6400/8950
2019-08-24T03:15:09.2215963Z .........................................................................i.......................... 6500/8950
2019-08-24T03:15:11.9608309Z .................................................................................................... 6600/8950
---
2019-08-24T03:19:39.0620138Z  finished in 18.703
2019-08-24T03:19:39.0774108Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-24T03:19:39.2290550Z 
2019-08-24T03:19:39.2290875Z running 149 tests
2019-08-24T03:19:42.4058059Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/149
2019-08-24T03:19:44.2645453Z ..iiii..............i.........iii.i......ii......
2019-08-24T03:19:44.2646077Z 
2019-08-24T03:19:44.2646121Z  finished in 5.186
2019-08-24T03:19:44.2802435Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-24T03:19:44.4417011Z 
---
2019-08-24T03:19:46.4119535Z  finished in 2.131
2019-08-24T03:19:46.4283943Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-24T03:19:46.5825056Z 
2019-08-24T03:19:46.5825539Z running 9 tests
2019-08-24T03:19:46.5835167Z iiiiiiiii
2019-08-24T03:19:46.5835524Z 
2019-08-24T03:19:46.5841450Z  finished in 0.155
2019-08-24T03:19:46.6015175Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-24T03:19:46.7529260Z 
---
2019-08-24T03:20:04.1904978Z  finished in 17.589
2019-08-24T03:20:04.2093488Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-24T03:20:04.3647911Z 
2019-08-24T03:20:04.3649305Z running 122 tests
2019-08-24T03:20:27.0031566Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-24T03:20:31.4339373Z .i.i......iii.i.....ii
2019-08-24T03:20:31.4342953Z 
2019-08-24T03:20:31.4343244Z  finished in 27.224
2019-08-24T03:20:31.4346967Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-24T03:20:31.4347564Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-24T03:27:37.0798186Z ---- [rustdoc] rustdoc/inline_cross/proc_macro.rs stdout ----
2019-08-24T03:27:37.0798271Z 
2019-08-24T03:27:37.0798400Z error: rustdoc failed!
2019-08-24T03:27:37.0798447Z status: exit code: 1
2019-08-24T03:27:37.0799419Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/proc_macro/auxiliary/proc_macro/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/proc_macro" "/checkout/src/test/rustdoc/inline_cross/auxiliary/proc_macro.rs" "--proc-macro-crate"
2019-08-24T03:27:37.0799958Z ------------------------------------------
2019-08-24T03:27:37.0799988Z 
2019-08-24T03:27:37.0800166Z ------------------------------------------
2019-08-24T03:27:37.0800222Z stderr:
---
2019-08-24T03:27:37.0801088Z ---- [rustdoc] rustdoc/proc-macro.rs stdout ----
2019-08-24T03:27:37.0801115Z 
2019-08-24T03:27:37.0801148Z error: rustdoc failed!
2019-08-24T03:27:37.0801183Z status: exit code: 1
2019-08-24T03:27:37.0801688Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/proc-macro/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/proc-macro" "/checkout/src/test/rustdoc/proc-macro.rs" "--proc-macro-crate" "--document-private-items"
2019-08-24T03:27:37.0802131Z ------------------------------------------
2019-08-24T03:27:37.0802160Z 
2019-08-24T03:27:37.0802344Z ------------------------------------------
2019-08-24T03:27:37.0802381Z stderr:
---
2019-08-24T03:27:37.0803443Z ---- [rustdoc] rustdoc/rustc-macro-crate.rs stdout ----
2019-08-24T03:27:37.0803499Z 
2019-08-24T03:27:37.0803537Z error: rustdoc failed!
2019-08-24T03:27:37.0803573Z status: exit code: 1
2019-08-24T03:27:37.0804271Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/rustc-macro-crate/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/rustc-macro-crate" "/checkout/src/test/rustdoc/rustc-macro-crate.rs" "--proc-macro-crate"
2019-08-24T03:27:37.0804539Z ------------------------------------------
2019-08-24T03:27:37.0804586Z 
2019-08-24T03:27:37.0804775Z ------------------------------------------
2019-08-24T03:27:37.0804813Z stderr:
---
2019-08-24T03:27:37.0812648Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-24T03:27:37.0812903Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-24T03:27:37.0813260Z 
2019-08-24T03:27:37.0813413Z 
2019-08-24T03:27:37.0815291Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-24T03:27:37.0815822Z 
2019-08-24T03:27:37.0816116Z 
2019-08-24T03:27:37.0819135Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-24T03:27:37.0819438Z Build completed unsuccessfully in 1:12:58
2019-08-24T03:27:37.0819438Z Build completed unsuccessfully in 1:12:58
2019-08-24T03:27:37.0868736Z == clock drift check ==
2019-08-24T03:27:37.0882605Z   local time: Sat Aug 24 03:27:37 UTC 2019
2019-08-24T03:27:37.1759892Z   network time: Sat, 24 Aug 2019 03:27:37 GMT
2019-08-24T03:27:37.1762872Z == end clock drift check ==
2019-08-24T03:27:39.4491938Z ##[error]Bash exited with code '1'.
2019-08-24T03:27:39.4534226Z ##[section]Starting: Checkout
2019-08-24T03:27:39.4536888Z ==============================================================================
2019-08-24T03:27:39.4537034Z Task         : Get sources
2019-08-24T03:27:39.4537083Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
