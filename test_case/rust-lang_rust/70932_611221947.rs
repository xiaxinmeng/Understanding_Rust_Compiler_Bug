plain
2020-04-08T21:09:44.0525101Z ========================== Starting Command Output ===========================
2020-04-08T21:09:44.0530684Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/138cb05b-80ee-4319-b2b7-1d23fd049e1f.sh
2020-04-08T21:09:44.0530971Z 
2020-04-08T21:09:44.0534867Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T21:09:44.0551703Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70932/merge to s
2020-04-08T21:09:44.0554813Z Task         : Get sources
2020-04-08T21:09:44.0555096Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T21:09:44.0555321Z Version      : 1.0.0
2020-04-08T21:09:44.0555472Z Author       : Microsoft
---
2020-04-08T21:09:45.3801266Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T21:09:45.3807467Z ##[command]git config gc.auto 0
2020-04-08T21:09:45.3810328Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T21:09:45.3812879Z ##[command]git config --get-all http.proxy
2020-04-08T21:09:45.3818270Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70932/merge:refs/remotes/pull/70932/merge
---
2020-04-08T21:13:10.7064865Z Looks like docker image is the same as before, not uploading
2020-04-08T21:13:18.7643513Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-08T21:13:18.7880442Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-08T21:13:18.7911020Z == clock drift check ==
2020-04-08T21:13:18.7919276Z   local time: Wed Apr  8 21:13:18 UTC 2020
2020-04-08T21:13:18.9298491Z   network time: Wed, 08 Apr 2020 21:13:18 GMT
2020-04-08T21:13:18.9319894Z Starting sccache server...
2020-04-08T21:13:18.9994441Z configure: processing command line
2020-04-08T21:13:18.9994653Z configure: 
2020-04-08T21:13:18.9995427Z configure: rust.dist-src        := False
---
2020-04-08T21:17:52.0201180Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T21:17:54.0151693Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T21:17:54.9106485Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T21:17:56.2373994Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T21:18:04.0311300Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T21:18:06.6903499Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T21:18:10.4644108Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T21:18:14.1269566Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T21:18:22.2013251Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T21:39:20.1726331Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T21:39:22.0808874Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T21:39:24.3077354Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T21:39:26.2008240Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T21:39:37.5406740Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T21:39:41.0257396Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T21:39:46.6174189Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T21:39:52.1970422Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T21:40:04.0065050Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T22:04:02.2133381Z .................................................................................................... 1700/9879
2020-04-08T22:04:06.0979036Z .................................................................................................... 1800/9879
2020-04-08T22:04:15.1784107Z ..................................................................................................i. 1900/9879
2020-04-08T22:04:22.7219311Z .................................................................................................... 2000/9879
2020-04-08T22:04:28.2617801Z ........................................................................................iiiii....... 2100/9879
2020-04-08T22:04:48.9679197Z .................................................................................................... 2300/9879
2020-04-08T22:04:50.9924128Z .................................................................................................... 2400/9879
2020-04-08T22:04:53.1506267Z ...............................................F.................................................... 2500/9879
2020-04-08T22:04:58.7749156Z ....F............................................................................................... 2600/9879
---
2020-04-08T22:07:39.8076499Z ..............................................................i...............i..................... 5000/9879
2020-04-08T22:07:47.2219224Z .................................................................................................... 5100/9879
2020-04-08T22:07:54.9196614Z .................................................................................................... 5200/9879
2020-04-08T22:08:00.2815187Z .......i............................................................................................ 5300/9879
2020-04-08T22:08:10.5041129Z ................................................................................................ii.i 5400/9879
2020-04-08T22:08:15.5088641Z i........i...i...................................................................................... 5500/9879
2020-04-08T22:08:24.1789950Z .........................................i.......................................................... 5700/9879
2020-04-08T22:08:34.3439929Z .............................................................ii..................................... 5800/9879
2020-04-08T22:08:41.2362684Z i................................................................................................... 5900/9879
2020-04-08T22:08:45.4987801Z .................................................................................................... 6000/9879
2020-04-08T22:08:45.4987801Z .................................................................................................... 6000/9879
2020-04-08T22:08:54.9208661Z ..............................................................................................ii...i 6100/9879
2020-04-08T22:09:06.3779859Z ..ii...........i.................................................................................... 6200/9879
2020-04-08T22:09:20.6142228Z .................................................................................................... 6400/9879
2020-04-08T22:09:25.6923807Z .................................................................................................... 6500/9879
2020-04-08T22:09:25.6923807Z .................................................................................................... 6500/9879
2020-04-08T22:09:41.0090222Z ........................i..ii....................................................................... 6600/9879
2020-04-08T22:10:00.8260869Z .................................................................................................... 6800/9879
2020-04-08T22:10:02.5109757Z ........................i........................................................................... 6900/9879
2020-04-08T22:10:04.2359932Z .................................................................................................... 7000/9879
2020-04-08T22:10:06.0160488Z ...............................................................i.................................... 7100/9879
---
2020-04-08T22:11:38.2261239Z .................................................................................................... 7800/9879
2020-04-08T22:11:42.7213274Z .................................................................................................... 7900/9879
2020-04-08T22:11:48.5555556Z .................................................................................................... 8000/9879
2020-04-08T22:11:54.8389466Z ............................i....................................................................... 8100/9879
2020-04-08T22:12:01.6325968Z ............................................................................iiiiii.iiii.i........... 8200/9879
2020-04-08T22:12:15.1034607Z .....................i......i....................................................................... 8400/9879
2020-04-08T22:12:18.8895003Z .................................................................................................... 8500/9879
2020-04-08T22:12:28.2590709Z .................................................................................................... 8600/9879
2020-04-08T22:12:38.5371724Z .................................................................................................... 8700/9879
---
2020-04-08T22:14:26.3882730Z failures:
2020-04-08T22:14:26.3929636Z 
2020-04-08T22:14:26.3931046Z ---- [ui] ui/array-slice-vec/slice-pat-type-mismatches.rs stdout ----
2020-04-08T22:14:26.3931621Z 
2020-04-08T22:14:26.3932158Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-04-08T22:14:26.3932721Z status: exit code: 101
2020-04-08T22:14:26.3935223Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/slice-pat-type-mismatches.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/slice-pat-type-mismatches" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/slice-pat-type-mismatches/auxiliary"
2020-04-08T22:14:26.3937619Z ------------------------------------------
2020-04-08T22:14:26.3938109Z 
2020-04-08T22:14:26.3938794Z ------------------------------------------
2020-04-08T22:14:26.3939289Z stderr:
2020-04-08T22:14:26.3939289Z stderr:
2020-04-08T22:14:26.3939996Z ------------------------------------------
2020-04-08T22:14:26.3940606Z error[E0425]: cannot find value `does_not_exist` in this scope
2020-04-08T22:14:26.3941537Z   --> /checkout/src/test/ui/array-slice-vec/slice-pat-type-mismatches.rs:26:11
2020-04-08T22:14:26.3942801Z    |
2020-04-08T22:14:26.3943304Z LL |     match does_not_exist { //~ ERROR cannot find value `does_not_exist` in this scope
2020-04-08T22:14:26.3944168Z 
2020-04-08T22:14:26.3944584Z error[E0529]: expected an array or slice, found `std::string::String`
2020-04-08T22:14:26.3945537Z   --> /checkout/src/test/ui/array-slice-vec/slice-pat-type-mismatches.rs:3:9
2020-04-08T22:14:26.3945994Z    |
2020-04-08T22:14:26.3945994Z    |
2020-04-08T22:14:26.3946537Z LL |         ['f', 'o', ..] => {}
2020-04-08T22:14:26.3947329Z    |         ^^^^^^^^^^^^^^ pattern cannot match with input type `std::string::String`
2020-04-08T22:14:26.3948577Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_typeck/check/pat.rs:1384:36
2020-04-08T22:14:26.3949594Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-08T22:14:26.3949974Z 
2020-04-08T22:14:26.3950386Z error: internal compiler error: unexpected panic
2020-04-08T22:14:26.3950386Z error: internal compiler error: unexpected panic
2020-04-08T22:14:26.3950719Z 
2020-04-08T22:14:26.3951072Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-08T22:14:26.3951421Z 
2020-04-08T22:14:26.3952493Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-08T22:14:26.3954179Z note: rustc 1.44.0-nightly (7d26961a1 2020-04-08) running on x86_64-unknown-linux-gnu
2020-04-08T22:14:26.3954600Z 
2020-04-08T22:14:26.3954600Z 
2020-04-08T22:14:26.3955428Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-08T22:14:26.3956309Z error: aborting due to 2 previous errors
2020-04-08T22:14:26.3956610Z 
2020-04-08T22:14:26.3956989Z Some errors have detailed explanations: E0425, E0529.
2020-04-08T22:14:26.3957735Z For more information about an error, try `rustc --explain E0425`.
2020-04-08T22:14:26.3957735Z For more information about an error, try `rustc --explain E0425`.
2020-04-08T22:14:26.3958114Z 
2020-04-08T22:14:26.3958671Z ------------------------------------------
2020-04-08T22:14:26.3959003Z 
2020-04-08T22:14:26.3959232Z 
2020-04-08T22:14:26.3959775Z ---- [ui] ui/error-codes/E0528.rs stdout ----
2020-04-08T22:14:26.3960133Z 
2020-04-08T22:14:26.3960538Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-04-08T22:14:26.3960982Z status: exit code: 101
2020-04-08T22:14:26.3963126Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0528.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0528" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0528/auxiliary"
2020-04-08T22:14:26.3965281Z ------------------------------------------
2020-04-08T22:14:26.3965636Z 
2020-04-08T22:14:26.3966176Z ------------------------------------------
2020-04-08T22:14:26.3966577Z stderr:
2020-04-08T22:14:26.3966577Z stderr:
2020-04-08T22:14:26.3967174Z ------------------------------------------
2020-04-08T22:14:26.3968372Z error[E0528]: pattern requires at least 3 elements but array has 2
2020-04-08T22:14:26.3970366Z   --> /checkout/src/test/ui/error-codes/E0528.rs:4:10
2020-04-08T22:14:26.3973010Z    |
2020-04-08T22:14:26.3974482Z LL |         &[a, b, c, rest @ ..] => {
2020-04-08T22:14:26.3975611Z    |          ^^^^^^^^^^^^^^^^^^^^ pattern cannot match array of 2 elements
2020-04-08T22:14:26.3976642Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_typeck/check/pat.rs:1384:36
2020-04-08T22:14:26.3977145Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-08T22:14:26.3977389Z 
2020-04-08T22:14:26.3977625Z error: internal compiler error: unexpected panic
2020-04-08T22:14:26.3977625Z error: internal compiler error: unexpected panic
2020-04-08T22:14:26.3977825Z 
2020-04-08T22:14:26.3978047Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-08T22:14:26.3978251Z 
2020-04-08T22:14:26.3978923Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-08T22:14:26.3979706Z note: rustc 1.44.0-nightly (7d26961a1 2020-04-08) running on x86_64-unknown-linux-gnu
2020-04-08T22:14:26.3979985Z 
2020-04-08T22:14:26.3979985Z 
2020-04-08T22:14:26.3980630Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-08T22:14:26.3981348Z error: aborting due to previous error
2020-04-08T22:14:26.3981518Z 
2020-04-08T22:14:26.3981991Z For more information about this error, try `rustc --explain E0528`.
2020-04-08T22:14:26.3982215Z 
2020-04-08T22:14:26.3982215Z 
2020-04-08T22:14:26.3982598Z ------------------------------------------
2020-04-08T22:14:26.3982777Z 
2020-04-08T22:14:26.3982875Z 
2020-04-08T22:14:26.3983249Z ---- [ui] ui/error-codes/E0730.rs stdout ----
2020-04-08T22:14:26.3983432Z 
2020-04-08T22:14:26.3983716Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-04-08T22:14:26.3984031Z status: exit code: 101
2020-04-08T22:14:26.3986087Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0730.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0730" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0730/auxiliary"
2020-04-08T22:14:26.3987735Z ------------------------------------------
2020-04-08T22:14:26.3987915Z 
2020-04-08T22:14:26.3988281Z ------------------------------------------
2020-04-08T22:14:26.3988488Z stderr:
---
2020-04-08T22:14:26.3991308Z 
2020-04-08T22:14:26.3991783Z error[E0730]: cannot pattern-match on an array without a fixed length
2020-04-08T22:14:26.3992322Z   --> /checkout/src/test/ui/error-codes/E0730.rs:6:9
2020-04-08T22:14:26.3992547Z    |
2020-04-08T22:14:26.3993081Z LL |         [1, 2, ..] => true, //~ ERROR cannot pattern-match on an array without a fixed length
2020-04-08T22:14:26.3993584Z 
2020-04-08T22:14:26.3994181Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_typeck/check/pat.rs:1384:36
2020-04-08T22:14:26.3994702Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-08T22:14:26.3994946Z 
2020-04-08T22:14:26.3994946Z 
2020-04-08T22:14:26.3995165Z error: internal compiler error: unexpected panic
2020-04-08T22:14:26.3995384Z 
2020-04-08T22:14:26.3995607Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-08T22:14:26.3995815Z 
2020-04-08T22:14:26.3996450Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-08T22:14:26.3997225Z note: rustc 1.44.0-nightly (7d26961a1 2020-04-08) running on x86_64-unknown-linux-gnu
2020-04-08T22:14:26.3997484Z 
2020-04-08T22:14:26.3997484Z 
2020-04-08T22:14:26.3998144Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-08T22:14:26.3998694Z error: aborting due to previous error
2020-04-08T22:14:26.3998866Z 
2020-04-08T22:14:26.3999328Z For more information about this error, try `rustc --explain E0730`.
2020-04-08T22:14:26.3999555Z 
---
2020-04-08T22:14:26.4002550Z test result: FAILED. 9816 passed; 3 failed; 60 ignored; 0 measured; 0 filtered out
2020-04-08T22:14:26.4002845Z 
2020-04-08T22:14:26.4005816Z 
2020-04-08T22:14:26.4006149Z 
2020-04-08T22:14:26.4010482Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-08T22:14:26.4014947Z 
2020-04-08T22:14:26.4015049Z 
2020-04-08T22:14:26.4015701Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-08T22:14:26.4016109Z Build completed unsuccessfully in 0:59:44
2020-04-08T22:14:26.4016109Z Build completed unsuccessfully in 0:59:44
2020-04-08T22:14:26.4016719Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-08T22:14:26.4017162Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-08T22:14:26.4031592Z == clock drift check ==
2020-04-08T22:14:26.4046304Z   local time: Wed Apr  8 22:14:26 UTC 2020
2020-04-08T22:14:26.6740990Z   network time: Wed, 08 Apr 2020 22:14:26 GMT
2020-04-08T22:14:27.0947007Z 
2020-04-08T22:14:27.0947007Z 
2020-04-08T22:14:27.1016222Z ##[error]Bash exited with code '1'.
2020-04-08T22:14:27.1034439Z ##[section]Finishing: Run build
2020-04-08T22:14:27.1086439Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70932/merge to s
2020-04-08T22:14:27.1092791Z Task         : Get sources
2020-04-08T22:14:27.1093158Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T22:14:27.1093500Z Version      : 1.0.0
2020-04-08T22:14:27.1093745Z Author       : Microsoft
2020-04-08T22:14:27.1093745Z Author       : Microsoft
2020-04-08T22:14:27.1094115Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T22:14:27.1094543Z ==============================================================================
2020-04-08T22:14:27.4411796Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T22:14:27.4454123Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70932/merge to s
2020-04-08T22:14:27.4546857Z Cleaning up task key
2020-04-08T22:14:27.4548147Z Start cleaning up orphan processes.
2020-04-08T22:14:27.4739211Z Terminate orphan process: pid (4380) (python)
2020-04-08T22:14:27.4904448Z ##[section]Finishing: Finalize Job
