plain
2019-09-09T07:59:47.7929501Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T07:59:47.8136967Z ##[command]git config gc.auto 0
2019-09-09T07:59:47.8222859Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T07:59:47.8287059Z ##[command]git config --get-all http.proxy
2019-09-09T07:59:47.8435405Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64303/merge:refs/remotes/pull/64303/merge
---
2019-09-09T09:05:16.2569712Z .................................................................................................... 1500/9009
2019-09-09T09:05:21.9695349Z .................................................................................................... 1600/9009
2019-09-09T09:05:35.1023325Z ......................................................i...............i............................. 1700/9009
2019-09-09T09:05:43.4361938Z .................................................................................................... 1800/9009
2019-09-09T09:05:58.4886515Z .............................................iiiii.................................................. 1900/9009
2019-09-09T09:06:09.9672688Z .................................................................................................... 2100/9009
2019-09-09T09:06:12.6632661Z .................................................................................................... 2200/9009
2019-09-09T09:06:16.4389938Z .................................................................................................... 2300/9009
2019-09-09T09:06:24.7639969Z .................................................................................................... 2400/9009
---
2019-09-09T09:09:31.2082113Z ...................................i...............i................................................ 4700/9009
2019-09-09T09:09:43.3809189Z .................................................................................................... 4800/9009
2019-09-09T09:09:50.1798028Z .................................................................................................... 4900/9009
2019-09-09T09:10:01.4660686Z .................................................................................................... 5000/9009
2019-09-09T09:10:07.8443237Z .................ii.ii.............................................................................. 5100/9009
2019-09-09T09:10:18.9440435Z .................................................................................................... 5300/9009
2019-09-09T09:10:29.4941339Z ................................................................................i................... 5400/9009
2019-09-09T09:10:37.5735658Z .................................................................................................... 5500/9009
2019-09-09T09:10:43.6959917Z .................................................................................................... 5600/9009
2019-09-09T09:10:43.6959917Z .................................................................................................... 5600/9009
2019-09-09T09:10:54.7684189Z ..........................................................................ii...i..ii...........i.... 5700/9009
2019-09-09T09:11:21.2311709Z .................................................................................................... 5900/9009
2019-09-09T09:11:29.5143969Z .................................................................................................... 6000/9009
2019-09-09T09:11:29.5143969Z .................................................................................................... 6000/9009
2019-09-09T09:11:35.1004982Z ............................................................................i..ii................... 6100/9009
2019-09-09T09:12:06.0851247Z .................................................................................................... 6300/9009
2019-09-09T09:12:08.3258973Z ...................................i................................................................ 6400/9009
2019-09-09T09:12:10.6202855Z .................................................................................................... 6500/9009
2019-09-09T09:12:13.3155327Z .......i............................................................................................ 6600/9009
---
2019-09-09T09:16:25.4522821Z diff of stderr:
2019-09-09T09:16:25.4522921Z 
2019-09-09T09:16:25.4523083Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-09T09:16:25.4523213Z 9    |
2019-09-09T09:16:25.4523561Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-09T09:16:25.4524140Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-09T09:16:25.4524490Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-09T09:16:25.4524670Z 12 
2019-09-09T09:16:25.4525044Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-09T09:16:25.4525444Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-09T09:16:25.4525790Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-09-09T09:16:25.4525928Z 15    |
2019-09-09T09:16:25.4526252Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-09T09:16:25.4526475Z 
2019-09-09T09:16:25.4526585Z The actual stderr differed from the expected stderr.
2019-09-09T09:16:25.4526947Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-09T09:16:25.4526947Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-09T09:16:25.4527267Z To update references, rerun the tests and pass the `--bless` flag
2019-09-09T09:16:25.4527831Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-09-09T09:16:25.4528614Z error: 1 errors occurred comparing output.
2019-09-09T09:16:25.4528758Z status: exit code: 1
2019-09-09T09:16:25.4528758Z status: exit code: 1
2019-09-09T09:16:25.4529695Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-09-09T09:16:25.4530331Z ------------------------------------------
2019-09-09T09:16:25.4530489Z 
2019-09-09T09:16:25.4530843Z ------------------------------------------
2019-09-09T09:16:25.4531043Z stderr:
2019-09-09T09:16:25.4531043Z stderr:
2019-09-09T09:16:25.4531387Z ------------------------------------------
2019-09-09T09:16:25.4531570Z error[E0308]: cannot coerce intrinsics to function pointers
2019-09-09T09:16:25.4532229Z   --> /checkout/src/test/ui/reify-intrinsic.rs:6:64
2019-09-09T09:16:25.4532398Z    |
2019-09-09T09:16:25.4533033Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-09-09T09:16:25.4533324Z    |                                                                |
2019-09-09T09:16:25.4533463Z    |                                                                cannot coerce intrinsics to function pointers
2019-09-09T09:16:25.4533720Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-09T09:16:25.4533887Z    |
2019-09-09T09:16:25.4533887Z    |
2019-09-09T09:16:25.4534231Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-09T09:16:25.4534614Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-09T09:16:25.4534770Z 
2019-09-09T09:16:25.4535320Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-09T09:16:25.4535798Z    |
2019-09-09T09:16:25.4535798Z    |
2019-09-09T09:16:25.4536120Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-09T09:16:25.4536393Z 
2019-09-09T09:16:25.4536511Z error: aborting due to 2 previous errors
2019-09-09T09:16:25.4536606Z 
2019-09-09T09:16:25.4536714Z Some errors have detailed explanations: E0308, E0606.
---
2019-09-09T09:16:25.4558643Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-09T09:16:25.4559025Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-09T09:16:25.4574872Z 
2019-09-09T09:16:25.4581306Z 
2019-09-09T09:16:25.4583464Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-09T09:16:25.4583757Z 
2019-09-09T09:16:25.4583783Z 
2019-09-09T09:16:25.4592516Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T09:16:25.4592755Z Build completed unsuccessfully in 1:09:26
2019-09-09T09:16:25.4592755Z Build completed unsuccessfully in 1:09:26
2019-09-09T09:16:25.4645722Z == clock drift check ==
2019-09-09T09:16:25.4663323Z   local time: Mon Sep  9 09:16:25 UTC 2019
2019-09-09T09:16:25.7936289Z   network time: Mon, 09 Sep 2019 09:16:25 GMT
2019-09-09T09:16:25.7936406Z == end clock drift check ==
2019-09-09T09:16:26.5119411Z ##[error]Bash exited with code '1'.
2019-09-09T09:16:26.5157244Z ##[section]Starting: Checkout
2019-09-09T09:16:26.5159665Z ==============================================================================
2019-09-09T09:16:26.5159743Z Task         : Get sources
2019-09-09T09:16:26.5159791Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
