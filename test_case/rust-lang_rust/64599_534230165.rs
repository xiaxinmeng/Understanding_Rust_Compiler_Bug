plain
2019-09-23T17:22:24.4000191Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-23T17:22:24.4162072Z ##[command]git config gc.auto 0
2019-09-23T17:22:24.4229480Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-23T17:22:24.4275979Z ##[command]git config --get-all http.proxy
2019-09-23T17:22:24.4408179Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64599/merge:refs/remotes/pull/64599/merge
---
2019-09-23T18:22:16.9884487Z .................................................................................................... 1500/9038
2019-09-23T18:22:22.8032792Z .................................................................................................... 1600/9038
2019-09-23T18:22:34.3430995Z .........................................................................i...............i.......... 1700/9038
2019-09-23T18:22:40.8800065Z .................................................................................................... 1800/9038
2019-09-23T18:22:49.1773042Z ................................................................iiiii............................... 1900/9038
2019-09-23T18:23:07.3300491Z .................................................................................................... 2100/9038
2019-09-23T18:23:09.7093134Z .................................................................................................... 2200/9038
2019-09-23T18:23:12.8359920Z .................................................................................................... 2300/9038
2019-09-23T18:23:20.7377848Z .................................................................................................... 2400/9038
---
2019-09-23T18:26:09.9543033Z .....................................................i...............i.............................. 4700/9038
2019-09-23T18:26:18.6927815Z .................................................................................................... 4800/9038
2019-09-23T18:26:26.8350086Z .................................................................................................... 4900/9038
2019-09-23T18:26:33.8669958Z .................................................................................................... 5000/9038
2019-09-23T18:26:43.0384381Z ........................................ii.ii....................................................... 5100/9038
2019-09-23T18:26:52.5163035Z .................................................................................................... 5300/9038
2019-09-23T18:27:02.5371258Z .................................................................................................... 5400/9038
2019-09-23T18:27:09.5490533Z .....i.............................................................................................. 5500/9038
2019-09-23T18:27:14.5351140Z .................................................................................................... 5600/9038
2019-09-23T18:27:14.5351140Z .................................................................................................... 5600/9038
2019-09-23T18:27:25.5516848Z .................................................................................................... 5700/9038
2019-09-23T18:27:38.0641378Z ii...i..ii...........i.............................................................................. 5800/9038
2019-09-23T18:27:58.3532537Z .................................................................................................... 6000/9038
2019-09-23T18:28:06.7969394Z .................................................................................................... 6100/9038
2019-09-23T18:28:06.7969394Z .................................................................................................... 6100/9038
2019-09-23T18:28:20.8501563Z ..i..ii............................................................................................. 6200/9038
2019-09-23T18:28:38.3300115Z ..............................................................i..................................... 6400/9038
2019-09-23T18:28:40.3639399Z .................................................................................................... 6500/9038
2019-09-23T18:28:42.6040917Z ..................................i................................................................. 6600/9038
2019-09-23T18:28:46.4989221Z .................................................................................................... 6700/9038
---
2019-09-23T18:33:00.9584958Z  finished in 5.162
2019-09-23T18:33:00.9758006Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T18:33:01.1725467Z 
2019-09-23T18:33:01.1725887Z running 150 tests
2019-09-23T18:33:04.2364934Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-23T18:33:06.1092746Z ..iiii..............i.........iii.i.......ii......
2019-09-23T18:33:06.1094782Z 
2019-09-23T18:33:06.1098196Z  finished in 5.134
2019-09-23T18:33:06.1282852Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T18:33:06.2885071Z 
---
2019-09-23T18:33:08.2441072Z  finished in 2.115
2019-09-23T18:33:08.2618803Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T18:33:08.4164673Z 
2019-09-23T18:33:08.4165573Z running 9 tests
2019-09-23T18:33:08.4166638Z iiiiiiiii
2019-09-23T18:33:08.4167537Z 
2019-09-23T18:33:08.4167688Z  finished in 0.154
2019-09-23T18:33:08.4339233Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T18:33:08.6235281Z 
---
2019-09-23T18:33:25.7830387Z  finished in 17.349
2019-09-23T18:33:25.8062321Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T18:33:25.9908886Z 
2019-09-23T18:33:25.9909209Z running 123 tests
2019-09-23T18:33:48.7044248Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-23T18:33:52.9776075Z i.i.i......iii.i.....ii
2019-09-23T18:33:52.9777305Z 
2019-09-23T18:33:52.9777776Z  finished in 27.171
2019-09-23T18:33:52.9788486Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-23T18:33:52.9789593Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-23T18:40:58.9855450Z ---- [rustdoc] rustdoc/ffi.rs stdout ----
2019-09-23T18:40:58.9855737Z 
2019-09-23T18:40:58.9855905Z error: rustdoc failed!
2019-09-23T18:40:58.9855970Z status: exit code: 1
2019-09-23T18:40:58.9856493Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ffi/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ffi" "/checkout/src/test/rustdoc/ffi.rs"
2019-09-23T18:40:58.9856940Z ------------------------------------------
2019-09-23T18:40:58.9857092Z 
2019-09-23T18:40:58.9857455Z ------------------------------------------
2019-09-23T18:40:58.9857639Z stderr:
2019-09-23T18:40:58.9857639Z stderr:
2019-09-23T18:40:58.9858056Z ------------------------------------------
2019-09-23T18:40:58.9858234Z error: internal compiler error: src/librustc_metadata/decoder.rs:1219: asyncness: expect functions entry.
2019-09-23T18:40:58.9858804Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:812:9
2019-09-23T18:40:58.9858853Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-23T18:40:58.9859122Z error: aborting due to previous error
2019-09-23T18:40:58.9859237Z 
---
2019-09-23T18:40:58.9878651Z 
2019-09-23T18:40:58.9878996Z ------------------------------------------
2019-09-23T18:40:58.9879060Z stderr:
2019-09-23T18:40:58.9879396Z ------------------------------------------
2019-09-23T18:40:58.9880137Z error: internal compiler error: src/librustc_metadata/decoder.rs:1219: asyncness: expect functions entry.
2019-09-23T18:40:58.9880641Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:812:9
2019-09-23T18:40:58.9880701Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-23T18:40:58.9880936Z error: aborting due to previous error
2019-09-23T18:40:58.9880999Z 
---
2019-09-23T18:40:58.9881824Z ---- [rustdoc] rustdoc/issue-34274.rs stdout ----
2019-09-23T18:40:58.9882019Z 
2019-09-23T18:40:58.9882194Z error: rustdoc failed!
2019-09-23T18:40:58.9882263Z status: exit code: 1
2019-09-23T18:40:58.9883124Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34274/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34274" "/checkout/src/test/rustdoc/issue-34274.rs"
2019-09-23T18:40:58.9883653Z ------------------------------------------
2019-09-23T18:40:58.9883685Z 
2019-09-23T18:40:58.9884206Z ------------------------------------------
2019-09-23T18:40:58.9884246Z stderr:
2019-09-23T18:40:58.9884246Z stderr:
2019-09-23T18:40:58.9884440Z ------------------------------------------
2019-09-23T18:40:58.9884670Z error: internal compiler error: src/librustc_metadata/decoder.rs:1219: asyncness: expect functions entry.
2019-09-23T18:40:58.9885086Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:812:9
2019-09-23T18:40:58.9885262Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-23T18:40:58.9885420Z error: aborting due to previous error
2019-09-23T18:40:58.9885446Z 
---
2019-09-23T18:40:58.9892910Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-23T18:40:58.9893393Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-23T18:40:58.9896007Z 
2019-09-23T18:40:58.9896248Z 
2019-09-23T18:40:58.9902927Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-23T18:40:58.9903703Z 
2019-09-23T18:40:58.9903758Z 
2019-09-23T18:40:58.9957265Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-23T18:40:58.9957330Z Build completed unsuccessfully in 1:11:30
2019-09-23T18:40:58.9957330Z Build completed unsuccessfully in 1:11:30
2019-09-23T18:40:58.9962117Z == clock drift check ==
2019-09-23T18:40:58.9978121Z   local time: Mon Sep 23 18:40:58 UTC 2019
2019-09-23T18:40:59.0856312Z   network time: Mon, 23 Sep 2019 18:40:59 GMT
2019-09-23T18:40:59.0858717Z == end clock drift check ==
2019-09-23T18:41:01.2238088Z ##[error]Bash exited with code '1'.
2019-09-23T18:41:01.2274663Z ##[section]Starting: Checkout
2019-09-23T18:41:01.2276285Z ==============================================================================
2019-09-23T18:41:01.2276330Z Task         : Get sources
2019-09-23T18:41:01.2276576Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
