plain
2020-01-23T12:11:05.4177022Z ========================== Starting Command Output ===========================
2020-01-23T12:11:05.4180050Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/68973766-2d6c-43ab-a99b-53511cdf5744.sh
2020-01-23T12:11:05.4180275Z 
2020-01-23T12:11:05.4184302Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-23T12:11:05.4191726Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68481/merge to s
2020-01-23T12:11:05.4193700Z Task         : Get sources
2020-01-23T12:11:05.4194223Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T12:11:05.4194264Z Version      : 1.0.0
2020-01-23T12:11:05.4194299Z Author       : Microsoft
---
2020-01-23T12:11:06.3000823Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-23T12:11:06.3081755Z ##[command]git config gc.auto 0
2020-01-23T12:11:06.3158199Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-23T12:11:06.3223519Z ##[command]git config --get-all http.proxy
2020-01-23T12:11:06.3365252Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68481/merge:refs/remotes/pull/68481/merge
---
2020-01-23T13:08:49.0643262Z .................................................................................................... 1700/9546
2020-01-23T13:08:55.5088912Z .................................................................................................... 1800/9546
2020-01-23T13:09:07.4780645Z .....................i.............................................................................. 1900/9546
2020-01-23T13:09:15.3610997Z .................................................................................................... 2000/9546
2020-01-23T13:09:30.8841428Z ...........iiiii.................................................................................... 2100/9546
2020-01-23T13:09:40.7842756Z .................................................................................................... 2300/9546
2020-01-23T13:09:43.3255910Z .................................................................................................... 2400/9546
2020-01-23T13:09:48.8907696Z .................................................................................................... 2500/9546
2020-01-23T13:10:10.4739284Z .....................................................F.............................................. 2600/9546
---
2020-01-23T13:12:56.0559014Z .......................................................i...............i............................ 4900/9546
2020-01-23T13:13:04.4610586Z .................................................................................................... 5000/9546
2020-01-23T13:13:12.5545927Z ..................................................................................................i. 5100/9546
2020-01-23T13:13:17.8365684Z .................................................................................................... 5200/9546
2020-01-23T13:13:29.0899962Z ......................................................................ii.ii...........i............. 5300/9546
2020-01-23T13:13:38.5349430Z .......i............................................................................................ 5500/9546
2020-01-23T13:13:48.8493800Z .................................................................................................... 5600/9546
2020-01-23T13:13:55.6850593Z ........................................................i........................................... 5700/9546
2020-01-23T13:14:02.9298483Z .................................................................................................... 5800/9546
2020-01-23T13:14:02.9298483Z .................................................................................................... 5800/9546
2020-01-23T13:14:13.0534466Z .................................................................................................... 5900/9546
2020-01-23T13:14:20.9672092Z ...............................................ii...i..ii...........i............................... 6000/9546
2020-01-23T13:14:43.6549053Z .................................................................................................... 6200/9546
2020-01-23T13:14:52.3068451Z .................................................................................................... 6300/9546
2020-01-23T13:14:52.3068451Z .................................................................................................... 6300/9546
2020-01-23T13:15:02.4850847Z ...........................................................................i..ii.................... 6400/9546
2020-01-23T13:15:33.5358527Z .................................................................................................... 6600/9546
2020-01-23T13:15:38.5779341Z ...................................................i................................................ 6700/9546
2020-01-23T13:15:40.8758897Z .................................................................................................... 6800/9546
2020-01-23T13:15:43.2429984Z ..................................................i................................................. 6900/9546
---
2020-01-23T13:17:28.9688779Z .................................................................................................... 7600/9546
2020-01-23T13:17:34.9339008Z .................................................................................................... 7700/9546
2020-01-23T13:17:41.8019644Z .................................................................................................... 7800/9546
2020-01-23T13:17:52.9323830Z .................................................................................................... 7900/9546
2020-01-23T13:17:59.1315893Z ......iiiiiii....................................................................................... 8000/9546
2020-01-23T13:18:14.4703409Z .................................................................................................... 8200/9546
2020-01-23T13:18:26.5843016Z .................................................................................................... 8300/9546
2020-01-23T13:18:39.6865961Z .................................................................................................... 8400/9546
2020-01-23T13:18:46.2130969Z ....................................F............................................................... 8500/9546
---
2020-01-23T13:20:46.3861416Z 
2020-01-23T13:20:46.3861562Z 
2020-01-23T13:20:46.3861999Z ---- [ui] ui/suggestions/as-ref.rs stdout ----
2020-01-23T13:20:46.3862179Z 
2020-01-23T13:20:46.3862587Z error: Error: expected failure status (Some(1)) but received status None.
2020-01-23T13:20:46.3862821Z status: signal: 6
2020-01-23T13:20:46.3863939Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/as-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/as-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/as-ref/auxiliary" "-A" "unused"
2020-01-23T13:20:46.3864627Z ------------------------------------------
2020-01-23T13:20:46.3864795Z 
2020-01-23T13:20:46.3865211Z ------------------------------------------
2020-01-23T13:20:46.3865397Z stderr:
2020-01-23T13:20:46.3865397Z stderr:
2020-01-23T13:20:46.3865804Z ------------------------------------------
2020-01-23T13:20:46.3866018Z error[E0308]: mismatched types
2020-01-23T13:20:46.3866949Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:6:27
2020-01-23T13:20:46.3867268Z    |
2020-01-23T13:20:46.3867457Z LL |   opt.map(|arg| takes_ref(arg));
2020-01-23T13:20:46.3867987Z    |       ---                 ^^^ expected `&Foo`, found struct `Foo`
2020-01-23T13:20:46.3868408Z    |       help: consider using `as_ref` instead: `mismatched t`
2020-01-23T13:20:46.3868559Z 
2020-01-23T13:20:46.3868725Z error[E0308]: mismatched types
2020-01-23T13:20:46.3869207Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:8:37
2020-01-23T13:20:46.3869207Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:8:37
2020-01-23T13:20:46.3869438Z    |
2020-01-23T13:20:46.3869608Z LL |   opt.and_then(|arg| Some(takes_ref(arg)));
2020-01-23T13:20:46.3870082Z    |       --------                      ^^^ expected `&Foo`, found struct `Foo`
2020-01-23T13:20:46.3870471Z    |       help: consider using `as_ref` instead: `mismatched typesn`
2020-01-23T13:20:46.3870653Z 
2020-01-23T13:20:46.3870839Z error[E0308]: mismatched types
2020-01-23T13:20:46.3871296Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:11:27
2020-01-23T13:20:46.3871296Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:11:27
2020-01-23T13:20:46.3871501Z    |
2020-01-23T13:20:46.3871665Z LL |   opt.map(|arg| takes_ref(arg));
2020-01-23T13:20:46.3872147Z    |       ---                 ^^^ expected `&Foo`, found struct `Foo`
2020-01-23T13:20:46.3872543Z    |       help: consider using `as_ref` instead: `mismatched t`
2020-01-23T13:20:46.3872685Z 
2020-01-23T13:20:46.3872849Z error[E0308]: mismatched types
2020-01-23T13:20:46.3873310Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:13:35
2020-01-23T13:20:46.3873310Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:13:35
2020-01-23T13:20:46.3873510Z    |
2020-01-23T13:20:46.3873674Z LL |   opt.and_then(|arg| Ok(takes_ref(arg)));
2020-01-23T13:20:46.3874143Z    |       --------                    ^^^ expected `&Foo`, found struct `Foo`
2020-01-23T13:20:46.3874581Z    |       help: consider using `as_ref` instead: `mismatched typesn`
2020-01-23T13:20:46.3874933Z 
2020-01-23T13:20:46.3875100Z error[E0308]: mismatched types
2020-01-23T13:20:46.3875586Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:16:27
2020-01-23T13:20:46.3875586Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:16:27
2020-01-23T13:20:46.3875821Z    |
2020-01-23T13:20:46.3876006Z LL |   let y: Option<&usize> = x;
2020-01-23T13:20:46.3877235Z    |          |                |
2020-01-23T13:20:46.3877433Z    |          |                expected enum `std::option::Option`, found reference
2020-01-23T13:20:46.3877433Z    |          |                expected enum `std::option::Option`, found reference
2020-01-23T13:20:46.3877610Z    |          |                help: you can convert from `&Option<T>` to `Option<&T>` using `.as_ref()`: `x.as_ref()`
2020-01-23T13:20:46.3877957Z    |
2020-01-23T13:20:46.3877957Z    |
2020-01-23T13:20:46.3878113Z    = note:   expected enum `std::option::Option<&usize>`
2020-01-23T13:20:46.3878292Z            found reference `&std::option::Option<usize>`
2020-01-23T13:20:46.3880164Z error[E0308]: mismatched types
2020-01-23T13:20:46.3880725Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:19:35
2020-01-23T13:20:46.3880952Z    |
2020-01-23T13:20:46.3880952Z    |
2020-01-23T13:20:46.3881111Z LL |   let y: Result<&usize, &usize> = x;
2020-01-23T13:20:46.3881569Z    |          ----------------------   ^ expected enum `std::result::Result`, found reference
2020-01-23T13:20:46.3881957Z    |          expected due to this
2020-01-23T13:20:46.3882124Z    |
2020-01-23T13:20:46.3882124Z    |
2020-01-23T13:20:46.3882290Z    = note:   expected enum `std::result::Result<&usize, &usize>`
2020-01-23T13:20:46.3882457Z            found reference `&std::result::Result<usize, usize>`
2020-01-23T13:20:46.3882621Z help: you can convert from `&Result<T, E>` to `Result<&T, &E>` using `.as_ref()`
2020-01-23T13:20:46.3882776Z    |
2020-01-23T13:20:46.3882947Z LL |   let y: Result<&usize, &usize> = x.as_ref();
2020-01-23T13:20:46.3883272Z 
2020-01-23T13:20:46.3883442Z error[E0308]: mismatched types
2020-01-23T13:20:46.3883866Z   --> /checkout/src/test/ui/suggestions/as-ref.rs:23:34
2020-01-23T13:20:46.3884059Z    |
2020-01-23T13:20:46.3884059Z    |
2020-01-23T13:20:46.3884234Z LL |   let y: Result<&usize, usize> = x;
2020-01-23T13:20:46.3884698Z    |          ---------------------   ^ expected enum `std::result::Result`, found reference
2020-01-23T13:20:46.3885078Z    |          expected due to this
2020-01-23T13:20:46.3885229Z    |
2020-01-23T13:20:46.3885229Z    |
2020-01-23T13:20:46.3885402Z    = note:   expected enum `std::result::Result<&usize, usize>`
2020-01-23T13:20:46.3885566Z            found reference `&std::result::Result<usize, usize>`
2020-01-23T13:20:46.3885721Z 
2020-01-23T13:20:46.3885875Z free(): invalid pointer
2020-01-23T13:20:46.3886405Z ------------------------------------------
2020-01-23T13:20:46.3886902Z 
2020-01-23T13:20:46.3887045Z 
2020-01-23T13:20:46.3887519Z ---- [ui] ui/tcp-stress.rs stdout ----
---
2020-01-23T13:20:46.3893925Z test result: FAILED. 9493 passed; 3 failed; 50 ignored; 0 measured; 0 filtered out
2020-01-23T13:20:46.3894305Z 
2020-01-23T13:20:46.3897830Z 
2020-01-23T13:20:46.3899177Z 
2020-01-23T13:20:46.3901597Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-23T13:20:46.3908137Z 
2020-01-23T13:20:46.3908543Z 
2020-01-23T13:20:46.3909334Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-23T13:20:46.3909586Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-23T13:20:46.3909586Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-23T13:20:46.3913154Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-23T13:20:46.3913503Z Build completed unsuccessfully in 1:03:52
2020-01-23T13:20:46.3966223Z == clock drift check ==
2020-01-23T13:20:46.3984113Z   local time: Thu Jan 23 13:20:46 UTC 2020
2020-01-23T13:20:46.6976277Z   network time: Thu, 23 Jan 2020 13:20:46 GMT
2020-01-23T13:20:46.6978515Z == end clock drift check ==
2020-01-23T13:20:47.1623147Z 
2020-01-23T13:20:47.1728478Z ##[error]Bash exited with code '1'.
2020-01-23T13:20:47.1752860Z ##[section]Finishing: Run build
2020-01-23T13:20:47.1774678Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68481/merge to s
2020-01-23T13:20:47.1776611Z Task         : Get sources
2020-01-23T13:20:47.1776669Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T13:20:47.1776711Z Version      : 1.0.0
2020-01-23T13:20:47.1776747Z Author       : Microsoft
2020-01-23T13:20:47.1776747Z Author       : Microsoft
2020-01-23T13:20:47.1776805Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-23T13:20:47.1776863Z ==============================================================================
2020-01-23T13:20:47.6220161Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-23T13:20:47.6263054Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68481/merge to s
2020-01-23T13:20:47.6401045Z Cleaning up task key
2020-01-23T13:20:47.6401913Z Start cleaning up orphan processes.
2020-01-23T13:20:47.6541252Z Terminate orphan process: pid (4152) (python)
2020-01-23T13:20:47.6780332Z ##[section]Finishing: Finalize Job
