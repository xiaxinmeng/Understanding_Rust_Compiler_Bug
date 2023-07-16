plain
2019-10-03T00:39:49.1068069Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T00:39:49.1257499Z ##[command]git config gc.auto 0
2019-10-03T00:39:49.1338061Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T00:39:49.1392436Z ##[command]git config --get-all http.proxy
2019-10-03T00:39:49.1612063Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64906/merge:refs/remotes/pull/64906/merge
---
2019-10-03T01:40:01.6216765Z .................................................................................................... 1500/9098
2019-10-03T01:40:08.3257113Z .................................................................................................... 1600/9098
2019-10-03T01:40:17.4219727Z .................................................................................................... 1700/9098
2019-10-03T01:40:27.0294659Z .........i..............i........................................................................... 1800/9098
2019-10-03T01:40:34.3829590Z ...................................................................................................i 1900/9098
2019-10-03T01:40:50.4219082Z iiii................................................................................................ 2000/9098
2019-10-03T01:40:59.2076544Z .................................................................................................... 2200/9098
2019-10-03T01:41:01.8001665Z .................................................................................................... 2300/9098
2019-10-03T01:41:07.9700024Z .................................................................................................... 2400/9098
2019-10-03T01:41:13.4819395Z .................................................................................................... 2500/9098
---
2019-10-03T01:44:06.4676365Z ......................................................................................i............. 4700/9098
2019-10-03T01:44:14.3516014Z ..i................................................................................................. 4800/9098
2019-10-03T01:44:24.3672219Z .................................................................................................... 4900/9098
2019-10-03T01:44:30.0627407Z .................................................................................................... 5000/9098
2019-10-03T01:44:41.8067147Z ..............................................................................ii.ii................. 5100/9098
2019-10-03T01:44:51.1397208Z .................................................................................................... 5300/9098
2019-10-03T01:45:00.7009668Z .................................................................................................... 5400/9098
2019-10-03T01:45:07.8618092Z ............................................i....................................................... 5500/9098
2019-10-03T01:45:15.0148712Z .................................................................................................... 5600/9098
2019-10-03T01:45:15.0148712Z .................................................................................................... 5600/9098
2019-10-03T01:45:25.8345769Z .................................................................................................... 5700/9098
2019-10-03T01:45:33.4307768Z .........................................ii...i..ii...........i..................................... 5800/9098
2019-10-03T01:45:58.6965443Z .................................................................................................... 6000/9098
2019-10-03T01:46:07.7812297Z .................................................................................................... 6100/9098
2019-10-03T01:46:07.7812297Z .................................................................................................... 6100/9098
2019-10-03T01:46:22.1992674Z ............................................i..ii................................................... 6200/9098
2019-10-03T01:46:44.3781802Z .................................................................................................... 6400/9098
2019-10-03T01:46:46.7438279Z ....i............................................................................................... 6500/9098
2019-10-03T01:46:49.0793734Z ............................................................................i....................... 6600/9098
2019-10-03T01:46:51.8526926Z .................................................................................................... 6700/9098
---
2019-10-03T01:50:55.4200841Z diff of stderr:
2019-10-03T01:50:55.4200871Z 
2019-10-03T01:50:55.4200924Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-10-03T01:50:55.4200989Z 9    |
2019-10-03T01:50:55.4201235Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-10-03T01:50:55.4201495Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-10-03T01:50:55.4201735Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-10-03T01:50:55.4201779Z 12 
2019-10-03T01:50:55.4202104Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-03T01:50:55.4202413Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-03T01:50:55.4202612Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-10-03T01:50:55.4202668Z 15    |
2019-10-03T01:50:55.4203728Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-10-03T01:50:55.4203799Z 
2019-10-03T01:50:55.4203867Z The actual stderr differed from the expected stderr.
2019-10-03T01:50:55.4204170Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-10-03T01:50:55.4204170Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-10-03T01:50:55.4204411Z To update references, rerun the tests and pass the `--bless` flag
2019-10-03T01:50:55.4204678Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-10-03T01:50:55.4204776Z error: 1 errors occurred comparing output.
2019-10-03T01:50:55.4204839Z status: exit code: 1
2019-10-03T01:50:55.4204839Z status: exit code: 1
2019-10-03T01:50:55.4205539Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-10-03T01:50:55.4206422Z ------------------------------------------
2019-10-03T01:50:55.4206474Z 
2019-10-03T01:50:55.4206886Z ------------------------------------------
2019-10-03T01:50:55.4206962Z stderr:
2019-10-03T01:50:55.4206962Z stderr:
2019-10-03T01:50:55.4207165Z ------------------------------------------
2019-10-03T01:50:55.4207389Z error[E0308]: cannot coerce intrinsics to function pointers
2019-10-03T01:50:55.4207682Z   --> /checkout/src/test/ui/reify-intrinsic.rs:6:64
2019-10-03T01:50:55.4207728Z    |
2019-10-03T01:50:55.4207955Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-10-03T01:50:55.4208080Z    |                                                                |
2019-10-03T01:50:55.4208132Z    |                                                                cannot coerce intrinsics to function pointers
2019-10-03T01:50:55.4208211Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-10-03T01:50:55.4208258Z    |
2019-10-03T01:50:55.4208258Z    |
2019-10-03T01:50:55.4208495Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-10-03T01:50:55.4208885Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-10-03T01:50:55.4208919Z 
2019-10-03T01:50:55.4209211Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-03T01:50:55.4209488Z    |
2019-10-03T01:50:55.4209488Z    |
2019-10-03T01:50:55.4209728Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-10-03T01:50:55.4209811Z 
2019-10-03T01:50:55.4209850Z error: aborting due to 2 previous errors
2019-10-03T01:50:55.4209894Z 
2019-10-03T01:50:55.4209935Z Some errors have detailed explanations: E0308, E0606.
---
2019-10-03T01:50:55.4243654Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-03T01:50:55.4243747Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-03T01:50:55.4260017Z 
2019-10-03T01:50:55.4260468Z 
2019-10-03T01:50:55.4263564Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-03T01:50:55.4265882Z 
2019-10-03T01:50:55.4266119Z 
2019-10-03T01:50:55.4275076Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-03T01:50:55.4275333Z Build completed unsuccessfully in 1:03:30
2019-10-03T01:50:55.4275333Z Build completed unsuccessfully in 1:03:30
2019-10-03T01:50:55.4332381Z == clock drift check ==
2019-10-03T01:50:55.4351187Z   local time: Thu Oct  3 01:50:55 UTC 2019
2019-10-03T01:50:55.5852823Z   network time: Thu, 03 Oct 2019 01:50:55 GMT
2019-10-03T01:50:55.5855301Z == end clock drift check ==
2019-10-03T01:50:56.6596454Z ##[error]Bash exited with code '1'.
2019-10-03T01:50:56.6640124Z ##[section]Starting: Checkout
2019-10-03T01:50:56.6642057Z ==============================================================================
2019-10-03T01:50:56.6642119Z Task         : Get sources
2019-10-03T01:50:56.6642157Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
