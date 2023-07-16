plain
2020-02-10T21:16:04.3738146Z ========================== Starting Command Output ===========================
2020-02-10T21:16:04.3739375Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b7c60885-cd42-4005-b6de-fbb3217c4f3b.sh
2020-02-10T21:16:04.3739421Z 
2020-02-10T21:16:04.3742723Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-10T21:16:04.3747857Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-10T21:16:04.3749434Z Task         : Get sources
2020-02-10T21:16:04.3749465Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T21:16:04.3749494Z Version      : 1.0.0
2020-02-10T21:16:04.3749523Z Author       : Microsoft
---
2020-02-10T21:16:05.2041904Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-10T21:16:05.2117749Z ##[command]git config gc.auto 0
2020-02-10T21:16:05.2186960Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-10T21:16:05.2250780Z ##[command]git config --get-all http.proxy
2020-02-10T21:16:05.2392119Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69038/merge:refs/remotes/pull/69038/merge
---
2020-02-10T22:12:50.9746056Z .................................................................................................... 1700/9625
2020-02-10T22:12:55.7227809Z .................................................................................................... 1800/9625
2020-02-10T22:13:07.5722556Z .............................i...................................................................... 1900/9625
2020-02-10T22:13:14.2055778Z .................................................................................................... 2000/9625
2020-02-10T22:13:27.9675106Z ...................iiiii............................................................................ 2100/9625
2020-02-10T22:13:37.5152393Z .................................................................................................... 2300/9625
2020-02-10T22:13:39.8342465Z .................................................................................................... 2400/9625
2020-02-10T22:13:44.5163986Z .................................................................................................... 2500/9625
2020-02-10T22:14:04.6911641Z .................................................................................................... 2600/9625
---
2020-02-10T22:16:35.1191277Z ......................................................................i...............i............. 4900/9625
2020-02-10T22:16:42.4680448Z .................................................................................................... 5000/9625
2020-02-10T22:16:50.1364069Z .................................................................................................... 5100/9625
2020-02-10T22:16:54.6249787Z ............i....................................................................................... 5200/9625
2020-02-10T22:17:05.2259031Z ......................................................................................ii.ii........i 5300/9625
2020-02-10T22:17:12.7905735Z ........................i........................................................................... 5500/9625
2020-02-10T22:17:20.8853804Z .................................................................................................... 5600/9625
2020-02-10T22:17:28.9210739Z ..........................................................................i......................... 5700/9625
2020-02-10T22:17:36.0222048Z .................................................................................................... 5800/9625
2020-02-10T22:17:36.0222048Z .................................................................................................... 5800/9625
2020-02-10T22:17:42.1331552Z .................................................................................................... 5900/9625
2020-02-10T22:17:51.8170801Z ..................................................................ii...i..ii...........i............ 6000/9625
2020-02-10T22:18:12.2764491Z .................................................................................................... 6200/9625
2020-02-10T22:18:19.5860612Z .................................................................................................... 6300/9625
2020-02-10T22:18:19.5860612Z .................................................................................................... 6300/9625
2020-02-10T22:18:27.2474248Z ..............................................................................................i..ii. 6400/9625
2020-02-10T22:18:55.4449095Z .................................................................................................... 6600/9625
2020-02-10T22:19:04.9639452Z .................................................................................i.................. 6700/9625
2020-02-10T22:19:07.0652415Z .................................................................................................... 6800/9625
2020-02-10T22:19:09.2905335Z .........................................................................................i.......... 6900/9625
---
2020-02-10T22:20:44.9493709Z .................................................................................................... 7600/9625
2020-02-10T22:20:48.8117411Z .................................................................................................... 7700/9625
2020-02-10T22:20:54.1515616Z .................................................................................................... 7800/9625
2020-02-10T22:21:02.1518884Z .................................................................................................... 7900/9625
2020-02-10T22:21:10.7530908Z ...................................................................iiiiiii.i........................ 8000/9625
2020-02-10T22:21:25.8828478Z .......i......i..................................................................................... 8200/9625
2020-02-10T22:21:31.2802725Z .................................................................................................... 8300/9625
2020-02-10T22:21:44.3010359Z .................................................................................................... 8400/9625
2020-02-10T22:21:53.5602129Z .................................................................................................... 8500/9625
---
2020-02-10T22:24:15.7910478Z  finished in 7.276
2020-02-10T22:24:15.7910791Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-10T22:24:15.7910829Z 
2020-02-10T22:24:15.7910871Z running 178 tests
2020-02-10T22:24:18.6187918Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-10T22:24:20.9070859Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-10T22:24:20.9072005Z 
2020-02-10T22:24:20.9074078Z  finished in 5.381
2020-02-10T22:24:20.9248390Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-10T22:24:21.0871722Z 
---
2020-02-10T22:24:23.7910095Z  finished in 2.071
2020-02-10T22:24:23.7910393Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-10T22:24:23.7910434Z 
2020-02-10T22:24:23.7910516Z running 9 tests
2020-02-10T22:24:23.7910821Z iiiiiiiii
2020-02-10T22:24:23.7911167Z 
2020-02-10T22:24:23.7911224Z  finished in 0.151
2020-02-10T22:24:23.7911525Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-10T22:24:23.7911565Z 
---
2020-02-10T22:24:42.6950763Z  finished in 19.511
2020-02-10T22:24:42.7146198Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-10T22:24:43.3022233Z 
2020-02-10T22:24:43.3060338Z running 116 tests
2020-02-10T22:24:56.0315892Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-10T22:24:57.7877785Z ....iiii.....ii.
2020-02-10T22:24:57.7881816Z 
2020-02-10T22:24:57.7887144Z  finished in 15.074
2020-02-10T22:24:57.7892155Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-10T22:24:57.7892776Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-10T22:38:04.5315773Z 
2020-02-10T22:38:04.5327440Z    Doc-tests core
2020-02-10T22:38:09.1630912Z 
2020-02-10T22:38:09.1631834Z running 2471 tests
2020-02-10T22:38:17.9778801Z ......iiiii......................................................................................... 100/2471
2020-02-10T22:38:27.0103272Z ..................................................................................ii................ 200/2471
2020-02-10T22:38:46.5435238Z .................i.................................................................................. 400/2471
2020-02-10T22:38:46.5435238Z .................i.................................................................................. 400/2471
2020-02-10T22:38:55.8034017Z ......................................................................i..i..................iiii.... 500/2471
2020-02-10T22:39:11.3983262Z .................................................................................................... 700/2471
2020-02-10T22:39:19.5972233Z .................................................................................................... 800/2471
2020-02-10T22:39:27.6918051Z .................................................................................................... 900/2471
2020-02-10T22:39:35.8973210Z .................................................................................................... 1000/2471
---
2020-02-10T22:42:54.0602964Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-02-10T22:42:54.0719500Z .................. 600/761
2020-02-10T22:42:56.0997571Z .....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-02-10T22:42:56.0997909Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-02-10T22:42:56.1006534Z ....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-02-10T22:42:56.1014394Z thread '.<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
2020-02-10T22:42:56.1023122Z ...thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-10T22:42:56.1029596Z  right: `2`', src/libstd/sync/mutex.rs:657:13
2020-02-10T22:42:56.1076915Z ..........thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:792:13
2020-02-10T22:42:56.1089883Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:768:13
2020-02-10T22:42:56.1102512Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:704:13
---
2020-02-10T22:42:58.7918766Z .thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1727:37
2020-02-10T22:43:04.2842444Z .............
2020-02-10T22:43:04.2843490Z failures:
2020-02-10T22:43:04.2843685Z 
2020-02-10T22:43:04.2844532Z ---- backtrace::tests::debug_backtrace_fmt stdout ----
2020-02-10T22:43:04.2844934Z uncaptured: Backtrace { inner: Disabled }
2020-02-10T22:43:04.2845428Z captured: Backtrace { inner: Captured(Mutex { data: Capture { actual_start: 2, resolved: false, frames: [BacktraceFrame { frame: Frame { ip: 0x55cb610e94ed, symbol_address: 0x55cb610e9470 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x55cb610e9460, symbol_address: 0x55cb610e9450 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x55cb60fa0440, symbol_address: 0x55cb60fa03d0 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe38955990f, symbol_address: 0x7fe3895598d0 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe3892baeb7, symbol_address: 0x7fe3892baea0 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe389578acf, symbol_address: 0x7fe3895786c0 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe38954b689, symbol_address: 0x7fe38954b660 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe389550419, symbol_address: 0x7fe3895503f0 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe3892baeb7, symbol_address: 0x7fe3892baea0 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe389551cf6, symbol_address: 0x7fe389551c70 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe38928358f, symbol_address: 0x7fe389283550 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe3892b8d80, symbol_address: 0x7fe3892b8cf0 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe3897be6db, symbol_address: 0x7fe3897be600 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x7fe388d4c88f, symbol_address: 0x7fe388d4c889 }, symbols: [] }, BacktraceFrame { frame: Frame { ip: 0x0, symbol_address: 0x0 }, symbols: [] }] } }) }
2020-02-10T22:43:04.2845803Z display print: stack backtrace:
2020-02-10T22:43:04.2846007Z    0: std::backtrace::tests::debug_backtrace_fmt
2020-02-10T22:43:04.2846252Z    1: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
2020-02-10T22:43:04.2846897Z    3: test::run_test::run_test_inner::{{closure}}
2020-02-10T22:43:04.2847105Z    4: std::sys_common::backtrace::__rust_begin_short_backtrace
2020-02-10T22:43:04.2847305Z    5: std::panicking::try::do_call
2020-02-10T22:43:04.2847517Z    6: __rust_maybe_catch_panic
2020-02-10T22:43:04.2847517Z    6: __rust_maybe_catch_panic
2020-02-10T22:43:04.2847713Z    7: core::ops::function::FnOnce::call_once{{vtable.shim}}
2020-02-10T22:43:04.2847913Z    8: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
2020-02-10T22:43:04.2848130Z    9: std::sys::unix::thread::Thread::new::thread_start
2020-02-10T22:43:04.2848320Z   10: start_thread
2020-02-10T22:43:04.2848528Z   11: __clone
2020-02-10T22:43:04.2848856Z 
2020-02-10T22:43:04.2849934Z resolved: Backtrace { inner: Captured(Mutex { data: Capture { actual_start: 2, resolved: true, frames: [BacktraceFrame { frame: Frame { ip: 0x55cb610e94ed, symbol_address: 0x55cb610e9470 }, symbols: [BacktraceSymbol { name: Some(std::backtrace::Backtrace::create::h26aebea81d476f1b), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x55cb610e9460, symbol_address: 0x55cb610e9450 }, symbols: [BacktraceSymbol { name: Some(std::backtrace::Backtrace::force_capture::hf39c0dfe00bac326), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x55cb60fa0440, symbol_address: 0x55cb60fa03d0 }, symbols: [BacktraceSymbol { name: Some(std::backtrace::tests::debug_backtrace_fmt::h71b86f37709cb25c), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe38955990f, symbol_address: 0x7fe3895598d0 }, symbols: [BacktraceSymbol { name: Some(<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h32a44b4cd9f1561a), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe3892baeb7, symbol_address: 0x7fe3892baea0 }, symbols: [BacktraceSymbol { name: Some("__rust_maybe_catch_panic"), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe389578acf, symbol_address: 0x7fe3895786c0 }, symbols: [BacktraceSymbol { name: Some(test::run_test::run_test_inner::{{closure}}::hdb23088fbce26e99), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe38954b689, symbol_address: 0x7fe38954b660 }, symbols: [BacktraceSymbol { name: Some(std::sys_common::backtrace::__rust_begin_short_backtrace::h4642da7dd96fa758), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe389550419, symbol_address: 0x7fe3895503f0 }, symbols: [BacktraceSymbol { name: Some(std::panicking::try::do_call::h81299d9bc08a2e2c), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe3892baeb7, symbol_address: 0x7fe3892baea0 }, symbols: [BacktraceSymbol { name: Some("__rust_maybe_catch_panic"), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe389551cf6, symbol_address: 0x7fe389551c70 }, symbols: [BacktraceSymbol { name: Some(core::ops::function::FnOnce::call_once{{vtable.shim}}::he89292d7784fbe20), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe38928358f, symbol_address: 0x7fe389283550 }, symbols: [BacktraceSymbol { name: Some(<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hfe0665775bd2b721), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe3892b8d80, symbol_address: 0x7fe3892b8cf0 }, symbols: [BacktraceSymbol { name: Some(std::sys::unix::thread::Thread::new::thread_start::h36d1e478cadb3b18), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe3897be6db, symbol_address: 0x7fe3897be600 }, symbols: [BacktraceSymbol { name: Some("start_thread"), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x7fe388d4c88f, symbol_address: 0x7fe388d4c889 }, symbols: [BacktraceSymbol { name: Some("__clone"), filename: None, lineno: None }] }, BacktraceFrame { frame: Frame { ip: 0x0, symbol_address: 0x0 }, symbols: [BacktraceSymbol { name: None, filename: None, lineno: None }] }] } }) }
2020-02-10T22:43:04.2851175Z thread '<unnamed>' panicked at 'not implemented', src/libstd/backtrace.rs:389:9
2020-02-10T22:43:04.2851669Z 
2020-02-10T22:43:04.2851836Z 
2020-02-10T22:43:04.2852015Z failures:
2020-02-10T22:43:04.2854945Z     backtrace::tests::debug_backtrace_fmt
2020-02-10T22:43:04.2854945Z     backtrace::tests::debug_backtrace_fmt
2020-02-10T22:43:04.2855208Z 
2020-02-10T22:43:04.2855422Z test result: FAILED. 760 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-10T22:43:04.2855622Z 
2020-02-10T22:43:04.2868610Z error: test failed, to rerun pass '-p std --lib'
2020-02-10T22:43:04.2897647Z 
2020-02-10T22:43:04.2902435Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "std" "--" "--quiet"
2020-02-10T22:43:04.2902533Z expected success, got: exit code: 101
2020-02-10T22:43:04.2902656Z 
---
2020-02-10T22:43:04.2994938Z   local time: Mon Feb 10 22:43:04 UTC 2020
2020-02-10T22:43:04.8525558Z   network time: Mon, 10 Feb 2020 22:43:04 GMT
2020-02-10T22:43:04.8526973Z == end clock drift check ==
2020-02-10T22:43:05.2451095Z 
2020-02-10T22:43:05.2565917Z ##[error]Bash exited with code '1'.
2020-02-10T22:43:05.2580653Z ##[section]Finishing: Run build
2020-02-10T22:43:05.2599451Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-10T22:43:05.2601027Z Task         : Get sources
2020-02-10T22:43:05.2601079Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T22:43:05.2601130Z Version      : 1.0.0
2020-02-10T22:43:05.2601162Z Author       : Microsoft
2020-02-10T22:43:05.2601162Z Author       : Microsoft
2020-02-10T22:43:05.2601213Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-10T22:43:05.2601251Z ==============================================================================
2020-02-10T22:43:05.6753233Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-10T22:43:05.6790155Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-10T22:43:05.6903259Z Cleaning up task key
2020-02-10T22:43:05.6904113Z Start cleaning up orphan processes.
2020-02-10T22:43:05.7010552Z Terminate orphan process: pid (4584) (python)
2020-02-10T22:43:05.7259042Z ##[section]Finishing: Finalize Job
