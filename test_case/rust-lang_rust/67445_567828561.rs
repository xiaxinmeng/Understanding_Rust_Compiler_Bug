plain
2019-12-20T06:48:19.3450265Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-20T06:48:19.3461481Z ##[command]git config gc.auto 0
2019-12-20T06:48:19.3466885Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-20T06:48:19.3470746Z ##[command]git config --get-all http.proxy
2019-12-20T06:48:19.3478082Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67445/merge:refs/remotes/pull/67445/merge
---
2019-12-20T07:40:47.2053321Z .................................................................................................... 1600/9380
2019-12-20T07:40:50.9736684Z .................................................................................................... 1700/9380
2019-12-20T07:41:01.8828096Z ...................................................................i................................ 1800/9380
2019-12-20T07:41:08.6053491Z .................................................................................................... 1900/9380
2019-12-20T07:41:22.2217581Z ....................................................iiiii........................................... 2000/9380
2019-12-20T07:41:31.7281124Z .................................................................................................... 2200/9380
2019-12-20T07:41:33.9747421Z .................................................................................................... 2300/9380
2019-12-20T07:41:36.9953071Z .................................................................................................... 2400/9380
2019-12-20T07:41:57.3997502Z .................................................................................................... 2500/9380
---
2019-12-20T07:44:12.7199724Z .............................................................i...............i...................... 4800/9380
2019-12-20T07:44:19.1978801Z .................................................................................................... 4900/9380
2019-12-20T07:44:26.4845957Z .................................................................................................... 5000/9380
2019-12-20T07:44:30.8742116Z .....i.............................................................................................. 5100/9380
2019-12-20T07:44:39.8137444Z .......................................................................ii.ii...........i............ 5200/9380
2019-12-20T07:44:47.1810158Z .......i............................................................................................ 5400/9380
2019-12-20T07:44:55.7448600Z .................................................................................................... 5500/9380
2019-12-20T07:45:01.4870547Z .....................................................i.............................................. 5600/9380
2019-12-20T07:45:07.4884740Z .................................................................................................... 5700/9380
2019-12-20T07:45:07.4884740Z .................................................................................................... 5700/9380
2019-12-20T07:45:16.3037473Z .................................................................................................... 5800/9380
2019-12-20T07:45:22.4683490Z ..........................................ii..i..ii...........i..................................... 5900/9380
2019-12-20T07:45:41.6395364Z .................................................................................................... 6100/9380
2019-12-20T07:45:48.4844140Z .................................................................................................... 6200/9380
2019-12-20T07:45:48.4844140Z .................................................................................................... 6200/9380
2019-12-20T07:45:56.2723980Z ..................................................................i..ii............................. 6300/9380
2019-12-20T07:46:20.7110291Z .................................................................................................... 6500/9380
2019-12-20T07:46:22.4121731Z ......................................i............................................................. 6600/9380
2019-12-20T07:46:24.2492115Z .................................................................................................... 6700/9380
2019-12-20T07:46:26.2718440Z ..............................i..................................................................... 6800/9380
---
2019-12-20T07:47:52.2419648Z .................................................................................................... 7400/9380
2019-12-20T07:47:56.5150180Z .................................................................................................... 7500/9380
2019-12-20T07:48:01.1194114Z .................................................................................................... 7600/9380
2019-12-20T07:48:09.6208007Z .................................................................................................... 7700/9380
2019-12-20T07:48:17.4428531Z ....................................................iiii............................................ 7800/9380
2019-12-20T07:48:30.2112627Z .................................................................................................... 8000/9380
2019-12-20T07:48:35.9163079Z .................................................................................................... 8100/9380
2019-12-20T07:48:49.3709625Z .................................................................................................... 8200/9380
2019-12-20T07:48:56.0653347Z .................................................................................................... 8300/9380
---
2019-12-20T07:50:36.2506805Z 
2019-12-20T07:50:36.2507011Z 25 LL | pub const X: () = unimplemented!();
2019-12-20T07:50:36.2507725Z 26    | ------------------^^^^^^^^^^^^^^^^-
2019-12-20T07:50:36.2507999Z 27    |                   |
2019-12-20T07:50:36.2508449Z -    |                   the evaluated program panicked at 'not yet implemented', $DIR/const_panic.rs:10:19
2019-12-20T07:50:36.2509320Z +    |                   the evaluated program panicked at 'not implemented', $DIR/const_panic.rs:10:19
2019-12-20T07:50:36.2510419Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2510794Z 31 
2019-12-20T07:50:36.2510862Z 
2019-12-20T07:50:36.2510917Z 
2019-12-20T07:50:36.2510917Z 
2019-12-20T07:50:36.2510964Z The actual stderr differed from the expected stderr.
2019-12-20T07:50:36.2511606Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/const_panic.stderr
2019-12-20T07:50:36.2511887Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T07:50:36.2512154Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic.rs`
2019-12-20T07:50:36.2512249Z error: 1 errors occurred comparing output.
2019-12-20T07:50:36.2512291Z status: exit code: 1
2019-12-20T07:50:36.2512291Z status: exit code: 1
2019-12-20T07:50:36.2513684Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/auxiliary" "-A" "unused"
2019-12-20T07:50:36.2514162Z ------------------------------------------
2019-12-20T07:50:36.2514192Z 
2019-12-20T07:50:36.2514376Z ------------------------------------------
2019-12-20T07:50:36.2514432Z stderr:
2019-12-20T07:50:36.2514432Z stderr:
2019-12-20T07:50:36.2514612Z ------------------------------------------
2019-12-20T07:50:36.2514655Z error: any use of this value will cause an error
2019-12-20T07:50:36.2514895Z   --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:4:19
2019-12-20T07:50:36.2514939Z    |
2019-12-20T07:50:36.2514978Z LL | pub const Z: () = panic!("cheese");
2019-12-20T07:50:36.2515388Z    |                   |
2019-12-20T07:50:36.2515388Z    |                   |
2019-12-20T07:50:36.2515675Z    |                   the evaluated program panicked at 'cheese', /checkout/src/test/ui/consts/const-eval/const_panic.rs:4:19
2019-12-20T07:50:36.2515791Z    = note: `#[deny(const_err)]` on by default
2019-12-20T07:50:36.2516218Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2516253Z 
2019-12-20T07:50:36.2516289Z error: any use of this value will cause an error
2019-12-20T07:50:36.2516289Z error: any use of this value will cause an error
2019-12-20T07:50:36.2516511Z   --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:7:19
2019-12-20T07:50:36.2516551Z    |
2019-12-20T07:50:36.2516587Z LL | pub const Y: () = unreachable!();
2019-12-20T07:50:36.2516924Z    |                   |
2019-12-20T07:50:36.2517207Z    |                   the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic.rs:7:19
2019-12-20T07:50:36.2517272Z    |
2019-12-20T07:50:36.2517718Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2517718Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2517755Z 
2019-12-20T07:50:36.2517813Z error: any use of this value will cause an error
2019-12-20T07:50:36.2518022Z   --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:10:19
2019-12-20T07:50:36.2518062Z    |
2019-12-20T07:50:36.2518116Z LL | pub const X: () = unimplemented!();
2019-12-20T07:50:36.2518298Z    | ------------------^^^^^^^^^^^^^^^^-
2019-12-20T07:50:36.2518511Z    |                   |
2019-12-20T07:50:36.2518829Z    |                   the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic.rs:10:19
2019-12-20T07:50:36.2519165Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2519220Z 
2019-12-20T07:50:36.2519257Z error: aborting due to 3 previous errors
2019-12-20T07:50:36.2519282Z 
---
2019-12-20T07:50:36.2520205Z 
2019-12-20T07:50:36.2520240Z 25 LL | const X: () = unimplemented!();
2019-12-20T07:50:36.2520422Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-12-20T07:50:36.2520479Z 27    |               |
2019-12-20T07:50:36.2520726Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore.rs:11:15
2019-12-20T07:50:36.2520986Z +    |               the evaluated program panicked at 'not implemented', $DIR/const_panic_libcore.rs:11:15
2019-12-20T07:50:36.2521471Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2521518Z 31 
2019-12-20T07:50:36.2521544Z 
2019-12-20T07:50:36.2521583Z 
2019-12-20T07:50:36.2521583Z 
2019-12-20T07:50:36.2521620Z The actual stderr differed from the expected stderr.
2019-12-20T07:50:36.2521882Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/const_panic_libcore.stderr
2019-12-20T07:50:36.2522103Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T07:50:36.2522328Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore.rs`
2019-12-20T07:50:36.2522419Z error: 1 errors occurred comparing output.
2019-12-20T07:50:36.2522456Z status: exit code: 1
2019-12-20T07:50:36.2522456Z status: exit code: 1
2019-12-20T07:50:36.2523407Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/auxiliary" "-A" "unused"
2019-12-20T07:50:36.2523884Z ------------------------------------------
2019-12-20T07:50:36.2523927Z 
2019-12-20T07:50:36.2524098Z ------------------------------------------
2019-12-20T07:50:36.2524199Z stderr:
2019-12-20T07:50:36.2524199Z stderr:
2019-12-20T07:50:36.2524382Z ------------------------------------------
2019-12-20T07:50:36.2524439Z error: any use of this value will cause an error
2019-12-20T07:50:36.2524642Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:5:15
2019-12-20T07:50:36.2524683Z    |
2019-12-20T07:50:36.2524736Z LL | const Z: () = panic!("cheese");
2019-12-20T07:50:36.2524940Z    |               |
2019-12-20T07:50:36.2524940Z    |               |
2019-12-20T07:50:36.2525196Z    |               the evaluated program panicked at 'cheese', /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:5:15
2019-12-20T07:50:36.2525274Z    = note: `#[deny(const_err)]` on by default
2019-12-20T07:50:36.2525541Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2525573Z 
2019-12-20T07:50:36.2525614Z error: any use of this value will cause an error
2019-12-20T07:50:36.2525614Z error: any use of this value will cause an error
2019-12-20T07:50:36.2525829Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:8:15
2019-12-20T07:50:36.2525866Z    |
2019-12-20T07:50:36.2525907Z LL | const Y: () = unreachable!();
2019-12-20T07:50:36.2526125Z    |               |
2019-12-20T07:50:36.2526389Z    |               the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:8:15
2019-12-20T07:50:36.2526450Z    |
2019-12-20T07:50:36.2526867Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2526867Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2526901Z 
2019-12-20T07:50:36.2526954Z error: any use of this value will cause an error
2019-12-20T07:50:36.2527411Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15
2019-12-20T07:50:36.2527457Z    |
2019-12-20T07:50:36.2527492Z LL | const X: () = unimplemented!();
2019-12-20T07:50:36.2527686Z    | --------------^^^^^^^^^^^^^^^^-
2019-12-20T07:50:36.2527725Z    |               |
2019-12-20T07:50:36.2527987Z    |               the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15
2019-12-20T07:50:36.2528484Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2528519Z 
2019-12-20T07:50:36.2528574Z error: aborting due to 3 previous errors
2019-12-20T07:50:36.2528598Z 
---
2019-12-20T07:50:36.2529142Z 
2019-12-20T07:50:36.2529197Z 25 LL | const X: () = unimplemented!();
2019-12-20T07:50:36.2529382Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-12-20T07:50:36.2529423Z 27    |               |
2019-12-20T07:50:36.2529745Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore_main.rs:15:15
2019-12-20T07:50:36.2530019Z +    |               the evaluated program panicked at 'not implemented', $DIR/const_panic_libcore_main.rs:15:15
2019-12-20T07:50:36.2530354Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2530559Z 31 
2019-12-20T07:50:36.2530584Z 
2019-12-20T07:50:36.2530605Z 
2019-12-20T07:50:36.2530605Z 
2019-12-20T07:50:36.2530659Z The actual stderr differed from the expected stderr.
2019-12-20T07:50:36.2530932Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/const_panic_libcore_main.stderr
2019-12-20T07:50:36.2531212Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T07:50:36.2531690Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore_main.rs`
2019-12-20T07:50:36.2531759Z error: 1 errors occurred comparing output.
2019-12-20T07:50:36.2531815Z status: exit code: 1
2019-12-20T07:50:36.2531815Z status: exit code: 1
2019-12-20T07:50:36.2532694Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/auxiliary" "-A" "unused"
2019-12-20T07:50:36.2532983Z ------------------------------------------
2019-12-20T07:50:36.2533018Z 
2019-12-20T07:50:36.2533215Z ------------------------------------------
2019-12-20T07:50:36.2533252Z stderr:
2019-12-20T07:50:36.2533252Z stderr:
2019-12-20T07:50:36.2533425Z ------------------------------------------
2019-12-20T07:50:36.2533484Z error: any use of this value will cause an error
2019-12-20T07:50:36.2533694Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:9:15
2019-12-20T07:50:36.2533736Z    |
2019-12-20T07:50:36.2533790Z LL | const Z: () = panic!("cheese");
2019-12-20T07:50:36.2534002Z    |               |
2019-12-20T07:50:36.2534002Z    |               |
2019-12-20T07:50:36.2534254Z    |               the evaluated program panicked at 'cheese', /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:9:15
2019-12-20T07:50:36.2534515Z    = note: `#[deny(const_err)]` on by default
2019-12-20T07:50:36.2534971Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2535003Z 
2019-12-20T07:50:36.2535037Z error: any use of this value will cause an error
2019-12-20T07:50:36.2535037Z error: any use of this value will cause an error
2019-12-20T07:50:36.2535237Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:12:15
2019-12-20T07:50:36.2535293Z    |
2019-12-20T07:50:36.2535326Z LL | const Y: () = unreachable!();
2019-12-20T07:50:36.2535541Z    |               |
2019-12-20T07:50:36.2535811Z    |               the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:12:15
2019-12-20T07:50:36.2535855Z    |
2019-12-20T07:50:36.2536117Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2536117Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2536157Z 
2019-12-20T07:50:36.2536194Z error: any use of this value will cause an error
2019-12-20T07:50:36.2536471Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15
2019-12-20T07:50:36.2536514Z    |
2019-12-20T07:50:36.2536548Z LL | const X: () = unimplemented!();
2019-12-20T07:50:36.2536923Z    | --------------^^^^^^^^^^^^^^^^-
2019-12-20T07:50:36.2536962Z    |               |
2019-12-20T07:50:36.2537453Z    |               the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15
2019-12-20T07:50:36.2537780Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-20T07:50:36.2537813Z 
2019-12-20T07:50:36.2537866Z error: aborting due to 3 previous errors
2019-12-20T07:50:36.2537890Z 
---
2019-12-20T07:50:36.2559282Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-20T07:50:36.2559381Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-20T07:50:36.2585307Z 
2019-12-20T07:50:36.2585378Z 
2019-12-20T07:50:36.2586998Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-20T07:50:36.2587478Z 
2019-12-20T07:50:36.2587503Z 
2019-12-20T07:50:36.2625330Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-20T07:50:36.2625437Z Build completed unsuccessfully in 0:56:37
2019-12-20T07:50:36.2625437Z Build completed unsuccessfully in 0:56:37
2019-12-20T07:50:36.2630774Z == clock drift check ==
2019-12-20T07:50:36.2646580Z   local time: Fri Dec 20 07:50:36 UTC 2019
2019-12-20T07:50:36.5523217Z   network time: Fri, 20 Dec 2019 07:50:36 GMT
2019-12-20T07:50:36.5523341Z == end clock drift check ==
2019-12-20T07:50:37.7841143Z 
2019-12-20T07:50:37.7935749Z ##[error]Bash exited with code '1'.
2019-12-20T07:50:37.7968241Z ##[section]Starting: Checkout
2019-12-20T07:50:37.7969662Z ==============================================================================
2019-12-20T07:50:37.7969704Z Task         : Get sources
2019-12-20T07:50:37.7969758Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
