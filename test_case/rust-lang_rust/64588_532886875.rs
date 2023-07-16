plain
2019-09-18T20:58:58.2206881Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-18T20:58:58.2384917Z ##[command]git config gc.auto 0
2019-09-18T20:58:58.2465784Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-18T20:58:58.2543303Z ##[command]git config --get-all http.proxy
2019-09-18T20:58:58.2683861Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64588/merge:refs/remotes/pull/64588/merge
---
2019-09-18T21:59:31.8557487Z .................................................................................................... 1500/9030
2019-09-18T21:59:37.6321557Z .................................................................................................... 1600/9030
2019-09-18T21:59:49.5001865Z ...................................................................i...............i................ 1700/9030
2019-09-18T21:59:56.1663037Z .................................................................................................... 1800/9030
2019-09-18T22:00:10.6050991Z ..........................................................iiiii..................................... 1900/9030
2019-09-18T22:00:21.7350141Z .................................................................................................... 2100/9030
2019-09-18T22:00:24.1806094Z .................................................................................................... 2200/9030
2019-09-18T22:00:27.3755876Z .................................................................................................... 2300/9030
2019-09-18T22:00:35.4036201Z .................................................................................................... 2400/9030
---
2019-09-18T22:03:38.2704331Z ..............................................i...............i..................................... 4700/9030
2019-09-18T22:03:49.0331077Z .................................................................................................... 4800/9030
2019-09-18T22:03:57.3522527Z .................................................................................................... 4900/9030
2019-09-18T22:04:06.8582592Z .................................................................................................... 5000/9030
2019-09-18T22:04:15.3679384Z ..............................ii.ii................................................................. 5100/9030
2019-09-18T22:04:25.7632906Z .................................................................................................... 5300/9030
2019-09-18T22:04:37.2534459Z ..............................................................................................i..... 5400/9030
2019-09-18T22:04:46.0264933Z .................................................................................................... 5500/9030
2019-09-18T22:04:51.1719657Z .................................................................................................... 5600/9030
2019-09-18T22:04:51.1719657Z .................................................................................................... 5600/9030
2019-09-18T22:05:02.4027651Z .........................................................................................ii...i..ii. 5700/9030
2019-09-18T22:05:29.6016174Z .................................................................................................... 5900/9030
2019-09-18T22:05:40.4288818Z .................................................................................................... 6000/9030
2019-09-18T22:05:40.4288818Z .................................................................................................... 6000/9030
2019-09-18T22:05:49.2800637Z ...........................................................................................i..ii.... 6100/9030
2019-09-18T22:06:19.0967741Z .................................................................................................... 6300/9030
2019-09-18T22:06:23.8216645Z ...................................................i................................................ 6400/9030
2019-09-18T22:06:26.2129941Z .................................................................................................... 6500/9030
2019-09-18T22:06:28.8756334Z .......................i............................................................................ 6600/9030
---
2019-09-18T22:10:44.9045767Z diff of stderr:
2019-09-18T22:10:44.9045982Z 
2019-09-18T22:10:44.9046470Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-18T22:10:44.9046927Z 9    |
2019-09-18T22:10:44.9047537Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-18T22:10:44.9048113Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-18T22:10:44.9048651Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-18T22:10:44.9048942Z 12 
2019-09-18T22:10:44.9049485Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-18T22:10:44.9051053Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-18T22:10:44.9051636Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-09-18T22:10:44.9051927Z 15    |
2019-09-18T22:10:44.9052479Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-18T22:10:44.9052967Z 
2019-09-18T22:10:44.9053225Z The actual stderr differed from the expected stderr.
2019-09-18T22:10:44.9053921Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-18T22:10:44.9053921Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-18T22:10:44.9054408Z To update references, rerun the tests and pass the `--bless` flag
2019-09-18T22:10:44.9054909Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-09-18T22:10:44.9055516Z error: 1 errors occurred comparing output.
2019-09-18T22:10:44.9055736Z status: exit code: 1
2019-09-18T22:10:44.9055736Z status: exit code: 1
2019-09-18T22:10:44.9056551Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-09-18T22:10:44.9057270Z ------------------------------------------
2019-09-18T22:10:44.9057519Z 
2019-09-18T22:10:44.9057944Z ------------------------------------------
2019-09-18T22:10:44.9058190Z stderr:
2019-09-18T22:10:44.9058190Z stderr:
2019-09-18T22:10:44.9058578Z ------------------------------------------
2019-09-18T22:10:44.9058856Z error[E0308]: cannot coerce intrinsics to function pointers
2019-09-18T22:10:44.9059268Z   --> /checkout/src/test/ui/reify-intrinsic.rs:6:64
2019-09-18T22:10:44.9059542Z    |
2019-09-18T22:10:44.9060403Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-09-18T22:10:44.9062669Z    |                                                                |
2019-09-18T22:10:44.9062856Z    |                                                                cannot coerce intrinsics to function pointers
2019-09-18T22:10:44.9063031Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-18T22:10:44.9063459Z    |
2019-09-18T22:10:44.9063459Z    |
2019-09-18T22:10:44.9064111Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-18T22:10:44.9064537Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-18T22:10:44.9064695Z 
2019-09-18T22:10:44.9065102Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-18T22:10:44.9065670Z    |
2019-09-18T22:10:44.9065670Z    |
2019-09-18T22:10:44.9066162Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-18T22:10:44.9066532Z 
2019-09-18T22:10:44.9066665Z error: aborting due to 2 previous errors
2019-09-18T22:10:44.9066814Z 
2019-09-18T22:10:44.9066951Z Some errors have detailed explanations: E0308, E0606.
---
2019-09-18T22:10:44.9092676Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-18T22:10:44.9093021Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-18T22:10:44.9111310Z 
2019-09-18T22:10:44.9111615Z 
2019-09-18T22:10:44.9116596Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-18T22:10:44.9117563Z 
2019-09-18T22:10:44.9117870Z 
2019-09-18T22:10:44.9121218Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-18T22:10:44.9122204Z Build completed unsuccessfully in 1:04:41
2019-09-18T22:10:44.9122204Z Build completed unsuccessfully in 1:04:41
2019-09-18T22:10:44.9177694Z == clock drift check ==
2019-09-18T22:10:44.9200908Z   local time: Wed Sep 18 22:10:44 UTC 2019
2019-09-18T22:10:45.0738430Z   network time: Wed, 18 Sep 2019 22:10:45 GMT
2019-09-18T22:10:45.0739498Z == end clock drift check ==
2019-09-18T22:10:46.0054231Z ##[error]Bash exited with code '1'.
2019-09-18T22:10:46.0127749Z ##[section]Starting: Checkout
2019-09-18T22:10:46.0129596Z ==============================================================================
2019-09-18T22:10:46.0129809Z Task         : Get sources
2019-09-18T22:10:46.0129851Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
