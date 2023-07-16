plain
2020-02-11T05:55:13.3249558Z ========================== Starting Command Output ===========================
2020-02-11T05:55:13.3253085Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/96919f58-5db0-4fea-838a-68740c70b6f4.sh
2020-02-11T05:55:13.3253260Z 
2020-02-11T05:55:13.3257265Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T05:55:13.3265794Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-11T05:55:13.3267815Z Task         : Get sources
2020-02-11T05:55:13.3267867Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T05:55:13.3267976Z Version      : 1.0.0
2020-02-11T05:55:13.3268026Z Author       : Microsoft
---
2020-02-11T05:55:17.3029856Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T05:55:17.3258529Z ##[command]git config gc.auto 0
2020-02-11T05:55:17.3330899Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T05:55:17.3388811Z ##[command]git config --get-all http.proxy
2020-02-11T05:55:17.3540301Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69038/merge:refs/remotes/pull/69038/merge
---
2020-02-11T06:52:58.0411246Z .................................................................................................... 1700/9627
2020-02-11T06:53:03.0853920Z .................................................................................................... 1800/9627
2020-02-11T06:53:15.1972515Z ..............................i..................................................................... 1900/9627
2020-02-11T06:53:22.8226645Z .................................................................................................... 2000/9627
2020-02-11T06:53:36.9238794Z ....................iiiii........................................................................... 2100/9627
2020-02-11T06:53:46.9627118Z .................................................................................................... 2300/9627
2020-02-11T06:53:49.4799703Z .................................................................................................... 2400/9627
2020-02-11T06:53:54.3827739Z .................................................................................................... 2500/9627
2020-02-11T06:54:15.2727399Z .................................................................................................... 2600/9627
---
2020-02-11T06:56:52.3913431Z .......................................................................i...............i............ 4900/9627
2020-02-11T06:57:00.3619317Z .................................................................................................... 5000/9627
2020-02-11T06:57:08.3286637Z .................................................................................................... 5100/9627
2020-02-11T06:57:13.0986735Z .............i...................................................................................... 5200/9627
2020-02-11T06:57:24.2617408Z .......................................................................................ii.ii........ 5300/9627
2020-02-11T06:57:28.1223700Z i...i............................................................................................... 5400/9627
2020-02-11T06:57:40.3140210Z .................................................................................................... 5600/9627
2020-02-11T06:57:48.7055736Z ...........................................................................i........................ 5700/9627
2020-02-11T06:57:56.2455576Z .................................................................................................... 5800/9627
2020-02-11T06:58:02.6855609Z .................................................................................................... 5900/9627
2020-02-11T06:58:02.6855609Z .................................................................................................... 5900/9627
2020-02-11T06:58:12.8497605Z ...................................................................ii...i..ii...........i........... 6000/9627
2020-02-11T06:58:34.5472427Z .................................................................................................... 6200/9627
2020-02-11T06:58:40.7603534Z .................................................................................................... 6300/9627
2020-02-11T06:58:44.9003334Z ...............................................................................................i..ii 6400/9627
2020-02-11T06:58:58.1058778Z .................................................................................................... 6500/9627
---
2020-02-11T07:01:04.4009413Z .................................................................................................... 7600/9627
2020-02-11T07:01:08.9304356Z .................................................................................................... 7700/9627
2020-02-11T07:01:14.8001259Z .................................................................................................... 7800/9627
2020-02-11T07:01:23.2245154Z .................................................................................................... 7900/9627
2020-02-11T07:01:32.3393610Z .....................................................................iiiiiii.i...................... 8000/9627
2020-02-11T07:01:48.4429376Z .........i......i................................................................................... 8200/9627
2020-02-11T07:01:54.0032513Z .................................................................................................... 8300/9627
2020-02-11T07:02:07.7930097Z .................................................................................................... 8400/9627
2020-02-11T07:02:17.7559210Z .................................................................................................... 8500/9627
---
2020-02-11T07:04:45.0441677Z  finished in 7.540
2020-02-11T07:04:45.0653942Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T07:04:45.2689233Z 
2020-02-11T07:04:45.2690673Z running 178 tests
2020-02-11T07:04:48.4193459Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-11T07:04:50.7810855Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-11T07:04:50.7815510Z 
2020-02-11T07:04:50.7821706Z  finished in 5.717
2020-02-11T07:04:50.8037470Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T07:04:50.9793120Z 
---
2020-02-11T07:04:53.0700010Z  finished in 2.266
2020-02-11T07:04:53.0906164Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T07:04:53.2498191Z 
2020-02-11T07:04:53.2499521Z running 9 tests
2020-02-11T07:04:53.2500357Z iiiiiiiii
2020-02-11T07:04:53.2500956Z 
2020-02-11T07:04:53.2504868Z  finished in 0.159
2020-02-11T07:04:53.2713726Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T07:04:53.4698941Z 
---
2020-02-11T07:05:15.2037580Z  finished in 21.932
2020-02-11T07:05:15.2283597Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T07:05:15.4308601Z 
2020-02-11T07:05:15.4312769Z running 116 tests
2020-02-11T07:05:28.6842595Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-11T07:05:30.5454037Z ....iiii.....ii.
2020-02-11T07:05:30.5454578Z 
2020-02-11T07:05:30.5454634Z  finished in 15.317
2020-02-11T07:05:30.5462132Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T07:05:30.5462748Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-11T07:18:31.4353757Z 
2020-02-11T07:18:31.4356506Z    Doc-tests core
2020-02-11T07:18:36.0320697Z 
2020-02-11T07:18:36.0321556Z running 2471 tests
2020-02-11T07:18:45.7496250Z ......iiiii......................................................................................... 100/2471
2020-02-11T07:18:54.4835926Z ..................................................................................ii................ 200/2471
2020-02-11T07:19:15.4987207Z .................i.................................................................................. 400/2471
2020-02-11T07:19:15.4987207Z .................i.................................................................................. 400/2471
2020-02-11T07:19:25.1676560Z ......................................................................i..i..................iiii.... 500/2471
2020-02-11T07:19:41.7570196Z .................................................................................................... 700/2471
2020-02-11T07:19:50.3840714Z .................................................................................................... 800/2471
2020-02-11T07:19:58.9389203Z .................................................................................................... 900/2471
2020-02-11T07:20:07.7482262Z .................................................................................................... 1000/2471
---
2020-02-11T07:23:36.6971979Z ..thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1727:37
2020-02-11T07:23:42.1861603Z ............
2020-02-11T07:23:42.1862372Z failures:
2020-02-11T07:23:42.1862554Z 
2020-02-11T07:23:42.1886031Z ---- backtrace::tests::debug_backtrace_fmt stdout ----
2020-02-11T07:23:42.1886292Z uncaptured: disabled backtrace
2020-02-11T07:23:42.1886685Z captured: Backtrace [{ fn: "std::backtrace::tests::debug_backtrace_fmt" }, { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" }, { fn: "__rust_maybe_catch_panic" }, { fn: "test::run_test::run_test_inner::{{closure}}" }, { fn: "std::sys_common::backtrace::__rust_begin_short_backtrace" }, { fn: "std::panicking::try::do_call" }, { fn: "__rust_maybe_catch_panic" }, { fn: "core::ops::function::FnOnce::call_once{{vtable.shim}}" }, { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" }, { fn: "std::sys::unix::thread::Thread::new::thread_start" }, { fn: "start_thread" }, { fn: "__clone" }]
2020-02-11T07:23:42.1886939Z display print: stack backtrace:
2020-02-11T07:23:42.1887398Z    0: std::backtrace::tests::debug_backtrace_fmt
2020-02-11T07:23:42.1887583Z    1: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
2020-02-11T07:23:42.1888619Z    3: test::run_test::run_test_inner::{{closure}}
2020-02-11T07:23:42.1888771Z    4: std::sys_common::backtrace::__rust_begin_short_backtrace
2020-02-11T07:23:42.1889081Z    5: std::panicking::try::do_call
2020-02-11T07:23:42.1889230Z    6: __rust_maybe_catch_panic
2020-02-11T07:23:42.1889230Z    6: __rust_maybe_catch_panic
2020-02-11T07:23:42.1889394Z    7: core::ops::function::FnOnce::call_once{{vtable.shim}}
2020-02-11T07:23:42.1889551Z    8: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
2020-02-11T07:23:42.1889702Z    9: std::sys::unix::thread::Thread::new::thread_start
2020-02-11T07:23:42.1889866Z   10: start_thread
2020-02-11T07:23:42.1890011Z   11: __clone
2020-02-11T07:23:42.1890301Z display print alt: stack backtrace:
2020-02-11T07:23:42.1890301Z display print alt: stack backtrace:
2020-02-11T07:23:42.1890870Z    0:     0x55ded47ff28d - std::backtrace::Backtrace::create::hf703babbcd3e85d1
2020-02-11T07:23:42.1891329Z    1:     0x55ded47ff200 - std::backtrace::Backtrace::force_capture::hb95f1d1c7097ba31
2020-02-11T07:23:42.1891777Z    2:     0x55ded484b064 - std::backtrace::tests::debug_backtrace_fmt::h568ed6c2496cee04
2020-02-11T07:23:42.1892230Z    3:     0x7f5137e398cf - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hb6b372f16c36fb60
2020-02-11T07:23:42.1892652Z    4:     0x7f5137b9b727 - __rust_maybe_catch_panic
2020-02-11T07:23:42.1893084Z    5:     0x7f5137e58a8f - test::run_test::run_test_inner::{{closure}}::h048df9d5557c3257
2020-02-11T07:23:42.1893524Z    6:     0x7f5137e2b649 - std::sys_common::backtrace::__rust_begin_short_backtrace::h99192f66d51e6047
2020-02-11T07:23:42.1893953Z    7:     0x7f5137e30319 - std::panicking::try::do_call::ha91683607659b9cc
2020-02-11T07:23:42.1894335Z    8:     0x7f5137b9b727 - __rust_maybe_catch_panic
2020-02-11T07:23:42.1894792Z    9:     0x7f5137e31c06 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hd0f6a17b7e19090f
2020-02-11T07:23:42.1895243Z   10:     0x7f5137b63adf - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::ha37d21f7f8ae446d
2020-02-11T07:23:42.1895676Z   11:     0x7f5137b995f0 - std::sys::unix::thread::Thread::new::thread_start::h75b8fb4f22922630
2020-02-11T07:23:42.1896079Z   12:     0x7f513809e6db - start_thread
2020-02-11T07:23:42.1896440Z   13:     0x7f513762d88f - __clone
2020-02-11T07:23:42.1896797Z   14:                0x0 - <unknown>
2020-02-11T07:23:42.1896956Z 
2020-02-11T07:23:42.1897192Z resolved: Backtrace [{ fn: "std::backtrace::tests::debug_backtrace_fmt" }, { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" }, { fn: "__rust_maybe_catch_panic" }, { fn: "test::run_test::run_test_inner::{{closure}}" }, { fn: "std::sys_common::backtrace::__rust_begin_short_backtrace" }, { fn: "std::panicking::try::do_call" }, { fn: "__rust_maybe_catch_panic" }, { fn: "core::ops::function::FnOnce::call_once{{vtable.shim}}" }, { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" }, { fn: "std::sys::unix::thread::Thread::new::thread_start" }, { fn: "start_thread" }, { fn: "__clone" }]
2020-02-11T07:23:42.1897404Z resolved alt: Backtrace [
2020-02-11T07:23:42.1897556Z     { fn: "std::backtrace::tests::debug_backtrace_fmt" },
2020-02-11T07:23:42.1897740Z     { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" },
2020-02-11T07:23:42.1897898Z     { fn: "__rust_maybe_catch_panic" },
2020-02-11T07:23:42.1898065Z     { fn: "test::run_test::run_test_inner::{{closure}}" },
2020-02-11T07:23:42.1898220Z     { fn: "std::sys_common::backtrace::__rust_begin_short_backtrace" },
2020-02-11T07:23:42.1898372Z     { fn: "std::panicking::try::do_call" },
2020-02-11T07:23:42.1898535Z     { fn: "__rust_maybe_catch_panic" },
2020-02-11T07:23:42.1898691Z     { fn: "core::ops::function::FnOnce::call_once{{vtable.shim}}" },
2020-02-11T07:23:42.1898928Z     { fn: "<alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once" },
2020-02-11T07:23:42.1899114Z     { fn: "std::sys::unix::thread::Thread::new::thread_start" },
2020-02-11T07:23:42.1899262Z     { fn: "start_thread" },
2020-02-11T07:23:42.1899419Z     { fn: "__clone" },
2020-02-11T07:23:42.1899562Z ]
2020-02-11T07:23:42.1899965Z thread '<unnamed>' panicked at 'not implemented', src/libstd/backtrace.rs:426:9
2020-02-11T07:23:42.1900388Z 
2020-02-11T07:23:42.1900509Z 
2020-02-11T07:23:42.1900650Z failures:
2020-02-11T07:23:42.1900815Z     backtrace::tests::debug_backtrace_fmt
2020-02-11T07:23:42.1900815Z     backtrace::tests::debug_backtrace_fmt
2020-02-11T07:23:42.1900943Z 
2020-02-11T07:23:42.1901179Z test result: FAILED. 760 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-11T07:23:42.1901324Z 
2020-02-11T07:23:42.1906247Z error: test failed, to rerun pass '-p std --lib'
2020-02-11T07:23:42.1916705Z 
2020-02-11T07:23:42.1917600Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "std" "--" "--quiet"
2020-02-11T07:23:42.1917702Z expected success, got: exit code: 101
2020-02-11T07:23:42.1917753Z 
---
2020-02-11T07:23:42.8250111Z   local time: Tue Feb 11 07:23:42 UTC 2020
2020-02-11T07:23:42.8250542Z   network time: Tue, 11 Feb 2020 07:23:42 GMT
2020-02-11T07:23:42.8250741Z == end clock drift check ==
2020-02-11T07:23:42.9119128Z 
2020-02-11T07:23:42.9212569Z ##[error]Bash exited with code '1'.
2020-02-11T07:23:42.9271872Z ##[section]Finishing: Run build
2020-02-11T07:23:42.9294247Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-11T07:23:42.9296193Z Task         : Get sources
2020-02-11T07:23:42.9296262Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T07:23:42.9296331Z Version      : 1.0.0
2020-02-11T07:23:42.9296377Z Author       : Microsoft
2020-02-11T07:23:42.9296377Z Author       : Microsoft
2020-02-11T07:23:42.9296429Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-11T07:23:42.9296501Z ==============================================================================
2020-02-11T07:23:43.3566859Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-11T07:23:43.3608313Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69038/merge to s
2020-02-11T07:23:43.3729495Z Cleaning up task key
2020-02-11T07:23:43.3730452Z Start cleaning up orphan processes.
2020-02-11T07:23:43.3836652Z Terminate orphan process: pid (3833) (python)
2020-02-11T07:23:43.4317095Z ##[section]Finishing: Finalize Job
