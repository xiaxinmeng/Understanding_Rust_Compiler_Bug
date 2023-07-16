plain
2020-02-27T23:55:41.7969837Z ========================== Starting Command Output ===========================
2020-02-27T23:55:41.7973427Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/42f145df-e4e8-4fab-8ea2-0efccc0a35b0.sh
2020-02-27T23:55:41.7973837Z 
2020-02-27T23:55:41.7977481Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-27T23:55:41.7997712Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-02-27T23:55:41.8001542Z Task         : Get sources
2020-02-27T23:55:41.8001859Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-27T23:55:41.8002178Z Version      : 1.0.0
2020-02-27T23:55:41.8002387Z Author       : Microsoft
---
2020-02-27T23:55:44.7804987Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-27T23:55:44.7813103Z ##[command]git config gc.auto 0
2020-02-27T23:55:44.7817691Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-27T23:55:44.7822943Z ##[command]git config --get-all http.proxy
2020-02-27T23:55:44.7832463Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69503/merge:refs/remotes/pull/69503/merge
---
2020-02-28T00:52:22.1726908Z .................................................................................................... 1700/9735
2020-02-28T00:52:26.3806350Z .................................................................................................... 1800/9735
2020-02-28T00:52:36.8735297Z ......................................................................i............................. 1900/9735
2020-02-28T00:52:42.8475931Z .................................................................................................... 2000/9735
2020-02-28T00:52:56.6385347Z ............................................................iiiii................................... 2100/9735
2020-02-28T00:53:06.3907548Z .................................................................................................... 2300/9735
2020-02-28T00:53:08.5835658Z .................................................................................................... 2400/9735
2020-02-28T00:53:11.5308739Z .................................................................................................... 2500/9735
2020-02-28T00:53:30.0275010Z .................................................................................................... 2600/9735
---
2020-02-28T00:56:00.4211499Z ....................i...............i............................................................... 5000/9735
2020-02-28T00:56:09.5793008Z .................................................................................................... 5100/9735
2020-02-28T00:56:14.6413097Z ...............................................................i.................................... 5200/9735
2020-02-28T00:56:20.5604597Z .................................................................................................... 5300/9735
2020-02-28T00:56:28.6350028Z ........................................ii.ii........i...i.......................................... 5400/9735
2020-02-28T00:56:37.0993546Z .................................................................................................... 5600/9735
2020-02-28T00:56:45.9611236Z .................................................................................................... 5700/9735
2020-02-28T00:56:52.3911071Z ...............................i.................................................................... 5800/9735
2020-02-28T00:56:58.1627203Z .................................................................................................... 5900/9735
2020-02-28T00:56:58.1627203Z .................................................................................................... 5900/9735
2020-02-28T00:57:08.2211925Z .................................................................................................... 6000/9735
2020-02-28T00:57:17.0830005Z ......................ii...i..ii...........i........................................................ 6100/9735
2020-02-28T00:57:33.4182542Z .................................................................................................... 6300/9735
2020-02-28T00:57:37.1999117Z .................................................................................................... 6400/9735
2020-02-28T00:57:37.1999117Z .................................................................................................... 6400/9735
2020-02-28T00:57:43.2491311Z .....................................................i..ii.......................................... 6500/9735
2020-02-28T00:58:07.3167105Z .................................................................................................... 6700/9735
2020-02-28T00:58:09.4782867Z .............................................i...................................................... 6800/9735
2020-02-28T00:58:11.4599601Z .................................................................................................... 6900/9735
2020-02-28T00:58:13.5419146Z ...........................................................................i........................ 7000/9735
---
2020-02-28T00:59:41.5473498Z .................................................................................................... 7700/9735
2020-02-28T00:59:45.9291463Z .................................................................................................... 7800/9735
2020-02-28T00:59:50.8271164Z .................................................................................................... 7900/9735
2020-02-28T00:59:58.2722960Z ....................i............................................................................... 8000/9735
2020-02-28T01:00:06.1123918Z .....................................................................iiiiiii.i...................... 8100/9735
2020-02-28T01:00:19.9482106Z ..........i......i.................................................................................. 8300/9735
2020-02-28T01:00:24.7887951Z .................................................................................................... 8400/9735
2020-02-28T01:00:36.2037252Z .................................................................................................... 8500/9735
2020-02-28T01:00:44.5652335Z .................................................................................................... 8600/9735
---
2020-02-28T01:02:28.8071526Z ---- [ui] ui/async-await/async-closure.rs stdout ----
2020-02-28T01:02:28.8071790Z 
2020-02-28T01:02:28.8072296Z error: test compilation failed although it shouldn't!
2020-02-28T01:02:28.8072616Z status: exit code: 101
2020-02-28T01:02:28.8074916Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/auxiliary"
2020-02-28T01:02:28.8076804Z ------------------------------------------
2020-02-28T01:02:28.8077015Z 
2020-02-28T01:02:28.8077449Z ------------------------------------------
2020-02-28T01:02:28.8077711Z stderr:
2020-02-28T01:02:28.8077711Z stderr:
2020-02-28T01:02:28.8078154Z ------------------------------------------
2020-02-28T01:02:28.8080836Z error: internal compiler error: src/librustc_infer/traits/codegen/mod.rs:57: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Sorts(ExpectedFound { expected: impl std::future::Future, found: std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6 x:u8 {WakeOnceThenComplete, ()}]> }))` selecting `Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>)` during codegen
2020-02-28T01:02:28.8083079Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:881:9
2020-02-28T01:02:28.8083555Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-28T01:02:28.8083837Z 
2020-02-28T01:02:28.8084095Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-28T01:02:28.8084095Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-28T01:02:28.8084345Z 
2020-02-28T01:02:28.8085110Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-28T01:02:28.8085428Z 
2020-02-28T01:02:28.8094686Z note: rustc 1.43.0-nightly (a2bdb25fc 2020-02-27) running on x86_64-unknown-linux-gnu
2020-02-28T01:02:28.8094992Z 
2020-02-28T01:02:28.8095862Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-28T01:02:28.8096444Z error: aborting due to previous error
2020-02-28T01:02:28.8096611Z 
2020-02-28T01:02:28.8096708Z 
2020-02-28T01:02:28.8098011Z ------------------------------------------
---
2020-02-28T01:02:28.8100293Z stdout:
2020-02-28T01:02:28.8100666Z ------------------------------------------
2020-02-28T01:02:28.8101089Z ---------------------------------------
2020-02-28T01:02:28.8101336Z trace does not match position list
2020-02-28T01:02:28.8102102Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:109"]
2020-02-28T01:02:28.8102717Z --- stdout
2020-02-28T01:02:28.8103081Z backtrace-debuginfo.rs:109
2020-02-28T01:02:28.8103466Z backtrace-debuginfo.rs:126
2020-02-28T01:02:28.8103851Z backtrace-debuginfo.rs:189
2020-02-28T01:02:28.8103851Z backtrace-debuginfo.rs:189
2020-02-28T01:02:28.8104007Z 
2020-02-28T01:02:28.8104300Z --- stderr
2020-02-28T01:02:28.8104458Z test case 6
2020-02-28T01:02:28.8104993Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-02-28T01:02:28.8105330Z stack backtrace:
2020-02-28T01:02:28.8106011Z    0:     0x7f3ac4b26a96 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3c80db6fc0f0282b
2020-02-28T01:02:28.8106736Z    1:     0x7f3ac4b608fc - core::fmt::write::h564d5e39a82c5e47
2020-02-28T01:02:28.8107364Z    2:     0x7f3ac4b17247 - std::io::Write::write_fmt::hb18044bc440132c8
2020-02-28T01:02:28.8108041Z    3:     0x7f3ac4b2bae4 - std::panicking::default_hook::{{closure}}::h3bfa0b5e727d6127
2020-02-28T01:02:28.8108717Z    4:     0x7f3ac4b2b807 - std::panicking::default_hook::h263a3b8c83bb95ac
2020-02-28T01:02:28.8109398Z    5:     0x7f3ac4b2c145 - std::panicking::rust_panic_with_hook::hf0e6d5712d6b9f0c
2020-02-28T01:02:28.8110059Z    6:     0x5614b2dad942 - std::panicking::begin_panic::h1b09e101992f136c
2020-02-28T01:02:28.8110463Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-28T01:02:28.8111108Z    7:     0x5614b2da6096 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-28T01:02:28.8111755Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-02-28T01:02:28.8112398Z    8:     0x5614b2da63b3 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-28T01:02:28.8113030Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-28T01:02:28.8113659Z    9:     0x5614b2da7321 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-28T01:02:28.8114309Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-28T01:02:28.8114960Z   10:     0x5614b2d9d1ee - std::rt::lang_start::{{closure}}::h920a3537b5ffb253
2020-02-28T01:02:28.8115356Z                                at /checkout/src/libstd/rt.rs:67
2020-02-28T01:02:28.8115970Z   11:     0x7f3ac4b2bc03 - std::panicking::try::do_call::h5c88b1366425125f
2020-02-28T01:02:28.8116525Z   12:     0x7f3ac4b3f1f7 - __rust_maybe_catch_panic
2020-02-28T01:02:28.8117102Z   13:     0x7f3ac4b2c6dc - std::rt::lang_start_internal::h4699f00642ed28be
2020-02-28T01:02:28.8117729Z   14:     0x5614b2d9d1d5 - std::rt::lang_start::h968fef142335f1fc
2020-02-28T01:02:28.8118098Z                                at /checkout/src/libstd/rt.rs:67
2020-02-28T01:02:28.8118552Z   15:     0x5614b2da7492 - main
2020-02-28T01:02:28.8119000Z   16:     0x7f3ac44cfb97 - __libc_start_main
2020-02-28T01:02:28.8119431Z   17:     0x5614b2d96c7a - _start
2020-02-28T01:02:28.8119849Z   18:                0x0 - <unknown>
2020-02-28T01:02:28.8120379Z ---------------------------------------
2020-02-28T01:02:28.8120626Z trace does not match position list
2020-02-28T01:02:28.8120626Z trace does not match position list
2020-02-28T01:02:28.8121375Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:120"]
2020-02-28T01:02:28.8122044Z --- stdout
2020-02-28T01:02:28.8122420Z backtrace-debuginfo.rs:120
2020-02-28T01:02:28.8122892Z backtrace-debuginfo.rs:126
2020-02-28T01:02:28.8123248Z backtrace-debuginfo.rs:189
2020-02-28T01:02:28.8123248Z backtrace-debuginfo.rs:189
2020-02-28T01:02:28.8123394Z 
2020-02-28T01:02:28.8123660Z --- stderr
2020-02-28T01:02:28.8123804Z test case 9
2020-02-28T01:02:28.8124472Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-02-28T01:02:28.8124810Z stack backtrace:
2020-02-28T01:02:28.8125689Z    0:     0x7fbfd37c1a96 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3c80db6fc0f0282b
2020-02-28T01:02:28.8127219Z    1:     0x7fbfd37fb8fc - core::fmt::write::h564d5e39a82c5e47
2020-02-28T01:02:28.8127847Z    2:     0x7fbfd37b2247 - std::io::Write::write_fmt::hb18044bc440132c8
2020-02-28T01:02:28.8128527Z    3:     0x7fbfd37c6ae4 - std::panicking::default_hook::{{closure}}::h3bfa0b5e727d6127
2020-02-28T01:02:28.8129205Z    4:     0x7fbfd37c6807 - std::panicking::default_hook::h263a3b8c83bb95ac
2020-02-28T01:02:28.8129886Z    5:     0x7fbfd37c7145 - std::panicking::rust_panic_with_hook::hf0e6d5712d6b9f0c
2020-02-28T01:02:28.8130541Z    6:     0x55e854ec6942 - std::panicking::begin_panic::h1b09e101992f136c
2020-02-28T01:02:28.8130945Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-28T01:02:28.8131588Z    7:     0x55e854ebf1d0 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-28T01:02:28.8132236Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-02-28T01:02:28.8132866Z    8:     0x55e854ebf3b3 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-28T01:02:28.8133519Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-28T01:02:28.8135134Z    9:     0x55e854ec0321 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-28T01:02:28.8135792Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-28T01:02:28.8136473Z   10:     0x55e854eb61ee - std::rt::lang_start::{{closure}}::h920a3537b5ffb253
2020-02-28T01:02:28.8136988Z                                at /checkout/src/libstd/rt.rs:67
2020-02-28T01:02:28.8137539Z   11:     0x7fbfd37c6c03 - std::panicking::try::do_call::h5c88b1366425125f
2020-02-28T01:02:28.8138068Z   12:     0x7fbfd37da1f7 - __rust_maybe_catch_panic
2020-02-28T01:02:28.8138605Z   13:     0x7fbfd37c76dc - std::rt::lang_start_internal::h4699f00642ed28be
2020-02-28T01:02:28.8139171Z   14:     0x55e854eb61d5 - std::rt::lang_start::h968fef142335f1fc
2020-02-28T01:02:28.8139530Z                                at /checkout/src/libstd/rt.rs:67
2020-02-28T01:02:28.8139953Z   15:     0x55e854ec0492 - main
2020-02-28T01:02:28.8140354Z   16:     0x7fbfd316ab97 - __libc_start_main
2020-02-28T01:02:28.8140766Z   17:     0x55e854eafc7a - _start
2020-02-28T01:02:28.8141147Z   18:                0x0 - <unknown>
2020-02-28T01:02:28.8141392Z 
2020-02-28T01:02:28.8141743Z ------------------------------------------
2020-02-28T01:02:28.8141933Z stderr:
2020-02-28T01:02:28.8142276Z ------------------------------------------
---
2020-02-28T01:02:28.8144649Z ---- [ui] ui/type_length_limit.rs stdout ----
2020-02-28T01:02:28.8144827Z 
2020-02-28T01:02:28.8145029Z error: ui test compiled successfully!
2020-02-28T01:02:28.8145255Z status: exit code: 0
2020-02-28T01:02:28.8147189Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary"
2020-02-28T01:02:28.8148727Z ------------------------------------------
2020-02-28T01:02:28.8148902Z 
2020-02-28T01:02:28.8149264Z ------------------------------------------
2020-02-28T01:02:28.8149467Z stderr:
---
2020-02-28T01:02:28.8154177Z test result: FAILED. 9678 passed; 3 failed; 54 ignored; 0 measured; 0 filtered out
2020-02-28T01:02:28.8154482Z 
2020-02-28T01:02:28.8154585Z 
2020-02-28T01:02:28.8154681Z 
2020-02-28T01:02:28.8163013Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-28T01:02:28.8167143Z 
2020-02-28T01:02:28.8167245Z 
2020-02-28T01:02:28.8167945Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-28T01:02:28.8168364Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-28T01:02:28.8168364Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-28T01:02:28.8169390Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-28T01:02:28.8169741Z Build completed unsuccessfully in 0:59:37
2020-02-28T01:02:28.8170023Z == clock drift check ==
2020-02-28T01:02:28.8181568Z   local time: Fri Feb 28 01:02:28 UTC 2020
2020-02-28T01:02:29.1090977Z   network time: Fri, 28 Feb 2020 01:02:29 GMT
2020-02-28T01:02:29.1091606Z == end clock drift check ==
2020-02-28T01:02:29.6143483Z 
2020-02-28T01:02:29.6211203Z ##[error]Bash exited with code '1'.
2020-02-28T01:02:29.6224855Z ##[section]Finishing: Run build
2020-02-28T01:02:29.6271976Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-02-28T01:02:29.6277040Z Task         : Get sources
2020-02-28T01:02:29.6277400Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-28T01:02:29.6277715Z Version      : 1.0.0
2020-02-28T01:02:29.6277934Z Author       : Microsoft
2020-02-28T01:02:29.6277934Z Author       : Microsoft
2020-02-28T01:02:29.6278304Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-28T01:02:29.6278711Z ==============================================================================
2020-02-28T01:02:29.9448211Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-28T01:02:29.9493578Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-02-28T01:02:29.9575142Z Cleaning up task key
2020-02-28T01:02:29.9576389Z Start cleaning up orphan processes.
2020-02-28T01:02:29.9728852Z Terminate orphan process: pid (4048) (python)
2020-02-28T01:02:29.9965009Z ##[section]Finishing: Finalize Job
