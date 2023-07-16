
   Compiling compiler_crash v0.1.0 (/tmp/compiler_crash)
error: internal compiler error: compiler/rustc_middle/src/ty/context.rs:703:13: node_type: no type for node `expr {
                                        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
                                        res
                                    } (hir_id=HirId { owner: DefId(0:3 ~ compiler_crash[58a7]::main), local_id: 41 })`

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/897e37553bba8b42751c67658967889d11ecd120/compiler/rustc_errors/src/lib.rs:1462:9
stack backtrace:
   0:     0x7fde11129d40 - std::backtrace_rs::backtrace::libunwind::trace::h32eb3e08e874dd27
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fde11129d40 - std::backtrace_rs::backtrace::trace_unsynchronized::haa3f451d27bc11a5
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fde11129d40 - std::sys_common::backtrace::_print_fmt::h5b94a01bb4289bb5
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fde11129d40 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb070b7fa7e3175df
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fde11184bfe - core::fmt::write::hd5207aebbb9a86e9
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/fmt/mod.rs:1202:17
   5:     0x7fde1111a935 - std::io::Write::write_fmt::h3bd699bbd129ab8a
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/io/mod.rs:1679:15
   6:     0x7fde1112c9f3 - std::sys_common::backtrace::_print::h7a21be552fdf58da
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fde1112c9f3 - std::sys_common::backtrace::print::ha85c41fe4dd80b13
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fde1112c9f3 - std::panicking::default_hook::{{closure}}::h04cca40023d0eeca
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:295:22
   9:     0x7fde1112c6df - std::panicking::default_hook::haa3ca8c310ed5402
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:314:9
  10:     0x7fde0fb9e2e1 - rustc_driver[cfb34b1539811fe8]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fde1112d22d - std::panicking::rust_panic_with_hook::h7b190ce1a948faac
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:702:17
  12:     0x7fde10b24e51 - std[71cb4861428b0c25]::panicking::begin_panic::<rustc_errors[f77a66b68db622d5]::ExplicitBug>::{closure#0}
  13:     0x7fde10b22966 - std[71cb4861428b0c25]::sys_common::backtrace::__rust_end_short_backtrace::<std[71cb4861428b0c25]::panicking::begin_panic<rustc_errors[f77a66b68db622d5]::ExplicitBug>::{closure#0}, !>
  14:     0x7fde10b7c316 - std[71cb4861428b0c25]::panicking::begin_panic::<rustc_errors[f77a66b68db622d5]::ExplicitBug>
  15:     0x7fde10b1f5f6 - std[71cb4861428b0c25]::panic::panic_any::<rustc_errors[f77a66b68db622d5]::ExplicitBug>
  16:     0x7fde10b1e1dd - <rustc_errors[f77a66b68db622d5]::HandlerInner>::bug::<&alloc[188ed69dc0d14b4b]::string::String>
  17:     0x7fde10b1dc10 - <rustc_errors[f77a66b68db622d5]::Handler>::bug::<&alloc[188ed69dc0d14b4b]::string::String>
  18:     0x7fde10bda7ed - rustc_middle[a9ca1c3f9fd197cd]::ty::context::tls::with_context_opt::<rustc_middle[a9ca1c3f9fd197cd]::ty::context::tls::with_opt<rustc_middle[a9ca1c3f9fd197cd]::util::bug::opt_span_bug_fmt<rustc_span[8f00505efff112e2]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  19:     0x7fde10bddc86 - rustc_middle[a9ca1c3f9fd197cd]::util::bug::opt_span_bug_fmt::<rustc_span[8f00505efff112e2]::span_encoding::Span>
  20:     0x7fde0e57ba83 - rustc_middle[a9ca1c3f9fd197cd]::util::bug::bug_fmt
  21:     0x7fde0eb9c59d - <rustc_middle[a9ca1c3f9fd197cd]::ty::context::TypeckResults>::expr_ty_adjusted
  22:     0x7fde1099f2b5 - <rustc_infer[ed26b14e1208c12f]::infer::InferCtxt as rustc_trait_selection[f6cb7b5468c1337d]::traits::error_reporting::suggestions::InferCtxtExt>::note_obligation_cause_code::<rustc_middle[a9ca1c3f9fd197cd]::ty::Predicate>
  23:     0x7fde109aec5f - <rustc_infer[ed26b14e1208c12f]::infer::InferCtxt as rustc_trait_selection[f6cb7b5468c1337d]::traits::error_reporting::InferCtxtPrivExt>::note_obligation_cause
  24:     0x7fde109a6068 - <rustc_infer[ed26b14e1208c12f]::infer::InferCtxt as rustc_trait_selection[f6cb7b5468c1337d]::traits::error_reporting::InferCtxtExt>::report_selection_error
  25:     0x7fde109b258a - <rustc_infer[ed26b14e1208c12f]::infer::InferCtxt as rustc_trait_selection[f6cb7b5468c1337d]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  26:     0x7fde109a4b50 - <rustc_infer[ed26b14e1208c12f]::infer::InferCtxt as rustc_trait_selection[f6cb7b5468c1337d]::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
  27:     0x7fde0e82e5f7 - <rustc_typeck[4a3b2f91c49b3c3e]::check::fn_ctxt::FnCtxt>::check_call
  28:     0x7fde0e7ddf27 - <rustc_typeck[4a3b2f91c49b3c3e]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:     0x7fde0e825772 - <rustc_typeck[4a3b2f91c49b3c3e]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  30:     0x7fde0e7de050 - <rustc_typeck[4a3b2f91c49b3c3e]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  31:     0x7fde0f1d84d6 - <rustc_typeck[4a3b2f91c49b3c3e]::check::fn_ctxt::FnCtxt>::check_return_expr
  32:     0x7fde0f1d086a - rustc_typeck[4a3b2f91c49b3c3e]::check::check::check_fn
  33:     0x7fde0f1c3a75 - <rustc_infer[ed26b14e1208c12f]::infer::InferCtxtBuilder>::enter::<&rustc_middle[a9ca1c3f9fd197cd]::ty::context::TypeckResults, <rustc_typeck[4a3b2f91c49b3c3e]::check::inherited::InheritedBuilder>::enter<rustc_typeck[4a3b2f91c49b3c3e]::check::typeck_with_fallback<rustc_typeck[4a3b2f91c49b3c3e]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[a9ca1c3f9fd197cd]::ty::context::TypeckResults>::{closure#0}>
  34:     0x7fde0f1c1c29 - rustc_typeck[4a3b2f91c49b3c3e]::check::typeck
  35:     0x7fde0f46d8ad - <rustc_query_system[860ed7b39cdfb46b]::dep_graph::graph::DepGraph<rustc_middle[a9ca1c3f9fd197cd]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[a9ca1c3f9fd197cd]::ty::context::TyCtxt, rustc_span[8f00505efff112e2]::def_id::LocalDefId, &rustc_middle[a9ca1c3f9fd197cd]::ty::context::TypeckResults>
  36:     0x7fde0f460664 - rustc_query_system[860ed7b39cdfb46b]::query::plumbing::try_execute_query::<rustc_query_impl[d6191eea714bd250]::plumbing::QueryCtxt, rustc_query_system[860ed7b39cdfb46b]::query::caches::DefaultCache<rustc_span[8f00505efff112e2]::def_id::LocalDefId, &rustc_middle[a9ca1c3f9fd197cd]::ty::context::TypeckResults>>
  37:     0x7fde0fadacae - <rustc_query_impl[d6191eea714bd250]::Queries as rustc_middle[a9ca1c3f9fd197cd]::ty::query::QueryEngine>::typeck
  38:     0x7fde0f91b057 - rustc_data_structures[62ce3989ce8140ee]::sync::par_for_each_in::<&[rustc_span[8f00505efff112e2]::def_id::LocalDefId], <rustc_middle[a9ca1c3f9fd197cd]::hir::map::Map>::par_body_owners<rustc_typeck[4a3b2f91c49b3c3e]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  39:     0x7fde0f91ae13 - rustc_typeck[4a3b2f91c49b3c3e]::check::typeck_item_bodies
  40:     0x7fde0f8252e5 - <rustc_query_system[860ed7b39cdfb46b]::dep_graph::graph::DepGraph<rustc_middle[a9ca1c3f9fd197cd]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[a9ca1c3f9fd197cd]::ty::context::TyCtxt, (), ()>
  41:     0x7fde0f8242be - rustc_query_system[860ed7b39cdfb46b]::query::plumbing::try_execute_query::<rustc_query_impl[d6191eea714bd250]::plumbing::QueryCtxt, rustc_query_system[860ed7b39cdfb46b]::query::caches::DefaultCache<(), ()>>
  42:     0x7fde0f823cc7 - rustc_query_system[860ed7b39cdfb46b]::query::plumbing::get_query::<rustc_query_impl[d6191eea714bd250]::queries::typeck_item_bodies, rustc_query_impl[d6191eea714bd250]::plumbing::QueryCtxt>
  43:     0x7fde0ef11a9f - <rustc_session[c5946fac61f8bc34]::session::Session>::time::<(), rustc_typeck[4a3b2f91c49b3c3e]::check_crate::{closure#7}>
  44:     0x7fde0ef1165f - rustc_typeck[4a3b2f91c49b3c3e]::check_crate
  45:     0x7fde0ef10c27 - rustc_interface[3182dd864eff9d7d]::passes::analysis
  46:     0x7fde0f96f382 - <rustc_query_system[860ed7b39cdfb46b]::dep_graph::graph::DepGraph<rustc_middle[a9ca1c3f9fd197cd]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[a9ca1c3f9fd197cd]::ty::context::TyCtxt, (), core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>
  47:     0x7fde0f96ea4a - rustc_query_system[860ed7b39cdfb46b]::query::plumbing::try_execute_query::<rustc_query_impl[d6191eea714bd250]::plumbing::QueryCtxt, rustc_query_system[860ed7b39cdfb46b]::query::caches::DefaultCache<(), core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>>
  48:     0x7fde0f96e517 - rustc_query_system[860ed7b39cdfb46b]::query::plumbing::get_query::<rustc_query_impl[d6191eea714bd250]::queries::analysis, rustc_query_impl[d6191eea714bd250]::plumbing::QueryCtxt>
  49:     0x7fde0ea5a897 - <rustc_interface[3182dd864eff9d7d]::passes::QueryContext>::enter::<rustc_driver[cfb34b1539811fe8]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>
  50:     0x7fde0ea4e28c - rustc_interface[3182dd864eff9d7d]::interface::create_compiler_and_run::<core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>, rustc_driver[cfb34b1539811fe8]::run_compiler::{closure#1}>
  51:     0x7fde0ea4cac1 - <scoped_tls[e395fa6e23b19669]::ScopedKey<rustc_span[8f00505efff112e2]::SessionGlobals>>::set::<rustc_interface[3182dd864eff9d7d]::interface::run_compiler<core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>, rustc_driver[cfb34b1539811fe8]::run_compiler::{closure#1}>::{closure#0}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>
  52:     0x7fde0ea4c7af - std[71cb4861428b0c25]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3182dd864eff9d7d]::util::run_in_thread_pool_with_globals<rustc_interface[3182dd864eff9d7d]::interface::run_compiler<core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>, rustc_driver[cfb34b1539811fe8]::run_compiler::{closure#1}>::{closure#0}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>::{closure#0}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>
  53:     0x7fde0fa01d70 - <<std[71cb4861428b0c25]::thread::Builder>::spawn_unchecked_<rustc_interface[3182dd864eff9d7d]::util::run_in_thread_pool_with_globals<rustc_interface[3182dd864eff9d7d]::interface::run_compiler<core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>, rustc_driver[cfb34b1539811fe8]::run_compiler::{closure#1}>::{closure#0}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>::{closure#0}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>::{closure#1} as core[8c92e53db3fc2eaa]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  54:     0x7fde11137003 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h49f797984e2121bf
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
  55:     0x7fde11137003 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfa4f3d0ee6440e0b
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
  56:     0x7fde11137003 - std::sys::unix::thread::Thread::new::thread_start::h62ca48b42d48a8fc
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys/unix/thread.rs:108:17
  57:     0x7fde0d2578fd - <unknown>
  58:     0x7fde0d2d9a60 - <unknown>
  59:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0 (897e37553 2022-11-02) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `compiler_crash`
