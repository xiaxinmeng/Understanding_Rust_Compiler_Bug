plain
2019-09-09T13:13:03.6427924Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T13:13:03.6600759Z ##[command]git config gc.auto 0
2019-09-09T13:13:03.6678220Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T13:13:03.6731178Z ##[command]git config --get-all http.proxy
2019-09-09T13:13:03.6855883Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64290/merge:refs/remotes/pull/64290/merge
---
2019-09-09T14:10:39.3747417Z .................................................................................................... 1500/9009
2019-09-09T14:10:44.8197175Z .................................................................................................... 1600/9009
2019-09-09T14:10:56.6772832Z ......................................................i...............i............................. 1700/9009
2019-09-09T14:11:04.1131343Z .................................................................................................... 1800/9009
2019-09-09T14:11:17.5894928Z .............................................iiiii.................................................. 1900/9009
2019-09-09T14:11:27.9588314Z .................................................................................................... 2100/9009
2019-09-09T14:11:30.4133699Z .................................................................................................... 2200/9009
2019-09-09T14:11:33.7094644Z .................................................................................................... 2300/9009
2019-09-09T14:11:41.1296092Z .................................................................................................... 2400/9009
---
2019-09-09T14:14:30.9949983Z ...................................i...............i................................................ 4700/9009
2019-09-09T14:14:41.8061496Z .................................................................................................... 4800/9009
2019-09-09T14:14:48.0032567Z .................................................................................................... 4900/9009
2019-09-09T14:14:57.9602627Z .................................................................................................... 5000/9009
2019-09-09T14:15:03.7858314Z .................ii.ii.............................................................................. 5100/9009
2019-09-09T14:15:13.6136946Z .................................................................................................... 5300/9009
2019-09-09T14:15:23.2382431Z ................................................................................i................... 5400/9009
2019-09-09T14:15:30.5779151Z .................................................................................................... 5500/9009
2019-09-09T14:15:36.1790671Z .................................................................................................... 5600/9009
2019-09-09T14:15:36.1790671Z .................................................................................................... 5600/9009
2019-09-09T14:15:46.4958931Z ..........................................................................ii...i..ii...........i.... 5700/9009
2019-09-09T14:16:09.9222039Z .................................................................................................... 5900/9009
2019-09-09T14:16:18.9789599Z .................................................................................................... 6000/9009
2019-09-09T14:16:18.9789599Z .................................................................................................... 6000/9009
2019-09-09T14:16:24.7599270Z ............................................................................i..ii................... 6100/9009
2019-09-09T14:16:53.0522846Z .................................................................................................... 6300/9009
2019-09-09T14:16:54.9589152Z ...................................i................................................................ 6400/9009
2019-09-09T14:16:56.8720180Z .................................................................................................... 6500/9009
2019-09-09T14:16:59.4793333Z .......i............................................................................................ 6600/9009
---
2019-09-09T14:21:31.2340909Z  finished in 18.925
2019-09-09T14:21:31.2522573Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T14:21:31.4015001Z 
2019-09-09T14:21:31.4015838Z running 150 tests
2019-09-09T14:21:34.6704268Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-09T14:21:36.7697805Z ..iiii..............i.........iii.i.......ii......
2019-09-09T14:21:37.4263163Z 
2019-09-09T14:21:37.4276165Z  finished in 5.517
2019-09-09T14:21:37.4284289Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T14:21:37.4291320Z 
---
2019-09-09T14:21:39.1192697Z  finished in 2.332
2019-09-09T14:21:39.1400140Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T14:21:39.3089400Z 
2019-09-09T14:21:39.3089687Z running 9 tests
2019-09-09T14:21:39.3090967Z iiiiiiiii
2019-09-09T14:21:39.3091263Z 
2019-09-09T14:21:39.3092011Z  finished in 0.169
2019-09-09T14:21:39.3417748Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T14:21:39.9187946Z 
---
2019-09-09T14:21:58.0771838Z  finished in 18.735
2019-09-09T14:21:58.0962064Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T14:21:58.2602499Z 
2019-09-09T14:21:58.2603824Z running 123 tests
2019-09-09T14:22:21.9038204Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-09T14:22:26.4555786Z i.i.i......iii.i.....ii
2019-09-09T14:22:26.4559430Z 
2019-09-09T14:22:26.4559832Z  finished in 28.359
2019-09-09T14:22:26.4568753Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T14:22:26.4569549Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-09T14:23:25.7951650Z failures:
2019-09-09T14:23:25.7951679Z 
2019-09-09T14:23:25.7952110Z ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
2019-09-09T14:23:25.7952143Z 
2019-09-09T14:23:25.7952451Z error: /checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs:3: unexpected error: '3:1: 15:13: `main` function not found in crate `hash_stable_is_unstable` [E0601]'
2019-09-09T14:23:25.7952537Z error: 1 unexpected errors found, 0 expected errors not found
2019-09-09T14:23:25.7952596Z status: exit code: 1
2019-09-09T14:23:25.7952596Z status: exit code: 1
2019-09-09T14:23:25.7953197Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
2019-09-09T14:23:25.7953302Z unexpected errors (from JSON output): [
2019-09-09T14:23:25.7953341Z     Error {
2019-09-09T14:23:25.7953554Z         line_num: 3,
2019-09-09T14:23:25.7953608Z         kind: Some(
2019-09-09T14:23:25.7953676Z         ),
2019-09-09T14:23:25.7953676Z         ),
2019-09-09T14:23:25.7953741Z         msg: "3:1: 15:13: `main` function not found in crate `hash_stable_is_unstable` [E0601]",
2019-09-09T14:23:25.7953822Z ]
2019-09-09T14:23:25.7953845Z 
2019-09-09T14:23:25.7954107Z thread '[ui] ui-fulldeps/hash-stable-is-unstable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1512:13
2019-09-09T14:23:25.7954160Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
2019-09-09T14:23:25.7954865Z 
2019-09-09T14:23:25.7955070Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-09T14:23:25.7964136Z 
2019-09-09T14:23:25.7964272Z 
2019-09-09T14:23:25.7965731Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T14:23:25.7966265Z 
2019-09-09T14:23:25.7966290Z 
2019-09-09T14:23:25.7972507Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T14:23:25.7972608Z Build completed unsuccessfully in 1:03:39
2019-09-09T14:23:25.7972608Z Build completed unsuccessfully in 1:03:39
2019-09-09T14:23:25.8025995Z == clock drift check ==
2019-09-09T14:23:25.8034831Z   local time: Mon Sep  9 14:23:25 UTC 2019
2019-09-09T14:23:25.9530113Z   network time: Mon, 09 Sep 2019 14:23:25 GMT
2019-09-09T14:23:25.9533990Z == end clock drift check ==
2019-09-09T14:23:26.7514977Z ##[error]Bash exited with code '1'.
2019-09-09T14:23:26.7573810Z ##[section]Starting: Checkout
2019-09-09T14:23:26.7577182Z ==============================================================================
2019-09-09T14:23:26.7577243Z Task         : Get sources
2019-09-09T14:23:26.7577329Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
