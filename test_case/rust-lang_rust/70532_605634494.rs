plain
2020-03-29T12:16:06.4336146Z ========================== Starting Command Output ===========================
2020-03-29T12:16:06.4338619Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a0029630-2da7-4691-a3a0-12f0e47bfe33.sh
2020-03-29T12:16:06.4338906Z 
2020-03-29T12:16:06.4342841Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-29T12:16:06.4362860Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70532/merge to s
2020-03-29T12:16:06.4366056Z Task         : Get sources
2020-03-29T12:16:06.4366362Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-29T12:16:06.4366677Z Version      : 1.0.0
2020-03-29T12:16:06.4366878Z Author       : Microsoft
---
2020-03-29T12:16:07.4289309Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-29T12:16:07.4298503Z ##[command]git config gc.auto 0
2020-03-29T12:16:07.4304893Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-29T12:16:07.4310704Z ##[command]git config --get-all http.proxy
2020-03-29T12:16:07.4316266Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70532/merge:refs/remotes/pull/70532/merge
---
2020-03-29T12:24:04.4138668Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-29T12:24:05.8036122Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-29T12:24:07.3442198Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-29T12:24:07.6276726Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-29T12:24:16.8064284Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-29T12:24:18.3488324Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-29T12:24:22.5671685Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-29T12:24:26.5223936Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-29T12:24:56.4851152Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-29T12:44:48.3173165Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-29T12:44:49.9092395Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-29T12:44:51.7659660Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-29T12:44:52.9796776Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-29T12:45:03.0885590Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-29T12:45:05.4414253Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-29T12:45:10.3421560Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-29T12:45:15.3703915Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-29T12:45:51.3649711Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-29T13:08:43.5108738Z .................................................................................................... 1700/9855
2020-03-29T13:08:47.2895194Z .................................................................................................... 1800/9855
2020-03-29T13:08:56.6235180Z .........................................................................................i.......... 1900/9855
2020-03-29T13:09:03.0139337Z .................................................................................................... 2000/9855
2020-03-29T13:09:09.4660996Z ...............................................................................iiiii................ 2100/9855
2020-03-29T13:09:29.1788277Z .................................................................................................... 2300/9855
2020-03-29T13:09:31.2220129Z .................................................................................................... 2400/9855
2020-03-29T13:09:33.5411210Z .................................................................................................... 2500/9855
2020-03-29T13:09:42.1078742Z .................................................................................................... 2600/9855
---
2020-03-29T13:12:27.4312321Z .....................................................i...............i.............................. 5000/9855
2020-03-29T13:12:34.7310858Z .................................................................................................... 5100/9855
2020-03-29T13:12:41.8704576Z ..................................................................................................i. 5200/9855
2020-03-29T13:12:46.5603368Z .................................................................................................... 5300/9855
2020-03-29T13:12:56.6769110Z ....................................................................................ii.ii........i.. 5400/9855
2020-03-29T13:13:00.1411566Z .i.................................................................................................. 5500/9855
2020-03-29T13:13:08.7061035Z .............................i...................................................................... 5700/9855
2020-03-29T13:13:17.9064446Z ...............................................ii....................................i.............. 5800/9855
2020-03-29T13:13:24.8457880Z .................................................................................................... 5900/9855
2020-03-29T13:13:29.9021490Z .................................................................................................... 6000/9855
2020-03-29T13:13:29.9021490Z .................................................................................................... 6000/9855
2020-03-29T13:13:39.1642850Z ...............................................................................ii...i..ii........... 6100/9855
2020-03-29T13:13:50.8295646Z i................................................................................................... 6200/9855
2020-03-29T13:14:04.0345637Z .................................................................................................... 6400/9855
2020-03-29T13:14:07.5791184Z .................................................................................................... 6500/9855
2020-03-29T13:14:07.5791184Z .................................................................................................... 6500/9855
2020-03-29T13:14:19.2574329Z .........i..ii...................................................................................... 6600/9855
2020-03-29T13:14:38.6545639Z .................................................................................................... 6800/9855
2020-03-29T13:14:40.6559021Z .........i.......................................................................................... 6900/9855
2020-03-29T13:14:42.7709173Z .................................................................................................... 7000/9855
2020-03-29T13:14:44.8481289Z ..............................................i..................................................... 7100/9855
---
2020-03-29T13:16:19.2748746Z .................................................................................................... 7800/9855
2020-03-29T13:16:24.0696420Z .................................................................................................... 7900/9855
2020-03-29T13:16:29.7682517Z .................................................................................................... 8000/9855
2020-03-29T13:16:37.3407606Z ......i............................................................................................. 8100/9855
2020-03-29T13:16:45.0623459Z .......................................................iiiiiiiiii.i................................. 8200/9855
2020-03-29T13:16:52.8696916Z ...................................................................................................i 8300/9855
2020-03-29T13:17:03.0078803Z .................................................................................................... 8500/9855
2020-03-29T13:17:14.9009923Z .................................................................................................... 8600/9855
2020-03-29T13:17:23.9279642Z .................................................................................................... 8700/9855
2020-03-29T13:17:28.7870180Z .................................................................................................... 8800/9855
---
2020-03-29T13:19:14.3625410Z ---- [ui] ui/issues/issue-23611-enum-swap-in-drop.rs stdout ----
2020-03-29T13:19:14.3625861Z 
2020-03-29T13:19:14.3626512Z error: test compilation failed although it shouldn't!
2020-03-29T13:19:14.3626992Z status: exit code: 101
2020-03-29T13:19:14.3629164Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23611-enum-swap-in-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23611-enum-swap-in-drop/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23611-enum-swap-in-drop/auxiliary"
2020-03-29T13:19:14.3631182Z ------------------------------------------
2020-03-29T13:19:14.3631596Z 
2020-03-29T13:19:14.3632172Z ------------------------------------------
2020-03-29T13:19:14.3632566Z stderr:
2020-03-29T13:19:14.3632566Z stderr:
2020-03-29T13:19:14.3633210Z ------------------------------------------
2020-03-29T13:19:14.3633922Z thread 'rustc' panicked at 'type mismatch when copying!
2020-03-29T13:19:14.3635112Z src: for<'r, 's> fn(u32, &'r str, &'s std::cell::RefCell<std::vec::Vec<u32>>), dest: for<'r> fn(u32, &'r str, &std::cell::RefCell<std::vec::Vec<u32>>)', src/librustc_mir/interpret/place.rs:883:9
2020-03-29T13:19:14.3639769Z 
2020-03-29T13:19:14.3640050Z error: internal compiler error: unexpected panic
2020-03-29T13:19:14.3640247Z 
2020-03-29T13:19:14.3640578Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-29T13:19:14.3640578Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-29T13:19:14.3640947Z 
2020-03-29T13:19:14.3641735Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-29T13:19:14.3642529Z note: rustc 1.44.0-nightly (4c7764a3a 2020-03-29) running on x86_64-unknown-linux-gnu
2020-03-29T13:19:14.3642800Z 
2020-03-29T13:19:14.3642800Z 
2020-03-29T13:19:14.3643453Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-29T13:19:14.3643906Z 
2020-03-29T13:19:14.3644299Z ------------------------------------------
2020-03-29T13:19:14.3644474Z 
2020-03-29T13:19:14.3644571Z 
---
2020-03-29T13:19:14.3650085Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-29T13:19:14.3650499Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-29T13:19:14.3665176Z 
2020-03-29T13:19:14.3665328Z 
2020-03-29T13:19:14.3669383Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-29T13:19:14.3672080Z 
2020-03-29T13:19:14.3672173Z 
2020-03-29T13:19:14.3675523Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-29T13:19:14.3675905Z Build completed unsuccessfully in 0:58:31
2020-03-29T13:19:14.3675905Z Build completed unsuccessfully in 0:58:31
2020-03-29T13:19:14.3731489Z == clock drift check ==
2020-03-29T13:19:14.3755063Z   local time: Sun Mar 29 13:19:14 UTC 2020
2020-03-29T13:19:14.4785006Z   network time: Sun, 29 Mar 2020 13:19:14 GMT
2020-03-29T13:19:14.4788761Z == end clock drift check ==
2020-03-29T13:19:14.9292024Z 
2020-03-29T13:19:14.9357530Z ##[error]Bash exited with code '1'.
2020-03-29T13:19:14.9373512Z ##[section]Finishing: Run build
2020-03-29T13:19:14.9423674Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70532/merge to s
2020-03-29T13:19:14.9428814Z Task         : Get sources
2020-03-29T13:19:14.9429157Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-29T13:19:14.9429493Z Version      : 1.0.0
2020-03-29T13:19:14.9429715Z Author       : Microsoft
2020-03-29T13:19:14.9429715Z Author       : Microsoft
2020-03-29T13:19:14.9430066Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-29T13:19:14.9430488Z ==============================================================================
2020-03-29T13:19:15.2668273Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-29T13:19:15.2730681Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70532/merge to s
2020-03-29T13:19:15.2821303Z Cleaning up task key
2020-03-29T13:19:15.2822578Z Start cleaning up orphan processes.
2020-03-29T13:19:15.2990320Z Terminate orphan process: pid (4689) (python)
2020-03-29T13:19:15.3191199Z ##[section]Finishing: Finalize Job
