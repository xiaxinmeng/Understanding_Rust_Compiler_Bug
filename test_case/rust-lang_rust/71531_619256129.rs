plain
2020-04-24T21:00:38.1310623Z ========================== Starting Command Output ===========================
2020-04-24T21:00:38.1313011Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/176453ac-3c65-422f-a561-e356b850901b.sh
2020-04-24T21:00:38.1313258Z 
2020-04-24T21:00:38.1317492Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T21:00:38.1338129Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71531/merge to s
2020-04-24T21:00:38.1341259Z Task         : Get sources
2020-04-24T21:00:38.1341535Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T21:00:38.1341804Z Version      : 1.0.0
2020-04-24T21:00:38.1342006Z Author       : Microsoft
---
2020-04-24T21:00:39.1254590Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T21:00:39.1259803Z ##[command]git config gc.auto 0
2020-04-24T21:00:39.1263367Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T21:00:39.1266594Z ##[command]git config --get-all http.proxy
2020-04-24T21:00:39.1272721Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71531/merge:refs/remotes/pull/71531/merge
---
2020-04-24T21:03:17.5171404Z  ---> cb2676f08729
2020-04-24T21:03:17.5172180Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-24T21:03:17.5172747Z  ---> Using cache
2020-04-24T21:03:17.5173056Z  ---> df25ce111862
2020-04-24T21:03:17.5173964Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-24T21:03:17.5174939Z  ---> 599b9ac96b27
2020-04-24T21:03:17.5175155Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-24T21:03:17.5175746Z  ---> Using cache
2020-04-24T21:03:17.5176057Z  ---> 091087e35a36
---
2020-04-24T21:03:17.5528154Z Looks like docker image is the same as before, not uploading
2020-04-24T21:03:25.4663514Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-24T21:03:25.4944497Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-24T21:03:25.4972269Z == clock drift check ==
2020-04-24T21:03:25.4980011Z   local time: Fri Apr 24 21:03:25 UTC 2020
2020-04-24T21:03:25.7895999Z   network time: Fri, 24 Apr 2020 21:03:25 GMT
2020-04-24T21:03:25.7921255Z Starting sccache server...
2020-04-24T21:03:25.8755369Z configure: processing command line
2020-04-24T21:03:25.8755683Z configure: 
2020-04-24T21:03:25.8756550Z configure: rust.dist-src        := False
---
2020-04-24T21:08:27.7919150Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T21:08:29.1522223Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T21:08:30.6561636Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T21:08:32.1475708Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T21:08:40.1460083Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T21:08:43.0807410Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T21:08:47.2824454Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T21:08:51.2503427Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T21:08:59.5654214Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T21:29:46.5151791Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T21:29:47.9915840Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T21:29:49.7004737Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T21:29:49.9055300Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T21:29:59.7214251Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T21:30:01.6451286Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T21:30:06.1643799Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T21:30:10.3336246Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T21:30:20.0288168Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T21:50:31.1002405Z .................................................................................................... 1700/9922
2020-04-24T21:50:34.2473628Z .................................................................................................... 1800/9922
2020-04-24T21:50:40.2694593Z .................................................................................................... 1900/9922
2020-04-24T21:50:46.1909830Z ......i............................................................................................. 2000/9922
2020-04-24T21:50:50.7284423Z ................................................................................................iiii 2100/9922
2020-04-24T21:51:00.7636311Z i................................................................................................... 2200/9922
2020-04-24T21:51:06.8341632Z .................................................................................................... 2400/9922
2020-04-24T21:51:08.5422446Z .................................................................................................... 2500/9922
2020-04-24T21:51:12.7369263Z .................................................................................................... 2600/9922
2020-04-24T21:51:25.6007522Z .................................................................................................... 2700/9922
---
2020-04-24T21:53:21.9286007Z .................................................................................................... 5100/9922
2020-04-24T21:53:27.1959017Z .................................................................................................... 5200/9922
2020-04-24T21:53:30.6222815Z ...................i................................................................................ 5300/9922
2020-04-24T21:53:37.6458583Z .........i.......................................................................................... 5400/9922
2020-04-24T21:53:41.6132292Z .........ii.ii........i...i......................................................................... 5500/9922
2020-04-24T21:53:47.2040941Z ........................................................i........................................... 5700/9922
2020-04-24T21:53:53.4652599Z .........................................................................................ii......... 5800/9922
2020-04-24T21:53:58.4549876Z ............................i....................................................................... 5900/9922
2020-04-24T21:54:02.4396925Z .................................................................................................... 6000/9922
2020-04-24T21:54:02.4396925Z .................................................................................................... 6000/9922
2020-04-24T21:54:09.9443672Z .................................................................................................... 6100/9922
2020-04-24T21:54:17.3779445Z ......................ii...i..ii...........i........................................................ 6200/9922
2020-04-24T21:54:28.5489766Z .................................................................................................... 6400/9922
2020-04-24T21:54:31.0193781Z .................................................................................................... 6500/9922
2020-04-24T21:54:31.0193781Z .................................................................................................... 6500/9922
2020-04-24T21:54:35.8962618Z ....................................................i..ii........................................... 6600/9922
2020-04-24T21:54:53.8536771Z .................................................................................................... 6800/9922
2020-04-24T21:54:55.9284618Z .....................................................i.............................................. 6900/9922
2020-04-24T21:54:57.4291146Z .................................................................................................... 7000/9922
2020-04-24T21:54:58.9265368Z ...............................................................................................i.... 7100/9922
---
2020-04-24T21:56:08.1353835Z .................................................................................................... 7900/9922
2020-04-24T21:56:11.9943520Z .................................................................................................... 8000/9922
2020-04-24T21:56:16.7310541Z ..............................................................i..................................... 8100/9922
2020-04-24T21:56:24.0501684Z .................................................................................................... 8200/9922
2020-04-24T21:56:28.1143990Z ...........iiiiii.iiiii.i........................................................................... 8300/9922
2020-04-24T21:56:37.9075030Z .................................................................................................... 8500/9922
2020-04-24T21:56:42.3279840Z .................................................................................................... 8600/9922
2020-04-24T21:56:52.8005792Z .................................................................................................... 8700/9922
2020-04-24T21:56:57.5898087Z .................................................................................................... 8800/9922
---
2020-04-24T21:58:10.2711120Z ..........................i......................................................................... 9900/9922
2020-04-24T21:58:16.3193466Z ......................
2020-04-24T21:58:16.3193923Z failures:
2020-04-24T21:58:16.3229134Z 
2020-04-24T21:58:16.3229805Z ---- [ui] ui/treat-err-as-bug/err.rs stdout ----
2020-04-24T21:58:16.3230045Z 
2020-04-24T21:58:16.3230165Z 6 
2020-04-24T21:58:16.3230165Z 6 
2020-04-24T21:58:16.3230546Z 7 thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:930:13
2020-04-24T21:58:16.3231048Z -    0: backtrace::backtrace::libunwind::trace
2020-04-24T21:58:16.3231374Z -              at src/backtrace/libunwind.rs:86
2020-04-24T21:58:16.3231683Z -    1: backtrace::backtrace::trace_unsynchronized
2020-04-24T21:58:16.3231987Z -              at src/backtrace/mod.rs:66
---
2020-04-24T21:58:16.3235796Z -              at src/libstd/sys_common/backtrace.rs:49
2020-04-24T21:58:16.3236133Z -    8: std::panicking::default_hook::{{closure}}
2020-04-24T21:58:16.3236441Z -              at src/libstd/panicking.rs:198
2020-04-24T21:58:16.3236724Z -    9: std::panicking::default_hook
2020-04-24T21:58:16.3237031Z -              at src/libstd/panicking.rs:218
2020-04-24T21:58:16.3237382Z -   10: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
2020-04-24T21:58:16.3237717Z -              at ./src/liballoc/boxed.rs:1022
2020-04-24T21:58:16.3238008Z -   11: rustc_driver::report_ice
2020-04-24T21:58:16.3238297Z -              at src/librustc_driver/lib.rs:1157
2020-04-24T21:58:16.3238599Z -   12: std::panicking::rust_panic_with_hook
2020-04-24T21:58:16.3238911Z -              at src/libstd/panicking.rs:515
2020-04-24T21:58:16.3239193Z -   13: std::panicking::begin_panic
2020-04-24T21:58:16.3239483Z -              at ./src/libstd/panicking.rs:438
2020-04-24T21:58:16.3239824Z -   14: rustc_errors::HandlerInner::panic_if_treat_err_as_bug
2020-04-24T21:58:16.3240146Z -              at src/librustc_errors/lib.rs:930
2020-04-24T21:58:16.3240458Z -   15: rustc_errors::HandlerInner::bump_err_count
2020-04-24T21:58:16.3240782Z -              at src/librustc_errors/lib.rs:916
2020-04-24T21:58:16.3241095Z -   16: rustc_errors::HandlerInner::emit_diagnostic
2020-04-24T21:58:16.3241405Z -              at src/librustc_errors/lib.rs:761
2020-04-24T21:58:16.3241725Z -   17: rustc_errors::Handler::emit_diagnostic
2020-04-24T21:58:16.3242027Z -              at src/librustc_errors/lib.rs:695
2020-04-24T21:58:16.3242369Z -   18: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
2020-04-24T21:58:16.3242901Z -              at src/librustc_errors/diagnostic_builder.rs:100
2020-04-24T21:58:16.3243306Z -   19: rustc_middle::mir::interpret::error::ConstEvalErr::report_as_error::{{closure}}
2020-04-24T21:58:16.3243693Z -              at src/librustc_middle/mir/interpret/error.rs:83
2020-04-24T21:58:16.3244106Z -   20: rustc_middle::mir::interpret::error::ConstEvalErr::struct_generic::{{closure}}
2020-04-24T21:58:16.3244557Z -              at src/librustc_middle/mir/interpret/error.rs:165
2020-04-24T21:58:16.3244944Z -   21: rustc_middle::mir::interpret::error::ConstEvalErr::struct_generic
2020-04-24T21:58:16.3245324Z -              at src/librustc_middle/mir/interpret/error.rs:194
2020-04-24T21:58:16.3245694Z -   22: rustc_middle::mir::interpret::error::ConstEvalErr::struct_error
2020-04-24T21:58:16.3246057Z -              at src/librustc_middle/mir/interpret/error.rs:79
2020-04-24T21:58:16.3246445Z -   23: rustc_middle::mir::interpret::error::ConstEvalErr::report_as_error
2020-04-24T21:58:16.3246815Z -              at src/librustc_middle/mir/interpret/error.rs:83
2020-04-24T21:58:16.3247206Z -   24: rustc_mir::const_eval::eval_queries::const_eval_raw_provider::{{closure}}
2020-04-24T21:58:16.3247603Z -              at src/librustc_mir/const_eval/eval_queries.rs:319
2020-04-24T21:58:16.3247922Z -   25: core::result::Result<T,E>::map_err
2020-04-24T21:58:16.3248221Z -              at ./src/libcore/result.rs:612
2020-04-24T21:58:16.3248580Z -   26: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
2020-04-24T21:58:16.3248948Z -              at src/librustc_mir/const_eval/eval_queries.rs:309
2020-04-24T21:58:16.3249547Z -   27: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_raw>::compute
2020-04-24T21:58:16.3250082Z -              at ./src/librustc_middle/ty/query/plumbing.rs:362
2020-04-24T21:58:16.3250454Z -   28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
2020-04-24T21:58:16.3250830Z -              at ./src/librustc_query_system/dep_graph/graph.rs:303
2020-04-24T21:58:16.3251215Z -   29: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-24T21:58:16.3251582Z -              at ./src/librustc_query_system/dep_graph/graph.rs:200
2020-04-24T21:58:16.3251993Z -   30: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
2020-04-24T21:58:16.3252408Z -              at ./src/librustc_query_system/query/plumbing.rs:593
2020-04-24T21:58:16.3252973Z -   31: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
2020-04-24T21:58:16.3253477Z -              at ./src/librustc_middle/ty/query/plumbing.rs:71
2020-04-24T21:58:16.3253839Z -   32: rustc_middle::ty::context::tls::enter_context::{{closure}}
2020-04-24T21:58:16.3254189Z -              at ./src/librustc_middle/ty/context.rs:1698
2020-04-24T21:58:16.3254529Z -   33: rustc_middle::ty::context::tls::set_tlv
2020-04-24T21:58:16.3254857Z -              at ./src/librustc_middle/ty/context.rs:1682
2020-04-24T21:58:16.3255186Z -   34: rustc_middle::ty::context::tls::enter_context
2020-04-24T21:58:16.3255531Z -              at ./src/librustc_middle/ty/context.rs:1698
2020-04-24T21:58:16.3256060Z -   35: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
2020-04-24T21:58:16.3256535Z -              at ./src/librustc_middle/ty/query/plumbing.rs:71
2020-04-24T21:58:16.3256924Z -   36: rustc_middle::ty::context::tls::with_related_context::{{closure}}
2020-04-24T21:58:16.3257284Z -              at ./src/librustc_middle/ty/context.rs:1786
2020-04-24T21:58:16.3257636Z -   37: rustc_middle::ty::context::tls::with_context::{{closure}}
2020-04-24T21:58:16.3258000Z -              at ./src/librustc_middle/ty/context.rs:1770
2020-04-24T21:58:16.3258333Z -   38: rustc_middle::ty::context::tls::with_context_opt
2020-04-24T21:58:16.3258717Z -              at ./src/librustc_middle/ty/context.rs:1759
2020-04-24T21:58:16.3259063Z -   39: rustc_middle::ty::context::tls::with_context
2020-04-24T21:58:16.3259395Z -              at ./src/librustc_middle/ty/context.rs:1770
2020-04-24T21:58:16.3259736Z -   40: rustc_middle::ty::context::tls::with_related_context
2020-04-24T21:58:16.3260091Z -              at ./src/librustc_middle/ty/context.rs:1783
2020-04-24T21:58:16.3260628Z -   41: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
2020-04-24T21:58:16.3261094Z -              at ./src/librustc_middle/ty/query/plumbing.rs:60
2020-04-24T21:58:16.3261498Z -   42: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
2020-04-24T21:58:16.3261878Z -              at ./src/librustc_query_system/query/plumbing.rs:583
2020-04-24T21:58:16.3262230Z -   43: rustc_query_system::query::plumbing::with_diagnostics
2020-04-24T21:58:16.3262607Z -              at ./src/librustc_query_system/query/plumbing.rs:293
2020-04-24T21:58:16.3262965Z -   44: rustc_query_system::query::plumbing::force_query_with_job
2020-04-24T21:58:16.3263324Z -              at ./src/librustc_query_system/query/plumbing.rs:582
2020-04-24T21:58:16.3263693Z -   45: rustc_query_system::query::plumbing::try_execute_query
2020-04-24T21:58:16.3264049Z -              at ./src/librustc_query_system/query/plumbing.rs:410
2020-04-24T21:58:16.3264416Z -   46: rustc_query_system::query::plumbing::get_query::{{closure}}
2020-04-24T21:58:16.3264798Z -              at ./src/librustc_query_system/query/plumbing.rs:627
2020-04-24T21:58:16.3265270Z -   47: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
2020-04-24T21:58:16.3265716Z -              at ./src/librustc_query_system/query/caches.rs:91
2020-04-24T21:58:16.3266063Z -   48: rustc_query_system::query::plumbing::try_get_cached
2020-04-24T21:58:16.3266420Z -              at ./src/librustc_query_system/query/plumbing.rs:368
2020-04-24T21:58:16.3266775Z -   49: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3267125Z -              at ./src/librustc_query_system/query/plumbing.rs:619
2020-04-24T21:58:16.3267471Z -   50: rustc_middle::ty::query::TyCtxtAt::const_eval_raw
2020-04-24T21:58:16.3267832Z -              at ./src/librustc_middle/ty/query/plumbing.rs:467
2020-04-24T21:58:16.3272665Z -   51: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_raw
2020-04-24T21:58:16.3273490Z -              at ./src/librustc_middle/ty/query/plumbing.rs:430
2020-04-24T21:58:16.3274061Z -   52: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-24T21:58:16.3274822Z -              at src/librustc_mir/const_eval/eval_queries.rs:254
2020-04-24T21:58:16.3275736Z -   53: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-24T21:58:16.3276535Z -              at ./src/librustc_middle/ty/query/plumbing.rs:362
2020-04-24T21:58:16.3277075Z -   54: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
2020-04-24T21:58:16.3277615Z -              at ./src/librustc_query_system/dep_graph/graph.rs:303
2020-04-24T21:58:16.3278166Z -   55: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-24T21:58:16.3278701Z -              at ./src/librustc_query_system/dep_graph/graph.rs:200
2020-04-24T21:58:16.3279296Z -   56: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
2020-04-24T21:58:16.3279888Z -              at ./src/librustc_query_system/query/plumbing.rs:593
2020-04-24T21:58:16.3280698Z -   57: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
2020-04-24T21:58:16.3281405Z -              at ./src/librustc_middle/ty/query/plumbing.rs:71
2020-04-24T21:58:16.3282105Z -   58: rustc_middle::ty::context::tls::enter_context::{{closure}}
2020-04-24T21:58:16.3282615Z -              at ./src/librustc_middle/ty/context.rs:1698
2020-04-24T21:58:16.3283081Z -   59: rustc_middle::ty::context::tls::set_tlv
2020-04-24T21:58:16.3283668Z -              at ./src/librustc_middle/ty/context.rs:1682
2020-04-24T21:58:16.3284181Z -   60: rustc_middle::ty::context::tls::enter_context
2020-04-24T21:58:16.3284705Z -              at ./src/librustc_middle/ty/context.rs:1698
2020-04-24T21:58:16.3285257Z -   61: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
2020-04-24T21:58:16.3285731Z -              at ./src/librustc_middle/ty/query/plumbing.rs:71
2020-04-24T21:58:16.3286119Z -   62: rustc_middle::ty::context::tls::with_related_context::{{closure}}
2020-04-24T21:58:16.3286479Z -              at ./src/librustc_middle/ty/context.rs:1786
2020-04-24T21:58:16.3286836Z -   63: rustc_middle::ty::context::tls::with_context::{{closure}}
2020-04-24T21:58:16.3287199Z -              at ./src/librustc_middle/ty/context.rs:1770
2020-04-24T21:58:16.3287534Z -   64: rustc_middle::ty::context::tls::with_context_opt
2020-04-24T21:58:16.3287867Z -              at ./src/librustc_middle/ty/context.rs:1759
2020-04-24T21:58:16.3288209Z -   65: rustc_middle::ty::context::tls::with_context
2020-04-24T21:58:16.3288543Z -              at ./src/librustc_middle/ty/context.rs:1770
2020-04-24T21:58:16.3288886Z -   66: rustc_middle::ty::context::tls::with_related_context
2020-04-24T21:58:16.3289240Z -              at ./src/librustc_middle/ty/context.rs:1783
2020-04-24T21:58:16.3289740Z -   67: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
2020-04-24T21:58:16.3290198Z -              at ./src/librustc_middle/ty/query/plumbing.rs:60
2020-04-24T21:58:16.3295707Z -   68: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
2020-04-24T21:58:16.3296269Z -              at ./src/librustc_query_system/query/plumbing.rs:583
2020-04-24T21:58:16.3296630Z -   69: rustc_query_system::query::plumbing::with_diagnostics
2020-04-24T21:58:16.3297008Z -              at ./src/librustc_query_system/query/plumbing.rs:293
2020-04-24T21:58:16.3297370Z -   70: rustc_query_system::query::plumbing::force_query_with_job
2020-04-24T21:58:16.3297735Z -              at ./src/librustc_query_system/query/plumbing.rs:582
2020-04-24T21:58:16.3298109Z -   71: rustc_query_system::query::plumbing::try_execute_query
2020-04-24T21:58:16.3298466Z -              at ./src/librustc_query_system/query/plumbing.rs:410
2020-04-24T21:58:16.3298831Z -   72: rustc_query_system::query::plumbing::get_query::{{closure}}
2020-04-24T21:58:16.3309427Z -              at ./src/librustc_query_system/query/plumbing.rs:627
2020-04-24T21:58:16.3310001Z -   73: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
2020-04-24T21:58:16.3310527Z -              at ./src/librustc_query_system/query/caches.rs:91
2020-04-24T21:58:16.3310953Z -   74: rustc_query_system::query::plumbing::try_get_cached
2020-04-24T21:58:16.3311369Z -              at ./src/librustc_query_system/query/plumbing.rs:368
2020-04-24T21:58:16.3311768Z -   75: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3312304Z -              at ./src/librustc_query_system/query/plumbing.rs:619
2020-04-24T21:58:16.3312665Z -   76: rustc_middle::ty::query::TyCtxtAt::const_eval_validated
2020-04-24T21:58:16.3313020Z -              at ./src/librustc_middle/ty/query/plumbing.rs:467
2020-04-24T21:58:16.3313455Z -   77: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_validated
2020-04-24T21:58:16.3313852Z -              at ./src/librustc_middle/ty/query/plumbing.rs:430
2020-04-24T21:58:16.3314223Z -   78: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-24T21:58:16.3314607Z -              at src/librustc_mir/const_eval/eval_queries.rs:231
2020-04-24T21:58:16.3315359Z -   79: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-24T21:58:16.3315899Z -              at src/librustc_middle/ty/query/plumbing.rs:362
2020-04-24T21:58:16.3316271Z -   80: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
2020-04-24T21:58:16.3316702Z -              at ./src/librustc_query_system/dep_graph/graph.rs:303
2020-04-24T21:58:16.3317098Z -   81: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-24T21:58:16.3317467Z -              at ./src/librustc_query_system/dep_graph/graph.rs:200
2020-04-24T21:58:16.3317876Z -   82: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
2020-04-24T21:58:16.3318287Z -              at ./src/librustc_query_system/query/plumbing.rs:593
2020-04-24T21:58:16.3318849Z -   83: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
2020-04-24T21:58:16.3319341Z -              at src/librustc_middle/ty/query/plumbing.rs:71
2020-04-24T21:58:16.3319714Z -   84: rustc_middle::ty::context::tls::enter_context::{{closure}}
2020-04-24T21:58:16.3320061Z -              at src/librustc_middle/ty/context.rs:1698
2020-04-24T21:58:16.3320382Z -   85: rustc_middle::ty::context::tls::set_tlv
2020-04-24T21:58:16.3320724Z -              at src/librustc_middle/ty/context.rs:1682
2020-04-24T21:58:16.3321051Z -   86: rustc_middle::ty::context::tls::enter_context
2020-04-24T21:58:16.3321381Z -              at src/librustc_middle/ty/context.rs:1698
2020-04-24T21:58:16.3321921Z -   87: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
2020-04-24T21:58:16.3322389Z -              at src/librustc_middle/ty/query/plumbing.rs:71
2020-04-24T21:58:16.3322765Z -   88: rustc_middle::ty::context::tls::with_related_context::{{closure}}
2020-04-24T21:58:16.3323139Z -              at src/librustc_middle/ty/context.rs:1786
2020-04-24T21:58:16.3323490Z -   89: rustc_middle::ty::context::tls::with_context::{{closure}}
2020-04-24T21:58:16.3323834Z -              at src/librustc_middle/ty/context.rs:1770
2020-04-24T21:58:16.3324184Z -   90: rustc_middle::ty::context::tls::with_context_opt
2020-04-24T21:58:16.3324518Z -              at src/librustc_middle/ty/context.rs:1759
2020-04-24T21:58:16.3324843Z -   91: rustc_middle::ty::context::tls::with_context
2020-04-24T21:58:16.3325188Z -              at src/librustc_middle/ty/context.rs:1770
2020-04-24T21:58:16.3325525Z -   92: rustc_middle::ty::context::tls::with_related_context
2020-04-24T21:58:16.3325860Z -              at src/librustc_middle/ty/context.rs:1783
2020-04-24T21:58:16.3326372Z -   93: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
2020-04-24T21:58:16.3326830Z -              at src/librustc_middle/ty/query/plumbing.rs:60
2020-04-24T21:58:16.3327228Z -   94: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
2020-04-24T21:58:16.3327611Z -              at ./src/librustc_query_system/query/plumbing.rs:583
2020-04-24T21:58:16.3327962Z -   95: rustc_query_system::query::plumbing::with_diagnostics
2020-04-24T21:58:16.3328335Z -              at ./src/librustc_query_system/query/plumbing.rs:293
2020-04-24T21:58:16.3328695Z -   96: rustc_query_system::query::plumbing::force_query_with_job
2020-04-24T21:58:16.3329052Z -              at ./src/librustc_query_system/query/plumbing.rs:582
2020-04-24T21:58:16.3329417Z -   97: rustc_query_system::query::plumbing::try_execute_query
2020-04-24T21:58:16.3329772Z -              at ./src/librustc_query_system/query/plumbing.rs:410
2020-04-24T21:58:16.3330138Z -   98: rustc_query_system::query::plumbing::get_query::{{closure}}
2020-04-24T21:58:16.3330570Z -              at ./src/librustc_query_system/query/plumbing.rs:627
2020-04-24T21:58:16.3331040Z -   99: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
2020-04-24T21:58:16.3331469Z -              at ./src/librustc_query_system/query/caches.rs:91
2020-04-24T21:58:16.3331830Z -  100: rustc_query_system::query::plumbing::try_get_cached
2020-04-24T21:58:16.3332221Z -              at ./src/librustc_query_system/query/plumbing.rs:368
2020-04-24T21:58:16.3332573Z -  101: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3332936Z -              at ./src/librustc_query_system/query/plumbing.rs:619
2020-04-24T21:58:16.3333291Z -  102: rustc_middle::ty::query::TyCtxtAt::const_eval_validated
2020-04-24T21:58:16.3333639Z -              at src/librustc_middle/ty/query/plumbing.rs:467
2020-04-24T21:58:16.3334066Z -  103: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::const_eval_validated
2020-04-24T21:58:16.3334462Z -              at src/librustc_middle/ty/query/plumbing.rs:430
2020-04-24T21:58:16.3334902Z -  104: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id
2020-04-24T21:58:16.3335330Z -              at src/librustc_middle/mir/interpret/queries.rs:74
2020-04-24T21:58:16.3335764Z -  105: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_poly
2020-04-24T21:58:16.3336177Z -              at src/librustc_middle/mir/interpret/queries.rs:22
2020-04-24T21:58:16.3336613Z -  106: <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item
2020-04-24T21:58:16.3336987Z -              at src/librustc_lint/builtin.rs:0
2020-04-24T21:58:16.3337390Z -  107: <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item
2020-04-24T21:58:16.3337780Z -              at src/librustc_lint/passes.rs:113
2020-04-24T21:58:16.3338233Z -  108: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}::{{closure}}
2020-04-24T21:58:16.3338658Z -              at ./src/librustc_lint/late.rs:42
2020-04-24T21:58:16.3338997Z -  109: rustc_lint::late::LateContextAndPass<T>::with_param_env
2020-04-24T21:58:16.3339323Z -              at ./src/librustc_lint/late.rs:73
2020-04-24T21:58:16.3339761Z -  110: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}
2020-04-24T21:58:16.3340155Z -              at ./src/librustc_lint/late.rs:131
2020-04-24T21:58:16.3340495Z -  111: rustc_lint::late::LateContextAndPass<T>::with_lint_attrs
2020-04-24T21:58:16.3340837Z -              at ./src/librustc_lint/late.rs:61
2020-04-24T21:58:16.3341240Z -  112: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item
2020-04-24T21:58:16.3341612Z -              at ./src/librustc_lint/late.rs:130
2020-04-24T21:58:16.3341952Z -  113: rustc_hir::intravisit::Visitor::visit_nested_item
2020-04-24T21:58:16.3342308Z -              at ./<::rustc_ast::visit::walk_list macros>:2
2020-04-24T21:58:16.3342623Z -  114: rustc_hir::intravisit::walk_mod
2020-04-24T21:58:16.3343124Z -              at ./src/librustc_hir/intravisit.rs:479
2020-04-24T21:58:16.3343469Z -  115: rustc_lint::late::LateContextAndPass<T>::process_mod
2020-04-24T21:58:16.3343901Z -              at ./src/librustc_lint/late.rs:79
2020-04-24T21:58:16.3344324Z -  116: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_mod
2020-04-24T21:58:16.3344700Z -              at ./src/librustc_lint/late.rs:236
2020-04-24T21:58:16.3344997Z -  117: rustc_hir::intravisit::walk_crate
2020-04-24T21:58:16.3345322Z -              at ./src/librustc_hir/intravisit.rs:465
2020-04-24T21:58:16.3345764Z -  118: rustc_lint::late::late_lint_pass_crate::{{closure}}
2020-04-24T21:58:16.3346092Z -              at ./src/librustc_lint/late.rs:427
2020-04-24T21:58:16.3346449Z -  119: rustc_lint::late::LateContextAndPass<T>::with_lint_attrs
2020-04-24T21:58:16.3346834Z -              at ./src/librustc_lint/late.rs:61
2020-04-24T21:58:16.3347138Z -  120: rustc_lint::late::late_lint_pass_crate
2020-04-24T21:58:16.3347458Z -              at ./src/librustc_lint/late.rs:422
2020-04-24T21:58:16.3347756Z -  121: rustc_lint::late::late_lint_crate
2020-04-24T21:58:16.3348056Z -              at ./src/librustc_lint/late.rs:441
2020-04-24T21:58:16.3350301Z -  122: rustc_lint::late::check_crate::{{closure}}::{{closure}}
2020-04-24T21:58:16.3350829Z -              at ./src/librustc_lint/late.rs:471
2020-04-24T21:58:16.3351250Z -  123: rustc_data_structures::profiling::VerboseTimingGuard::run
2020-04-24T21:58:16.3351673Z -              at ./src/librustc_data_structures/profiling.rs:573
2020-04-24T21:58:16.3352363Z -  124: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-24T21:58:16.3352771Z -              at ./src/librustc_session/utils.rs:9
2020-04-24T21:58:16.3353157Z -  125: rustc_lint::late::check_crate::{{closure}}
2020-04-24T21:58:16.3353521Z -              at ./src/librustc_lint/late.rs:469
2020-04-24T21:58:16.3353876Z -  126: rustc_data_structures::sync::join
2020-04-24T21:58:16.3354267Z -              at ./src/librustc_data_structures/sync.rs:160
2020-04-24T21:58:16.3354620Z -  127: rustc_lint::late::check_crate
2020-04-24T21:58:16.3354966Z -              at ./src/librustc_lint/late.rs:467
2020-04-24T21:58:16.3355425Z -  128: rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}::{{closure}}
2020-04-24T21:58:16.3355885Z -              at src/librustc_interface/passes.rs:880
2020-04-24T21:58:16.3356292Z -  129: rustc_data_structures::profiling::VerboseTimingGuard::run
2020-04-24T21:58:16.3356727Z -              at ./src/librustc_data_structures/profiling.rs:573
2020-04-24T21:58:16.3357170Z -  130: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-24T21:58:16.3357747Z -              at ./src/librustc_session/utils.rs:9
2020-04-24T21:58:16.3358269Z -  131: rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}
2020-04-24T21:58:16.3358760Z -              at src/librustc_interface/passes.rs:879
2020-04-24T21:58:16.3359180Z -  132: core::ops::function::FnOnce::call_once
2020-04-24T21:58:16.3359616Z -              at ./src/libcore/ops/function.rs:232
2020-04-24T21:58:16.3360138Z -  133: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
2020-04-24T21:58:16.3360614Z -              at ./src/libstd/panic.rs:318
2020-04-24T21:58:16.3361014Z -  134: std::panicking::try::do_call
2020-04-24T21:58:16.3361410Z -              at ./src/libstd/panicking.rs:331
2020-04-24T21:58:16.3361790Z -  135: std::panicking::try::do_try
2020-04-24T21:58:16.3362176Z -              at src/libstd/panicking.rs:298
2020-04-24T21:58:16.3362551Z -  136: std::panicking::try
2020-04-24T21:58:16.3362928Z -              at ./src/libstd/panicking.rs:274
2020-04-24T21:58:16.3363382Z -  137: std::panic::catch_unwind
2020-04-24T21:58:16.3363723Z -              at ./src/libstd/panic.rs:394
2020-04-24T21:58:16.3364124Z -  138: rustc_interface::passes::analysis::{{closure}}::{{closure}}
2020-04-24T21:58:16.3364524Z -              at src/librustc_interface/passes.rs:866
2020-04-24T21:58:16.3364906Z -  139: core::ops::function::FnOnce::call_once
2020-04-24T21:58:16.3365271Z -              at ./src/libcore/ops/function.rs:232
2020-04-24T21:58:16.3365725Z -  140: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
2020-04-24T21:58:16.3372915Z -              at ./src/libstd/panic.rs:318
2020-04-24T21:58:16.3373234Z -  141: std::panicking::try::do_call
2020-04-24T21:58:16.3373538Z -              at ./src/libstd/panicking.rs:331
2020-04-24T21:58:16.3374070Z -  142: std::panicking::try::do_try
2020-04-24T21:58:16.3374413Z -              at src/libstd/panicking.rs:298
2020-04-24T21:58:16.3374725Z -  143: std::panicking::try
2020-04-24T21:58:16.3375074Z -              at ./src/libstd/panicking.rs:274
2020-04-24T21:58:16.3375397Z -  144: std::panic::catch_unwind
2020-04-24T21:58:16.3375721Z -              at ./src/libstd/panic.rs:394
2020-04-24T21:58:16.3376495Z -  145: rustc_interface::passes::analysis::{{closure}}
2020-04-24T21:58:16.3376891Z -              at src/librustc_interface/passes.rs:862
2020-04-24T21:58:16.3377624Z -  146: rustc_data_structures::profiling::VerboseTimingGuard::run
2020-04-24T21:58:16.3378528Z -              at ./src/librustc_data_structures/profiling.rs:573
2020-04-24T21:58:16.3379210Z -  147: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-24T21:58:16.3379633Z -              at ./src/librustc_session/utils.rs:9
2020-04-24T21:58:16.3380001Z -  148: rustc_interface::passes::analysis
2020-04-24T21:58:16.3380748Z -              at src/librustc_interface/passes.rs:861
2020-04-24T21:58:16.3381421Z -  149: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
2020-04-24T21:58:16.3382035Z -              at ./src/librustc_middle/ty/query/plumbing.rs:362
2020-04-24T21:58:16.3382641Z -  150: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
2020-04-24T21:58:16.3383144Z -              at ./src/librustc_query_system/dep_graph/graph.rs:303
2020-04-24T21:58:16.3383934Z -  151: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
2020-04-24T21:58:16.3384453Z -              at ./src/librustc_query_system/dep_graph/graph.rs:336
2020-04-24T21:58:16.3385006Z -  152: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
2020-04-24T21:58:16.3385554Z -              at ./src/librustc_query_system/query/plumbing.rs:585
2020-04-24T21:58:16.3386300Z -  153: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
2020-04-24T21:58:16.3387078Z -              at ./src/librustc_middle/ty/query/plumbing.rs:71
2020-04-24T21:58:16.3387670Z -  154: rustc_middle::ty::context::tls::enter_context::{{closure}}
2020-04-24T21:58:16.3388154Z -              at ./src/librustc_middle/ty/context.rs:1698
2020-04-24T21:58:16.3388862Z -  155: rustc_middle::ty::context::tls::set_tlv
2020-04-24T21:58:16.3389302Z -              at ./src/librustc_middle/ty/context.rs:1682
2020-04-24T21:58:16.3389740Z -  156: rustc_middle::ty::context::tls::enter_context
2020-04-24T21:58:16.3390200Z -              at ./src/librustc_middle/ty/context.rs:1698
2020-04-24T21:58:16.3390904Z -  157: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
2020-04-24T21:58:16.3391535Z -              at ./src/librustc_middle/ty/query/plumbing.rs:71
2020-04-24T21:58:16.3392048Z -  158: rustc_middle::ty::context::tls::with_related_context::{{closure}}
2020-04-24T21:58:16.3392528Z -              at ./src/librustc_middle/ty/context.rs:1786
2020-04-24T21:58:16.3392996Z -  159: rustc_middle::ty::context::tls::with_context::{{closure}}
2020-04-24T21:58:16.3393476Z -              at ./src/librustc_middle/ty/context.rs:1770
2020-04-24T21:58:16.3393930Z -  160: rustc_middle::ty::context::tls::with_context_opt
2020-04-24T21:58:16.3394375Z -              at ./src/librustc_middle/ty/context.rs:1759
2020-04-24T21:58:16.3395057Z -  161: rustc_middle::ty::context::tls::with_context
2020-04-24T21:58:16.3395443Z -              at ./src/librustc_middle/ty/context.rs:1770
2020-04-24T21:58:16.3395840Z -  162: rustc_middle::ty::context::tls::with_related_context
2020-04-24T21:58:16.3396254Z -              at ./src/librustc_middle/ty/context.rs:1783
2020-04-24T21:58:16.3396934Z -  163: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
2020-04-24T21:58:16.3397477Z -              at ./src/librustc_middle/ty/query/plumbing.rs:60
2020-04-24T21:58:16.3397941Z -  164: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
2020-04-24T21:58:16.3398383Z -              at ./src/librustc_query_system/query/plumbing.rs:583
2020-04-24T21:58:16.3399016Z -  165: rustc_query_system::query::plumbing::with_diagnostics
2020-04-24T21:58:16.3399390Z -              at ./src/librustc_query_system/query/plumbing.rs:293
2020-04-24T21:58:16.3399922Z -  166: rustc_query_system::query::plumbing::force_query_with_job
2020-04-24T21:58:16.3400338Z -              at ./src/librustc_query_system/query/plumbing.rs:582
2020-04-24T21:58:16.3400940Z -  167: rustc_query_system::query::plumbing::try_execute_query
2020-04-24T21:58:16.3401369Z -              at ./src/librustc_query_system/query/plumbing.rs:410
2020-04-24T21:58:16.3401791Z -  168: rustc_query_system::query::plumbing::get_query::{{closure}}
2020-04-24T21:58:16.3402232Z -              at ./src/librustc_query_system/query/plumbing.rs:627
2020-04-24T21:58:16.3402778Z -  169: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
2020-04-24T21:58:16.3403391Z -              at ./src/librustc_query_system/query/caches.rs:91
2020-04-24T21:58:16.3403818Z -  170: rustc_query_system::query::plumbing::try_get_cached
2020-04-24T21:58:16.3404228Z -              at ./src/librustc_query_system/query/plumbing.rs:368
2020-04-24T21:58:16.3404622Z -  171: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3405141Z -              at ./src/librustc_query_system/query/plumbing.rs:619
2020-04-24T21:58:16.3405547Z -  172: rustc_middle::ty::query::TyCtxtAt::analysis
2020-04-24T21:58:16.3405946Z -              at ./src/librustc_middle/ty/query/plumbing.rs:467
2020-04-24T21:58:16.3406426Z -  173: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::analysis
2020-04-24T21:58:16.3406872Z -              at ./src/librustc_middle/ty/query/plumbing.rs:430
2020-04-24T21:58:16.3407531Z -  174: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
2020-04-24T21:58:16.3408113Z -              at src/librustc_driver/lib.rs:383
2020-04-24T21:58:16.3408568Z -  175: rustc_middle::ty::context::tls::enter_global::{{closure}}
2020-04-24T21:58:16.3409053Z -              at ./src/librustc_middle/ty/context.rs:1721
2020-04-24T21:58:16.3409530Z -  176: rustc_middle::ty::context::tls::enter_context::{{closure}}
2020-04-24T21:58:16.3409995Z -              at ./src/librustc_middle/ty/context.rs:1698
2020-04-24T21:58:16.3410433Z -  177: rustc_middle::ty::context::tls::set_tlv
2020-04-24T21:58:16.3410867Z -              at ./src/librustc_middle/ty/context.rs:1682
2020-04-24T21:58:16.3411307Z -  178: rustc_middle::ty::context::tls::enter_context
2020-04-24T21:58:16.3411996Z -              at ./src/librustc_middle/ty/context.rs:1698
2020-04-24T21:58:16.3412379Z -  179: rustc_middle::ty::context::tls::enter_global
2020-04-24T21:58:16.3412762Z -              at ./src/librustc_middle/ty/context.rs:1721
2020-04-24T21:58:16.3413157Z -  180: rustc_interface::passes::QueryContext::enter
2020-04-24T21:58:16.3413538Z -              at ./src/librustc_interface/passes.rs:709
2020-04-24T21:58:16.3413933Z -  181: rustc_driver::run_compiler::{{closure}}::{{closure}}
2020-04-24T21:58:16.3414331Z -              at src/librustc_driver/lib.rs:383
2020-04-24T21:58:16.3414781Z -  182: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
2020-04-24T21:58:16.3415335Z -              at ./src/librustc_interface/queries.rs:385
2020-04-24T21:58:16.3415718Z -  183: rustc_driver::run_compiler::{{closure}}
2020-04-24T21:58:16.3416074Z -              at src/librustc_driver/lib.rs:283
2020-04-24T21:58:16.3416478Z -  184: rustc_interface::interface::run_compiler_in_existing_thread_pool
2020-04-24T21:58:16.3416901Z -              at ./src/librustc_interface/interface.rs:199
2020-04-24T21:58:16.3417304Z -  185: rustc_interface::interface::run_compiler::{{closure}}
2020-04-24T21:58:16.3417702Z -              at ./src/librustc_interface/interface.rs:213
2020-04-24T21:58:16.3418180Z -  186: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
2020-04-24T21:58:16.3418611Z -              at ./src/librustc_interface/util.rs:152
2020-04-24T21:58:16.3419139Z -  187: scoped_tls::ScopedKey<T>::set
2020-04-24T21:58:16.3419468Z -              at src/lib.rs:137
2020-04-24T21:58:16.3419874Z -  188: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
2020-04-24T21:58:16.3420286Z -              at ./src/librustc_interface/util.rs:148
2020-04-24T21:58:16.3420645Z -  189: scoped_tls::ScopedKey<T>::set
2020-04-24T21:58:16.3420961Z -              at src/lib.rs:137
2020-04-24T21:58:16.3421355Z -  190: rustc_ast::attr::with_globals::{{closure}}
2020-04-24T21:58:16.3421853Z -              at ./src/librustc_ast/attr/mod.rs:44
2020-04-24T21:58:16.3422210Z -  191: scoped_tls::ScopedKey<T>::set
2020-04-24T21:58:16.3422524Z -              at src/lib.rs:137
2020-04-24T21:58:16.3422837Z -  192: rustc_ast::attr::with_globals
2020-04-24T21:58:16.3423202Z -              at ./src/librustc_ast/attr/mod.rs:44
2020-04-24T21:58:16.3423593Z -  193: rustc_interface::util::spawn_thread_pool::{{closure}}
2020-04-24T21:58:16.3423982Z -              at ./src/librustc_interface/util.rs:147
2020-04-24T21:58:16.3424391Z -  194: rustc_interface::util::scoped_thread::{{closure}}
2020-04-24T21:58:16.3424774Z -              at ./src/librustc_interface/util.rs:122
2020-04-24T21:58:16.3425356Z +    1: core::fmt::write
2020-04-24T21:58:16.3425533Z +    2: std::io::Write::write_fmt
2020-04-24T21:58:16.3425747Z +    3: std::panicking::default_hook::{{closure}}
2020-04-24T21:58:16.3425978Z +    4: std::panicking::default_hook
2020-04-24T21:58:16.3425978Z +    4: std::panicking::default_hook
2020-04-24T21:58:16.3426479Z +    5: rustc_driver::report_ice
2020-04-24T21:58:16.3427375Z +    6: std::panicking::rust_panic_with_hook
2020-04-24T21:58:16.3427644Z +    7: std::panicking::begin_panic
2020-04-24T21:58:16.3428169Z +    8: rustc_errors::HandlerInner::emit_diagnostic
2020-04-24T21:58:16.3429013Z +    9: rustc_errors::Handler::emit_diagnostic
2020-04-24T21:58:16.3429347Z +   10: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
2020-04-24T21:58:16.3429891Z +   11: rustc_middle::mir::interpret::error::ConstEvalErr::struct_generic::{{closure}}
2020-04-24T21:58:16.3430302Z +   12: rustc_middle::mir::interpret::error::ConstEvalErr::report_as_error
2020-04-24T21:58:16.3430685Z +   13: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
2020-04-24T21:58:16.3431273Z +   14: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_raw>::compute
2020-04-24T21:58:16.3431979Z +   15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-24T21:58:16.3432337Z +   16: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3432677Z +   17: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-24T21:58:16.3433284Z +   18: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-24T21:58:16.3434025Z +   19: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-24T21:58:16.3434368Z +   20: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3434724Z +   21: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-24T21:58:16.3435332Z +   22: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-24T21:58:16.3435934Z +   23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-24T21:58:16.3436398Z +   24: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3436804Z +   25: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_poly
2020-04-24T21:58:16.3437288Z +   26: <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item
2020-04-24T21:58:16.3437771Z +   27: <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item
2020-04-24T21:58:16.3438379Z +   28: rustc_hir::intravisit::Visitor::visit_nested_item
2020-04-24T21:58:16.3438670Z +   29: rustc_hir::intravisit::walk_mod
2020-04-24T21:58:16.3439048Z +   30: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_mod
2020-04-24T21:58:16.3439410Z +   31: rustc_hir::intravisit::walk_crate
2020-04-24T21:58:16.3439901Z +   32: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-24T21:58:16.3440229Z +   33: rustc_data_structures::sync::join
2020-04-24T21:58:16.3440558Z +   34: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
2020-04-24T21:58:16.3440861Z +   35: std::panicking::try
2020-04-24T21:58:16.3441293Z +   36: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
2020-04-24T21:58:16.3441558Z +   37: std::panicking::try
2020-04-24T21:58:16.3441804Z +   38: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-24T21:58:16.3442268Z +   39: rustc_interface::passes::analysis
2020-04-24T21:58:16.3442884Z +   40: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
2020-04-24T21:58:16.3443512Z +   41: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
2020-04-24T21:58:16.3443802Z +   42: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3444050Z +   43: rustc_middle::ty::context::tls::enter_global
2020-04-24T21:58:16.3444331Z +   44: rustc_interface::interface::run_compiler_in_existing_thread_pool
2020-04-24T21:58:16.3444576Z +   45: scoped_tls::ScopedKey<T>::set
2020-04-24T21:58:16.3444883Z +   46: rustc_ast::attr::with_globals
2020-04-24T21:58:16.3445358Z 400 
2020-04-24T21:58:16.3445526Z 401 error: internal compiler error: unexpected panic
2020-04-24T21:58:16.3445679Z 
2020-04-24T21:58:16.3445790Z 404 
2020-04-24T21:58:16.3445790Z 404 
2020-04-24T21:58:16.3446441Z 405 note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-24T21:58:16.3447117Z - note: rustc 1.44.0-dev running on x86_64-unknown-linux-gnu
2020-04-24T21:58:16.3447561Z + note: rustc 1.44.0-nightly (2a82c0c9e 2020-04-24) running on x86_64-unknown-linux-gnu
2020-04-24T21:58:16.3447764Z 408 
2020-04-24T21:58:16.3447764Z 408 
2020-04-24T21:58:16.3448298Z 409 note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z treat-err-as-bug -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-24T21:58:16.3448671Z 
2020-04-24T21:58:16.3448857Z 
2020-04-24T21:58:16.3449012Z The actual stderr differed from the expected stderr.
2020-04-24T21:58:16.3449012Z The actual stderr differed from the expected stderr.
2020-04-24T21:58:16.3449484Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/err.stderr
2020-04-24T21:58:16.3449946Z To update references, rerun the tests and pass the `--bless` flag
2020-04-24T21:58:16.3450375Z To only update this specific test, also pass `--test-args treat-err-as-bug/err.rs`
2020-04-24T21:58:16.3450685Z error: 1 errors occurred comparing output.
2020-04-24T21:58:16.3450880Z status: exit code: 101
2020-04-24T21:58:16.3450880Z status: exit code: 101
2020-04-24T21:58:16.3452414Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/auxiliary"
2020-04-24T21:58:16.3453653Z ------------------------------------------
2020-04-24T21:58:16.3453780Z 
2020-04-24T21:58:16.3454070Z ------------------------------------------
2020-04-24T21:58:16.3454221Z stderr:
2020-04-24T21:58:16.3454221Z stderr:
2020-04-24T21:58:16.3454500Z ------------------------------------------
2020-04-24T21:58:16.3454722Z error[E0080]: could not evaluate static initializer
2020-04-24T21:58:16.3455211Z   --> /checkout/src/test/ui/treat-err-as-bug/err.rs:11:21
2020-04-24T21:58:16.3455426Z    |
2020-04-24T21:58:16.3455728Z LL | pub static C: u32 = 0 - 1;
2020-04-24T21:58:16.3456091Z 
2020-04-24T21:58:16.3456091Z 
2020-04-24T21:58:16.3456506Z thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:930:13
2020-04-24T21:58:16.3457101Z    0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
2020-04-24T21:58:16.3457379Z    1: core::fmt::write
2020-04-24T21:58:16.3457555Z    2: std::io::Write::write_fmt
2020-04-24T21:58:16.3457761Z    3: std::panicking::default_hook::{{closure}}
2020-04-24T21:58:16.3457761Z    3: std::panicking::default_hook::{{closure}}
2020-04-24T21:58:16.3457986Z    4: std::panicking::default_hook
2020-04-24T21:58:16.3458169Z    5: rustc_driver::report_ice
2020-04-24T21:58:16.3458359Z    6: std::panicking::rust_panic_with_hook
2020-04-24T21:58:16.3458555Z    7: std::panicking::begin_panic
2020-04-24T21:58:16.3458784Z    8: rustc_errors::HandlerInner::emit_diagnostic
2020-04-24T21:58:16.3459009Z    9: rustc_errors::Handler::emit_diagnostic
2020-04-24T21:58:16.3459258Z   10: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
2020-04-24T21:58:16.3459587Z   11: rustc_middle::mir::interpret::error::ConstEvalErr::struct_generic::{{closure}}
2020-04-24T21:58:16.3459913Z   12: rustc_middle::mir::interpret::error::ConstEvalErr::report_as_error
2020-04-24T21:58:16.3460203Z   13: rustc_mir::const_eval::eval_queries::const_eval_raw_provider
2020-04-24T21:58:16.3460689Z   14: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_raw>::compute
2020-04-24T21:58:16.3461274Z   15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-24T21:58:16.3461562Z   16: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3461832Z   17: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-24T21:58:16.3462319Z   18: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-24T21:58:16.3462816Z   19: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-24T21:58:16.3463178Z   20: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3463447Z   21: rustc_mir::const_eval::eval_queries::const_eval_validated_provider
2020-04-24T21:58:16.3464022Z   22: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::const_eval_validated>::compute
2020-04-24T21:58:16.3464522Z   23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
2020-04-24T21:58:16.3464787Z   24: rustc_query_system::query::plumbing::get_query
2020-04-24T21:58:16.3465109Z   25: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_poly
2020-04-24T21:58:16.3465513Z   26: <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item
2020-04-24T21:58:16.3465989Z   27: <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item
2020-04-24T21:58:16.3466306Z   28: rustc_hir::intravisit::Visitor::visit_nested_item
2020-04-24T21:58:16.3466545Z   29: rustc_hir::intravisit::walk_mod
2020-04-24T21:58:16.3466830Z   30: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_mod
2020-04-24T21:58:16.3467118Z   31: rustc_hir::intravisit::walk_crate
2020-04-24T21:58:16.3467444Z   32: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-24T21:58:16.3467703Z   33: rustc_data_structures::sync::join
2020-04-24T21:58:16.3467985Z   34: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
2020-04-24T21:58:16.3468392Z   35: std::panicking::try
2020-04-24T21:58:16.3468654Z   36: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
2020-04-24T21:58:16.3469335Z   38: rustc_session::utils::<impl rustc_session::session::Session>::time
2020-04-24T21:58:16.3469592Z   39: rustc_interface::passes::analysis
2020-04-24T21:58:16.3469592Z   39: rustc_interface::passes::analysis
2020-04-24T21:58:16.3470013Z   40: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
2020-04-24T21:58:16.3470506Z   41: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
