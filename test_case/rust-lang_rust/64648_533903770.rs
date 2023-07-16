plain
2019-09-22T16:46:06.7920243Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-22T16:46:06.8123174Z ##[command]git config gc.auto 0
2019-09-22T16:46:06.8200989Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-22T16:46:06.8291806Z ##[command]git config --get-all http.proxy
2019-09-22T16:46:06.8446557Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64648/merge:refs/remotes/pull/64648/merge
---
2019-09-22T17:51:37.0253512Z .................................................................................................... 1500/9036
2019-09-22T17:51:43.5218397Z .................................................................................................... 1600/9036
2019-09-22T17:51:56.4772421Z ........................................................................i...............i........... 1700/9036
2019-09-22T17:52:03.8271365Z .................................................................................................... 1800/9036
2019-09-22T17:52:13.1273440Z ...............................................................iiiii................................ 1900/9036
2019-09-22T17:52:33.6691948Z .................................................................................................... 2100/9036
2019-09-22T17:52:36.4761723Z .................................................................................................... 2200/9036
2019-09-22T17:52:40.1150835Z .................................................................................................... 2300/9036
2019-09-22T17:52:49.2263296Z .................................................................................................... 2400/9036
---
2019-09-22T17:55:57.9557479Z ....................................................i...............i............................... 4700/9036
2019-09-22T17:56:07.9496927Z .................................................................................................... 4800/9036
2019-09-22T17:56:16.7839514Z .................................................................................................... 4900/9036
2019-09-22T17:56:24.6962228Z .................................................................................................... 5000/9036
2019-09-22T17:56:34.7980427Z .......................................ii.ii........................................................ 5100/9036
2019-09-22T17:56:45.1126253Z .................................................................................................... 5300/9036
2019-09-22T17:56:56.2193956Z .................................................................................................... 5400/9036
2019-09-22T17:57:04.2131535Z ....i............................................................................................... 5500/9036
2019-09-22T17:57:09.6635442Z .................................................................................................... 5600/9036
2019-09-22T17:57:09.6635442Z .................................................................................................... 5600/9036
2019-09-22T17:57:22.0217784Z ...................................................................................................i 5700/9036
2019-09-22T17:57:35.6089041Z i...i...ii..........i............................................................................... 5800/9036
2019-09-22T17:57:58.1368304Z .................................................................................................... 6000/9036
2019-09-22T17:58:05.6709270Z .................................................................................................... 6100/9036
2019-09-22T17:58:05.6709270Z .................................................................................................... 6100/9036
2019-09-22T17:58:20.4191948Z .i..ii.............................................................................................. 6200/9036
2019-09-22T17:58:40.0825809Z ............................................................i....................................... 6400/9036
2019-09-22T17:58:42.4108041Z .................................................................................................... 6500/9036
2019-09-22T17:58:45.1423435Z ................................i................................................................... 6600/9036
2019-09-22T17:58:49.6308707Z .................................................................................................... 6700/9036
---
2019-09-22T18:03:07.1368834Z diff of stderr:
2019-09-22T18:03:07.1369135Z 
2019-09-22T18:03:07.1369419Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-22T18:03:07.1369699Z 9    |
2019-09-22T18:03:07.1370293Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-22T18:03:07.1371006Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-22T18:03:07.1371979Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-22T18:03:07.1372400Z 12 
2019-09-22T18:03:07.1373398Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-22T18:03:07.1375483Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-22T18:03:07.1377267Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-09-22T18:03:07.1378055Z 15    |
2019-09-22T18:03:07.1379444Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-22T18:03:07.1380097Z 
2019-09-22T18:03:07.1416710Z The actual stderr differed from the expected stderr.
2019-09-22T18:03:07.1418817Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-22T18:03:07.1418817Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-22T18:03:07.1420174Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T18:03:07.1421417Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-09-22T18:03:07.1423196Z error: 1 errors occurred comparing output.
2019-09-22T18:03:07.1424458Z status: exit code: 1
2019-09-22T18:03:07.1424458Z status: exit code: 1
2019-09-22T18:03:07.1426255Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-09-22T18:03:07.1428398Z ------------------------------------------
2019-09-22T18:03:07.1429287Z 
2019-09-22T18:03:07.1430395Z ------------------------------------------
2019-09-22T18:03:07.1438893Z stderr:
2019-09-22T18:03:07.1438893Z stderr:
2019-09-22T18:03:07.1439517Z ------------------------------------------
2019-09-22T18:03:07.1439752Z error[E0308]: cannot coerce intrinsics to function pointers
2019-09-22T18:03:07.1443221Z   --> /checkout/src/test/ui/reify-intrinsic.rs:6:64
2019-09-22T18:03:07.1444013Z    |
2019-09-22T18:03:07.1444597Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-09-22T18:03:07.1445330Z    |                                                                |
2019-09-22T18:03:07.1445539Z    |                                                                cannot coerce intrinsics to function pointers
2019-09-22T18:03:07.1445720Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-22T18:03:07.1445910Z    |
2019-09-22T18:03:07.1445910Z    |
2019-09-22T18:03:07.1446403Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-22T18:03:07.1449292Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-22T18:03:07.1449581Z 
2019-09-22T18:03:07.1450145Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-22T18:03:07.1452426Z    |
2019-09-22T18:03:07.1452426Z    |
2019-09-22T18:03:07.1452928Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-22T18:03:07.1453311Z 
2019-09-22T18:03:07.1453467Z error: aborting due to 2 previous errors
2019-09-22T18:03:07.1453680Z 
2019-09-22T18:03:07.1454365Z Some errors have detailed explanations: E0308, E0606.
---
2019-09-22T18:03:07.1458801Z test result: FAILED. 8997 passed; 1 failed; 38 ignored; 0 measured; 0 filtered out
2019-09-22T18:03:07.1458999Z 
2019-09-22T18:03:07.1459229Z 
2019-09-22T18:03:07.1459400Z 
2019-09-22T18:03:07.1461201Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-22T18:03:07.1461795Z 
2019-09-22T18:03:07.1461940Z 
2019-09-22T18:03:07.1462432Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-22T18:03:07.1462677Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-22T18:03:07.1462677Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-22T18:03:07.1464147Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-22T18:03:07.1464455Z Build completed unsuccessfully in 1:09:40
2019-09-22T18:03:07.1514741Z == clock drift check ==
2019-09-22T18:03:07.1529799Z   local time: Sun Sep 22 18:03:07 UTC 2019
2019-09-22T18:03:07.4355515Z   network time: Sun, 22 Sep 2019 18:03:07 GMT
2019-09-22T18:03:07.4356015Z == end clock drift check ==
2019-09-22T18:03:08.2878673Z ##[error]Bash exited with code '1'.
2019-09-22T18:03:08.2915783Z ##[section]Starting: Checkout
2019-09-22T18:03:08.2917757Z ==============================================================================
2019-09-22T18:03:08.2917808Z Task         : Get sources
2019-09-22T18:03:08.2917853Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
