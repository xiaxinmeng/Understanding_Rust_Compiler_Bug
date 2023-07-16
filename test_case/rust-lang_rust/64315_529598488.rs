plain
2019-09-09T16:49:34.2515106Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T16:49:34.2719726Z ##[command]git config gc.auto 0
2019-09-09T16:49:34.2795277Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T16:49:34.2855097Z ##[command]git config --get-all http.proxy
2019-09-09T16:49:34.3015612Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64315/merge:refs/remotes/pull/64315/merge
---
2019-09-09T17:52:54.5027823Z .................................................................................................... 1500/9003
2019-09-09T17:53:00.5974301Z .................................................................................................... 1600/9003
2019-09-09T17:53:13.9823871Z .....................................................i...............i.............................. 1700/9003
2019-09-09T17:53:22.3220824Z .................................................................................................... 1800/9003
2019-09-09T17:53:37.4936748Z ............................................iiiii................................................... 1900/9003
2019-09-09T17:53:49.0597588Z .................................................................................................... 2100/9003
2019-09-09T17:53:51.7544041Z .................................................................................................... 2200/9003
2019-09-09T17:53:55.8652033Z .................................................................................................... 2300/9003
2019-09-09T17:54:04.1199923Z .................................................................................................... 2400/9003
---
2019-09-09T17:57:10.8319400Z ...............................i...............i.................................................... 4700/9003
2019-09-09T17:57:23.1323423Z .................................................................................................... 4800/9003
2019-09-09T17:57:29.6578180Z .................................................................................................... 4900/9003
2019-09-09T17:57:40.9849553Z .................................................................................................... 5000/9003
2019-09-09T17:57:47.0396280Z .............ii.ii.................................................................................. 5100/9003
2019-09-09T17:57:58.1136921Z .................................................................................................... 5300/9003
2019-09-09T17:58:08.6500350Z ............................................................................i....................... 5400/9003
2019-09-09T17:58:16.6235277Z .................................................................................................... 5500/9003
2019-09-09T17:58:23.1208065Z .................................................................................................... 5600/9003
2019-09-09T17:58:23.1208065Z .................................................................................................... 5600/9003
2019-09-09T17:58:34.1581291Z ......................................................................ii...i..ii...........i........ 5700/9003
2019-09-09T17:59:00.4301843Z .................................................................................................... 5900/9003
2019-09-09T17:59:10.0587990Z .................................................................................................... 6000/9003
2019-09-09T17:59:10.0587990Z .................................................................................................... 6000/9003
2019-09-09T17:59:15.8076540Z ........................................................................i..ii....................... 6100/9003
2019-09-09T17:59:45.8996212Z .................................................................................................... 6300/9003
2019-09-09T17:59:47.9656291Z ...............................i.................................................................... 6400/9003
2019-09-09T17:59:50.0932348Z .................................................................................................... 6500/9003
2019-09-09T17:59:52.7333043Z ...i................................................................................................ 6600/9003
---
2019-09-09T18:04:49.4871579Z  finished in 20.795
2019-09-09T18:04:49.5048169Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T18:04:49.6694647Z 
2019-09-09T18:04:49.6694988Z running 150 tests
2019-09-09T18:04:53.1259139Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-09T18:04:55.2603512Z ..iiii..............i.........iii.i.......ii......
2019-09-09T18:04:55.2607662Z 
2019-09-09T18:04:55.2609364Z  finished in 5.756
2019-09-09T18:04:55.2797014Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T18:04:55.4434312Z 
---
2019-09-09T18:04:57.7266360Z  finished in 2.447
2019-09-09T18:04:57.7514917Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T18:04:57.9194927Z 
2019-09-09T18:04:57.9196765Z running 9 tests
2019-09-09T18:04:57.9198587Z iiiiiiiii
2019-09-09T18:04:57.9199421Z 
2019-09-09T18:04:57.9202207Z  finished in 0.168
2019-09-09T18:04:57.9391299Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T18:04:58.1090873Z 
2019-09-09T18:04:58.1090873Z 
2019-09-09T18:04:58.1091968Z running 104 tests
2019-09-09T18:05:16.7787625Z ............................F...F................................................................... 100/104
2019-09-09T18:05:17.5474254Z ....
2019-09-09T18:05:17.5476638Z failures:
2019-09-09T18:05:17.5478058Z 
2019-09-09T18:05:17.5480059Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2019-09-09T18:05:17.5481986Z 
2019-09-09T18:05:17.5483736Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-09-09T18:05:17.5485134Z status: exit code: 1
2019-09-09T18:05:17.5487525Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2019-09-09T18:05:17.5492137Z ------------------------------------------
2019-09-09T18:05:17.5492920Z 
2019-09-09T18:05:17.5493540Z ------------------------------------------
2019-09-09T18:05:17.5493718Z stderr:
2019-09-09T18:05:17.5493718Z stderr:
2019-09-09T18:05:17.5494018Z ------------------------------------------
2019-09-09T18:05:17.5494206Z error: `mir_built(make_extern)` should be clean but is not
2019-09-09T18:05:17.5494993Z    |
2019-09-09T18:05:17.5494993Z    |
2019-09-09T18:05:17.5495174Z LL | pub extern "C" fn make_extern() {}
2019-09-09T18:05:17.5496455Z 
2019-09-09T18:05:17.5496721Z error: aborting due to previous error
2019-09-09T18:05:17.5496920Z 
2019-09-09T18:05:17.5498145Z 
2019-09-09T18:05:17.5498145Z 
2019-09-09T18:05:17.5499195Z ------------------------------------------
2019-09-09T18:05:17.5502195Z 
2019-09-09T18:05:17.5502619Z 
2019-09-09T18:05:17.5504118Z ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
2019-09-09T18:05:17.5504470Z 
2019-09-09T18:05:17.5504887Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-09-09T18:05:17.5505079Z status: exit code: 1
2019-09-09T18:05:17.5506076Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
2019-09-09T18:05:17.5507106Z ------------------------------------------
2019-09-09T18:05:17.5507289Z 
2019-09-09T18:05:17.5507610Z ------------------------------------------
2019-09-09T18:05:17.5507956Z stderr:
2019-09-09T18:05:17.5507956Z stderr:
2019-09-09T18:05:17.5508717Z ------------------------------------------
2019-09-09T18:05:17.5508984Z error: `mir_built(Foo::make_method_extern)` should be clean but is not
2019-09-09T18:05:17.5509660Z    |
2019-09-09T18:05:17.5509660Z    |
2019-09-09T18:05:17.5509832Z LL |     pub extern fn make_method_extern(&self) { }
2019-09-09T18:05:17.5510165Z 
2019-09-09T18:05:17.5510328Z error: aborting due to previous error
2019-09-09T18:05:17.5510470Z 
2019-09-09T18:05:17.5510625Z 
---
2019-09-09T18:05:17.5517188Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-09T18:05:17.5517241Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-09T18:05:17.5517271Z 
2019-09-09T18:05:17.5517310Z 
2019-09-09T18:05:17.5519835Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T18:05:17.5520205Z 
2019-09-09T18:05:17.5520234Z 
2019-09-09T18:05:17.5520282Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T18:05:17.5520335Z Build completed unsuccessfully in 1:08:43
2019-09-09T18:05:17.5520335Z Build completed unsuccessfully in 1:08:43
2019-09-09T18:05:17.5571567Z == clock drift check ==
2019-09-09T18:05:17.5588125Z   local time: Mon Sep  9 18:05:17 UTC 2019
2019-09-09T18:05:17.7146754Z   network time: Mon, 09 Sep 2019 18:05:17 GMT
2019-09-09T18:05:17.7149198Z == end clock drift check ==
2019-09-09T18:05:20.5808492Z ##[error]Bash exited with code '1'.
2019-09-09T18:05:20.5851626Z ##[section]Starting: Checkout
2019-09-09T18:05:20.5853289Z ==============================================================================
2019-09-09T18:05:20.5853356Z Task         : Get sources
2019-09-09T18:05:20.5853399Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
