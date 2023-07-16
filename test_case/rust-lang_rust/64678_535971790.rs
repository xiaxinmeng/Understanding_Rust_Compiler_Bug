plain
2019-09-27T13:41:56.4376100Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T13:41:56.4600836Z ##[command]git config gc.auto 0
2019-09-27T13:41:56.4666118Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T13:41:56.4716458Z ##[command]git config --get-all http.proxy
2019-09-27T13:41:57.1552969Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64678/merge:refs/remotes/pull/64678/merge
---
2019-09-27T14:38:59.7840595Z .................................................................................................... 1500/9046
2019-09-27T14:39:05.1507249Z .................................................................................................... 1600/9046
2019-09-27T14:39:16.6546947Z .........................................................................i...............i.......... 1700/9046
2019-09-27T14:39:22.8108079Z .................................................................................................... 1800/9046
2019-09-27T14:39:30.3224050Z ................................................................iiiii............................... 1900/9046
2019-09-27T14:39:48.0472160Z .................................................................................................... 2100/9046
2019-09-27T14:39:50.1887258Z .................................................................................................... 2200/9046
2019-09-27T14:39:53.0547110Z .................................................................................................... 2300/9046
2019-09-27T14:40:00.5574653Z .................................................................................................... 2400/9046
---
2019-09-27T14:42:40.4372780Z ....................................................i...............i............................... 4700/9046
2019-09-27T14:42:48.7630217Z .................................................................................................... 4800/9046
2019-09-27T14:42:56.3446587Z .................................................................................................... 4900/9046
2019-09-27T14:43:03.0566922Z .................................................................................................... 5000/9046
2019-09-27T14:43:11.6239736Z ........................................ii.ii....................................................... 5100/9046
2019-09-27T14:43:20.4331840Z .................................................................................................... 5300/9046
2019-09-27T14:43:29.8978807Z .................................................................................................... 5400/9046
2019-09-27T14:43:36.7012499Z .....i.............................................................................................. 5500/9046
2019-09-27T14:43:41.3162913Z .................................................................................................... 5600/9046
2019-09-27T14:43:41.3162913Z .................................................................................................... 5600/9046
2019-09-27T14:43:52.1387625Z .................................................................................................... 5700/9046
2019-09-27T14:44:03.8692633Z ii...i..ii...........i.............................................................................. 5800/9046
2019-09-27T14:44:22.8484848Z .................................................................................................... 6000/9046
2019-09-27T14:44:28.3338627Z .................................................................................................... 6100/9046
2019-09-27T14:44:28.3338627Z .................................................................................................... 6100/9046
2019-09-27T14:44:40.7745376Z ...i..ii............................................................................................ 6200/9046
2019-09-27T14:44:49.3838639Z .................................................................................................F.. 6300/9046
2019-09-27T14:44:59.3607434Z .................................................................................................... 6500/9046
2019-09-27T14:45:01.5324690Z ...................................i................................................................ 6600/9046
2019-09-27T14:45:05.1314974Z .................................................................................................... 6700/9046
2019-09-27T14:45:18.2467829Z .................................................................................................... 6800/9046
---
2019-09-27T14:48:41.2381561Z 
2019-09-27T14:48:41.2381673Z 7 LL | | }
2019-09-27T14:48:41.2381803Z 8    | |_^
2019-09-27T14:48:41.2381912Z 9    |
2019-09-27T14:48:41.2382239Z -    = note: first defined in crate `std`.
2019-09-27T14:48:41.2382399Z +    = note: first defined in crate `std` (which `duplicate_entry_error` depends on).
2019-09-27T14:48:41.2382676Z 12 error: aborting due to previous error
2019-09-27T14:48:41.2382791Z 13 
2019-09-27T14:48:41.2382886Z 
2019-09-27T14:48:41.2382980Z 
2019-09-27T14:48:41.2382980Z 
2019-09-27T14:48:41.2383114Z The actual stderr differed from the expected stderr.
2019-09-27T14:48:41.2383484Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error/duplicate_entry_error.stderr
2019-09-27T14:48:41.2383864Z To update references, rerun the tests and pass the `--bless` flag
2019-09-27T14:48:41.2384219Z To only update this specific test, also pass `--test-args duplicate_entry_error.rs`
2019-09-27T14:48:41.2384501Z error: 1 errors occurred comparing output.
2019-09-27T14:48:41.2384614Z status: exit code: 1
2019-09-27T14:48:41.2384614Z status: exit code: 1
2019-09-27T14:48:41.2385293Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate_entry_error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error/auxiliary" "-A" "unused"
2019-09-27T14:48:41.2385987Z ------------------------------------------
2019-09-27T14:48:41.2386298Z 
2019-09-27T14:48:41.2387368Z ------------------------------------------
2019-09-27T14:48:41.2387600Z stderr:
2019-09-27T14:48:41.2387600Z stderr:
2019-09-27T14:48:41.2387973Z ------------------------------------------
2019-09-27T14:48:41.2388162Z error[E0152]: duplicate lang item found: `panic_impl`.
2019-09-27T14:48:41.2388737Z    |
2019-09-27T14:48:41.2388737Z    |
2019-09-27T14:48:41.2389096Z LL | / fn panic_impl(info: &PanicInfo) -> ! {
2019-09-27T14:48:41.2389304Z LL | | //~^ ERROR: duplicate lang item found: `panic_impl`.
2019-09-27T14:48:41.2389472Z LL | |     loop {}
2019-09-27T14:48:41.2389767Z    | |_^
2019-09-27T14:48:41.2389903Z    |
2019-09-27T14:48:41.2389903Z    |
2019-09-27T14:48:41.2390217Z    = note: first defined in crate `std` (which `duplicate_entry_error` depends on).
2019-09-27T14:48:41.2390452Z error: aborting due to previous error
2019-09-27T14:48:41.2390545Z 
2019-09-27T14:48:41.2390869Z For more information about this error, try `rustc --explain E0152`.
2019-09-27T14:48:41.2391003Z 
---
2019-09-27T14:48:41.2392547Z 
2019-09-27T14:48:41.2392654Z 6 LL | | }
2019-09-27T14:48:41.2392928Z 7    | |_^
2019-09-27T14:48:41.2393050Z 8    |
2019-09-27T14:48:41.2393321Z -    = note: first defined in crate `std`.
2019-09-27T14:48:41.2393639Z +    = note: first defined in crate `std` (which `panic_handler_std` depends on).
2019-09-27T14:48:41.2393882Z 11 error: argument should be `&PanicInfo`
2019-09-27T14:48:41.2394181Z 12   --> $DIR/panic-handler-std.rs:7:16
2019-09-27T14:48:41.2395372Z 
2019-09-27T14:48:41.2395410Z 
2019-09-27T14:48:41.2395410Z 
2019-09-27T14:48:41.2395448Z The actual stderr differed from the expected stderr.
2019-09-27T14:48:41.2395879Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std/panic-handler-std.stderr
2019-09-27T14:48:41.2396291Z To update references, rerun the tests and pass the `--bless` flag
2019-09-27T14:48:41.2396961Z To only update this specific test, also pass `--test-args panic-handler/panic-handler-std.rs`
2019-09-27T14:48:41.2397067Z error: 1 errors occurred comparing output.
2019-09-27T14:48:41.2397113Z status: exit code: 1
2019-09-27T14:48:41.2397113Z status: exit code: 1
2019-09-27T14:48:41.2397892Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/panic-handler-std.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std/auxiliary" "-A" "unused"
2019-09-27T14:48:41.2398238Z ------------------------------------------
2019-09-27T14:48:41.2398271Z 
2019-09-27T14:48:41.2398484Z ------------------------------------------
2019-09-27T14:48:41.2398547Z stderr:
2019-09-27T14:48:41.2398547Z stderr:
2019-09-27T14:48:41.2398756Z ------------------------------------------
2019-09-27T14:48:41.2398807Z error[E0152]: duplicate lang item found: `panic_impl`.
2019-09-27T14:48:41.2399128Z    |
2019-09-27T14:48:41.2399128Z    |
2019-09-27T14:48:41.2399349Z LL | / fn panic(info: PanicInfo) -> ! {
2019-09-27T14:48:41.2399396Z LL | |     loop {}
2019-09-27T14:48:41.2399496Z    | |_^
2019-09-27T14:48:41.2399536Z    |
2019-09-27T14:48:41.2399536Z    |
2019-09-27T14:48:41.2399604Z    = note: first defined in crate `std` (which `panic_handler_std` depends on).
2019-09-27T14:48:41.2399682Z error: argument should be `&PanicInfo`
2019-09-27T14:48:41.2400095Z   --> /checkout/src/test/ui/panic-handler/panic-handler-std.rs:7:16
2019-09-27T14:48:41.2400150Z    |
2019-09-27T14:48:41.2400150Z    |
2019-09-27T14:48:41.2400316Z LL | fn panic(info: PanicInfo) -> ! {
2019-09-27T14:48:41.2400376Z 
2019-09-27T14:48:41.2400426Z error: aborting due to 2 previous errors
2019-09-27T14:48:41.2400448Z 
2019-09-27T14:48:41.2400631Z For more information about this error, try `rustc --explain E0152`.
---
2019-09-27T14:48:41.2413873Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-27T14:48:41.2413960Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-27T14:48:41.2430899Z 
2019-09-27T14:48:41.2430996Z 
2019-09-27T14:48:41.2432845Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-27T14:48:41.2433147Z 
2019-09-27T14:48:41.2433171Z 
2019-09-27T14:48:41.2440482Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-27T14:48:41.2440721Z Build completed unsuccessfully in 0:59:55
2019-09-27T14:48:41.2440721Z Build completed unsuccessfully in 0:59:55
2019-09-27T14:48:41.2484300Z == clock drift check ==
2019-09-27T14:48:41.2501321Z   local time: Fri Sep 27 14:48:41 UTC 2019
2019-09-27T14:48:41.3613672Z   network time: Fri, 27 Sep 2019 14:48:41 GMT
2019-09-27T14:48:41.3618156Z == end clock drift check ==
2019-09-27T14:48:42.9095458Z ##[error]Bash exited with code '1'.
2019-09-27T14:48:42.9135158Z ##[section]Starting: Checkout
2019-09-27T14:48:42.9137508Z ==============================================================================
2019-09-27T14:48:42.9137584Z Task         : Get sources
2019-09-27T14:48:42.9137631Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
