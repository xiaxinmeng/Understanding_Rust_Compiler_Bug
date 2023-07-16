plain
2020-02-27T00:53:35.7758347Z ========================== Starting Command Output ===========================
2020-02-27T00:53:35.7762650Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/53a3cf22-e32f-4c4b-b7b1-70f5b122b7c3.sh
2020-02-27T00:53:35.7762866Z 
2020-02-27T00:53:35.7766647Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-27T00:53:35.7782271Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-02-27T00:53:35.7786296Z Task         : Get sources
2020-02-27T00:53:35.7786755Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-27T00:53:35.7786974Z Version      : 1.0.0
2020-02-27T00:53:35.7787121Z Author       : Microsoft
---
2020-02-27T00:53:36.7787686Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-27T00:53:36.7791863Z ##[command]git config gc.auto 0
2020-02-27T00:53:36.7794913Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-27T00:53:36.7797422Z ##[command]git config --get-all http.proxy
2020-02-27T00:53:36.7803963Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69503/merge:refs/remotes/pull/69503/merge
---
2020-02-27T01:47:14.3275516Z .................................................................................................... 1700/9733
2020-02-27T01:47:18.1325135Z .................................................................................................... 1800/9733
2020-02-27T01:47:27.3004023Z ....................................................................i............................... 1900/9733
2020-02-27T01:47:33.2843051Z .................................................................................................... 2000/9733
2020-02-27T01:47:44.9968757Z ..........................................................iiiii..................................... 2100/9733
2020-02-27T01:47:53.8075497Z .................................................................................................... 2300/9733
2020-02-27T01:47:55.6488894Z .................................................................................................... 2400/9733
2020-02-27T01:47:58.4167282Z .................................................................................................... 2500/9733
2020-02-27T01:48:17.0087453Z .................................................................................................... 2600/9733
---
2020-02-27T01:50:28.2729025Z ..................i...............i................................................................. 5000/9733
2020-02-27T01:50:36.1581529Z .................................................................................................... 5100/9733
2020-02-27T01:50:40.7165873Z .............................................................i...................................... 5200/9733
2020-02-27T01:50:45.8582177Z .................................................................................................... 5300/9733
2020-02-27T01:50:52.6574042Z ......................................ii.ii........i...i............................................ 5400/9733
2020-02-27T01:50:59.7542560Z .................................................................................................... 5600/9733
2020-02-27T01:51:07.2894771Z .................................................................................................... 5700/9733
2020-02-27T01:51:12.9439250Z .............................i...................................................................... 5800/9733
2020-02-27T01:51:17.9780437Z .................................................................................................... 5900/9733
2020-02-27T01:51:17.9780437Z .................................................................................................... 5900/9733
2020-02-27T01:51:27.3169320Z .................................................................................................... 6000/9733
2020-02-27T01:51:36.4180305Z ....................ii...i..ii...........i.......................................................... 6100/9733
2020-02-27T01:51:50.4316035Z .................................................................................................... 6300/9733
2020-02-27T01:51:56.4422226Z .................................................................................................... 6400/9733
2020-02-27T01:51:56.4422226Z .................................................................................................... 6400/9733
2020-02-27T01:52:08.9975170Z ...................................................i..ii............................................ 6500/9733
2020-02-27T01:52:29.9629895Z .................................................................................................... 6700/9733
2020-02-27T01:52:31.8796343Z ...........................................i........................................................ 6800/9733
2020-02-27T01:52:33.6593108Z .................................................................................................... 6900/9733
2020-02-27T01:52:35.5478583Z .........................................................................i.......................... 7000/9733
---
2020-02-27T01:53:54.6199378Z .................................................................................................... 7700/9733
2020-02-27T01:53:58.8413907Z .................................................................................................... 7800/9733
2020-02-27T01:54:02.8442414Z .................................................................................................... 7900/9733
2020-02-27T01:54:09.2785360Z ..................i................................................................................. 8000/9733
2020-02-27T01:54:16.2207534Z ...................................................................iiiiiii.i........................ 8100/9733
2020-02-27T01:54:28.8119420Z ........i......i.................................................................................... 8300/9733
2020-02-27T01:54:33.1225647Z .................................................................................................... 8400/9733
2020-02-27T01:54:43.5720872Z .................................................................................................... 8500/9733
2020-02-27T01:54:51.0182782Z .................................................................................................... 8600/9733
---
2020-02-27T01:56:23.6545590Z ---- [ui] ui/async-await/async-closure.rs stdout ----
2020-02-27T01:56:23.6545970Z 
2020-02-27T01:56:23.6546565Z error: test compilation failed although it shouldn't!
2020-02-27T01:56:23.6546980Z status: exit code: 101
2020-02-27T01:56:23.6548695Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/auxiliary"
2020-02-27T01:56:23.6550378Z ------------------------------------------
2020-02-27T01:56:23.6550702Z 
2020-02-27T01:56:23.6551178Z ------------------------------------------
2020-02-27T01:56:23.6551544Z stderr:
2020-02-27T01:56:23.6551544Z stderr:
2020-02-27T01:56:23.6552020Z ------------------------------------------
2020-02-27T01:56:23.6554197Z error: internal compiler error: src/librustc_infer/traits/codegen/mod.rs:57: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Sorts(ExpectedFound { expected: impl std::future::Future, found: std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6 x:u8 {WakeOnceThenComplete, ()}]> }))` selecting `Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>)` during codegen
2020-02-27T01:56:23.6559644Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:881:9
2020-02-27T01:56:23.6559970Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-27T01:56:23.6561443Z 
2020-02-27T01:56:23.6561657Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-27T01:56:23.6561657Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-27T01:56:23.6561816Z 
2020-02-27T01:56:23.6562465Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-27T01:56:23.6562678Z 
2020-02-27T01:56:23.6566005Z note: rustc 1.43.0-nightly (e36411df5 2020-02-27) running on x86_64-unknown-linux-gnu
2020-02-27T01:56:23.6566248Z 
2020-02-27T01:56:23.6566795Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-27T01:56:23.6567246Z error: aborting due to previous error
2020-02-27T01:56:23.6567379Z 
2020-02-27T01:56:23.6567455Z 
2020-02-27T01:56:23.6567743Z ------------------------------------------
---
2020-02-27T01:56:23.6569370Z stdout:
2020-02-27T01:56:23.6569673Z ------------------------------------------
2020-02-27T01:56:23.6570001Z ---------------------------------------
2020-02-27T01:56:23.6570195Z trace does not match position list
2020-02-27T01:56:23.6570723Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:109"]
2020-02-27T01:56:23.6571198Z --- stdout
2020-02-27T01:56:23.6571462Z backtrace-debuginfo.rs:109
2020-02-27T01:56:23.6571774Z backtrace-debuginfo.rs:126
2020-02-27T01:56:23.6572068Z backtrace-debuginfo.rs:189
2020-02-27T01:56:23.6572068Z backtrace-debuginfo.rs:189
2020-02-27T01:56:23.6572197Z 
2020-02-27T01:56:23.6572423Z --- stderr
2020-02-27T01:56:23.6572545Z test case 6
2020-02-27T01:56:23.6572966Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-02-27T01:56:23.6573237Z stack backtrace:
2020-02-27T01:56:23.6573827Z    0:     0x7f6b61e695a6 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h12d63982e159c087
2020-02-27T01:56:23.6574406Z    1:     0x7f6b61ea3aac - core::fmt::write::h0479a0ef9cd0b105
2020-02-27T01:56:23.6574896Z    2:     0x7f6b61e59717 - std::io::Write::write_fmt::h2f8886774a20d1f4
2020-02-27T01:56:23.6575618Z    3:     0x7f6b61e6e5e4 - std::panicking::default_hook::{{closure}}::h7ca95c7592065e0e
2020-02-27T01:56:23.6576536Z    4:     0x7f6b61e6e30e - std::panicking::default_hook::hf4a3054f78db14ca
2020-02-27T01:56:23.6577227Z    5:     0x7f6b61e6ec45 - std::panicking::rust_panic_with_hook::h81010cc18a196991
2020-02-27T01:56:23.6579591Z    6:     0x55d8a286ea2f - std::panicking::begin_panic::hc4d5043038025032
2020-02-27T01:56:23.6579943Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-27T01:56:23.6580690Z    7:     0x55d8a286bd76 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-27T01:56:23.6582542Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-02-27T01:56:23.6583472Z    8:     0x55d8a286c093 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-27T01:56:23.6584885Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-27T01:56:23.6585638Z    9:     0x55d8a286d1d3 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-27T01:56:23.6586139Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-27T01:56:23.6586675Z   10:     0x55d8a285ee7e - std::rt::lang_start::{{closure}}::h64f2a0ae042706bf
2020-02-27T01:56:23.6586991Z                                at /checkout/src/libstd/rt.rs:67
2020-02-27T01:56:23.6587455Z   11:     0x7f6b61e6e703 - std::panicking::try::do_call::h59783dd55f9ae61e
2020-02-27T01:56:23.6588050Z   12:     0x7f6b61e823a7 - __rust_maybe_catch_panic
2020-02-27T01:56:23.6588498Z   13:     0x7f6b61e6f1dc - std::rt::lang_start_internal::h6fb5949d8516d3ab
2020-02-27T01:56:23.6588979Z   14:     0x55d8a285ee65 - std::rt::lang_start::h8bf8a8bdf2f5cfbe
2020-02-27T01:56:23.6589291Z                                at /checkout/src/libstd/rt.rs:67
2020-02-27T01:56:23.6589644Z   15:     0x55d8a286d482 - main
2020-02-27T01:56:23.6634098Z   16:     0x7f6b61811b97 - __libc_start_main
2020-02-27T01:56:23.6635531Z   17:     0x55d8a2858c7a - _start
2020-02-27T01:56:23.6635928Z   18:                0x0 - <unknown>
2020-02-27T01:56:23.6636359Z ---------------------------------------
2020-02-27T01:56:23.6636556Z trace does not match position list
2020-02-27T01:56:23.6636556Z trace does not match position list
2020-02-27T01:56:23.6637071Z still need to find ["backtrace-debuginfo.rs:189", "backtrace-debuginfo.rs:126", "backtrace-debuginfo.rs:120"]
2020-02-27T01:56:23.6637582Z --- stdout
2020-02-27T01:56:23.6637852Z backtrace-debuginfo.rs:120
2020-02-27T01:56:23.6638837Z backtrace-debuginfo.rs:126
2020-02-27T01:56:23.6639275Z backtrace-debuginfo.rs:189
2020-02-27T01:56:23.6639275Z backtrace-debuginfo.rs:189
2020-02-27T01:56:23.6639401Z 
2020-02-27T01:56:23.6640248Z --- stderr
2020-02-27T01:56:23.6640389Z test case 9
2020-02-27T01:56:23.6641016Z thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace-debuginfo.rs:107:9
2020-02-27T01:56:23.6641276Z stack backtrace:
2020-02-27T01:56:23.6641813Z    0:     0x7f012756f5a6 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h12d63982e159c087
2020-02-27T01:56:23.6642402Z    1:     0x7f01275a9aac - core::fmt::write::h0479a0ef9cd0b105
2020-02-27T01:56:23.6642876Z    2:     0x7f012755f717 - std::io::Write::write_fmt::h2f8886774a20d1f4
2020-02-27T01:56:23.6645387Z    3:     0x7f01275745e4 - std::panicking::default_hook::{{closure}}::h7ca95c7592065e0e
2020-02-27T01:56:23.6646065Z    4:     0x7f012757430e - std::panicking::default_hook::hf4a3054f78db14ca
2020-02-27T01:56:23.6646623Z    5:     0x7f0127574c45 - std::panicking::rust_panic_with_hook::h81010cc18a196991
2020-02-27T01:56:23.6647362Z    6:     0x55e3b821da2f - std::panicking::begin_panic::hc4d5043038025032
2020-02-27T01:56:23.6647961Z                                at /checkout/src/libstd/panicking.rs:397
2020-02-27T01:56:23.6648533Z    7:     0x55e3b821aeb0 - backtrace_debuginfo::inner_inlined::h77ab595dd670898f
2020-02-27T01:56:23.6649091Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:107
2020-02-27T01:56:23.6649613Z    8:     0x55e3b821b093 - backtrace_debuginfo::outer::h681fb285db800a84
2020-02-27T01:56:23.6650450Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:126
2020-02-27T01:56:23.6650955Z    9:     0x55e3b821c1d3 - backtrace_debuginfo::main::h068f900789b8ef4b
2020-02-27T01:56:23.6651446Z                                at /checkout/src/test/ui/backtrace-debuginfo.rs:189
2020-02-27T01:56:23.6652124Z   10:     0x55e3b820de7e - std::rt::lang_start::{{closure}}::h64f2a0ae042706bf
2020-02-27T01:56:23.6652592Z                                at /checkout/src/libstd/rt.rs:67
2020-02-27T01:56:23.6653249Z   11:     0x7f0127574703 - std::panicking::try::do_call::h59783dd55f9ae61e
2020-02-27T01:56:23.6653735Z   12:     0x7f01275883a7 - __rust_maybe_catch_panic
2020-02-27T01:56:23.6654222Z   13:     0x7f01275751dc - std::rt::lang_start_internal::h6fb5949d8516d3ab
2020-02-27T01:56:23.6654714Z   14:     0x55e3b820de65 - std::rt::lang_start::h8bf8a8bdf2f5cfbe
2020-02-27T01:56:23.6655169Z                                at /checkout/src/libstd/rt.rs:67
2020-02-27T01:56:23.6655539Z   15:     0x55e3b821c482 - main
2020-02-27T01:56:23.6656858Z   16:     0x7f0126f17b97 - __libc_start_main
2020-02-27T01:56:23.6657207Z   17:     0x55e3b8207c7a - _start
2020-02-27T01:56:23.6657552Z   18:                0x0 - <unknown>
2020-02-27T01:56:23.6657764Z 
2020-02-27T01:56:23.6658182Z ------------------------------------------
2020-02-27T01:56:23.6658364Z stderr:
2020-02-27T01:56:23.6658654Z ------------------------------------------
---
2020-02-27T01:56:23.6663800Z 
2020-02-27T01:56:23.6664124Z ---- [ui] ui/issues/issue-22638.rs stdout ----
2020-02-27T01:56:23.6664309Z diff of stderr:
2020-02-27T01:56:23.6664410Z 
2020-02-27T01:56:23.6664837Z - error: reached the type-length limit while instantiating `D::matches::$CLOSURE`
2020-02-27T01:56:23.6671853Z -   --> $DIR/issue-22638.rs:53:5
2020-02-27T01:56:23.6677218Z + error: reached the type-length limit while instantiating `A::matches::$CLOSURE`
2020-02-27T01:56:23.6677986Z +   --> $DIR/issue-22638.rs:14:5
2020-02-27T01:56:23.6678162Z 3    |
2020-02-27T01:56:23.6678361Z 4 LL | /     pub fn matches<F: Fn()>(&self, f: &F) {
2020-02-27T01:56:23.6678692Z - LL | |
2020-02-27T01:56:23.6683298Z - LL | |         let &D(ref a) = self;
2020-02-27T01:56:23.6683949Z - LL | |         a.matches(f)
2020-02-27T01:56:23.6684159Z + LL | |         let &A(ref term) = self;
2020-02-27T01:56:23.6684577Z + LL | |         term.matches(f);
2020-02-27T01:56:23.6684891Z 9    | |_____^
2020-02-27T01:56:23.6685040Z 10    |
2020-02-27T01:56:23.6685139Z 
2020-02-27T01:56:23.6685223Z 
2020-02-27T01:56:23.6685223Z 
2020-02-27T01:56:23.6685402Z The actual stderr differed from the expected stderr.
2020-02-27T01:56:23.6685999Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638/issue-22638.stderr
2020-02-27T01:56:23.6686531Z To update references, rerun the tests and pass the `--bless` flag
2020-02-27T01:56:23.6687353Z To only update this specific test, also pass `--test-args issues/issue-22638.rs`
2020-02-27T01:56:23.6688054Z error: 1 errors occurred comparing output.
2020-02-27T01:56:23.6688242Z status: exit code: 1
2020-02-27T01:56:23.6688242Z status: exit code: 1
2020-02-27T01:56:23.6689730Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22638.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638/auxiliary"
2020-02-27T01:56:23.6691092Z ------------------------------------------
2020-02-27T01:56:23.6691228Z 
2020-02-27T01:56:23.6691493Z ------------------------------------------
2020-02-27T01:56:23.6691667Z stderr:
2020-02-27T01:56:23.6691667Z stderr:
2020-02-27T01:56:23.6691939Z ------------------------------------------
2020-02-27T01:56:23.6692466Z error: reached the type-length limit while instantiating `A::matches::<[closure@/checkout/...s/issue-22638.rs:30:33: 30:38]]>`
2020-02-27T01:56:23.6693282Z    |
2020-02-27T01:56:23.6693282Z    |
2020-02-27T01:56:23.6693457Z LL | /     pub fn matches<F: Fn()>(&self, f: &F) {
2020-02-27T01:56:23.6693692Z LL | |         let &A(ref term) = self;
2020-02-27T01:56:23.6693871Z LL | |         term.matches(f);
2020-02-27T01:56:23.6694148Z    | |_____^
2020-02-27T01:56:23.6694254Z    |
2020-02-27T01:56:23.6694254Z    |
2020-02-27T01:56:23.6694466Z    = note: consider adding a `#![type_length_limit="26214380"]` attribute to your crate
2020-02-27T01:56:23.6694810Z error: aborting due to previous error
2020-02-27T01:56:23.6694934Z 
2020-02-27T01:56:23.6695006Z 
2020-02-27T01:56:23.6695299Z ------------------------------------------
2020-02-27T01:56:23.6695299Z ------------------------------------------
2020-02-27T01:56:23.6695509Z 
2020-02-27T01:56:23.6695582Z 
2020-02-27T01:56:23.6695865Z ---- [ui] ui/type_length_limit.rs stdout ----
2020-02-27T01:56:23.6696000Z 
2020-02-27T01:56:23.6696151Z error: ui test compiled successfully!
2020-02-27T01:56:23.6696327Z status: exit code: 0
2020-02-27T01:56:23.6697700Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary"
2020-02-27T01:56:23.6698848Z ------------------------------------------
2020-02-27T01:56:23.6698985Z 
2020-02-27T01:56:23.6699267Z ------------------------------------------
2020-02-27T01:56:23.6699419Z stderr:
---
2020-02-27T01:56:23.6702656Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-27T01:56:23.6702984Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-27T01:56:23.6703165Z 
2020-02-27T01:56:23.6703414Z 
2020-02-27T01:56:23.6707002Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-27T01:56:23.6709356Z 
2020-02-27T01:56:23.6709429Z 
2020-02-27T01:56:23.6709623Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-27T01:56:23.6709867Z Build completed unsuccessfully in 0:56:48
2020-02-27T01:56:23.6709867Z Build completed unsuccessfully in 0:56:48
2020-02-27T01:56:23.6710052Z == clock drift check ==
2020-02-27T01:56:23.6710253Z   local time: Thu Feb 27 01:56:23 UTC 2020
2020-02-27T01:56:23.9593324Z   network time: Thu, 27 Feb 2020 01:56:23 GMT
2020-02-27T01:56:23.9593549Z == end clock drift check ==
2020-02-27T01:56:24.4732833Z 
2020-02-27T01:56:24.4797237Z ##[error]Bash exited with code '1'.
2020-02-27T01:56:24.4828799Z ##[section]Finishing: Run build
2020-02-27T01:56:24.4876654Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-02-27T01:56:24.4881456Z Task         : Get sources
2020-02-27T01:56:24.4881731Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-27T01:56:24.4882006Z Version      : 1.0.0
2020-02-27T01:56:24.4882180Z Author       : Microsoft
2020-02-27T01:56:24.4882180Z Author       : Microsoft
2020-02-27T01:56:24.4882460Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-27T01:56:24.4882801Z ==============================================================================
2020-02-27T01:56:24.7896745Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-27T01:56:24.7954882Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69503/merge to s
2020-02-27T01:56:24.8049228Z Cleaning up task key
2020-02-27T01:56:24.8050319Z Start cleaning up orphan processes.
2020-02-27T01:56:24.8210326Z Terminate orphan process: pid (4511) (python)
2020-02-27T01:56:24.8460807Z ##[section]Finishing: Finalize Job
