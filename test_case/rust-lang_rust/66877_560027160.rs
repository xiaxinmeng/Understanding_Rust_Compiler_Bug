plain
2019-11-30T21:34:25.3299406Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T21:34:25.3483919Z ##[command]git config gc.auto 0
2019-11-30T21:34:25.3561567Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T21:34:25.3621918Z ##[command]git config --get-all http.proxy
2019-11-30T21:34:26.1856222Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66877/merge:refs/remotes/pull/66877/merge
---
2019-11-30T22:31:54.0636402Z ................................................F................................................... 1600/9312
2019-11-30T22:31:58.6454717Z .................................................................................................... 1700/9312
2019-11-30T22:32:10.8948671Z ........................................i........................................................... 1800/9312
2019-11-30T22:32:18.7015201Z .................................................................................................... 1900/9312
2019-11-30T22:32:32.2725618Z .........................iiiii...................................................................... 2000/9312
2019-11-30T22:32:42.2076635Z .................................................................................................... 2200/9312
2019-11-30T22:32:44.7347369Z .................................................................................................... 2300/9312
2019-11-30T22:32:49.2775076Z .................................................................................................... 2400/9312
2019-11-30T22:33:10.5709879Z .................................................................................................... 2500/9312
---
2019-11-30T22:35:47.4864695Z ...........................i...............i........................................................ 4800/9312
2019-11-30T22:35:57.9164011Z .................................................................................................... 4900/9312
2019-11-30T22:36:03.7362102Z .................................................................................................... 5000/9312
2019-11-30T22:36:11.6655775Z .................................................................................................... 5100/9312
2019-11-30T22:36:19.1294620Z ................................ii.ii...........i................................................... 5200/9312
2019-11-30T22:36:28.4674214Z .................................................................................................... 5400/9312
2019-11-30T22:36:38.4218962Z .................................................................................................... 5500/9312
2019-11-30T22:36:45.3733213Z ..............i..................................................................................... 5600/9312
2019-11-30T22:36:51.5529855Z .................................................................................................... 5700/9312
2019-11-30T22:36:51.5529855Z .................................................................................................... 5700/9312
2019-11-30T22:37:02.3870521Z .................................................................................................... 5800/9312
2019-11-30T22:37:14.4431384Z ii...i..ii...........i.............................................................................. 5900/9312
2019-11-30T22:37:32.4680872Z .................................................................................................... 6100/9312
2019-11-30T22:37:38.3472543Z .................................................................................................... 6200/9312
2019-11-30T22:37:38.3472543Z .................................................................................................... 6200/9312
2019-11-30T22:37:51.8202813Z .......................i..ii........................................................................ 6300/9312
2019-11-30T22:38:11.0822381Z ...........................................................................................i........ 6500/9312
2019-11-30T22:38:13.3235364Z .................................................................................................... 6600/9312
2019-11-30T22:38:15.5495519Z ..................................................................................i................. 6700/9312
2019-11-30T22:38:18.2372071Z .................................................................................................... 6800/9312
---
2019-11-30T22:43:00.8582392Z 
2019-11-30T22:43:00.8583391Z ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
2019-11-30T22:43:00.8583700Z diff of stderr:
2019-11-30T22:43:00.8583883Z 
2019-11-30T22:43:00.8584390Z 17 note: ...which requires const-evaluating `std::mem::size_of`...
2019-11-30T22:43:00.8584845Z 18   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-11-30T22:43:00.8585097Z 19    |
2019-11-30T22:43:00.8585669Z - LL |     intrinsics::size_of::<T>()
2019-11-30T22:43:00.8586271Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-30T22:43:00.8587413Z + LL | pub const fn size_of<T>() -> usize {
2019-11-30T22:43:00.8588167Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-30T22:43:00.8588766Z 22 note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-11-30T22:43:00.8589339Z 23   --> $SRC_DIR/libcore/intrinsics.rs:LL:COL
2019-11-30T22:43:00.8589862Z 
2019-11-30T22:43:00.8590057Z 
2019-11-30T22:43:00.8590309Z The actual stderr differed from the expected stderr.
2019-11-30T22:43:00.8591259Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
2019-11-30T22:43:00.8591259Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
2019-11-30T22:43:00.8591755Z To update references, rerun the tests and pass the `--bless` flag
2019-11-30T22:43:00.8592257Z To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
2019-11-30T22:43:00.8592696Z error: 1 errors occurred comparing output.
2019-11-30T22:43:00.8592886Z status: exit code: 1
2019-11-30T22:43:00.8592886Z status: exit code: 1
2019-11-30T22:43:00.8593946Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
2019-11-30T22:43:00.8594657Z ------------------------------------------
2019-11-30T22:43:00.8595497Z 
2019-11-30T22:43:00.8595965Z ------------------------------------------
2019-11-30T22:43:00.8596142Z stderr:
2019-11-30T22:43:00.8596142Z stderr:
2019-11-30T22:43:00.8596465Z ------------------------------------------
2019-11-30T22:43:00.8597232Z error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}#0`
2019-11-30T22:43:00.8597968Z    |
2019-11-30T22:43:00.8597968Z    |
2019-11-30T22:43:00.8598115Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-11-30T22:43:00.8598608Z    |
2019-11-30T22:43:00.8598608Z    |
2019-11-30T22:43:00.8599103Z note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`...
2019-11-30T22:43:00.8599751Z    |
2019-11-30T22:43:00.8599751Z    |
2019-11-30T22:43:00.8599900Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-11-30T22:43:00.8600068Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-30T22:43:00.8601130Z note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`...
2019-11-30T22:43:00.8602181Z    |
2019-11-30T22:43:00.8602181Z    |
2019-11-30T22:43:00.8602302Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-11-30T22:43:00.8602747Z note: ...which requires const-evaluating `std::mem::size_of`...
2019-11-30T22:43:00.8603083Z   --> /checkout/src/libcore/mem/mod.rs:274:1
2019-11-30T22:43:00.8603254Z    |
2019-11-30T22:43:00.8603254Z    |
2019-11-30T22:43:00.8603744Z LL | pub const fn size_of<T>() -> usize {
2019-11-30T22:43:00.8603910Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-30T22:43:00.8604271Z note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-11-30T22:43:00.8604785Z    |
2019-11-30T22:43:00.8605083Z LL |     pub fn size_of<T>() -> usize;
2019-11-30T22:43:00.8605418Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-30T22:43:00.8605561Z    = note: ...which requires computing layout of `Foo`...
2019-11-30T22:43:00.8605561Z    = note: ...which requires computing layout of `Foo`...
2019-11-30T22:43:00.8605691Z    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
2019-11-30T22:43:00.8606064Z    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`, completing the cycle
2019-11-30T22:43:00.8606279Z note: cycle used when processing `Foo`
2019-11-30T22:43:00.8607409Z    |
2019-11-30T22:43:00.8607561Z LL | struct Foo {
2019-11-30T22:43:00.8607701Z    | ^^^^^^^^^^
2019-11-30T22:43:00.8607840Z 
---
2019-11-30T22:43:00.8623339Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-30T22:43:00.8623632Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-30T22:43:00.8639015Z 
2019-11-30T22:43:00.8639550Z 
2019-11-30T22:43:00.8644503Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-30T22:43:00.8645260Z 
2019-11-30T22:43:00.8645386Z 
2019-11-30T22:43:00.8649283Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-30T22:43:00.8649552Z Build completed unsuccessfully in 1:02:43
2019-11-30T22:43:00.8649552Z Build completed unsuccessfully in 1:02:43
2019-11-30T22:43:00.8700822Z == clock drift check ==
2019-11-30T22:43:00.8728287Z   local time: Sat Nov 30 22:43:00 UTC 2019
2019-11-30T22:43:01.1563548Z   network time: Sat, 30 Nov 2019 22:43:01 GMT
2019-11-30T22:43:01.1564070Z == end clock drift check ==
2019-11-30T22:43:02.0316236Z 
2019-11-30T22:43:02.0432604Z ##[error]Bash exited with code '1'.
2019-11-30T22:43:02.0478186Z ##[section]Starting: Checkout
2019-11-30T22:43:02.0480118Z ==============================================================================
2019-11-30T22:43:02.0480175Z Task         : Get sources
2019-11-30T22:43:02.0480223Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
