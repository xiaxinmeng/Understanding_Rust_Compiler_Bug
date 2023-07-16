plain
2020-02-29T12:29:50.7188074Z ========================== Starting Command Output ===========================
2020-02-29T12:29:50.7197445Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/af56d86d-f866-4dd3-a4d2-8188cb702011.sh
2020-02-29T12:29:50.7198189Z 
2020-02-29T12:29:50.7205486Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-29T12:29:50.7230034Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-02-29T12:29:50.7235064Z Task         : Get sources
2020-02-29T12:29:50.7235508Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T12:29:50.7235943Z Version      : 1.0.0
2020-02-29T12:29:50.7236257Z Author       : Microsoft
---
2020-02-29T12:29:51.7078881Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-29T12:29:51.7085000Z ##[command]git config gc.auto 0
2020-02-29T12:29:51.7089165Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-29T12:29:51.7093019Z ##[command]git config --get-all http.proxy
2020-02-29T12:29:51.7171725Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69503/merge:refs/remotes/pull/69503/merge
---
2020-02-29T13:34:53.0923700Z .................................................................................................... 1700/9739
2020-02-29T13:34:58.0140466Z .................................................................................................... 1800/9739
2020-02-29T13:35:09.9924515Z ......................................................................i............................. 1900/9739
2020-02-29T13:35:16.6326454Z .................................................................................................... 2000/9739
2020-02-29T13:35:31.9528790Z ............................................................iiiii................................... 2100/9739
2020-02-29T13:35:42.6191371Z .................................................................................................... 2300/9739
2020-02-29T13:35:44.9407679Z .................................................................................................... 2400/9739
2020-02-29T13:35:48.0129093Z .................................................................................................... 2500/9739
2020-02-29T13:36:08.2498225Z .................................................................................................... 2600/9739
---
2020-02-29T13:38:53.5182469Z .....................i...............i.............................................................. 5000/9739
2020-02-29T13:39:03.9788776Z .................................................................................................... 5100/9739
2020-02-29T13:39:10.1459184Z ................................................................i................................... 5200/9739
2020-02-29T13:39:16.9247829Z .................................................................................................... 5300/9739
2020-02-29T13:39:26.2333330Z ..........................................ii.ii........i...i........................................ 5400/9739
2020-02-29T13:39:34.8901536Z .................................................................................................... 5600/9739
2020-02-29T13:39:44.6136602Z .................................................................................................... 5700/9739
2020-02-29T13:39:51.8690057Z .................................i.................................................................. 5800/9739
2020-02-29T13:39:57.9118285Z .................................................................................................... 5900/9739
2020-02-29T13:39:57.9118285Z .................................................................................................... 5900/9739
2020-02-29T13:40:08.8800155Z .................................................................................................... 6000/9739
2020-02-29T13:40:18.5856796Z .........................ii...i..ii...........i..................................................... 6100/9739
2020-02-29T13:40:35.5165542Z .................................................................................................... 6300/9739
2020-02-29T13:40:42.7470872Z .................................................................................................... 6400/9739
2020-02-29T13:40:42.7470872Z .................................................................................................... 6400/9739
2020-02-29T13:40:54.3773601Z ........................................................i..ii....................................... 6500/9739
2020-02-29T13:41:21.6236308Z .................................................................................................... 6700/9739
2020-02-29T13:41:23.9755989Z ................................................i................................................... 6800/9739
2020-02-29T13:41:26.0243165Z .................................................................................................... 6900/9739
2020-02-29T13:41:28.2075630Z ..............................................................................i..................... 7000/9739
---
2020-02-29T13:43:05.3305331Z .................................................................................................... 7700/9739
2020-02-29T13:43:09.8865538Z .................................................................................................... 7800/9739
2020-02-29T13:43:15.3273901Z .................................................................................................... 7900/9739
2020-02-29T13:43:23.6390778Z ........................i........................................................................... 8000/9739
2020-02-29T13:43:32.6973155Z .........................................................................iiiiiii.i.................. 8100/9739
2020-02-29T13:43:49.3471757Z ..............i......i.............................................................................. 8300/9739
2020-02-29T13:43:54.7411448Z .................................................................................................... 8400/9739
2020-02-29T13:44:07.9592098Z .................................................................................................... 8500/9739
2020-02-29T13:44:18.3384738Z .................................................................................................... 8600/9739
---
2020-02-29T13:46:14.3161863Z ---- [ui] ui/async-await/async-closure.rs stdout ----
2020-02-29T13:46:14.3162382Z 
2020-02-29T13:46:14.3163356Z error: test compilation failed although it shouldn't!
2020-02-29T13:46:14.3163982Z status: exit code: 101
2020-02-29T13:46:14.3166457Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/auxiliary"
2020-02-29T13:46:14.3168374Z ------------------------------------------
2020-02-29T13:46:14.3168599Z 
2020-02-29T13:46:14.3169032Z ------------------------------------------
2020-02-29T13:46:14.3169264Z stderr:
2020-02-29T13:46:14.3169264Z stderr:
2020-02-29T13:46:14.3169831Z ------------------------------------------
2020-02-29T13:46:14.3172547Z error: internal compiler error: src/librustc_infer/traits/codegen/mod.rs:57: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Sorts(ExpectedFound { expected: impl std::future::Future, found: std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6 x:u8 {WakeOnceThenComplete, ()}]> }))` selecting `Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>)` during codegen
2020-02-29T13:46:14.3174748Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:881:9
2020-02-29T13:46:14.3175193Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-29T13:46:14.3175467Z 
2020-02-29T13:46:14.3175744Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-29T13:46:14.3175744Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-29T13:46:14.3176273Z 
2020-02-29T13:46:14.3177152Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-29T13:46:14.3178088Z note: rustc 1.43.0-nightly (9067b46d3 2020-02-29) running on x86_64-unknown-linux-gnu
2020-02-29T13:46:14.3178392Z 
2020-02-29T13:46:14.3178392Z 
2020-02-29T13:46:14.3179128Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-29T13:46:14.3179777Z error: aborting due to previous error
2020-02-29T13:46:14.3179966Z 
2020-02-29T13:46:14.3180076Z 
2020-02-29T13:46:14.3180519Z ------------------------------------------
---
2020-02-29T13:46:14.3182861Z stdout:
2020-02-29T13:46:14.3183287Z ------------------------------------------
2020-02-29T13:46:14.3184493Z ---------------------------------------
2020-02-29T13:46:14.3184795Z trace does not match position list
2020-02-29T13:46:14.3185556Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:109"]
2020-02-29T13:46:14.3186286Z --- stdout
2020-02-29T13:46:14.3186687Z backtrace-debuginfo.rs:109
2020-02-29T13:46:14.3187130Z backtrace-debuginfo.rs:126
2020-02-29T13:46:14.3187591Z backtrace-debuginfo.rs:189
2020-02-29T13:46:14.3187591Z backtrace-debuginfo.rs:189
2020-02-29T13:46:14.3187769Z 
2020-02-29T13:46:14.3188079Z --- stderr
2020-02-29T13:46:14.3188256Z test case 6
2020-02-29T13:46:14.3188888Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-02-29T13:46:14.3189258Z stack backtrace:
2020-02-29T13:46:14.3190297Z    0:     0x7f0b42354b36 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4fade9eb39e821e8
2020-02-29T13:46:14.3191157Z    1:     0x7f0b4238e9bc - core::fmt::write::h9c897e8ac5214c30
2020-02-29T13:46:14.3191970Z    2:     0x7f0b42344e17 - std::io::Write::write_fmt::h710dc4a75ed1eec9
2020-02-29T13:46:14.3192799Z    3:     0x7f0b42359b84 - std::panicking::default_hook::{{closure}}::hec4073609394c564
2020-02-29T13:46:14.3193594Z    4:     0x7f0b423598a7 - std::panicking::default_hook::h61c9b023c2feb166
2020-02-29T13:46:14.3194350Z    5:     0x7f0b4235a1e5 - std::panicking::rust_panic_with_hook::h79c3f433971a9014
2020-02-29T13:46:14.3195091Z    6:     0x55762fada1c2 - std::panicking::begin_panic::h595b8d8e54975b97
2020-02-29T13:46:14.3195577Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-29T13:46:14.3196295Z    7:     0x55762facf4c6 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-29T13:46:14.3197066Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-02-29T13:46:14.3197786Z    8:     0x55762facf7e3 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-29T13:46:14.3198505Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-29T13:46:14.3199244Z    9:     0x55762fad0499 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-29T13:46:14.3199963Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-29T13:46:14.3200698Z   10:     0x55762fad16be - std::rt::lang_start::{{closure}}::hb6af6e19dc7e557f
2020-02-29T13:46:14.3201174Z                                at /checkout/src/libstd/rt.rs:67
2020-02-29T13:46:14.3201854Z   11:     0x7f0b42359ca3 - std::panicking::try::do_call::h46b4055ecc2edadc
2020-02-29T13:46:14.3202484Z   12:     0x7f0b4236d2d7 - __rust_maybe_catch_panic
2020-02-29T13:46:14.3203160Z   13:     0x7f0b4235a77c - std::rt::lang_start_internal::h277d40e83abbfdfc
2020-02-29T13:46:14.3203871Z   14:     0x55762fad16a5 - std::rt::lang_start::h444f08f42fa40e4e
2020-02-29T13:46:14.3204293Z                                at /checkout/src/libstd/rt.rs:67
2020-02-29T13:46:14.3204827Z   15:     0x55762fad0552 - main
2020-02-29T13:46:14.3205323Z   16:     0x7f0b41cfdb97 - __libc_start_main
2020-02-29T13:46:14.3205811Z   17:     0x55762fac3c7a - _start
2020-02-29T13:46:14.3206301Z   18:                0x0 - <unknown>
2020-02-29T13:46:14.3206890Z ---------------------------------------
2020-02-29T13:46:14.3207168Z trace does not match position list
2020-02-29T13:46:14.3207168Z trace does not match position list
2020-02-29T13:46:14.3207929Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:120"]
2020-02-29T13:46:14.3208626Z --- stdout
2020-02-29T13:46:14.3209036Z backtrace-debuginfo.rs:120
2020-02-29T13:46:14.3209474Z backtrace-debuginfo.rs:126
2020-02-29T13:46:14.3209909Z backtrace-debuginfo.rs:189
2020-02-29T13:46:14.3209909Z backtrace-debuginfo.rs:189
2020-02-29T13:46:14.3210088Z 
2020-02-29T13:46:14.3210428Z --- stderr
2020-02-29T13:46:14.3210605Z test case 9
2020-02-29T13:46:14.3211208Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-02-29T13:46:14.3211595Z stack backtrace:
2020-02-29T13:46:14.3212376Z    0:     0x7f03165b6b36 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4fade9eb39e821e8
2020-02-29T13:46:14.3213204Z    1:     0x7f03165f09bc - core::fmt::write::h9c897e8ac5214c30
2020-02-29T13:46:14.3213917Z    2:     0x7f03165a6e17 - std::io::Write::write_fmt::h710dc4a75ed1eec9
2020-02-29T13:46:14.3215351Z    3:     0x7f03165bbb84 - std::panicking::default_hook::{{closure}}::hec4073609394c564
2020-02-29T13:46:14.3216134Z    4:     0x7f03165bb8a7 - std::panicking::default_hook::h61c9b023c2feb166
2020-02-29T13:46:14.3216921Z    5:     0x7f03165bc1e5 - std::panicking::rust_panic_with_hook::h79c3f433971a9014
2020-02-29T13:46:14.3217663Z    6:     0x55b0da2511c2 - std::panicking::begin_panic::h595b8d8e54975b97
2020-02-29T13:46:14.3218322Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-29T13:46:14.3231532Z    7:     0x55b0da246600 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-29T13:46:14.3232978Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-02-29T13:46:14.3233857Z    8:     0x55b0da2467e3 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-29T13:46:14.3234624Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-29T13:46:14.3235270Z    9:     0x55b0da247499 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-29T13:46:14.3236118Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-29T13:46:14.3236758Z   10:     0x55b0da2486be - std::rt::lang_start::{{closure}}::hb6af6e19dc7e557f
2020-02-29T13:46:14.3237138Z                                at /checkout/src/libstd/rt.rs:67
2020-02-29T13:46:14.3237719Z   11:     0x7f03165bbca3 - std::panicking::try::do_call::h46b4055ecc2edadc
2020-02-29T13:46:14.3238263Z   12:     0x7f03165cf2d7 - __rust_maybe_catch_panic
2020-02-29T13:46:14.3238810Z   13:     0x7f03165bc77c - std::rt::lang_start_internal::h277d40e83abbfdfc
2020-02-29T13:46:14.3239411Z   14:     0x55b0da2486a5 - std::rt::lang_start::h444f08f42fa40e4e
2020-02-29T13:46:14.3239768Z                                at /checkout/src/libstd/rt.rs:67
2020-02-29T13:46:14.3240202Z   15:     0x55b0da247552 - main
2020-02-29T13:46:14.3240634Z   16:     0x7f0315f5fb97 - __libc_start_main
2020-02-29T13:46:14.3241103Z   17:     0x55b0da23ac7a - _start
2020-02-29T13:46:14.3241497Z   18:                0x0 - <unknown>
2020-02-29T13:46:14.3241770Z 
2020-02-29T13:46:14.3242113Z ------------------------------------------
2020-02-29T13:46:14.3242306Z stderr:
2020-02-29T13:46:14.3242656Z ------------------------------------------
---
2020-02-29T13:46:14.3244856Z ---- [ui] ui/type_length_limit.rs stdout ----
2020-02-29T13:46:14.3245047Z 
2020-02-29T13:46:14.3245220Z error: ui test compiled successfully!
2020-02-29T13:46:14.3245436Z status: exit code: 0
2020-02-29T13:46:14.3247203Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary"
2020-02-29T13:46:14.3248678Z ------------------------------------------
2020-02-29T13:46:14.3248845Z 
2020-02-29T13:46:14.3249192Z ------------------------------------------
2020-02-29T13:46:14.3249383Z stderr:
---
2020-02-29T13:46:14.3259413Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-29T13:46:14.3259854Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-29T13:46:14.3272374Z 
2020-02-29T13:46:14.3272559Z 
2020-02-29T13:46:14.3334883Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-29T13:46:14.3338038Z 
2020-02-29T13:46:14.3338157Z 
2020-02-29T13:46:14.3338492Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-29T13:46:14.3338868Z Build completed unsuccessfully in 1:08:44
2020-02-29T13:46:14.3338868Z Build completed unsuccessfully in 1:08:44
2020-02-29T13:46:14.3340240Z == clock drift check ==
2020-02-29T13:46:14.3371743Z   local time: Sat Feb 29 13:46:14 UTC 2020
2020-02-29T13:46:14.6209195Z   network time: Sat, 29 Feb 2020 13:46:14 GMT
2020-02-29T13:46:14.6209550Z == end clock drift check ==
2020-02-29T13:46:15.0506625Z 
2020-02-29T13:46:15.0594824Z ##[error]Bash exited with code '1'.
2020-02-29T13:46:15.0610843Z ##[section]Finishing: Run build
2020-02-29T13:46:15.0703118Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-02-29T13:46:15.0709443Z Task         : Get sources
2020-02-29T13:46:15.0709933Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T13:46:15.0710389Z Version      : 1.0.0
2020-02-29T13:46:15.0710716Z Author       : Microsoft
2020-02-29T13:46:15.0710716Z Author       : Microsoft
2020-02-29T13:46:15.0711215Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-29T13:46:15.0711807Z ==============================================================================
2020-02-29T13:46:15.4288854Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-29T13:46:15.4347143Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-02-29T13:46:15.4449684Z Cleaning up task key
2020-02-29T13:46:15.4451234Z Start cleaning up orphan processes.
2020-02-29T13:46:15.4659968Z Terminate orphan process: pid (5576) (python)
2020-02-29T13:46:15.4901310Z ##[section]Finishing: Finalize Job
