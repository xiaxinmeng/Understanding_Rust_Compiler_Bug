plain
2020-02-11T01:47:27.8626224Z ========================== Starting Command Output ===========================
2020-02-11T01:47:27.8628303Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5d083af4-c170-4734-ab97-b3ff37cc4cf6.sh
2020-02-11T01:47:27.8628398Z 
2020-02-11T01:47:27.8631863Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T01:47:27.8639261Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-11T01:47:27.8641240Z Task         : Get sources
2020-02-11T01:47:27.8641289Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T01:47:27.8641391Z Version      : 1.0.0
2020-02-11T01:47:27.8641440Z Author       : Microsoft
---
2020-02-11T01:47:28.7357467Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T01:47:28.7460478Z ##[command]git config gc.auto 0
2020-02-11T01:47:28.7519256Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T01:47:28.7572386Z ##[command]git config --get-all http.proxy
2020-02-11T01:47:28.7717971Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69038/merge:refs/remotes/pull/69038/merge
---
2020-02-11T02:41:49.8584858Z .................................................................................................... 1700/9625
2020-02-11T02:41:54.4752166Z .................................................................................................... 1800/9625
2020-02-11T02:42:06.1287852Z .............................i...................................................................... 1900/9625
2020-02-11T02:42:13.0208603Z .................................................................................................... 2000/9625
2020-02-11T02:42:26.6059757Z ...................iiiii............................................................................ 2100/9625
2020-02-11T02:42:35.9417151Z .................................................................................................... 2300/9625
2020-02-11T02:42:38.2549629Z .................................................................................................... 2400/9625
2020-02-11T02:42:42.8879435Z .................................................................................................... 2500/9625
2020-02-11T02:43:02.6987570Z .................................................................................................... 2600/9625
---
2020-02-11T02:45:32.7715316Z ......................................................................i...............i............. 4900/9625
2020-02-11T02:45:40.3214876Z .................................................................................................... 5000/9625
2020-02-11T02:45:47.9733497Z .................................................................................................... 5100/9625
2020-02-11T02:45:52.4895429Z ............i....................................................................................... 5200/9625
2020-02-11T02:46:02.9822466Z ......................................................................................ii.ii........i 5300/9625
2020-02-11T02:46:10.5732347Z ........................i........................................................................... 5500/9625
2020-02-11T02:46:18.4010402Z .................................................................................................... 5600/9625
2020-02-11T02:46:26.3138018Z ..........................................................................i......................... 5700/9625
2020-02-11T02:46:33.4665059Z .................................................................................................... 5800/9625
2020-02-11T02:46:33.4665059Z .................................................................................................... 5800/9625
2020-02-11T02:46:39.6281997Z .................................................................................................... 5900/9625
2020-02-11T02:46:49.6106702Z ..................................................................ii...i..ii...........i............ 6000/9625
2020-02-11T02:47:09.9598649Z .................................................................................................... 6200/9625
2020-02-11T02:47:14.5688825Z .................................................................................................... 6300/9625
2020-02-11T02:47:14.5688825Z .................................................................................................... 6300/9625
2020-02-11T02:47:18.5480557Z ..............................................................................................i..ii. 6400/9625
2020-02-11T02:47:40.0152685Z .................................................................................................... 6600/9625
2020-02-11T02:47:49.5962252Z .................................................................................i.................. 6700/9625
2020-02-11T02:47:51.6551443Z .................................................................................................... 6800/9625
2020-02-11T02:47:53.8193228Z ........................................................................................i........... 6900/9625
---
2020-02-11T02:49:28.9011128Z .................................................................................................... 7600/9625
2020-02-11T02:49:33.1199700Z .................................................................................................... 7700/9625
2020-02-11T02:49:38.6562327Z .................................................................................................... 7800/9625
2020-02-11T02:49:46.8247164Z .................................................................................................... 7900/9625
2020-02-11T02:49:55.7827864Z ...................................................................iiiiiii.i........................ 8000/9625
2020-02-11T02:50:11.0183708Z .......i......i..................................................................................... 8200/9625
2020-02-11T02:50:16.4036921Z .................................................................................................... 8300/9625
2020-02-11T02:50:29.6543549Z .................................................................................................... 8400/9625
2020-02-11T02:50:38.9062476Z .................................................................................................... 8500/9625
---
2020-02-11T02:52:57.1405865Z  finished in 7.173
2020-02-11T02:52:57.1597707Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T02:52:57.3672262Z 
2020-02-11T02:52:57.3673084Z running 178 tests
2020-02-11T02:53:00.8333305Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-11T02:53:02.5611113Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-11T02:53:02.5611707Z 
2020-02-11T02:53:02.5612275Z  finished in 5.401
2020-02-11T02:53:02.5800326Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T02:53:02.7533306Z 
---
2020-02-11T02:53:04.7218865Z  finished in 2.141
2020-02-11T02:53:04.7417584Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T02:53:04.9050162Z 
2020-02-11T02:53:04.9050403Z running 9 tests
2020-02-11T02:53:04.9051226Z iiiiiiiii
2020-02-11T02:53:04.9051675Z 
2020-02-11T02:53:04.9054760Z  finished in 0.163
2020-02-11T02:53:04.9272731Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T02:53:05.1208304Z 
---
2020-02-11T02:53:24.7817601Z  finished in 19.855
2020-02-11T02:53:24.8044526Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T02:53:25.0069952Z 
2020-02-11T02:53:25.0070134Z running 116 tests
2020-02-11T02:53:37.8007809Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-11T02:53:40.3410150Z ....iiii.....ii.
2020-02-11T02:53:40.3410611Z 
2020-02-11T02:53:40.3410664Z  finished in 14.772
2020-02-11T02:53:40.3410947Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T02:53:40.3411284Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-11T03:05:58.1721788Z 
2020-02-11T03:05:58.1722608Z    Doc-tests core
2020-02-11T03:06:02.6451922Z 
2020-02-11T03:06:02.6452671Z running 2471 tests
2020-02-11T03:06:11.1973996Z ......iiiii......................................................................................... 100/2471
2020-02-11T03:06:19.5280245Z ..................................................................................ii................ 200/2471
2020-02-11T03:06:38.8220304Z .................i.................................................................................. 400/2471
2020-02-11T03:06:38.8220304Z .................i.................................................................................. 400/2471
2020-02-11T03:06:47.8501858Z ......................................................................i..i..................iiii.... 500/2471
2020-02-11T03:07:03.1207388Z .................................................................................................... 700/2471
2020-02-11T03:07:11.0746267Z .................................................................................................... 800/2471
2020-02-11T03:07:19.0356112Z .................................................................................................... 900/2471
2020-02-11T03:07:27.0157317Z .................................................................................................... 1000/2471
---
2020-02-11T03:10:38.4902251Z .................................................................................................... 500/761
2020-02-11T03:10:38.5292011Z ..................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-02-11T03:10:38.5305745Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-02-11T03:10:38.5308922Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-02-11T03:10:38.5323754Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-02-11T03:10:38.8265023Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-02-11T03:10:38.8283695Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2022:17
2020-02-11T03:10:38.8298776Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034.:21.
2020-02-11T03:10:38.8310410Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-02-11T03:10:40.8729366Z ....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-02-11T03:10:40.8746805Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-02-11T03:10:40.8758387Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-02-11T03:10:40.8764794Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-02-11T03:10:43.5621239Z .thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1727:37
2020-02-11T03:10:49.0539432Z .............
2020-02-11T03:10:49.0539586Z failures:
2020-02-11T03:10:49.0539641Z 
2020-02-11T03:10:49.0540647Z ---- backtrace::tests::debug_backtrace_fmt stdout ----
2020-02-11T03:10:49.0540766Z uncaptured: disabled backtrace
2020-02-11T03:10:49.0541075Z captured: Backtrace [{ fn: std::backtrace::Backtrace::create::hdeef3b580b4db842 }, { fn: std::backtrace::Backtrace::force_capture::haa57f8e507856668 }, { fn: std::backtrace::tests::debug_backtrace_fmt::ha299b18b904f6ae1 }, { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h5df0e898b301d82a }, { fn: __rust_maybe_catch_panic }, { fn: test::run_test::run_test_inner::{{closure}}::hfbb6c9beffe5df6d }, { fn: std::sys_common::backtrace::__rust_begin_short_backtrace::h3aede521c9ff7629 }, { fn: std::panicking::try::do_call::h02bc59bce1d83dd1 }, { fn: __rust_maybe_catch_panic }, { fn: core::ops::function::FnOnce::call_once{{vtable.shim}}::h0806fc5f2dbe969f }, { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h508ce35e9da003b7 }, { fn: std::sys::unix::thread::Thread::new::thread_start::h7d4e50e6e9ddd29f }, { fn: start_thread }, { fn: __clone }]
2020-02-11T03:10:49.0541335Z display print: stack backtrace:
2020-02-11T03:10:49.0541432Z    0: std::backtrace::tests::debug_backtrace_fmt
2020-02-11T03:10:49.0541560Z    1: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
2020-02-11T03:10:49.0542012Z    3: test::run_test::run_test_inner::{{closure}}
2020-02-11T03:10:49.0542120Z    4: std::sys_common::backtrace::__rust_begin_short_backtrace
2020-02-11T03:10:49.0542212Z    5: std::panicking::try::do_call
2020-02-11T03:10:49.0542299Z    6: __rust_maybe_catch_panic
2020-02-11T03:10:49.0542299Z    6: __rust_maybe_catch_panic
2020-02-11T03:10:49.0542407Z    7: core::ops::function::FnOnce::call_once{{vtable.shim}}
2020-02-11T03:10:49.0542506Z    8: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
2020-02-11T03:10:49.0542604Z    9: std::sys::unix::thread::Thread::new::thread_start
2020-02-11T03:10:49.0542849Z   10: start_thread
2020-02-11T03:10:49.0542931Z   11: __clone
2020-02-11T03:10:49.0542987Z 
2020-02-11T03:10:49.0543282Z resolved: Backtrace [{ fn: std::backtrace::Backtrace::create::hdeef3b580b4db842 }, { fn: std::backtrace::Backtrace::force_capture::haa57f8e507856668 }, { fn: std::backtrace::tests::debug_backtrace_fmt::ha299b18b904f6ae1 }, { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h5df0e898b301d82a }, { fn: __rust_maybe_catch_panic }, { fn: test::run_test::run_test_inner::{{closure}}::hfbb6c9beffe5df6d }, { fn: std::sys_common::backtrace::__rust_begin_short_backtrace::h3aede521c9ff7629 }, { fn: std::panicking::try::do_call::h02bc59bce1d83dd1 }, { fn: __rust_maybe_catch_panic }, { fn: core::ops::function::FnOnce::call_once{{vtable.shim}}::h0806fc5f2dbe969f }, { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h508ce35e9da003b7 }, { fn: std::sys::unix::thread::Thread::new::thread_start::h7d4e50e6e9ddd29f }, { fn: start_thread }, { fn: __clone }]
2020-02-11T03:10:49.0543524Z resolved alt: Backtrace [
2020-02-11T03:10:49.0543616Z     { fn: std::backtrace::Backtrace::create },
2020-02-11T03:10:49.0543712Z     { fn: std::backtrace::Backtrace::force_capture },
2020-02-11T03:10:49.0543825Z     { fn: std::backtrace::tests::debug_backtrace_fmt },
2020-02-11T03:10:49.0543924Z     { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once },
2020-02-11T03:10:49.0544031Z     { fn: __rust_maybe_catch_panic },
2020-02-11T03:10:49.0544146Z     { fn: test::run_test::run_test_inner::{{closure}} },
2020-02-11T03:10:49.0544241Z     { fn: std::sys_common::backtrace::__rust_begin_short_backtrace },
2020-02-11T03:10:49.0544335Z     { fn: std::panicking::try::do_call },
2020-02-11T03:10:49.0544440Z     { fn: __rust_maybe_catch_panic },
2020-02-11T03:10:49.0544533Z     { fn: core::ops::function::FnOnce::call_once{{vtable.shim}} },
2020-02-11T03:10:49.0544638Z     { fn: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once },
2020-02-11T03:10:49.0544770Z     { fn: std::sys::unix::thread::Thread::new::thread_start },
2020-02-11T03:10:49.0544858Z     { fn: start_thread },
2020-02-11T03:10:49.0544940Z     { fn: __clone },
2020-02-11T03:10:49.0545037Z ]
2020-02-11T03:10:49.0545577Z thread '<unnamed>' panicked at 'not implemented', src/libstd/backtrace.rs:424:9
2020-02-11T03:10:49.0545760Z 
2020-02-11T03:10:49.0545830Z 
2020-02-11T03:10:49.0545909Z failures:
2020-02-11T03:10:49.0545994Z     backtrace::tests::debug_backtrace_fmt
2020-02-11T03:10:49.0545994Z     backtrace::tests::debug_backtrace_fmt
2020-02-11T03:10:49.0546048Z 
2020-02-11T03:10:49.0546156Z test result: FAILED. 760 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-11T03:10:49.0546218Z 
2020-02-11T03:10:49.0571796Z error: test failed, to rerun pass '-p std --lib'
2020-02-11T03:10:49.0596308Z 
2020-02-11T03:10:49.0598160Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "std" "--" "--quiet"
2020-02-11T03:10:49.0598878Z expected success, got: exit code: 101
2020-02-11T03:10:49.0599162Z 
---
2020-02-11T03:10:49.0682926Z   local time: Tue Feb 11 03:10:49 UTC 2020
2020-02-11T03:10:49.2321271Z   network time: Tue, 11 Feb 2020 03:10:49 GMT
2020-02-11T03:10:49.2327735Z == end clock drift check ==
2020-02-11T03:10:49.6690983Z 
2020-02-11T03:10:49.6790038Z ##[error]Bash exited with code '1'.
2020-02-11T03:10:49.6801572Z ##[section]Finishing: Run build
2020-02-11T03:10:49.6822149Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-11T03:10:49.6823947Z Task         : Get sources
2020-02-11T03:10:49.6824000Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T03:10:49.6824051Z Version      : 1.0.0
2020-02-11T03:10:49.6824113Z Author       : Microsoft
2020-02-11T03:10:49.6824113Z Author       : Microsoft
2020-02-11T03:10:49.6824165Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-11T03:10:49.6824220Z ==============================================================================
2020-02-11T03:10:50.0998885Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-11T03:10:50.1040927Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-11T03:10:50.1149269Z Cleaning up task key
2020-02-11T03:10:50.1150081Z Start cleaning up orphan processes.
2020-02-11T03:10:50.1249897Z Terminate orphan process: pid (3795) (python)
2020-02-11T03:10:50.1474720Z ##[section]Finishing: Finalize Job
