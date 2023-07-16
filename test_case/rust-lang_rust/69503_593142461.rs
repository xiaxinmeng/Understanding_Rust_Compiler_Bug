plain
2020-03-01T19:24:49.9392629Z ========================== Starting Command Output ===========================
2020-03-01T19:24:49.9399120Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c2898291-53af-4e1a-9255-f824d5da49eb.sh
2020-03-01T19:24:49.9399756Z 
2020-03-01T19:24:49.9404681Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-01T19:24:49.9427520Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-03-01T19:24:49.9431507Z Task         : Get sources
2020-03-01T19:24:49.9431870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-01T19:24:49.9432185Z Version      : 1.0.0
2020-03-01T19:24:49.9432400Z Author       : Microsoft
---
2020-03-01T19:24:50.9312715Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-01T19:24:50.9318588Z ##[command]git config gc.auto 0
2020-03-01T19:24:50.9322732Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-01T19:24:50.9326446Z ##[command]git config --get-all http.proxy
2020-03-01T19:24:50.9333316Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69503/merge:refs/remotes/pull/69503/merge
---
2020-03-01T20:26:46.6202977Z .................................................................................................... 1700/9740
2020-03-01T20:26:50.9654197Z .................................................................................................... 1800/9740
2020-03-01T20:27:01.9846054Z ......................................................................i............................. 1900/9740
2020-03-01T20:27:08.1538034Z .................................................................................................... 2000/9740
2020-03-01T20:27:22.6063601Z ............................................................iiiii................................... 2100/9740
2020-03-01T20:27:32.9097920Z .................................................................................................... 2300/9740
2020-03-01T20:27:35.1615860Z .................................................................................................... 2400/9740
2020-03-01T20:27:38.1021004Z .................................................................................................... 2500/9740
2020-03-01T20:27:57.7177661Z .................................................................................................... 2600/9740
---
2020-03-01T20:30:31.8251621Z .....................i...............i.............................................................. 5000/9740
2020-03-01T20:30:41.1617551Z .................................................................................................... 5100/9740
2020-03-01T20:30:46.6014282Z ................................................................i................................... 5200/9740
2020-03-01T20:30:52.8656639Z .................................................................................................... 5300/9740
2020-03-01T20:31:01.2722766Z ..........................................ii.ii........i...i........................................ 5400/9740
2020-03-01T20:31:09.2636170Z .................................................................................................... 5600/9740
2020-03-01T20:31:18.6406914Z .................................................................................................... 5700/9740
2020-03-01T20:31:25.3488189Z .................................i.................................................................. 5800/9740
2020-03-01T20:31:30.9805655Z .................................................................................................... 5900/9740
2020-03-01T20:31:30.9805655Z .................................................................................................... 5900/9740
2020-03-01T20:31:41.4133357Z .................................................................................................... 6000/9740
2020-03-01T20:31:51.1365829Z .........................ii...i..ii...........i..................................................... 6100/9740
2020-03-01T20:32:07.4778743Z .................................................................................................... 6300/9740
2020-03-01T20:32:14.3895418Z .................................................................................................... 6400/9740
2020-03-01T20:32:14.3895418Z .................................................................................................... 6400/9740
2020-03-01T20:32:22.7828929Z ........................................................i..ii....................................... 6500/9740
2020-03-01T20:32:49.0672854Z .................................................................................................... 6700/9740
2020-03-01T20:32:51.4046102Z ................................................i................................................... 6800/9740
2020-03-01T20:32:53.4962408Z .................................................................................................... 6900/9740
2020-03-01T20:32:55.6004820Z ..............................................................................i..................... 7000/9740
---
2020-03-01T20:34:30.3346346Z .................................................................................................... 7700/9740
2020-03-01T20:34:34.7400813Z .................................................................................................... 7800/9740
2020-03-01T20:34:39.8057618Z .................................................................................................... 7900/9740
2020-03-01T20:34:47.6651480Z ........................i........................................................................... 8000/9740
2020-03-01T20:34:55.7147494Z .........................................................................iiiiiii.i.................. 8100/9740
2020-03-01T20:35:11.2692381Z ..............i......i.............................................................................. 8300/9740
2020-03-01T20:35:16.5683465Z .................................................................................................... 8400/9740
2020-03-01T20:35:29.3102548Z .................................................................................................... 8500/9740
2020-03-01T20:35:39.1675178Z .................................................................................................... 8600/9740
---
2020-03-01T20:37:33.0548224Z ---- [ui] ui/async-await/async-closure.rs stdout ----
2020-03-01T20:37:33.0548579Z 
2020-03-01T20:37:33.0549119Z error: test compilation failed although it shouldn't!
2020-03-01T20:37:33.0549718Z status: exit code: 101
2020-03-01T20:37:33.0551623Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/auxiliary"
2020-03-01T20:37:33.0553343Z ------------------------------------------
2020-03-01T20:37:33.0554728Z 
2020-03-01T20:37:33.0555190Z ------------------------------------------
2020-03-01T20:37:33.0555378Z stderr:
2020-03-01T20:37:33.0555378Z stderr:
2020-03-01T20:37:33.0555712Z ------------------------------------------
2020-03-01T20:37:33.0557768Z error: internal compiler error: src/librustc_infer/traits/codegen/mod.rs:57: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Sorts(ExpectedFound { expected: impl std::future::Future, found: std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6 x:u8 {WakeOnceThenComplete, ()}]> }))` selecting `Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>)` during codegen
2020-03-01T20:37:33.0559468Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:881:9
2020-03-01T20:37:33.0559817Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-01T20:37:33.0560029Z 
2020-03-01T20:37:33.0560239Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-01T20:37:33.0560239Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-01T20:37:33.0560424Z 
2020-03-01T20:37:33.0561026Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-01T20:37:33.0561734Z note: rustc 1.43.0-nightly (afd302357 2020-03-01) running on x86_64-unknown-linux-gnu
2020-03-01T20:37:33.0561963Z 
2020-03-01T20:37:33.0561963Z 
2020-03-01T20:37:33.0562528Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-01T20:37:33.0563034Z error: aborting due to previous error
2020-03-01T20:37:33.0563183Z 
2020-03-01T20:37:33.0563268Z 
2020-03-01T20:37:33.0563603Z ------------------------------------------
---
2020-03-01T20:37:33.0565425Z stdout:
2020-03-01T20:37:33.0565752Z ------------------------------------------
2020-03-01T20:37:33.0566141Z ---------------------------------------
2020-03-01T20:37:33.0566482Z trace does not match position list
2020-03-01T20:37:33.0567107Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:109"]
2020-03-01T20:37:33.0567677Z --- stdout
2020-03-01T20:37:33.0567981Z backtrace-debuginfo.rs:109
2020-03-01T20:37:33.0568320Z backtrace-debuginfo.rs:126
2020-03-01T20:37:33.0568674Z backtrace-debuginfo.rs:189
2020-03-01T20:37:33.0568674Z backtrace-debuginfo.rs:189
2020-03-01T20:37:33.0568815Z 
2020-03-01T20:37:33.0569055Z --- stderr
2020-03-01T20:37:33.0569192Z test case 6
2020-03-01T20:37:33.0569682Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-03-01T20:37:33.0569973Z stack backtrace:
2020-03-01T20:37:33.0570684Z    0:     0x7f819b082116 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h01f3d31fa62a15e9
2020-03-01T20:37:33.0571348Z    1:     0x7f819b0bbcec - core::fmt::write::he4eab6d155c79710
2020-03-01T20:37:33.0571893Z    2:     0x7f819b072357 - std::io::Write::write_fmt::h64ba7e374271db48
2020-03-01T20:37:33.0572494Z    3:     0x7f819b087114 - std::panicking::default_hook::{{closure}}::h5b0cd4700da2ce3c
2020-03-01T20:37:33.0573107Z    4:     0x7f819b086e5e - std::panicking::default_hook::h74ca98337656b9ab
2020-03-01T20:37:33.0573698Z    5:     0x7f819b087775 - std::panicking::rust_panic_with_hook::h47f60f1e16dfea17
2020-03-01T20:37:33.0574279Z    6:     0x562abf8296d2 - std::panicking::begin_panic::hd45e56c477c55888
2020-03-01T20:37:33.0574659Z                                at /checkout/src/libstd/panicking.rs:397
2020-03-01T20:37:33.0575285Z    7:     0x562abf81ea16 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-03-01T20:37:33.0575882Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-03-01T20:37:33.0576455Z    8:     0x562abf81ed33 - backtrace_debuginfo::outer::h681fb285db800a84
2020-03-01T20:37:33.0577013Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-03-01T20:37:33.0577588Z    9:     0x562abf81f969 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-03-01T20:37:33.0578150Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-03-01T20:37:33.0578722Z   10:     0x562abf820bfe - std::rt::lang_start::{{closure}}::hdaaeb4763e1c14cd
2020-03-01T20:37:33.0582478Z                                at /checkout/src/libstd/rt.rs:67
2020-03-01T20:37:33.0583255Z   11:     0x7f819b087233 - std::panicking::try::do_call::h101465c0960b71bc
2020-03-01T20:37:33.0583825Z   12:     0x7f819b09a5f7 - __rust_maybe_catch_panic
2020-03-01T20:37:33.0584650Z   13:     0x7f819b087d0c - std::rt::lang_start_internal::hfb490b0e7a907f5b
2020-03-01T20:37:33.0585314Z   14:     0x562abf820be5 - std::rt::lang_start::h336a9d45633bcea4
2020-03-01T20:37:33.0585660Z                                at /checkout/src/libstd/rt.rs:67
2020-03-01T20:37:33.0586081Z   15:     0x562abf81fa22 - main
2020-03-01T20:37:33.0586467Z   16:     0x7f819aa2ab97 - __libc_start_main
2020-03-01T20:37:33.0586849Z   17:     0x562abf812c7a - _start
2020-03-01T20:37:33.0587228Z   18:                0x0 - <unknown>
2020-03-01T20:37:33.0587688Z ---------------------------------------
2020-03-01T20:37:33.0587909Z trace does not match position list
2020-03-01T20:37:33.0587909Z trace does not match position list
2020-03-01T20:37:33.0588501Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:120"]
2020-03-01T20:37:33.0589046Z --- stdout
2020-03-01T20:37:33.0589365Z backtrace-debuginfo.rs:120
2020-03-01T20:37:33.0589707Z backtrace-debuginfo.rs:126
2020-03-01T20:37:33.0590044Z backtrace-debuginfo.rs:189
2020-03-01T20:37:33.0590044Z backtrace-debuginfo.rs:189
2020-03-01T20:37:33.0590186Z 
2020-03-01T20:37:33.0590436Z --- stderr
2020-03-01T20:37:33.0590581Z test case 9
2020-03-01T20:37:33.0591381Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-03-01T20:37:33.0591703Z stack backtrace:
2020-03-01T20:37:33.0592513Z    0:     0x7f7763f67116 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h01f3d31fa62a15e9
2020-03-01T20:37:33.0593212Z    1:     0x7f7763fa0cec - core::fmt::write::he4eab6d155c79710
2020-03-01T20:37:33.0593775Z    2:     0x7f7763f57357 - std::io::Write::write_fmt::h64ba7e374271db48
2020-03-01T20:37:33.0594375Z    3:     0x7f7763f6c114 - std::panicking::default_hook::{{closure}}::h5b0cd4700da2ce3c
2020-03-01T20:37:33.0594969Z    4:     0x7f7763f6be5e - std::panicking::default_hook::h74ca98337656b9ab
2020-03-01T20:37:33.0595575Z    5:     0x7f7763f6c775 - std::panicking::rust_panic_with_hook::h47f60f1e16dfea17
2020-03-01T20:37:33.0596153Z    6:     0x557500b906d2 - std::panicking::begin_panic::hd45e56c477c55888
2020-03-01T20:37:33.0596603Z                                at /checkout/src/libstd/panicking.rs:397
2020-03-01T20:37:33.0597210Z    7:     0x557500b85b50 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-03-01T20:37:33.0597790Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-03-01T20:37:33.0598351Z    8:     0x557500b85d33 - backtrace_debuginfo::outer::h681fb285db800a84
2020-03-01T20:37:33.0598933Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-03-01T20:37:33.0599487Z    9:     0x557500b86969 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-03-01T20:37:33.0600045Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-03-01T20:37:33.0600638Z   10:     0x557500b87bfe - std::rt::lang_start::{{closure}}::hdaaeb4763e1c14cd
2020-03-01T20:37:33.0601538Z   11:     0x7f7763f6c233 - std::panicking::try::do_call::h101465c0960b71bc
2020-03-01T20:37:33.0602034Z   12:     0x7f7763f7f5f7 - __rust_maybe_catch_panic
2020-03-01T20:37:33.0602034Z   12:     0x7f7763f7f5f7 - __rust_maybe_catch_panic
2020-03-01T20:37:33.0602542Z   13:     0x7f7763f6cd0c - std::rt::lang_start_internal::hfb490b0e7a907f5b
2020-03-01T20:37:33.0603102Z   14:     0x557500b87be5 - std::rt::lang_start::h336a9d45633bcea4
2020-03-01T20:37:33.0603439Z                                at /checkout/src/libstd/rt.rs:67
2020-03-01T20:37:33.0603843Z   15:     0x557500b86a22 - main
2020-03-01T20:37:33.0604237Z   16:     0x7f776390fb97 - __libc_start_main
2020-03-01T20:37:33.0604613Z   17:     0x557500b79c7a - _start
2020-03-01T20:37:33.0604974Z   18:                0x0 - <unknown>
2020-03-01T20:37:33.0605229Z 
2020-03-01T20:37:33.0605543Z ------------------------------------------
2020-03-01T20:37:33.0605725Z stderr:
2020-03-01T20:37:33.0606050Z ------------------------------------------
---
2020-03-01T20:37:33.0608106Z ---- [ui] ui/type_length_limit.rs stdout ----
2020-03-01T20:37:33.0608285Z 
2020-03-01T20:37:33.0608448Z error: ui test compiled successfully!
2020-03-01T20:37:33.0608651Z status: exit code: 0
2020-03-01T20:37:33.0610323Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary"
2020-03-01T20:37:33.0611710Z ------------------------------------------
2020-03-01T20:37:33.0611869Z 
2020-03-01T20:37:33.0612190Z ------------------------------------------
2020-03-01T20:37:33.0612450Z stderr:
---
2020-03-01T20:37:33.0619787Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-01T20:37:33.0620324Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-01T20:37:33.0620537Z 
2020-03-01T20:37:33.0620624Z 
2020-03-01T20:37:33.0624000Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-01T20:37:33.0626413Z 
2020-03-01T20:37:33.0626502Z 
2020-03-01T20:37:33.0626729Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-01T20:37:33.0627023Z Build completed unsuccessfully in 1:06:01
2020-03-01T20:37:33.0627023Z Build completed unsuccessfully in 1:06:01
2020-03-01T20:37:33.0647517Z == clock drift check ==
2020-03-01T20:37:33.0668690Z   local time: Sun Mar  1 20:37:33 UTC 2020
2020-03-01T20:37:33.3674128Z   network time: Sun, 01 Mar 2020 20:37:33 GMT
2020-03-01T20:37:33.3681187Z == end clock drift check ==
2020-03-01T20:37:33.8884835Z 
2020-03-01T20:37:33.8956660Z ##[error]Bash exited with code '1'.
2020-03-01T20:37:33.8970486Z ##[section]Finishing: Run build
2020-03-01T20:37:33.9022065Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-03-01T20:37:33.9027238Z Task         : Get sources
2020-03-01T20:37:33.9027596Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-01T20:37:33.9027914Z Version      : 1.0.0
2020-03-01T20:37:33.9028131Z Author       : Microsoft
2020-03-01T20:37:33.9028131Z Author       : Microsoft
2020-03-01T20:37:33.9028501Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-01T20:37:33.9028906Z ==============================================================================
2020-03-01T20:37:34.2168439Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-01T20:37:34.2212082Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-03-01T20:37:34.2304557Z Cleaning up task key
2020-03-01T20:37:34.2305715Z Start cleaning up orphan processes.
2020-03-01T20:37:34.2464830Z Terminate orphan process: pid (4336) (python)
2020-03-01T20:37:34.2710511Z ##[section]Finishing: Finalize Job
