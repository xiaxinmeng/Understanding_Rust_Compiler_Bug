plain
2020-02-10T23:48:51.8492051Z ========================== Starting Command Output ===========================
2020-02-10T23:48:51.8494984Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/417dfd62-653a-4527-a4ee-443c8b0afbb2.sh
2020-02-10T23:48:51.8495026Z 
2020-02-10T23:48:51.8500240Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-10T23:48:51.8509413Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-10T23:48:51.8511345Z Task         : Get sources
2020-02-10T23:48:51.8511397Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T23:48:51.8511430Z Version      : 1.0.0
2020-02-10T23:48:51.8511464Z Author       : Microsoft
---
2020-02-10T23:48:52.9108329Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-10T23:48:52.9127073Z ##[command]git config gc.auto 0
2020-02-10T23:48:52.9129383Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-10T23:48:52.9135038Z ##[command]git config --get-all http.proxy
2020-02-10T23:48:52.9141518Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69038/merge:refs/remotes/pull/69038/merge
---
2020-02-11T00:50:02.8990924Z .................................................................................................... 1700/9625
2020-02-11T00:50:08.2310806Z .................................................................................................... 1800/9625
2020-02-11T00:50:21.6133458Z .............................i...................................................................... 1900/9625
2020-02-11T00:50:29.3927065Z .................................................................................................... 2000/9625
2020-02-11T00:50:44.5634448Z ...................iiiii............................................................................ 2100/9625
2020-02-11T00:50:54.9954849Z .................................................................................................... 2300/9625
2020-02-11T00:50:57.6704352Z .................................................................................................... 2400/9625
2020-02-11T00:51:02.7746790Z .................................................................................................... 2500/9625
2020-02-11T00:51:24.8966987Z .................................................................................................... 2600/9625
---
2020-02-11T00:54:09.3531654Z ......................................................................i...............i............. 4900/9625
2020-02-11T00:54:17.5519514Z .................................................................................................... 5000/9625
2020-02-11T00:54:26.1257590Z .................................................................................................... 5100/9625
2020-02-11T00:54:31.1064342Z ............i....................................................................................... 5200/9625
2020-02-11T00:54:42.7562099Z ......................................................................................ii.ii........i 5300/9625
2020-02-11T00:54:51.1065455Z ........................i........................................................................... 5500/9625
2020-02-11T00:54:59.6613256Z .................................................................................................... 5600/9625
2020-02-11T00:55:08.4184073Z ..........................................................................i......................... 5700/9625
2020-02-11T00:55:16.3522925Z .................................................................................................... 5800/9625
2020-02-11T00:55:16.3522925Z .................................................................................................... 5800/9625
2020-02-11T00:55:24.0572843Z .................................................................................................... 5900/9625
2020-02-11T00:55:33.6166860Z ..................................................................ii...i..ii...........i............ 6000/9625
2020-02-11T00:55:56.1044635Z .................................................................................................... 6200/9625
2020-02-11T00:56:04.1885828Z .................................................................................................... 6300/9625
2020-02-11T00:56:04.1885828Z .................................................................................................... 6300/9625
2020-02-11T00:56:12.6853888Z ..............................................................................................i..ii. 6400/9625
2020-02-11T00:56:39.3559346Z .................................................................................................... 6600/9625
2020-02-11T00:56:50.0721973Z .................................................................................i.................. 6700/9625
2020-02-11T00:56:52.4072838Z .................................................................................................... 6800/9625
2020-02-11T00:56:54.8276718Z ........................................................................................i........... 6900/9625
---
2020-02-11T00:58:40.3730992Z .................................................................................................... 7600/9625
2020-02-11T00:58:44.8278281Z .................................................................................................... 7700/9625
2020-02-11T00:58:50.7144208Z .................................................................................................... 7800/9625
2020-02-11T00:58:59.4862106Z .................................................................................................... 7900/9625
2020-02-11T00:59:08.8693534Z ...................................................................iiiiiii.i........................ 8000/9625
2020-02-11T00:59:25.2352247Z .......i......i..................................................................................... 8200/9625
2020-02-11T00:59:30.9963361Z .................................................................................................... 8300/9625
2020-02-11T00:59:45.4167546Z .................................................................................................... 8400/9625
2020-02-11T00:59:55.4477199Z .................................................................................................... 8500/9625
---
2020-02-11T01:02:27.5543684Z  finished in 7.653
2020-02-11T01:02:27.5752988Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T01:02:27.7854296Z 
2020-02-11T01:02:27.7854647Z running 178 tests
2020-02-11T01:02:30.9494460Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-11T01:02:33.5114515Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-11T01:02:33.5116459Z 
2020-02-11T01:02:33.5119250Z  finished in 5.936
2020-02-11T01:02:33.5326449Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T01:02:33.7176048Z 
---
2020-02-11T01:02:35.8171611Z  finished in 2.284
2020-02-11T01:02:35.8406755Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T01:02:36.0145388Z 
2020-02-11T01:02:36.0146137Z running 9 tests
2020-02-11T01:02:36.0147023Z iiiiiiiii
2020-02-11T01:02:36.0148797Z 
2020-02-11T01:02:36.0149120Z  finished in 0.174
2020-02-11T01:02:36.0366402Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T01:02:36.2400017Z 
---
2020-02-11T01:02:57.4923112Z  finished in 21.455
2020-02-11T01:02:57.5178548Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T01:02:57.7336653Z 
2020-02-11T01:02:57.7337339Z running 116 tests
2020-02-11T01:03:11.7256230Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-11T01:03:13.6860888Z ....iiii.....ii.
2020-02-11T01:03:13.6861994Z 
2020-02-11T01:03:13.6866271Z  finished in 16.169
2020-02-11T01:03:13.6872003Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T01:03:13.6873254Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-11T01:17:32.9760218Z 
2020-02-11T01:17:32.9761619Z    Doc-tests core
2020-02-11T01:17:38.1837298Z 
2020-02-11T01:17:38.1837687Z running 2471 tests
2020-02-11T01:17:48.0256569Z ......iiiii......................................................................................... 100/2471
2020-02-11T01:17:57.6469667Z ..................................................................................ii................ 200/2471
2020-02-11T01:18:20.0964372Z .................i.................................................................................. 400/2471
2020-02-11T01:18:20.0964372Z .................i.................................................................................. 400/2471
2020-02-11T01:18:30.6779725Z ......................................................................i..i..................iiii.... 500/2471
2020-02-11T01:18:48.4805966Z .................................................................................................... 700/2471
2020-02-11T01:18:57.8216882Z .................................................................................................... 800/2471
2020-02-11T01:19:07.1382928Z .................................................................................................... 900/2471
2020-02-11T01:19:16.3098374Z .................................................................................................... 1000/2471
---
2020-02-11T01:22:55.5286584Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:871:13
2020-02-11T01:22:55.5286827Z .. 300/761
2020-02-11T01:22:55.6397193Z .................................................................................................... 400/761
2020-02-11T01:22:57.7257523Z .................................................................................................... 500/761
2020-02-11T01:22:57.7501238Z ..................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-02-11T01:22:57.7521768Z ....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-02-11T01:22:57.7553486Z .<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError..', thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-02-11T01:22:57.7563337Z ............src/libstd/sync/mpsc/mod.rs:2778.:.21
2020-02-11T01:22:57.9996391Z ...............................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-02-11T01:22:58.0049423Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-02-11T01:22:58.0076151Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-02-11T01:22:58.0350817Z .................. 600/761
2020-02-11T01:23:00.0657751Z ....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-02-11T01:23:00.0669705Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
---
2020-02-11T01:23:02.7421175Z thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1727:37
2020-02-11T01:23:08.2530966Z ..............
2020-02-11T01:23:08.2531089Z failures:
2020-02-11T01:23:08.2531121Z 
2020-02-11T01:23:08.2531867Z ---- backtrace::tests::debug_backtrace_fmt stdout ----
2020-02-11T01:23:08.2531924Z uncaptured: disabled backtrace
2020-02-11T01:23:08.2532129Z captured: Backtrace [ { fn: "std::backtrace::Backtrace::create::hc0c9abf45a573850" },  { fn: "std::backtrace::Backtrace::force_capture::hc439a737dfaa04f1" },  { fn: "std::backtrace::tests::debug_backtrace_fmt::h4c24c9a7f263a28f" },  { fn: "std::backtrace::tests::debug_backtrace_fmt::{{closure}}::hc35c4f50b6b35d53" },  { fn: "core::ops::function::FnOnce::call_once::hf1be62ec30e8403f" },  { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h46e5a10d3747a497" },  { fn: "__rust_maybe_catch_panic" },  { fn: "test::run_test::run_test_inner::{{closure}}::h06ed7df7a8568a36" },  { fn: "std::sys_common::backtrace::__rust_begin_short_backtrace::hca1d6956c2c841b0" },  { fn: "std::panicking::try::do_call::h3d11d5c5aa46f34c" },  { fn: "__rust_maybe_catch_panic" },  { fn: "core::ops::function::FnOnce::call_once{{vtable.shim}}::h0c0304b1966be275" },  { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd81dad5a20042ecf" },  { fn: "std::sys::unix::thread::Thread::new::thread_start::h4b273b4a85f28390" },  { fn: "start_thread" },  { fn: "__clone" },  { fn: "<unknown>" }]
2020-02-11T01:23:08.2532304Z display print: stack backtrace:
2020-02-11T01:23:08.2532356Z    0: std::backtrace::tests::debug_backtrace_fmt
2020-02-11T01:23:08.2532429Z    1: std::backtrace::tests::debug_backtrace_fmt::{{closure}}
2020-02-11T01:23:08.2532481Z    2: core::ops::function::FnOnce::call_once
2020-02-11T01:23:08.2532534Z    3: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
2020-02-11T01:23:08.2532869Z    5: test::run_test::run_test_inner::{{closure}}
2020-02-11T01:23:08.2532920Z    6: std::sys_common::backtrace::__rust_begin_short_backtrace
2020-02-11T01:23:08.2533082Z    7: std::panicking::try::do_call
2020-02-11T01:23:08.2533134Z    8: __rust_maybe_catch_panic
2020-02-11T01:23:08.2533134Z    8: __rust_maybe_catch_panic
2020-02-11T01:23:08.2533183Z    9: core::ops::function::FnOnce::call_once{{vtable.shim}}
2020-02-11T01:23:08.2533252Z   10: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
2020-02-11T01:23:08.2533306Z   11: std::sys::unix::thread::Thread::new::thread_start
2020-02-11T01:23:08.2533353Z   12: start_thread
2020-02-11T01:23:08.2533415Z   13: __clone
2020-02-11T01:23:08.2533445Z 
2020-02-11T01:23:08.2533627Z resolved: Backtrace [ { fn: "std::backtrace::Backtrace::create::hc0c9abf45a573850" },  { fn: "std::backtrace::Backtrace::force_capture::hc439a737dfaa04f1" },  { fn: "std::backtrace::tests::debug_backtrace_fmt::h4c24c9a7f263a28f" },  { fn: "std::backtrace::tests::debug_backtrace_fmt::{{closure}}::hc35c4f50b6b35d53" },  { fn: "core::ops::function::FnOnce::call_once::hf1be62ec30e8403f" },  { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h46e5a10d3747a497" },  { fn: "__rust_maybe_catch_panic" },  { fn: "test::run_test::run_test_inner::{{closure}}::h06ed7df7a8568a36" },  { fn: "std::sys_common::backtrace::__rust_begin_short_backtrace::hca1d6956c2c841b0" },  { fn: "std::panicking::try::do_call::h3d11d5c5aa46f34c" },  { fn: "__rust_maybe_catch_panic" },  { fn: "core::ops::function::FnOnce::call_once{{vtable.shim}}::h0c0304b1966be275" },  { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd81dad5a20042ecf" },  { fn: "std::sys::unix::thread::Thread::new::thread_start::h4b273b4a85f28390" },  { fn: "start_thread" },  { fn: "__clone" },  { fn: "<unknown>" }]
2020-02-11T01:23:08.2534089Z thread '<unnamed>' panicked at 'not implemented', src/libstd/backtrace.rs:418:9
2020-02-11T01:23:08.2534216Z 
2020-02-11T01:23:08.2534243Z 
2020-02-11T01:23:08.2534286Z failures:
2020-02-11T01:23:08.2534347Z     backtrace::tests::debug_backtrace_fmt
2020-02-11T01:23:08.2534347Z     backtrace::tests::debug_backtrace_fmt
2020-02-11T01:23:08.2534385Z 
2020-02-11T01:23:08.2534436Z test result: FAILED. 760 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-11T01:23:08.2534489Z 
2020-02-11T01:23:08.2540604Z error: test failed, to rerun pass '-p std --lib'
2020-02-11T01:23:08.2547649Z 
2020-02-11T01:23:08.2548406Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "std" "--" "--quiet"
2020-02-11T01:23:08.2548494Z expected success, got: exit code: 101
2020-02-11T01:23:08.2548544Z 
---
2020-02-11T01:23:08.2637046Z   local time: Tue Feb 11 01:23:08 UTC 2020
2020-02-11T01:23:10.0317026Z   network time: Tue, 11 Feb 2020 01:23:08 GMT
2020-02-11T01:23:10.0317721Z == end clock drift check ==
2020-02-11T01:23:10.0317763Z 
2020-02-11T01:23:10.0379607Z ##[error]Bash exited with code '1'.
2020-02-11T01:23:10.0423579Z ##[section]Finishing: Run build
2020-02-11T01:23:10.0447313Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-11T01:23:10.0449217Z Task         : Get sources
2020-02-11T01:23:10.0449269Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T01:23:10.0449517Z Version      : 1.0.0
2020-02-11T01:23:10.0449563Z Author       : Microsoft
2020-02-11T01:23:10.0449563Z Author       : Microsoft
2020-02-11T01:23:10.0449614Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-11T01:23:10.0449692Z ==============================================================================
2020-02-11T01:23:10.5146203Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-11T01:23:10.5172343Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-11T01:23:10.5294926Z Cleaning up task key
2020-02-11T01:23:10.5295906Z Start cleaning up orphan processes.
2020-02-11T01:23:10.5413480Z Terminate orphan process: pid (3789) (python)
2020-02-11T01:23:10.5460508Z ##[section]Finishing: Finalize Job
