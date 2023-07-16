
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/compiler/rustc_errors/src/lib.rs:1335:9
stack backtrace:
   0:     0x7f1c1ec9e09d - std::backtrace_rs::backtrace::libunwind::trace::h2a8892c156585d66
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f1c1ec9e09d - std::backtrace_rs::backtrace::trace_unsynchronized::h918c401bc86eed40
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f1c1ec9e09d - std::sys_common::backtrace::_print_fmt::h2a52b4cb52dc84fb
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f1c1ec9e09d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2f5ce3ea50e5b1d6
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f1c1ecf9e2c - core::fmt::write::h48a5b18f44037270
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/core/src/fmt/mod.rs:1196:17
   5:     0x7f1c1ec8f791 - std::io::Write::write_fmt::h1d1b08935a2fcf4b
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/io/mod.rs:1654:15
   6:     0x7f1c1eca0d75 - std::sys_common::backtrace::_print::h46d4f4f2d0dbcce0
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f1c1eca0d75 - std::sys_common::backtrace::print::h8334e6fa27a1dde4
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f1c1eca0d75 - std::panicking::default_hook::{{closure}}::hc13c7ff820b0fe22
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/panicking.rs:295:22
   9:     0x7f1c1eca0a96 - std::panicking::default_hook::h100ba67c99329739
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/panicking.rs:314:9
  10:     0x7f1c1f4f96c1 - rustc_driver[30390a19e428bc19]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f1c064970c3 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hcd4ded0cbdafbf79
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/alloc/src/boxed.rs:1965:9
  12:     0x7f1c064bee5d - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h79d97323e75efcc9
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/proc_macro/src/bridge/client.rs:339:21
  13:     0x7f1c1eca144a - std::panicking::rust_panic_with_hook::h0a6a04da5fd1d45f
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/panicking.rs:702:17
  14:     0x7f1c20593d71 - std[41be3fa6aba1786e]::panicking::begin_panic::<rustc_errors[e8101b8b045aac39]::ExplicitBug>::{closure#0}
  15:     0x7f1c20593786 - std[41be3fa6aba1786e]::sys_common::backtrace::__rust_end_short_backtrace::<std[41be3fa6aba1786e]::panicking::begin_panic<rustc_errors[e8101b8b045aac39]::ExplicitBug>::{closure#0}, !>
  16:     0x7f1c20510036 - std[41be3fa6aba1786e]::panicking::begin_panic::<rustc_errors[e8101b8b045aac39]::ExplicitBug>
  17:     0x7f1c205364a6 - std[41be3fa6aba1786e]::panic::panic_any::<rustc_errors[e8101b8b045aac39]::ExplicitBug>
  18:     0x7f1c20535f45 - <rustc_errors[e8101b8b045aac39]::HandlerInner>::bug::<&alloc[40ba6c0f2dbdab1e]::string::String>
  19:     0x7f1c205342a0 - <rustc_errors[e8101b8b045aac39]::Handler>::bug::<&alloc[40ba6c0f2dbdab1e]::string::String>
  20:     0x7f1c20590d4d - rustc_middle[70296c36ca3d2602]::ty::context::tls::with_opt::<rustc_middle[70296c36ca3d2602]::util::bug::opt_span_bug_fmt<rustc_span[db009ef5852f774a]::span_encoding::Span>::{closure#0}, ()>
  21:     0x7f1c20590e46 - rustc_middle[70296c36ca3d2602]::util::bug::opt_span_bug_fmt::<rustc_span[db009ef5852f774a]::span_encoding::Span>
  22:     0x7f1c20590dc3 - rustc_middle[70296c36ca3d2602]::util::bug::bug_fmt
  23:     0x7f1c2046715b - <rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor>::path_inferred_subst_iter
  24:     0x7f1c204676e3 - <rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[a412caeb98c40378]::intravisit::Visitor>::visit_expr
  25:     0x7f1c2046756c - <rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[a412caeb98c40378]::intravisit::Visitor>::visit_expr
  26:     0x7f1c2047dc0c - rustc_hir[a412caeb98c40378]::intravisit::walk_local::<rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor>
  27:     0x7f1c204671e8 - <rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[a412caeb98c40378]::intravisit::Visitor>::visit_local
  28:     0x7f1c2047da57 - rustc_hir[a412caeb98c40378]::intravisit::walk_block::<rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor>
  29:     0x7f1c2046757e - <rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[a412caeb98c40378]::intravisit::Visitor>::visit_expr
  30:     0x7f1c20481003 - rustc_hir[a412caeb98c40378]::intravisit::walk_expr::<rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor>
  31:     0x7f1c2046757e - <rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[a412caeb98c40378]::intravisit::Visitor>::visit_expr
  32:     0x7f1c2046757e - <rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[a412caeb98c40378]::intravisit::Visitor>::visit_expr
  33:     0x7f1c20481003 - rustc_hir[a412caeb98c40378]::intravisit::walk_expr::<rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor>
  34:     0x7f1c2046757e - <rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[a412caeb98c40378]::intravisit::Visitor>::visit_expr
  35:     0x7f1c2046757e - <rustc_infer[9428db4faa363431]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[a412caeb98c40378]::intravisit::Visitor>::visit_expr
  36:     0x7f1c20426284 - <rustc_infer[9428db4faa363431]::infer::InferCtxt>::emit_inference_failure_err
  37:     0x7f1c203790db - <rustc_infer[9428db4faa363431]::infer::InferCtxt as rustc_trait_selection[d0fea7d99d927b2c]::traits::error_reporting::InferCtxtPrivExt>::maybe_report_ambiguity
  38:     0x7f1c2036cd87 - <rustc_infer[9428db4faa363431]::infer::InferCtxt as rustc_trait_selection[d0fea7d99d927b2c]::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
  39:     0x7f1c20abb094 - <rustc_infer[9428db4faa363431]::infer::InferCtxtBuilder>::enter::<&rustc_middle[70296c36ca3d2602]::ty::context::TypeckResults, <rustc_typeck[a3bfe43d381eec93]::check::inherited::InheritedBuilder>::enter<rustc_typeck[a3bfe43d381eec93]::check::typeck_with_fallback<rustc_typeck[a3bfe43d381eec93]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[70296c36ca3d2602]::ty::context::TypeckResults>::{closure#0}>
  40:     0x7f1c20a4d2fa - rustc_typeck[a3bfe43d381eec93]::check::typeck
  41:     0x7f1c21b52ad0 - <rustc_query_system[c8ac68b744dd57fa]::dep_graph::graph::DepGraph<rustc_middle[70296c36ca3d2602]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[70296c36ca3d2602]::ty::context::TyCtxt, rustc_span[db009ef5852f774a]::def_id::LocalDefId, &rustc_middle[70296c36ca3d2602]::ty::context::TypeckResults>
  42:     0x7f1c21072b70 - rustc_query_system[c8ac68b744dd57fa]::query::plumbing::try_execute_query::<rustc_query_impl[51e1f4158b96ee22]::plumbing::QueryCtxt, rustc_query_system[c8ac68b744dd57fa]::query::caches::DefaultCache<rustc_span[db009ef5852f774a]::def_id::LocalDefId, &rustc_middle[70296c36ca3d2602]::ty::context::TypeckResults>>
  43:     0x7f1c20f9f03e - <rustc_query_impl[51e1f4158b96ee22]::Queries as rustc_middle[70296c36ca3d2602]::ty::query::QueryEngine>::typeck
  44:     0x7f1c20b0bf98 - <rustc_middle[70296c36ca3d2602]::hir::map::Map>::par_body_owners::<rustc_typeck[a3bfe43d381eec93]::check::typeck_item_bodies::{closure#0}>
  45:     0x7f1c2191174c - rustc_typeck[a3bfe43d381eec93]::check::typeck_item_bodies
  46:     0x7f1c21b721a3 - <rustc_query_system[c8ac68b744dd57fa]::dep_graph::graph::DepGraph<rustc_middle[70296c36ca3d2602]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[70296c36ca3d2602]::ty::context::TyCtxt, (), ()>
  47:     0x7f1c21c1d305 - rustc_query_system[c8ac68b744dd57fa]::query::plumbing::try_execute_query::<rustc_query_impl[51e1f4158b96ee22]::plumbing::QueryCtxt, rustc_query_system[c8ac68b744dd57fa]::query::caches::DefaultCache<(), ()>>
  48:     0x7f1c21c47841 - rustc_query_system[c8ac68b744dd57fa]::query::plumbing::get_query::<rustc_query_impl[51e1f4158b96ee22]::queries::typeck_item_bodies, rustc_query_impl[51e1f4158b96ee22]::plumbing::QueryCtxt>
  49:     0x7f1c21948133 - <rustc_session[f622d47d49ca7e5c]::session::Session>::time::<(), rustc_typeck[a3bfe43d381eec93]::check_crate::{closure#7}>
  50:     0x7f1c21934d5b - rustc_typeck[a3bfe43d381eec93]::check_crate
  51:     0x7f1c216f21c7 - rustc_interface[30bf44ec34e2f97]::passes::analysis
  52:     0x7f1c21b6d575 - <rustc_query_system[c8ac68b744dd57fa]::dep_graph::graph::DepGraph<rustc_middle[70296c36ca3d2602]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[70296c36ca3d2602]::ty::context::TyCtxt, (), core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>>
  53:     0x7f1c21c1276d - rustc_query_system[c8ac68b744dd57fa]::query::plumbing::try_execute_query::<rustc_query_impl[51e1f4158b96ee22]::plumbing::QueryCtxt, rustc_query_system[c8ac68b744dd57fa]::query::caches::DefaultCache<(), core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>>>
  54:     0x7f1c21c59dfe - rustc_query_system[c8ac68b744dd57fa]::query::plumbing::get_query::<rustc_query_impl[51e1f4158b96ee22]::queries::analysis, rustc_query_impl[51e1f4158b96ee22]::plumbing::QueryCtxt>
  55:     0x7f1c216af547 - <rustc_interface[30bf44ec34e2f97]::passes::QueryContext>::enter::<rustc_driver[30390a19e428bc19]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>>
  56:     0x7f1c2169970f - <rustc_interface[30bf44ec34e2f97]::interface::Compiler>::enter::<rustc_driver[30390a19e428bc19]::run_compiler::{closure#1}::{closure#2}, core[42e67a1b4ba6212c]::result::Result<core[42e67a1b4ba6212c]::option::Option<rustc_interface[30bf44ec34e2f97]::queries::Linker>, rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>>
  57:     0x7f1c216c2d5f - rustc_span[db009ef5852f774a]::with_source_map::<core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>, rustc_interface[30bf44ec34e2f97]::interface::create_compiler_and_run<core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>, rustc_driver[30390a19e428bc19]::run_compiler::{closure#1}>::{closure#1}>
  58:     0x7f1c2169a5a2 - <scoped_tls[2965e6335b4e6f3e]::ScopedKey<rustc_span[db009ef5852f774a]::SessionGlobals>>::set::<rustc_interface[30bf44ec34e2f97]::interface::run_compiler<core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>, rustc_driver[30390a19e428bc19]::run_compiler::{closure#1}>::{closure#0}, core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>>
  59:     0x7f1c216afbcf - std[41be3fa6aba1786e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[30bf44ec34e2f97]::util::run_in_thread_pool_with_globals<rustc_interface[30bf44ec34e2f97]::interface::run_compiler<core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>, rustc_driver[30390a19e428bc19]::run_compiler::{closure#1}>::{closure#0}, core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>>::{closure#0}, core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>>
  60:     0x7f1c216afd39 - <<std[41be3fa6aba1786e]::thread::Builder>::spawn_unchecked_<rustc_interface[30bf44ec34e2f97]::util::run_in_thread_pool_with_globals<rustc_interface[30bf44ec34e2f97]::interface::run_compiler<core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>, rustc_driver[30390a19e428bc19]::run_compiler::{closure#1}>::{closure#0}, core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>>::{closure#0}, core[42e67a1b4ba6212c]::result::Result<(), rustc_errors[e8101b8b045aac39]::ErrorGuaranteed>>::{closure#1} as core[42e67a1b4ba6212c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7f1c1ecab373 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::ha070fd431c3387e9
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/alloc/src/boxed.rs:1951:9
  62:     0x7f1c1ecab373 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd87ab2d0201316b5
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/alloc/src/boxed.rs:1951:9
  63:     0x7f1c1ecab373 - std::sys::unix::thread::Thread::new::thread_start::ha32118aaa6c8a9be
                               at /rustc/50b00252aeb77b10db04d65dc9e12ce758def4b5/library/std/src/sys/unix/thread.rs:108:17
  64:     0x7f1c1e88c54d - <unknown>
  65:     0x7f1c1e911874 - clone
  66:                0x0 - <unknown>
