plain
2019-10-03T13:47:00.7871066Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T13:47:00.8050909Z ##[command]git config gc.auto 0
2019-10-03T13:47:00.8128008Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T13:47:00.8193653Z ##[command]git config --get-all http.proxy
2019-10-03T13:47:00.8327881Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64582/merge:refs/remotes/pull/64582/merge
---
2019-10-03T14:47:48.9150847Z .................................................................................................... 1500/9095
2019-10-03T14:47:55.7736201Z .................................................................................................... 1600/9095
2019-10-03T14:48:05.2518903Z .................................................................................................... 1700/9095
2019-10-03T14:48:14.0193081Z ...i...............i................................................................................ 1800/9095
2019-10-03T14:48:21.0200991Z ..............................................................................................iiiii. 1900/9095
2019-10-03T14:48:43.5370938Z .................................................................................................... 2100/9095
2019-10-03T14:48:45.9416026Z .................................................................................................... 2200/9095
2019-10-03T14:48:48.5183855Z .................................................................................................... 2300/9095
2019-10-03T14:48:55.0018694Z .................................................................................................... 2400/9095
---
2019-10-03T14:51:53.4429343Z ..................................................................................i..............i.. 4700/9095
2019-10-03T14:52:01.6909842Z .................................................................................................... 4800/9095
2019-10-03T14:52:11.9987050Z .................................................................................................... 4900/9095
2019-10-03T14:52:17.9714464Z .................................................................................................... 5000/9095
2019-10-03T14:52:29.4722023Z .........................................................................ii.ii...................... 5100/9095
2019-10-03T14:52:39.2887376Z .................................................................................................... 5300/9095
2019-10-03T14:52:48.9851716Z .................................................................................................... 5400/9095
2019-10-03T14:52:56.3064144Z .......................................i............................................................ 5500/9095
2019-10-03T14:53:02.8095210Z .................................................................................................... 5600/9095
2019-10-03T14:53:02.8095210Z .................................................................................................... 5600/9095
2019-10-03T14:53:13.8924010Z .................................................................................................... 5700/9095
2019-10-03T14:53:24.7673739Z ....................................ii...i..ii...........i.......................................... 5800/9095
2019-10-03T14:53:47.0572494Z .................................................................................................... 6000/9095
2019-10-03T14:53:54.5233894Z .................................................................................................... 6100/9095
2019-10-03T14:53:54.5233894Z .................................................................................................... 6100/9095
2019-10-03T14:54:07.5782015Z .........................................i..ii...................................................... 6200/9095
2019-10-03T14:54:29.0396347Z .................................................................................................... 6400/9095
2019-10-03T14:54:31.1871830Z .i.................................................................................................. 6500/9095
2019-10-03T14:54:33.4387366Z .........................................................................i.......................... 6600/9095
2019-10-03T14:54:36.4172389Z .................................................................................................... 6700/9095
---
2019-10-03T14:58:44.1389355Z diff of stderr:
2019-10-03T14:58:44.1389707Z 
2019-10-03T14:58:44.1389941Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-10-03T14:58:44.1390137Z 9    |
2019-10-03T14:58:44.1390625Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-10-03T14:58:44.1391113Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-10-03T14:58:44.1391905Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-10-03T14:58:44.1392082Z 12 
2019-10-03T14:58:44.1392546Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-03T14:58:44.1393450Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-03T14:58:44.1394291Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-10-03T14:58:44.1394475Z 15    |
2019-10-03T14:58:44.1394895Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-10-03T14:58:44.1395371Z 
2019-10-03T14:58:44.1395514Z The actual stderr differed from the expected stderr.
2019-10-03T14:58:44.1395925Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-10-03T14:58:44.1395925Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-10-03T14:58:44.1396671Z To update references, rerun the tests and pass the `--bless` flag
2019-10-03T14:58:44.1397215Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-10-03T14:58:44.1398162Z error: 1 errors occurred comparing output.
2019-10-03T14:58:44.1398319Z status: exit code: 1
2019-10-03T14:58:44.1398319Z status: exit code: 1
2019-10-03T14:58:44.1400176Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-10-03T14:58:44.1400946Z ------------------------------------------
2019-10-03T14:58:44.1401128Z 
2019-10-03T14:58:44.1401639Z ------------------------------------------
2019-10-03T14:58:44.1401818Z stderr:
2019-10-03T14:58:44.1401818Z stderr:
2019-10-03T14:58:44.1402120Z ------------------------------------------
2019-10-03T14:58:44.1402299Z error[E0308]: cannot coerce intrinsics to function pointers
2019-10-03T14:58:44.1402649Z   --> /checkout/src/test/ui/reify-intrinsic.rs:6:64
2019-10-03T14:58:44.1402811Z    |
2019-10-03T14:58:44.1403607Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-10-03T14:58:44.1403961Z    |                                                                |
2019-10-03T14:58:44.1404116Z    |                                                                cannot coerce intrinsics to function pointers
2019-10-03T14:58:44.1404253Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-10-03T14:58:44.1404397Z    |
2019-10-03T14:58:44.1404397Z    |
2019-10-03T14:58:44.1404731Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-10-03T14:58:44.1405305Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-10-03T14:58:44.1405486Z 
2019-10-03T14:58:44.1405973Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-03T14:58:44.1406563Z    |
2019-10-03T14:58:44.1406563Z    |
2019-10-03T14:58:44.1406899Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-10-03T14:58:44.1407377Z 
2019-10-03T14:58:44.1407860Z error: aborting due to 2 previous errors
2019-10-03T14:58:44.1408084Z 
2019-10-03T14:58:44.1408248Z Some errors have detailed explanations: E0308, E0606.
---
2019-10-03T14:58:44.1430630Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-03T14:58:44.1430731Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-03T14:58:44.1445787Z 
2019-10-03T14:58:44.1445881Z 
2019-10-03T14:58:44.1447617Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-03T14:58:44.1448610Z 
2019-10-03T14:58:44.1448642Z 
2019-10-03T14:58:44.1458296Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-03T14:58:44.1458385Z Build completed unsuccessfully in 1:04:34
2019-10-03T14:58:44.1458385Z Build completed unsuccessfully in 1:04:34
2019-10-03T14:58:44.1525742Z == clock drift check ==
2019-10-03T14:58:44.1536624Z   local time: Thu Oct  3 14:58:44 UTC 2019
2019-10-03T14:58:44.2256948Z   network time: Thu, 03 Oct 2019 14:58:44 GMT
2019-10-03T14:58:44.2257012Z == end clock drift check ==
2019-10-03T14:58:45.2304970Z ##[error]Bash exited with code '1'.
2019-10-03T14:58:45.2344007Z ##[section]Starting: Checkout
2019-10-03T14:58:45.2345501Z ==============================================================================
2019-10-03T14:58:45.2345563Z Task         : Get sources
2019-10-03T14:58:45.2345601Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
