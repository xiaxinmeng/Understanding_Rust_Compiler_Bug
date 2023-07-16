plain
2020-01-14T13:27:53.0542787Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T13:27:53.0638618Z ##[command]git config gc.auto 0
2020-01-14T13:27:53.0721349Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T13:27:53.0768356Z ##[command]git config --get-all http.proxy
2020-01-14T13:27:53.0907000Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68213/merge:refs/remotes/pull/68213/merge
---
2020-01-14T14:17:53.7295446Z .................................................................................................... 1700/9523
2020-01-14T14:18:01.0087541Z .................................................................................................... 1800/9523
2020-01-14T14:18:09.6307928Z ............i....................................................................................... 1900/9523
2020-01-14T14:18:15.9110950Z .................................................................................................... 2000/9523
2020-01-14T14:18:29.4477132Z ..iiiii............................................................................................. 2100/9523
2020-01-14T14:18:37.1955318Z .................................................................................................... 2300/9523
2020-01-14T14:18:39.4339599Z .................................................................................................... 2400/9523
2020-01-14T14:18:44.4350691Z .................................................................................................... 2500/9523
2020-01-14T14:19:03.3660577Z .................................................................................................... 2600/9523
2020-01-14T14:19:03.3660577Z .................................................................................................... 2600/9523
2020-01-14T14:19:05.7977227Z .................................................................................................... 2700/9523
2020-01-14T14:19:14.8592919Z ..........................................................................i......................... 2800/9523
2020-01-14T14:19:20.5877334Z .................................................................................................... 2900/9523
2020-01-14T14:19:27.8651073Z .................................................................................................... 3000/9523
2020-01-14T14:19:32.5639234Z ...........i........................................................................................ 3100/9523
2020-01-14T14:19:40.6682455Z .................................................................................................... 3200/9523
2020-01-14T14:19:44.7194586Z ................................................................................................ii.. 3300/9523
2020-01-14T14:19:59.1628309Z .................................................................................................... 3500/9523
2020-01-14T14:20:06.2306785Z .......................................................................................i............ 3600/9523
2020-01-14T14:20:12.0830981Z ..................................i................................................................. 3700/9523
2020-01-14T14:20:17.1526082Z .................................................................................................... 3800/9523
---
2020-01-14T14:21:27.8405037Z .............................................i...............i...................................... 4900/9523
2020-01-14T14:21:36.0473939Z .................................................................................................... 5000/9523
2020-01-14T14:21:42.4260103Z ........................................................................................i........... 5100/9523
2020-01-14T14:21:47.0737069Z .................................................................................................... 5200/9523
2020-01-14T14:21:56.2373095Z ...........................................................ii.ii...........i........................ 5300/9523
2020-01-14T14:22:04.8282238Z .................................................................................................... 5500/9523
2020-01-14T14:22:14.0686211Z .................................................................................................... 5600/9523
2020-01-14T14:22:19.8050468Z ............................................i....................................................... 5700/9523
2020-01-14T14:22:26.8718737Z .................................................................................................... 5800/9523
2020-01-14T14:22:26.8718737Z .................................................................................................... 5800/9523
2020-01-14T14:22:36.5776819Z .................................................................................................... 5900/9523
2020-01-14T14:22:45.1567931Z ...................................ii...i..ii...........i........................................... 6000/9523
2020-01-14T14:23:02.9633588Z .................................................................................................... 6200/9523
2020-01-14T14:23:10.2896945Z .................................................................................................... 6300/9523
2020-01-14T14:23:10.2896945Z .................................................................................................... 6300/9523
2020-01-14T14:23:19.2133017Z ...............................................................i..ii................................ 6400/9523
2020-01-14T14:23:45.1253944Z .................................................................................................... 6600/9523
2020-01-14T14:23:47.2198589Z .......................................i............................................................ 6700/9523
2020-01-14T14:23:49.2335759Z .................................................................................................... 6800/9523
2020-01-14T14:23:51.6368441Z .......................................i............................................................ 6900/9523
---
2020-01-14T14:25:17.0145838Z .................................................................................................... 7500/9523
2020-01-14T14:25:20.9819947Z .................................................................................................... 7600/9523
2020-01-14T14:25:26.3773447Z .................................................................................................... 7700/9523
2020-01-14T14:25:32.3709877Z .................................................................................................... 7800/9523
2020-01-14T14:25:41.3196422Z ........................................................................................iiii........ 7900/9523
2020-01-14T14:25:55.9645395Z ......................i......i...................................................................... 8100/9523
2020-01-14T14:26:00.3807259Z .................................................................................................... 8200/9523
2020-01-14T14:26:11.2055753Z .................................................................................................... 8300/9523
2020-01-14T14:26:20.5019834Z .................................................................................................... 8400/9523
---
2020-01-14T14:28:02.9906472Z ---- [ui] ui/async-await/async-closure.rs stdout ----
2020-01-14T14:28:02.9906677Z 
2020-01-14T14:28:02.9907045Z error: test compilation failed although it shouldn't!
2020-01-14T14:28:02.9907214Z status: exit code: 101
2020-01-14T14:28:02.9908404Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/auxiliary"
2020-01-14T14:28:02.9909486Z ------------------------------------------
2020-01-14T14:28:02.9909611Z 
2020-01-14T14:28:02.9909909Z ------------------------------------------
2020-01-14T14:28:02.9910041Z stderr:
2020-01-14T14:28:02.9910041Z stderr:
2020-01-14T14:28:02.9910479Z ------------------------------------------
2020-01-14T14:28:02.9911314Z error: internal compiler error: src/librustc/traits/codegen/mod.rs:57: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Sorts(ExpectedFound { expected: impl std::future::Future, found: std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6 x:u8 {WakeOnceThenComplete, WakeOnceThenComplete, ()}]> }))` selecting `Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>)` during codegen
2020-01-14T14:28:02.9911963Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:872:9
2020-01-14T14:28:02.9912103Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-14T14:28:02.9912221Z 
2020-01-14T14:28:02.9912328Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-14T14:28:02.9912328Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-14T14:28:02.9912415Z 
2020-01-14T14:28:02.9913001Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-01-14T14:28:02.9913144Z 
2020-01-14T14:28:02.9913457Z note: rustc 1.42.0-nightly (fbb8c560f 2020-01-14) running on x86_64-unknown-linux-gnu
2020-01-14T14:28:02.9913589Z 
2020-01-14T14:28:02.9913931Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-01-14T14:28:02.9914178Z error: aborting due to previous error
2020-01-14T14:28:02.9914264Z 
2020-01-14T14:28:02.9914344Z 
2020-01-14T14:28:02.9914622Z ------------------------------------------
---
2020-01-14T14:28:02.9916605Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/backtrace-debuginfo/a"
2020-01-14T14:28:02.9916795Z stdout:
2020-01-14T14:28:02.9917123Z ------------------------------------------
2020-01-14T14:28:02.9917507Z ---------------------------------------
2020-01-14T14:28:02.9917668Z trace does not match position list
2020-01-14T14:28:02.9918080Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:109"]
2020-01-14T14:28:02.9918554Z --- stdout
2020-01-14T14:28:02.9919048Z backtrace-debuginfo.rs:109
2020-01-14T14:28:02.9919523Z backtrace-debuginfo.rs:126
2020-01-14T14:28:02.9919837Z backtrace-debuginfo.rs:189
2020-01-14T14:28:02.9919837Z backtrace-debuginfo.rs:189
2020-01-14T14:28:02.9919945Z 
2020-01-14T14:28:02.9920199Z --- stderr
2020-01-14T14:28:02.9920594Z test case 6
2020-01-14T14:28:02.9921347Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-01-14T14:28:02.9921415Z stack backtrace:
2020-01-14T14:28:02.9922563Z    0:     0x7fc9a432ec04 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h98d260756fee71e7
2020-01-14T14:28:02.9922793Z    1:     0x7fc9a436208c - core::fmt::write::he32f996c599ce729
2020-01-14T14:28:02.9923001Z    2:     0x7fc9a4320797 - std::io::Write::write_fmt::hbf823754d6cb75bb
2020-01-14T14:28:02.9923519Z    3:     0x7fc9a43332b4 - std::panicking::default_hook::{{closure}}::h3f261edee17a90a8
2020-01-14T14:28:02.9923801Z    4:     0x7fc9a4332fc9 - std::panicking::default_hook::hb4584ff9fed16986
2020-01-14T14:28:02.9924034Z    5:     0x7fc9a433391d - std::panicking::rust_panic_with_hook::he33a67e09024ef85
2020-01-14T14:28:02.9924269Z    6:     0x55b091419d0f - std::panicking::begin_panic::hf1dd9a1f3235c9d9
2020-01-14T14:28:02.9924314Z                                at /checkout/src/libstd/panicking.rs:390
2020-01-14T14:28:02.9924537Z    7:     0x55b091415e6c - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-01-14T14:28:02.9924770Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-01-14T14:28:02.9925150Z    8:     0x55b091416183 - backtrace_debuginfo::outer::h681fb285db800a84
2020-01-14T14:28:02.9925778Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-01-14T14:28:02.9926110Z    9:     0x55b091417473 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-01-14T14:28:02.9926381Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-01-14T14:28:02.9926644Z   10:     0x55b091409352 - std::rt::lang_start::{{closure}}::h516f275649ba7bad
2020-01-14T14:28:02.9926718Z                                at /checkout/src/libstd/rt.rs:67
2020-01-14T14:28:02.9926976Z   11:     0x7fc9a43333e3 - std::panicking::try::do_call::h59d6fcfd55920c02
2020-01-14T14:28:02.9927208Z   12:     0x7fc9a4345b6a - __rust_maybe_catch_panic
2020-01-14T14:28:02.9927483Z   13:     0x7fc9a4333eb3 - std::rt::lang_start_internal::hac6b02f39f1ed663
2020-01-14T14:28:02.9927732Z   14:     0x55b091409338 - std::rt::lang_start::hea2299236b895c14
2020-01-14T14:28:02.9927786Z                                at /checkout/src/libstd/rt.rs:67
2020-01-14T14:28:02.9928024Z   15:     0x55b091417792 - main
2020-01-14T14:28:02.9928248Z   16:     0x7fc9a3cdbb97 - __libc_start_main
2020-01-14T14:28:02.9928460Z   17:     0x55b091401aba - _start
2020-01-14T14:28:02.9928816Z   18:                0x0 - <unknown>
2020-01-14T14:28:02.9929013Z ---------------------------------------
2020-01-14T14:28:02.9929051Z trace does not match position list
2020-01-14T14:28:02.9929051Z trace does not match position list
2020-01-14T14:28:02.9929294Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:120"]
2020-01-14T14:28:02.9929465Z --- stdout
2020-01-14T14:28:02.9929619Z backtrace-debuginfo.rs:120
2020-01-14T14:28:02.9929792Z backtrace-debuginfo.rs:126
2020-01-14T14:28:02.9929947Z backtrace-debuginfo.rs:189
2020-01-14T14:28:02.9929947Z backtrace-debuginfo.rs:189
2020-01-14T14:28:02.9929970Z 
2020-01-14T14:28:02.9930124Z --- stderr
2020-01-14T14:28:02.9930160Z test case 9
2020-01-14T14:28:02.9930372Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-01-14T14:28:02.9930419Z stack backtrace:
2020-01-14T14:28:02.9930853Z    0:     0x7fc19b59fc04 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h98d260756fee71e7
2020-01-14T14:28:02.9931050Z    1:     0x7fc19b5d308c - core::fmt::write::he32f996c599ce729
2020-01-14T14:28:02.9931429Z    2:     0x7fc19b591797 - std::io::Write::write_fmt::hbf823754d6cb75bb
2020-01-14T14:28:02.9931667Z    3:     0x7fc19b5a42b4 - std::panicking::default_hook::{{closure}}::h3f261edee17a90a8
2020-01-14T14:28:02.9931877Z    4:     0x7fc19b5a3fc9 - std::panicking::default_hook::hb4584ff9fed16986
2020-01-14T14:28:02.9932092Z    5:     0x7fc19b5a491d - std::panicking::rust_panic_with_hook::he33a67e09024ef85
2020-01-14T14:28:02.9932318Z    6:     0x55b3128c7d0f - std::panicking::begin_panic::hf1dd9a1f3235c9d9
2020-01-14T14:28:02.9932361Z                                at /checkout/src/libstd/panicking.rs:390
2020-01-14T14:28:02.9932574Z    7:     0x55b3128c3fae - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-01-14T14:28:02.9932813Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-01-14T14:28:02.9933022Z    8:     0x55b3128c4183 - backtrace_debuginfo::outer::h681fb285db800a84
2020-01-14T14:28:02.9933311Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-01-14T14:28:02.9933569Z    9:     0x55b3128c5473 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-01-14T14:28:02.9933780Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-01-14T14:28:02.9933995Z   10:     0x55b3128b7352 - std::rt::lang_start::{{closure}}::h516f275649ba7bad
2020-01-14T14:28:02.9934056Z                                at /checkout/src/libstd/rt.rs:67
2020-01-14T14:28:02.9934265Z   11:     0x7fc19b5a43e3 - std::panicking::try::do_call::h59d6fcfd55920c02
2020-01-14T14:28:02.9934783Z   12:     0x7fc19b5b6b6a - __rust_maybe_catch_panic
2020-01-14T14:28:02.9934999Z   13:     0x7fc19b5a4eb3 - std::rt::lang_start_internal::hac6b02f39f1ed663
2020-01-14T14:28:02.9935280Z   14:     0x55b3128b7338 - std::rt::lang_start::hea2299236b895c14
2020-01-14T14:28:02.9935321Z                                at /checkout/src/libstd/rt.rs:67
2020-01-14T14:28:02.9935547Z   15:     0x55b3128c5792 - main
2020-01-14T14:28:02.9936246Z   16:     0x7fc19af4cb97 - __libc_start_main
2020-01-14T14:28:02.9936467Z   17:     0x55b3128afaba - _start
2020-01-14T14:28:02.9936698Z   18:                0x0 - <unknown>
2020-01-14T14:28:02.9936755Z 
2020-01-14T14:28:02.9936964Z ------------------------------------------
2020-01-14T14:28:02.9937026Z stderr:
2020-01-14T14:28:02.9937239Z ------------------------------------------
---
2020-01-14T14:28:02.9937900Z 
2020-01-14T14:28:02.9938667Z ---- [ui] ui/issues/issue-22638.rs stdout ----
2020-01-14T14:28:02.9938723Z diff of stderr:
2020-01-14T14:28:02.9938752Z 
2020-01-14T14:28:02.9939040Z - error: reached the type-length limit while instantiating `D::matches::$CLOSURE`
2020-01-14T14:28:02.9939427Z -   --> $DIR/issue-22638.rs:53:5
2020-01-14T14:28:02.9939812Z + error: reached the type-length limit while instantiating `A::matches::$CLOSURE`
2020-01-14T14:28:02.9939991Z +   --> $DIR/issue-22638.rs:14:5
2020-01-14T14:28:02.9940026Z 3    |
2020-01-14T14:28:02.9940061Z 4 LL | /     pub fn matches<F: Fn()>(&self, f: &F) {
2020-01-14T14:28:02.9940201Z - LL | |
2020-01-14T14:28:02.9940377Z - LL | |         let &D(ref a) = self;
2020-01-14T14:28:02.9940532Z - LL | |         a.matches(f)
2020-01-14T14:28:02.9940570Z + LL | |         let &A(ref term) = self;
2020-01-14T14:28:02.9940801Z + LL | |         term.matches(f);
2020-01-14T14:28:02.9940875Z 9    | |_____^
2020-01-14T14:28:02.9940923Z 10    |
2020-01-14T14:28:02.9940944Z 
2020-01-14T14:28:02.9940964Z 
2020-01-14T14:28:02.9940964Z 
2020-01-14T14:28:02.9940999Z The actual stderr differed from the expected stderr.
2020-01-14T14:28:02.9941244Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638/issue-22638.stderr
2020-01-14T14:28:02.9941636Z To update references, rerun the tests and pass the `--bless` flag
2020-01-14T14:28:02.9941847Z To only update this specific test, also pass `--test-args issues/issue-22638.rs`
2020-01-14T14:28:02.9942117Z error: 1 errors occurred comparing output.
2020-01-14T14:28:02.9942155Z status: exit code: 1
2020-01-14T14:28:02.9942155Z status: exit code: 1
2020-01-14T14:28:02.9943168Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22638.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638/auxiliary" "-A" "unused"
2020-01-14T14:28:02.9943501Z ------------------------------------------
2020-01-14T14:28:02.9943548Z 
2020-01-14T14:28:02.9943734Z ------------------------------------------
2020-01-14T14:28:02.9943772Z stderr:
2020-01-14T14:28:02.9943772Z stderr:
2020-01-14T14:28:02.9943965Z ------------------------------------------
2020-01-14T14:28:02.9944225Z error: reached the type-length limit while instantiating `A::matches::<[closure@/checkout/...s/issue-22638.rs:30:33: 30:38]]>`
2020-01-14T14:28:02.9944431Z   --> /checkout/src/test/ui/issues/issue-22638.rs:14:5
2020-01-14T14:28:02.9944488Z    |
2020-01-14T14:28:02.9944527Z LL | /     pub fn matches<F: Fn()>(&self, f: &F) {
2020-01-14T14:28:02.9944647Z LL | |         let &A(ref term) = self;
2020-01-14T14:28:02.9944839Z LL | |         term.matches(f);
2020-01-14T14:28:02.9944925Z    | |_____^
2020-01-14T14:28:02.9944958Z    |
2020-01-14T14:28:02.9944958Z    |
2020-01-14T14:28:02.9945377Z    = note: consider adding a `#![type_length_limit="26214380"]` attribute to your crate
2020-01-14T14:28:02.9945464Z error: aborting due to previous error
2020-01-14T14:28:02.9945493Z 
2020-01-14T14:28:02.9945536Z 
2020-01-14T14:28:02.9945805Z ------------------------------------------
2020-01-14T14:28:02.9945805Z ------------------------------------------
2020-01-14T14:28:02.9945837Z 
2020-01-14T14:28:02.9945863Z 
2020-01-14T14:28:02.9946077Z ---- [ui] ui/type_length_limit.rs stdout ----
2020-01-14T14:28:02.9946127Z 
2020-01-14T14:28:02.9946168Z error: ui test compiled successfully!
2020-01-14T14:28:02.9946213Z status: exit code: 0
2020-01-14T14:28:02.9947010Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary" "-A" "unused"
2020-01-14T14:28:02.9947342Z ------------------------------------------
2020-01-14T14:28:02.9947375Z 
2020-01-14T14:28:02.9947589Z ------------------------------------------
2020-01-14T14:28:02.9947650Z stderr:
---
2020-01-14T14:28:02.9962757Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-14T14:28:02.9962853Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-14T14:28:02.9975222Z 
2020-01-14T14:28:02.9975302Z 
2020-01-14T14:28:02.9977716Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-14T14:28:02.9978069Z 
2020-01-14T14:28:02.9978121Z 
2020-01-14T14:28:02.9983810Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-14T14:28:02.9983880Z Build completed unsuccessfully in 0:54:54
2020-01-14T14:28:02.9983880Z Build completed unsuccessfully in 0:54:54
2020-01-14T14:28:03.0033304Z == clock drift check ==
2020-01-14T14:28:03.0054241Z   local time: Tue Jan 14 14:28:03 UTC 2020
2020-01-14T14:28:03.5529120Z   network time: Tue, 14 Jan 2020 14:28:03 GMT
2020-01-14T14:28:03.5531966Z == end clock drift check ==
2020-01-14T14:28:03.9804286Z 
2020-01-14T14:28:03.9865746Z ##[error]Bash exited with code '1'.
2020-01-14T14:28:03.9906407Z ##[section]Starting: Checkout
2020-01-14T14:28:03.9908096Z ==============================================================================
2020-01-14T14:28:03.9908151Z Task         : Get sources
2020-01-14T14:28:03.9908232Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
