plain
2019-10-03T15:57:47.3757571Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T15:57:48.3559760Z ##[command]git config gc.auto 0
2019-10-03T15:57:48.3565359Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T15:57:48.3570368Z ##[command]git config --get-all http.proxy
2019-10-03T15:57:48.3574150Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64582/merge:refs/remotes/pull/64582/merge
---
2019-10-03T16:56:50.6760356Z .................................................................................................... 1500/9095
2019-10-03T16:56:57.2808378Z .................................................................................................... 1600/9095
2019-10-03T16:57:06.4970242Z .................................................................................................... 1700/9095
2019-10-03T16:57:15.2361356Z ...i...............i................................................................................ 1800/9095
2019-10-03T16:57:22.1828059Z ..............................................................................................iiiii. 1900/9095
2019-10-03T16:57:44.1234505Z .................................................................................................... 2100/9095
2019-10-03T16:57:46.5140339Z .................................................................................................... 2200/9095
2019-10-03T16:57:49.0877971Z .................................................................................................... 2300/9095
2019-10-03T16:57:55.3455746Z .................................................................................................... 2400/9095
---
2019-10-03T17:00:51.1216948Z .................................................................................i...............i.. 4700/9095
2019-10-03T17:00:59.6198236Z .................................................................................................... 4800/9095
2019-10-03T17:01:09.9232605Z .................................................................................................... 4900/9095
2019-10-03T17:01:16.0175869Z .................................................................................................... 5000/9095
2019-10-03T17:01:27.2948412Z .........................................................................ii.ii...................... 5100/9095
2019-10-03T17:01:36.9692297Z .................................................................................................... 5300/9095
2019-10-03T17:01:46.6271425Z .................................................................................................... 5400/9095
2019-10-03T17:01:53.8456462Z .......................................i............................................................ 5500/9095
2019-10-03T17:02:00.3433535Z .................................................................................................... 5600/9095
2019-10-03T17:02:00.3433535Z .................................................................................................... 5600/9095
2019-10-03T17:02:11.5181035Z .................................................................................................... 5700/9095
2019-10-03T17:02:22.3817048Z ....................................ii...i..ii...........i.......................................... 5800/9095
2019-10-03T17:02:44.4333382Z .................................................................................................... 6000/9095
2019-10-03T17:02:53.7558703Z .................................................................................................... 6100/9095
2019-10-03T17:02:53.7558703Z .................................................................................................... 6100/9095
2019-10-03T17:03:08.2643435Z .........................................i..ii...................................................... 6200/9095
2019-10-03T17:03:29.5660383Z .................................................................................................... 6400/9095
2019-10-03T17:03:31.8017693Z .i.................................................................................................. 6500/9095
2019-10-03T17:03:34.1199053Z .........................................................................i.......................... 6600/9095
2019-10-03T17:03:37.1623323Z .................................................................................................... 6700/9095
---
2019-10-03T17:07:44.8723893Z diff of stderr:
2019-10-03T17:07:44.8723922Z 
2019-10-03T17:07:44.8723968Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-10-03T17:07:44.8724013Z 9    |
2019-10-03T17:07:44.8724260Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-10-03T17:07:44.8724508Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-10-03T17:07:44.8724747Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-10-03T17:07:44.8724789Z 12 
2019-10-03T17:07:44.8725074Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-03T17:07:44.8726567Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-03T17:07:44.8726814Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-10-03T17:07:44.8726862Z 15    |
2019-10-03T17:07:44.8727142Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-10-03T17:07:44.8727207Z 
2019-10-03T17:07:44.8727265Z The actual stderr differed from the expected stderr.
2019-10-03T17:07:44.8727579Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-10-03T17:07:44.8727579Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-10-03T17:07:44.8727825Z To update references, rerun the tests and pass the `--bless` flag
2019-10-03T17:07:44.8728121Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-10-03T17:07:44.8728227Z error: 1 errors occurred comparing output.
2019-10-03T17:07:44.8728276Z status: exit code: 1
2019-10-03T17:07:44.8728276Z status: exit code: 1
2019-10-03T17:07:44.8729272Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-10-03T17:07:44.8729718Z ------------------------------------------
2019-10-03T17:07:44.8729745Z 
2019-10-03T17:07:44.8729916Z ------------------------------------------
2019-10-03T17:07:44.8729952Z stderr:
2019-10-03T17:07:44.8729952Z stderr:
2019-10-03T17:07:44.8730135Z ------------------------------------------
2019-10-03T17:07:44.8730176Z error[E0308]: cannot coerce intrinsics to function pointers
2019-10-03T17:07:44.8730363Z   --> /checkout/src/test/ui/reify-intrinsic.rs:6:64
2019-10-03T17:07:44.8730418Z    |
2019-10-03T17:07:44.8730615Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-10-03T17:07:44.8730721Z    |                                                                |
2019-10-03T17:07:44.8730765Z    |                                                                cannot coerce intrinsics to function pointers
2019-10-03T17:07:44.8730949Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-10-03T17:07:44.8731008Z    |
2019-10-03T17:07:44.8731008Z    |
2019-10-03T17:07:44.8731293Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-10-03T17:07:44.8731543Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-10-03T17:07:44.8731574Z 
2019-10-03T17:07:44.8731829Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-03T17:07:44.8732074Z    |
2019-10-03T17:07:44.8732074Z    |
2019-10-03T17:07:44.8732276Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-10-03T17:07:44.8732371Z 
2019-10-03T17:07:44.8732405Z error: aborting due to 2 previous errors
2019-10-03T17:07:44.8732428Z 
2019-10-03T17:07:44.8732479Z Some errors have detailed explanations: E0308, E0606.
---
2019-10-03T17:07:44.8761921Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-03T17:07:44.8762010Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-03T17:07:44.8778278Z 
2019-10-03T17:07:44.8778376Z 
2019-10-03T17:07:44.8780424Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-03T17:07:44.8780905Z 
2019-10-03T17:07:44.8780930Z 
2019-10-03T17:07:44.8787542Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-03T17:07:44.8787955Z Build completed unsuccessfully in 1:02:57
2019-10-03T17:07:44.8787955Z Build completed unsuccessfully in 1:02:57
2019-10-03T17:07:44.8839477Z == clock drift check ==
2019-10-03T17:07:44.8852995Z   local time: Thu Oct  3 17:07:44 UTC 2019
2019-10-03T17:07:45.0496626Z   network time: Thu, 03 Oct 2019 17:07:45 GMT
2019-10-03T17:07:45.0502762Z == end clock drift check ==
2019-10-03T17:07:46.2018817Z ##[error]Bash exited with code '1'.
2019-10-03T17:07:46.2065418Z ##[section]Starting: Checkout
2019-10-03T17:07:46.2067265Z ==============================================================================
2019-10-03T17:07:46.2067325Z Task         : Get sources
2019-10-03T17:07:46.2067395Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
