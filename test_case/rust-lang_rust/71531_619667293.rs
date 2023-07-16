plain
2020-04-27T00:54:06.0793558Z ========================== Starting Command Output ===========================
2020-04-27T00:54:06.0795707Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8dcff27c-e038-4a88-b856-65c765425864.sh
2020-04-27T00:54:06.0795941Z 
2020-04-27T00:54:06.0799290Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-27T00:54:06.0834125Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71531/merge to s
2020-04-27T00:54:06.0837138Z Task         : Get sources
2020-04-27T00:54:06.0837413Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T00:54:06.0837680Z Version      : 1.0.0
2020-04-27T00:54:06.0837881Z Author       : Microsoft
---
2020-04-27T00:54:07.0753830Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-27T00:54:07.0766146Z ##[command]git config gc.auto 0
2020-04-27T00:54:07.0771562Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-27T00:54:07.0779098Z ##[command]git config --get-all http.proxy
2020-04-27T00:54:07.0785723Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71531/merge:refs/remotes/pull/71531/merge
---
2020-04-27T00:56:11.5937572Z  ---> cb2676f08729
2020-04-27T00:56:11.5938721Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-27T00:56:11.5961923Z  ---> Using cache
2020-04-27T00:56:11.5962766Z  ---> df25ce111862
2020-04-27T00:56:11.5964379Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-27T00:56:11.5966555Z  ---> 599b9ac96b27
2020-04-27T00:56:11.5974927Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-27T00:56:11.5975842Z  ---> Using cache
2020-04-27T00:56:11.5976617Z  ---> 091087e35a36
---
2020-04-27T00:56:11.6346456Z Looks like docker image is the same as before, not uploading
2020-04-27T00:56:19.5670562Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-27T00:56:19.5913368Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-27T00:56:19.5941100Z == clock drift check ==
2020-04-27T00:56:19.5949994Z   local time: Mon Apr 27 00:56:19 UTC 2020
2020-04-27T00:56:19.7559589Z   network time: Mon, 27 Apr 2020 00:56:19 GMT
2020-04-27T00:56:19.7586233Z Starting sccache server...
2020-04-27T00:56:19.8367731Z configure: processing command line
2020-04-27T00:56:19.8368427Z configure: 
2020-04-27T00:56:19.8369661Z configure: rust.dist-src        := False
---
2020-04-27T00:58:49.3394731Z    Compiling unicode-width v0.1.6
2020-04-27T00:58:49.4414618Z    Compiling getopts v0.2.21
2020-04-27T00:58:59.4069889Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-27T00:59:07.6098141Z     Finished release [optimized] target(s) in 1m 00s
2020-04-27T00:59:07.6099399Z {"reason":"build-finished","success":true}
2020-04-27T00:59:07.6342051Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T00:59:08.2167935Z    Compiling cfg-if v0.1.10
2020-04-27T00:59:08.2168444Z    Compiling libc v0.2.69
2020-04-27T00:59:08.2646089Z    Compiling semver-parser v0.7.0
---
2020-04-27T01:01:33.3686382Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T01:01:34.7383302Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T01:01:36.2396542Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T01:01:36.7233446Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T01:01:46.2851700Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T01:01:47.9370579Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T01:01:52.2619908Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T01:01:56.2177046Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T01:02:06.2802031Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T01:17:06.8695784Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T01:17:14.2644209Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T01:19:43.8237910Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T01:19:44.5031833Z     Finished release [optimized] target(s) in 20m 36s
2020-04-27T01:19:44.5033045Z {"reason":"build-finished","success":true}
2020-04-27T01:19:44.5581663Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-27T01:19:44.5595661Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T01:19:44.8615290Z    Compiling cc v1.0.50
2020-04-27T01:19:44.8616721Z    Compiling core v0.0.0 (/checkout/src/libcore)
---
2020-04-27T01:20:31.7090503Z    Compiling unicode-width v0.1.6
2020-04-27T01:20:31.8027879Z    Compiling getopts v0.2.21
2020-04-27T01:20:43.7560857Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-27T01:20:53.0426533Z     Finished release [optimized] target(s) in 1m 08s
2020-04-27T01:20:53.0430845Z {"reason":"build-finished","success":true}
2020-04-27T01:20:53.0570973Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T01:20:53.6551707Z    Compiling cfg-if v0.1.10
2020-04-27T01:20:53.6553545Z    Compiling libc v0.2.69
2020-04-27T01:20:53.7061590Z    Compiling semver-parser v0.7.0
---
2020-04-27T01:23:35.2839558Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T01:23:36.8394111Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T01:23:38.6466894Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T01:23:38.9929621Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T01:23:49.1137042Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T01:23:51.1331801Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T01:23:55.5997542Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T01:23:59.8839666Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T01:24:09.9773630Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T01:37:56.4679985Z    Compiling rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-04-27T01:39:13.6531562Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T01:39:44.7425464Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T01:42:43.7968242Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T01:42:44.4373771Z {"reason":"build-finished","success":true}
2020-04-27T01:42:44.4848330Z Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-27T01:42:44.4888583Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2020-04-27T01:42:44.4902924Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T01:42:44.4903881Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-04-27T01:43:25.3456208Z    Compiling serde_derive v1.0.81
2020-04-27T01:43:52.3659727Z    Compiling serde_json v1.0.40
2020-04-27T01:43:53.7581459Z    Compiling rustfix v0.5.0
2020-04-27T01:43:56.8298648Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2020-04-27T01:44:10.5466438Z {"reason":"build-finished","success":true}
2020-04-27T01:44:10.5865560Z Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T01:44:13.4879354Z 
2020-04-27T01:44:13.4880262Z running 9927 tests
2020-04-27T01:44:27.1158557Z .................................................................................................... 100/9927
---
2020-04-27T01:46:17.3538018Z .................................................................................................... 1700/9927
2020-04-27T01:46:22.0585764Z .................................................................................................... 1800/9927
2020-04-27T01:46:30.2092780Z .................................................................................................... 1900/9927
2020-04-27T01:46:38.9093989Z ........i........................................................................................... 2000/9927
2020-04-27T01:46:45.5821875Z ..................................................................................................ii 2100/9927
2020-04-27T01:46:59.4694062Z iii................................................................................................. 2200/9927
2020-04-27T01:47:08.3130037Z .................................................................................................... 2400/9927
2020-04-27T01:47:10.7078081Z .................................................................................................... 2500/9927
2020-04-27T01:47:16.4496530Z .................................................................................................... 2600/9927
2020-04-27T01:47:35.0864931Z .................................................................................................... 2700/9927
---
2020-04-27T01:50:14.1917350Z .................................................................................................... 5100/9927
2020-04-27T01:50:21.2314405Z .................................................................................................... 5200/9927
2020-04-27T01:50:25.8993248Z .....................i.............................................................................. 5300/9927
2020-04-27T01:50:35.4625512Z ............i....................................................................................... 5400/9927
2020-04-27T01:50:40.8873211Z ............ii.ii........i...i...................................................................... 5500/9927
2020-04-27T01:50:48.5222184Z ...........................................................i........................................ 5700/9927
2020-04-27T01:50:56.9653718Z ............................................................................................ii...... 5800/9927
2020-04-27T01:51:03.4219084Z ...............................i.................................................................... 5900/9927
2020-04-27T01:51:08.8791551Z .................................................................................................... 6000/9927
2020-04-27T01:51:08.8791551Z .................................................................................................... 6000/9927
2020-04-27T01:51:18.8019744Z .................................................................................................... 6100/9927
2020-04-27T01:51:28.6674322Z .........................ii...i..ii...........i..................................................... 6200/9927
2020-04-27T01:51:44.5231880Z .................................................................................................... 6400/9927
2020-04-27T01:51:49.4009377Z .................................................................................................... 6500/9927
2020-04-27T01:51:49.4009377Z .................................................................................................... 6500/9927
2020-04-27T01:51:56.0081600Z .......................................................i..ii........................................ 6600/9927
2020-04-27T01:52:20.1087149Z .................................................................................................... 6800/9927
2020-04-27T01:52:23.8721273Z ........................................................i........................................... 6900/9927
2020-04-27T01:52:25.9416703Z .................................................................................................... 7000/9927
2020-04-27T01:52:28.0744721Z ..................................................................................................i. 7100/9927
---
2020-04-27T01:54:03.4943042Z .................................................................................................... 7900/9927
2020-04-27T01:54:08.9152172Z .................................................................................................... 8000/9927
2020-04-27T01:54:15.2104633Z ..................................................................i................................. 8100/9927
2020-04-27T01:54:25.0187479Z .................................................................................................... 8200/9927
2020-04-27T01:54:30.2600073Z ...............iiiiii.iiiii.i....................................................................... 8300/9927
2020-04-27T01:54:43.6049586Z .................................................................................................... 8500/9927
2020-04-27T01:54:49.2941898Z .................................................................................................... 8600/9927
2020-04-27T01:55:03.3640379Z .................................................................................................... 8700/9927
2020-04-27T01:55:09.9540806Z .................................................................................................... 8800/9927
---
2020-04-27T01:56:44.6818824Z ...............................i.................................................................... 9900/9927
2020-04-27T01:56:54.0869486Z ...........................
2020-04-27T01:56:54.0870773Z failures:
2020-04-27T01:56:54.0917560Z 
2020-04-27T01:56:54.0918326Z ---- [ui] ui/treat-err-as-bug/err.rs stdout ----
2020-04-27T01:56:54.0918673Z 
2020-04-27T01:56:54.0918799Z 6 
2020-04-27T01:56:54.0918799Z 6 
2020-04-27T01:56:54.0919335Z 7 thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:930:13
2020-04-27T01:56:54.0920066Z -    0: backtrace::backtrace::libunwind::trace
2020-04-27T01:56:54.0920551Z -    1: backtrace::backtrace::trace_unsynchronized
2020-04-27T01:56:54.0921011Z -    2: std::sys_common::backtrace::_print_fmt
2020-04-27T01:56:54.0921876Z -    3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
2020-04-27T01:56:54.0921876Z -    3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
2020-04-27T01:56:54.0922705Z -    4: core::fmt::write
2020-04-27T01:56:54.0923113Z -    5: std::io::Write::write_fmt
2020-04-27T01:56:54.0923535Z -    6: std::sys_common::backtrace::_print
2020-04-27T01:56:54.0923989Z -    7: std::sys_common::backtrace::print
2020-04-27T01:56:54.0924445Z -    8: std::panicking::default_hook::{{closure}}
2020-04-27T01:56:54.0924876Z -    9: std::panicking::default_hook
2020-04-27T01:56:54.0925434Z -   10: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
2020-04-27T01:56:54.0925894Z -   11: rustc_driver::report_ice
2020-04-27T01:56:54.0926310Z -   12: std::panicking::rust_panic_with_hook
2020-04-27T01:56:54.0926743Z -   13: std::panicking::begin_panic
2020-04-27T01:56:54.0927210Z -   14: rustc_errors::HandlerInner::panic_if_treat_err_as_bug
2020-04-27T01:56:54.0927691Z -   15: rustc_errors::HandlerInner::bump_err_count
2020-04-27T01:56:54.0928177Z -   16: rustc_errors::HandlerInner::emit_diagnostic
2020-04-27T01:56:54.0928630Z -   17: rustc_errors::Handler::emit_diagnostic
2020-04-27T01:56:54.0929361Z -   18: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
2020-04-27T01:56:54.0929984Z -   19: rustc_middle::mir::interpret::error::ConstEvalErr::report_as_error::{{closure}}
2020-04-27T01:56:54.0930709Z -   20: rustc_middle::mir::interpret::error::ConstEvalErr::struct_generic::{{closure}}
2020-04-27T01:56:54.0931328Z -   21: rustc_middle::mir::interpret::error::ConstEvalErr::struct_generic
2020-04-27T01:56:54.0931918Z -   22: rustc_middle::mir::interpret::error::ConstEvalErr::struct_error
2020-04-27T01:56:54.0932492Z -   23: rustc_middle::mir::interpret::error::ConstEvalErr::report_as_error
2020-04-27T01:56:54.0933100Z -   24: rustc_mir::const_eval::eval_queries::const_eval_raw_provider::{{closure}}
2020-04-27T01:56:54.0933834Z -   25: core::result::Result<T,E>::map_err
2020-04-27T01:56:54.0934339Z -   26: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
2020-04-27T01:56:54.0935226Z -   27: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_raw>::compute
2020-04-27T01:56:54.0936042Z -   28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
2020-04-27T01:56:54.0936609Z -   29: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-27T01:56:54.0937245Z -   30: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
2020-04-27T01:56:54.0938128Z -   31: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
2020-04-27T01:56:54.0938864Z -   32: rustc_middle::ty::context::tls::enter_context::{{closure}}
2020-04-27T01:56:54.0939377Z -   33: rustc_middle::ty::context::tls::set_tlv
2020-04-27T01:56:54.0939853Z -   34: rustc_middle::ty::context::tls::enter_context
2020-04-27T01:56:54.0940627Z -   35: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
2020-04-27T01:56:54.0941380Z -   36: rustc_middle::ty::context::tls::with_related_context::{{closure}}
2020-04-27T01:56:54.0941938Z -   37: rustc_middle::ty::context::tls::with_context::{{closure}}
2020-04-27T01:56:54.0942458Z -   38: rustc_middle::ty::context::tls::with_context_opt
2020-04-27T01:56:54.0942965Z -   39: rustc_middle::ty::context::tls::with_context
2020-04-27T01:56:54.0943466Z -   40: rustc_middle::ty::context::tls::with_related_context
2020-04-27T01:56:54.0944206Z -   41: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
2020-04-27T01:56:54.0944949Z -   42: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
2020-04-27T01:56:54.0947269Z -   43: rustc_query_system::query::plumbing::with_diagnostics
2020-04-27T01:56:54.0949142Z -   44: rustc_query_system::query::plumbing::force_query_with_job
2020-04-27T01:56:54.0949707Z -   45: rustc_query_system::query::plumbing::try_execute_query
2020-04-27T01:56:54.0950258Z -   46: rustc_query_system::query::plumbing::get_query::{{closure}}
2020-04-27T01:56:54.0950980Z -   47: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
2020-04-27T01:56:54.0952145Z -   48: rustc_query_system::query::plumbing::try_get_cached
2020-04-27T01:56:54.0952678Z -   49: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.0953192Z -   50: rustc_middle::ty::query::TyCtxtAt::const_eval_raw
2020-04-27T01:56:54.0953786Z -   51: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_raw
2020-04-27T01:56:54.0954389Z -   52: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-27T01:56:54.0955319Z -   53: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-27T01:56:54.0956275Z -   54: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
2020-04-27T01:56:54.0956838Z -   55: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-27T01:56:54.0957474Z -   56: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
2020-04-27T01:56:54.0958415Z -   57: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
2020-04-27T01:56:54.0959168Z -   58: rustc_middle::ty::context::tls::enter_context::{{closure}}
2020-04-27T01:56:54.0959682Z -   59: rustc_middle::ty::context::tls::set_tlv
2020-04-27T01:56:54.0960161Z -   60: rustc_middle::ty::context::tls::enter_context
2020-04-27T01:56:54.0960932Z -   61: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
2020-04-27T01:56:54.0961684Z -   62: rustc_middle::ty::context::tls::with_related_context::{{closure}}
2020-04-27T01:56:54.0962250Z -   63: rustc_middle::ty::context::tls::with_context::{{closure}}
2020-04-27T01:56:54.0962784Z -   64: rustc_middle::ty::context::tls::with_context_opt
2020-04-27T01:56:54.0963269Z -   65: rustc_middle::ty::context::tls::with_context
2020-04-27T01:56:54.0963771Z -   66: rustc_middle::ty::context::tls::with_related_context
2020-04-27T01:56:54.0967874Z -   67: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
2020-04-27T01:56:54.0968649Z -   68: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
2020-04-27T01:56:54.0969207Z -   69: rustc_query_system::query::plumbing::with_diagnostics
2020-04-27T01:56:54.0969750Z -   70: rustc_query_system::query::plumbing::force_query_with_job
2020-04-27T01:56:54.0970276Z -   71: rustc_query_system::query::plumbing::try_execute_query
2020-04-27T01:56:54.0970812Z -   72: rustc_query_system::query::plumbing::get_query::{{closure}}
2020-04-27T01:56:54.0971547Z -   73: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
2020-04-27T01:56:54.0978153Z -   74: rustc_query_system::query::plumbing::try_get_cached
2020-04-27T01:56:54.0978690Z -   75: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.0979258Z -   76: rustc_middle::ty::query::TyCtxtAt::const_eval_validated
2020-04-27T01:56:54.0979884Z -   77: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_validated
2020-04-27T01:56:54.0982956Z -   78: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-27T01:56:54.0985233Z -   79: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-27T01:56:54.0986066Z -   80: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
2020-04-27T01:56:54.0986639Z -   81: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-27T01:56:54.0987296Z -   82: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
2020-04-27T01:56:54.0988181Z -   83: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
2020-04-27T01:56:54.0988938Z -   84: rustc_middle::ty::context::tls::enter_context::{{closure}}
2020-04-27T01:56:54.0989480Z -   85: rustc_middle::ty::context::tls::set_tlv
2020-04-27T01:56:54.0989966Z -   86: rustc_middle::ty::context::tls::enter_context
2020-04-27T01:56:54.0990760Z -   87: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
2020-04-27T01:56:54.0991504Z -   88: rustc_middle::ty::context::tls::with_related_context::{{closure}}
2020-04-27T01:56:54.0992069Z -   89: rustc_middle::ty::context::tls::with_context::{{closure}}
2020-04-27T01:56:54.0992809Z -   90: rustc_middle::ty::context::tls::with_context_opt
2020-04-27T01:56:54.0993301Z -   91: rustc_middle::ty::context::tls::with_context
2020-04-27T01:56:54.0993810Z -   92: rustc_middle::ty::context::tls::with_related_context
2020-04-27T01:56:54.0994647Z -   93: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
2020-04-27T01:56:54.0995391Z -   94: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
2020-04-27T01:56:54.0995944Z -   95: rustc_query_system::query::plumbing::with_diagnostics
2020-04-27T01:56:54.0996482Z -   96: rustc_query_system::query::plumbing::force_query_with_job
2020-04-27T01:56:54.0997002Z -   97: rustc_query_system::query::plumbing::try_execute_query
2020-04-27T01:56:54.0997542Z -   98: rustc_query_system::query::plumbing::get_query::{{closure}}
2020-04-27T01:56:54.0998271Z -   99: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
2020-04-27T01:56:54.0998913Z -  100: rustc_query_system::query::plumbing::try_get_cached
2020-04-27T01:56:54.0999413Z -  101: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.0999948Z -  102: rustc_middle::ty::query::TyCtxtAt::const_eval_validated
2020-04-27T01:56:54.1000572Z -  103: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_validated
2020-04-27T01:56:54.1001314Z -  104: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id
2020-04-27T01:56:54.1002049Z -  105: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_poly
2020-04-27T01:56:54.1002750Z -  106: <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item
2020-04-27T01:56:54.1003452Z -  107: <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item
2020-04-27T01:56:54.1004214Z -  108: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}::{{closure}}
2020-04-27T01:56:54.1004866Z -  109: rustc_lint::late::LateContextAndPass<T>::with_param_env
2020-04-27T01:56:54.1005543Z -  110: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}
2020-04-27T01:56:54.1006169Z -  111: rustc_lint::late::LateContextAndPass<T>::with_lint_attrs
2020-04-27T01:56:54.1006795Z -  112: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item
2020-04-27T01:56:54.1007391Z -  113: rustc_hir::intravisit::Visitor::visit_nested_item
2020-04-27T01:56:54.1007850Z -  114: rustc_hir::intravisit::walk_mod
2020-04-27T01:56:54.1008334Z -  115: rustc_lint::late::LateContextAndPass<T>::process_mod
2020-04-27T01:56:54.1008965Z -  116: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_mod
2020-04-27T01:56:54.1009501Z -  117: rustc_hir::intravisit::walk_crate
2020-04-27T01:56:54.1009980Z -  118: rustc_lint::late::late_lint_pass_crate::{{closure}}
2020-04-27T01:56:54.1010532Z -  119: rustc_lint::late::LateContextAndPass<T>::with_lint_attrs
2020-04-27T01:56:54.1011011Z -  120: rustc_lint::late::late_lint_pass_crate
2020-04-27T01:56:54.1011449Z -  121: rustc_lint::late::late_lint_crate
2020-04-27T01:56:54.1011967Z -  122: rustc_lint::late::check_crate::{{closure}}::{{closure}}
2020-04-27T01:56:54.1012512Z -  123: rustc_data_structures::profiling::VerboseTimingGuard::run
2020-04-27T01:56:54.1013095Z -  124: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-27T01:56:54.1013994Z -  125: rustc_lint::late::check_crate::{{closure}}
2020-04-27T01:56:54.1014446Z -  126: rustc_data_structures::sync::join
2020-04-27T01:56:54.1014871Z -  127: rustc_lint::late::check_crate
2020-04-27T01:56:54.1015457Z -  128: rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}::{{closure}}
2020-04-27T01:56:54.1016058Z -  129: rustc_data_structures::profiling::VerboseTimingGuard::run
2020-04-27T01:56:54.1016633Z -  130: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-27T01:56:54.1017349Z -  131: rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}
2020-04-27T01:56:54.1017875Z -  132: core::ops::function::FnOnce::call_once
2020-04-27T01:56:54.1018508Z -  133: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
2020-04-27T01:56:54.1019051Z -  134: std::panicking::try::do_call
2020-04-27T01:56:54.1019458Z -  135: std::panicking::try::do_try
2020-04-27T01:56:54.1019842Z -  136: std::panicking::try
2020-04-27T01:56:54.1020238Z -  137: std::panic::catch_unwind
2020-04-27T01:56:54.1020721Z -  138: rustc_interface::passes::analysis::{{closure}}::{{closure}}
2020-04-27T01:56:54.1021217Z -  139: core::ops::function::FnOnce::call_once
2020-04-27T01:56:54.1021811Z -  140: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
2020-04-27T01:56:54.1022318Z -  141: std::panicking::try::do_call
2020-04-27T01:56:54.1022727Z -  142: std::panicking::try::do_try
2020-04-27T01:56:54.1023136Z -  143: std::panicking::try
2020-04-27T01:56:54.1023513Z -  144: std::panic::catch_unwind
2020-04-27T01:56:54.1023964Z -  145: rustc_interface::passes::analysis::{{closure}}
2020-04-27T01:56:54.1024505Z -  146: rustc_data_structures::profiling::VerboseTimingGuard::run
2020-04-27T01:56:54.1025079Z -  147: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-27T01:56:54.1025576Z -  148: rustc_interface::passes::analysis
2020-04-27T01:56:54.1026680Z -  149: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
2020-04-27T01:56:54.1027641Z -  150: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
2020-04-27T01:56:54.1028243Z -  151: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
2020-04-27T01:56:54.1028905Z -  152: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
2020-04-27T01:56:54.1029944Z -  153: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
2020-04-27T01:56:54.1030685Z -  154: rustc_middle::ty::context::tls::enter_context::{{closure}}
2020-04-27T01:56:54.1031193Z -  155: rustc_middle::ty::context::tls::set_tlv
2020-04-27T01:56:54.1031669Z -  156: rustc_middle::ty::context::tls::enter_context
2020-04-27T01:56:54.1032618Z -  157: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
2020-04-27T01:56:54.1034761Z -  158: rustc_middle::ty::context::tls::with_related_context::{{closure}}
2020-04-27T01:56:54.1035390Z -  159: rustc_middle::ty::context::tls::with_context::{{closure}}
2020-04-27T01:56:54.1036626Z -  160: rustc_middle::ty::context::tls::with_context_opt
2020-04-27T01:56:54.1037192Z -  161: rustc_middle::ty::context::tls::with_context
2020-04-27T01:56:54.1037718Z -  162: rustc_middle::ty::context::tls::with_related_context
2020-04-27T01:56:54.1038482Z -  163: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
2020-04-27T01:56:54.1039219Z -  164: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
2020-04-27T01:56:54.1039773Z -  165: rustc_query_system::query::plumbing::with_diagnostics
2020-04-27T01:56:54.1040327Z -  166: rustc_query_system::query::plumbing::force_query_with_job
2020-04-27T01:56:54.1040854Z -  167: rustc_query_system::query::plumbing::try_execute_query
2020-04-27T01:56:54.1041396Z -  168: rustc_query_system::query::plumbing::get_query::{{closure}}
2020-04-27T01:56:54.1042124Z -  169: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
2020-04-27T01:56:54.1042917Z -  170: rustc_query_system::query::plumbing::try_get_cached
2020-04-27T01:56:54.1043426Z -  171: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.1044239Z -  172: rustc_middle::ty::query::TyCtxtAt::analysis
2020-04-27T01:56:54.1044821Z -  173: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::analysis
2020-04-27T01:56:54.1045576Z -  174: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
2020-04-27T01:56:54.1046341Z -  175: rustc_middle::ty::context::tls::enter_global::{{closure}}
2020-04-27T01:56:54.1046945Z -  176: rustc_middle::ty::context::tls::enter_context::{{closure}}
2020-04-27T01:56:54.1047653Z -  177: rustc_middle::ty::context::tls::set_tlv
2020-04-27T01:56:54.1048990Z -  178: rustc_middle::ty::context::tls::enter_context
2020-04-27T01:56:54.1049485Z -  179: rustc_middle::ty::context::tls::enter_global
2020-04-27T01:56:54.1049977Z -  180: rustc_interface::passes::QueryContext::enter
2020-04-27T01:56:54.1050509Z -  181: rustc_driver::run_compiler::{{closure}}::{{closure}}
2020-04-27T01:56:54.1051107Z -  182: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
2020-04-27T01:56:54.1051652Z -  183: rustc_driver::run_compiler::{{closure}}
2020-04-27T01:56:54.1052190Z -  184: rustc_interface::interface::run_compiler_in_existing_thread_pool
2020-04-27T01:56:54.1052724Z -  185: rustc_interface::interface::run_compiler::{{closure}}
2020-04-27T01:56:54.1054719Z -  186: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
2020-04-27T01:56:54.1055354Z -  187: scoped_tls::ScopedKey<T>::set
2020-04-27T01:56:54.1055877Z -  188: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
2020-04-27T01:56:54.1056367Z -  189: scoped_tls::ScopedKey<T>::set
2020-04-27T01:56:54.1056835Z -  190: rustc_ast::attr::with_globals::{{closure}}
2020-04-27T01:56:54.1057276Z -  191: scoped_tls::ScopedKey<T>::set
2020-04-27T01:56:54.1057688Z -  192: rustc_ast::attr::with_globals
2020-04-27T01:56:54.1058179Z -  193: rustc_interface::util::spawn_thread_pool::{{closure}}
2020-04-27T01:56:54.1058690Z -  194: rustc_interface::util::scoped_thread::{{closure}}
2020-04-27T01:56:54.1059436Z +    1: core::fmt::write
2020-04-27T01:56:54.1059653Z +    2: std::io::Write::write_fmt
2020-04-27T01:56:54.1059918Z +    3: std::panicking::default_hook::{{closure}}
2020-04-27T01:56:54.1060203Z +    4: std::panicking::default_hook
2020-04-27T01:56:54.1060203Z +    4: std::panicking::default_hook
2020-04-27T01:56:54.1060437Z +    5: rustc_driver::report_ice
2020-04-27T01:56:54.1060680Z +    6: std::panicking::rust_panic_with_hook
2020-04-27T01:56:54.1060945Z +    7: std::panicking::begin_panic
2020-04-27T01:56:54.1061210Z +    8: rustc_errors::HandlerInner::emit_diagnostic
2020-04-27T01:56:54.1061493Z +    9: rustc_errors::Handler::emit_diagnostic
2020-04-27T01:56:54.1061825Z +   10: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
2020-04-27T01:56:54.1062219Z +   11: rustc_middle::mir::interpret::error::ConstEvalErr::struct_generic::{{closure}}
2020-04-27T01:56:54.1062626Z +   12: rustc_middle::mir::interpret::error::ConstEvalErr::report_as_error
2020-04-27T01:56:54.1063014Z +   13: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
2020-04-27T01:56:54.1063609Z +   14: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_raw>::compute
2020-04-27T01:56:54.1064198Z +   15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-27T01:56:54.1064944Z +   16: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.1065289Z +   17: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-27T01:56:54.1065918Z +   18: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-27T01:56:54.1066522Z +   19: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-27T01:56:54.1066857Z +   20: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.1067336Z +   21: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-27T01:56:54.1068387Z +   22: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-27T01:56:54.1068999Z +   23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-27T01:56:54.1069358Z +   24: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.1069764Z +   25: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_poly
2020-04-27T01:56:54.1070249Z +   26: <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item
2020-04-27T01:56:54.1070716Z +   27: <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item
2020-04-27T01:56:54.1071095Z +   28: rustc_hir::intravisit::Visitor::visit_nested_item
2020-04-27T01:56:54.1071386Z +   29: rustc_hir::intravisit::walk_mod
2020-04-27T01:56:54.1071764Z +   30: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_mod
2020-04-27T01:56:54.1072122Z +   31: rustc_hir::intravisit::walk_crate
2020-04-27T01:56:54.1072452Z +   32: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-27T01:56:54.1072798Z +   33: rustc_data_structures::sync::join
2020-04-27T01:56:54.1073126Z +   34: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-27T01:56:54.1073434Z +   35: std::panic::catch_unwind
2020-04-27T01:56:54.1073763Z +   36: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-27T01:56:54.1074088Z +   37: rustc_interface::passes::analysis
2020-04-27T01:56:54.1074620Z +   38: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
2020-04-27T01:56:54.1075241Z +   39: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
2020-04-27T01:56:54.1075601Z +   40: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.1075926Z +   41: rustc_middle::ty::context::tls::enter_global
2020-04-27T01:56:54.1076257Z +   42: rustc_interface::interface::run_compiler_in_existing_thread_pool
2020-04-27T01:56:54.1076561Z +   43: scoped_tls::ScopedKey<T>::set
2020-04-27T01:56:54.1076824Z +   44: rustc_ast::attr::with_globals
2020-04-27T01:56:54.1077750Z 205 
2020-04-27T01:56:54.1078201Z 206 error: internal compiler error: unexpected panic
2020-04-27T01:56:54.1078389Z 
2020-04-27T01:56:54.1078503Z 209 
2020-04-27T01:56:54.1078503Z 209 
2020-04-27T01:56:54.1079337Z 210 note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-27T01:56:54.1080036Z - note: rustc 1.44.0-dev running on x86_64-unknown-linux-gnu
2020-04-27T01:56:54.1080895Z + note: rustc 1.45.0-nightly (bf1be065e 2020-04-27) running on x86_64-unknown-linux-gnu
2020-04-27T01:56:54.1081502Z 213 
2020-04-27T01:56:54.1081502Z 213 
2020-04-27T01:56:54.1082182Z 214 note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z treat-err-as-bug -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-27T01:56:54.1082663Z 
2020-04-27T01:56:54.1082752Z 
2020-04-27T01:56:54.1082939Z The actual stderr differed from the expected stderr.
2020-04-27T01:56:54.1082939Z The actual stderr differed from the expected stderr.
2020-04-27T01:56:54.1083530Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/err.stderr
2020-04-27T01:56:54.1085051Z To update references, rerun the tests and pass the `--bless` flag
2020-04-27T01:56:54.1085654Z To only update this specific test, also pass `--test-args treat-err-as-bug/err.rs`
2020-04-27T01:56:54.1086058Z error: 1 errors occurred comparing output.
2020-04-27T01:56:54.1086276Z status: exit code: 101
2020-04-27T01:56:54.1086276Z status: exit code: 101
2020-04-27T01:56:54.1088811Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/auxiliary"
2020-04-27T01:56:54.1091124Z ------------------------------------------
2020-04-27T01:56:54.1091284Z 
2020-04-27T01:56:54.1091635Z ------------------------------------------
2020-04-27T01:56:54.1091822Z stderr:
2020-04-27T01:56:54.1091822Z stderr:
2020-04-27T01:56:54.1092190Z ------------------------------------------
2020-04-27T01:56:54.1092985Z error[E0080]: could not evaluate static initializer
2020-04-27T01:56:54.1093711Z   --> /checkout/src/test/ui/treat-err-as-bug/err.rs:10:21
2020-04-27T01:56:54.1093942Z    |
2020-04-27T01:56:54.1094576Z LL | pub static C: u32 = 0 - 1;
2020-04-27T01:56:54.1095558Z 
2020-04-27T01:56:54.1095558Z 
2020-04-27T01:56:54.1096132Z thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:930:13
2020-04-27T01:56:54.1096770Z    0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
2020-04-27T01:56:54.1097093Z    1: core::fmt::write
2020-04-27T01:56:54.1097301Z    2: std::io::Write::write_fmt
2020-04-27T01:56:54.1097570Z    3: std::panicking::default_hook::{{closure}}
2020-04-27T01:56:54.1097570Z    3: std::panicking::default_hook::{{closure}}
2020-04-27T01:56:54.1098145Z    4: std::panicking::default_hook
2020-04-27T01:56:54.1098371Z    5: rustc_driver::report_ice
2020-04-27T01:56:54.1098620Z    6: std::panicking::rust_panic_with_hook
2020-04-27T01:56:54.1098864Z    7: std::panicking::begin_panic
2020-04-27T01:56:54.1099129Z    8: rustc_errors::HandlerInner::emit_diagnostic
2020-04-27T01:56:54.1099411Z    9: rustc_errors::Handler::emit_diagnostic
2020-04-27T01:56:54.1099724Z   10: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
2020-04-27T01:56:54.1100101Z   11: rustc_middle::mir::interpret::error::ConstEvalErr::struct_generic::{{closure}}
2020-04-27T01:56:54.1100515Z   12: rustc_middle::mir::interpret::error::ConstEvalErr::report_as_error
2020-04-27T01:56:54.1100880Z   13: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
2020-04-27T01:56:54.1101461Z   14: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_raw>::compute
2020-04-27T01:56:54.1102063Z   15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-27T01:56:54.1102392Z   16: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.1102723Z   17: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-27T01:56:54.1103344Z   18: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-27T01:56:54.1103944Z   19: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-27T01:56:54.1104287Z   20: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.1104619Z   21: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-27T01:56:54.1105215Z   22: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-27T01:56:54.1105822Z   23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-27T01:56:54.1106142Z   24: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.1106538Z   25: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_poly
2020-04-27T01:56:54.1108146Z   26: <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item
2020-04-27T01:56:54.1108628Z   27: <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item
2020-04-27T01:56:54.1109128Z   28: rustc_hir::intravisit::Visitor::visit_nested_item
2020-04-27T01:56:54.1109432Z   29: rustc_hir::intravisit::walk_mod
2020-04-27T01:56:54.1109785Z   30: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_mod
2020-04-27T01:56:54.1110138Z   31: rustc_hir::intravisit::walk_crate
2020-04-27T01:56:54.1110472Z   32: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-27T01:56:54.1110792Z   33: rustc_data_structures::sync::join
2020-04-27T01:56:54.1111111Z   34: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-27T01:56:54.1111910Z   36: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-27T01:56:54.1112236Z   37: rustc_interface::passes::analysis
2020-04-27T01:56:54.1112236Z   37: rustc_interface::passes::analysis
2020-04-27T01:56:54.1112781Z   38: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
2020-04-27T01:56:54.1113375Z   39: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
2020-04-27T01:56:54.1113722Z   40: rustc_query_system::query::plumbing::get_query
2020-04-27T01:56:54.1114037Z   41: rustc_middle::ty::context::tls::enter_global
2020-04-27T01:56:54.1114648Z   43: scoped_tls::ScopedKey<T>::set
2020-04-27T01:56:54.1114903Z   44: rustc_ast::attr::with_globals
2020-04-27T01:56:54.1115203Z note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
2020-04-27T01:56:54.1115425Z 
2020-04-27T01:56:54.1115425Z 
2020-04-27T01:56:54.1115632Z error: internal compiler error: unexpected panic
2020-04-27T01:56:54.1115812Z 
2020-04-27T01:56:54.1116409Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-27T01:56:54.1116587Z 
2020-04-27T01:56:54.1117403Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-27T01:56:54.1118156Z note: rustc 1.45.0-nightly (bf1be065e 2020-04-27) running on x86_64-unknown-linux-gnu
2020-04-27T01:56:54.1118406Z 
2020-04-27T01:56:54.1118406Z 
2020-04-27T01:56:54.1119070Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z treat-err-as-bug -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-27T01:56:54.1119597Z query stack during panic:
2020-04-27T01:56:54.1119597Z query stack during panic:
2020-04-27T01:56:54.1120013Z #0 [const_eval_raw] const-evaluating `C`
2020-04-27T01:56:54.1120468Z #1 [const_eval_validated] const-evaluating + checking `C`
2020-04-27T01:56:54.1120951Z #2 [const_eval_validated] const-evaluating + checking `C`
2020-04-27T01:56:54.1121258Z #3 [analysis] running analysis passes on this crate
2020-04-27T01:56:54.1121606Z 
2020-04-27T01:56:54.1121969Z ------------------------------------------
2020-04-27T01:56:54.1122134Z 
2020-04-27T01:56:54.1122230Z 
2020-04-27T01:56:54.1122230Z 
2020-04-27T01:56:54.1122324Z 
2020-04-27T01:56:54.1122455Z failures:
2020-04-27T01:56:54.1122825Z     [ui] ui/treat-err-as-bug/err.rs
2020-04-27T01:56:54.1123441Z test result: FAILED. 9865 passed; 1 failed; 61 ignored; 0 measured; 0 filtered out
2020-04-27T01:56:54.1123673Z 
2020-04-27T01:56:54.1124218Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-27T01:56:54.1124580Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-27T01:56:54.1124580Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-27T01:56:54.1124786Z 
2020-04-27T01:56:54.1124889Z 
2020-04-27T01:56:54.1128362Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-27T01:56:54.1130694Z 
2020-04-27T01:56:54.1130786Z 
2020-04-27T01:56:54.1131269Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-27T01:56:54.1131615Z Build completed unsuccessfully in 0:58:48
2020-04-27T01:56:54.1131615Z Build completed unsuccessfully in 0:58:48
2020-04-27T01:56:54.1131837Z == clock drift check ==
2020-04-27T01:56:54.1132059Z   local time: Mon Apr 27 01:56:54 UTC 2020
2020-04-27T01:56:54.3928482Z   network time: Mon, 27 Apr 2020 01:56:54 GMT
2020-04-27T01:56:54.7528088Z 
2020-04-27T01:56:54.7528088Z 
2020-04-27T01:56:54.7598125Z ##[error]Bash exited with code '1'.
2020-04-27T01:56:54.7616069Z ##[section]Finishing: Run build
2020-04-27T01:56:54.7661416Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71531/merge to s
2020-04-27T01:56:54.7666266Z Task         : Get sources
2020-04-27T01:56:54.7666585Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T01:56:54.7666864Z Version      : 1.0.0
2020-04-27T01:56:54.7667064Z Author       : Microsoft
2020-04-27T01:56:54.7667064Z Author       : Microsoft
2020-04-27T01:56:54.7667406Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-27T01:56:54.7667757Z ==============================================================================
2020-04-27T01:56:55.0982314Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-27T01:56:55.1023334Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71531/merge to s
2020-04-27T01:56:55.1113383Z Cleaning up task key
2020-04-27T01:56:55.1114615Z Start cleaning up orphan processes.
2020-04-27T01:56:55.1289621Z Terminate orphan process: pid (3846) (python)
2020-04-27T01:56:55.1456024Z ##[section]Finishing: Finalize Job
