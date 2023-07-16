
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore\option.rs:345:21
stack backtrace:
   0: std::sys::windows::backtrace::unwind_backtrace
             at C:\Stuff\Sources\rustsrc\src\libstd\sys\windows\backtrace\mod.rs:95
   1: std::sys_common::backtrace::print
             at C:\Stuff\Sources\rustsrc\src\libstd\sys_common\backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:211
   3: std::panicking::default_hook
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:227
   4: rustc::util::common::panic_hook
             at C:\Stuff\Sources\rustsrc\src\librustc\util\common.rs:51
   5: std::panicking::rust_panic_with_hook
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:479
   6: std::panicking::continue_panic_fmt
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:390
   7: std::panicking::rust_begin_panic
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:325
   8: core::panicking::panic_fmt
             at C:\Stuff\Sources\rustsrc\src\libcore\panicking.rs:77
   9: core::panicking::panic
             at C:\Stuff\Sources\rustsrc\src\libcore\panicking.rs:52
  10: rustc::ty::fold::{{impl}}::fold_ty<closure,closure>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\fold.rs:221
  11: rustc::ty::structural_impls::{{impl}}::super_fold_with<rustc::ty::fold::BottomUpFolder<closure, closure>>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\structural_impls.rs:851
  12: rustc::ty::fold::{{impl}}::fold_ty<closure,closure>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\fold.rs:219
  13: rustc_typeck::check::wfcheck::check_where_clauses
             at C:\Stuff\Sources\rustsrc\src\librustc_typeck\check\wfcheck.rs:503
  14: rustc_typeck::check::wfcheck::check_fn_or_method
             at C:\Stuff\Sources\rustsrc\src\librustc_typeck\check\wfcheck.rs:544
  15: rustc::ty::context::tls::with_related_context<closure,()>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1964
  16: rustc::infer::InferCtxtBuilder::enter<closure,()>
             at C:\Stuff\Sources\rustsrc\src\librustc\infer\mod.rs:469
  17: rustc_typeck::check::wfcheck::check_associated_item
             at C:\Stuff\Sources\rustsrc\src\librustc_typeck\check\wfcheck.rs:182
  18: rustc_typeck::check::wfcheck::check_impl_item
             at C:\Stuff\Sources\rustsrc\src\librustc_typeck\check\wfcheck.rs:174
  19: rustc::ty::query::{{impl}}::compute
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:820
  20: rustc::ty::context::tls::with_context<closure,()>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1953
  21: rustc::dep_graph::graph::DepGraph::with_task_impl<rustc::ty::context::TyCtxt,rustc::hir::def_id::DefId,()>
             at C:\Stuff\Sources\rustsrc\src\librustc\dep_graph\graph.rs:266
  22: rustc::ty::context::tls::with_related_context<closure,((), rustc::dep_graph::graph::DepNodeIndex)>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1964
  23: rustc::ty::context::TyCtxt::force_query_with_job<rustc::ty::query::queries::check_impl_item_well_formed>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:543
  24: rustc::ty::context::TyCtxt::get_query<rustc::ty::query::queries::check_impl_item_well_formed>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:640
  25: rustc::ty::context::TyCtxt::ensure_query<rustc::ty::query::queries::check_impl_item_well_formed>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:602
  26: rustc_typeck::check::wfcheck::{{impl}}::visit_impl_item
             at C:\Stuff\Sources\rustsrc\src\librustc_typeck\check\wfcheck.rs:934
  27: rustc::hir::Crate::visit_all_item_likes<rustc::hir::itemlikevisit::DeepVisitor<rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor>>
             at C:\Stuff\Sources\rustsrc\src\librustc\hir\mod.rs:727
  28: rustc::session::Session::track_errors<closure,()>
             at C:\Stuff\Sources\rustsrc\src\librustc\session\mod.rs:320
  29: rustc::util::common::time<core::result::Result<(), rustc::util::common::ErrorReported>,closure>
             at C:\Stuff\Sources\rustsrc\src\librustc\util\common.rs:157
  30: rustc_typeck::check_crate
             at C:\Stuff\Sources\rustsrc\src\librustc_typeck\lib.rs:368
  31: rustc::ty::context::tls::enter_context<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, alloc::boxed::Box<Any>, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1868
  32: std::thread::local::LocalKey<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::result::Result<(), core::fmt::Error>>>::with<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::result::Result<(), core::fmt::Error>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, alloc::boxed::Box<Any>, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
             at C:\Stuff\Sources\rustsrc\src\libstd\thread\local.rs:248
  33: rustc::ty::context::TyCtxt::create_and_enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, alloc::boxed::Box<Any>, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1186
  34: rustc_driver::driver::compile_input
             at C:\Stuff\Sources\rustsrc\src\librustc_driver\driver.rs:285
  35: rustc_driver::run_compiler_with_pool
             at C:\Stuff\Sources\rustsrc\src\librustc_driver\lib.rs:562
  36: scoped_tls::ScopedKey<syntax::Globals>::set<syntax::Globals,closure,(core::result::Result<(), rustc::session::CompileIncomplete>, core::option::Option<rustc::session::Session>)>
             at C:\Users\Arnavion\.cargo\registry\src\github.com-1ecc6299db9ec823\scoped-tls-0.1.2\src\lib.rs:155
  37: std::panic::{{impl}}::call_once<(),closure>
             at C:\Stuff\Sources\rustsrc\src\libstd\panic.rs:313
  38: panic_unwind::__rust_maybe_catch_panic
             at C:\Stuff\Sources\rustsrc\src\libpanic_unwind\lib.rs:102
  39: std::panicking::try<(),std::panic::AssertUnwindSafe<closure>>
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:289
  40: rustc_driver::run<closure>
             at C:\Stuff\Sources\rustsrc\src\librustc_driver\lib.rs:187
  41: rustc_driver::main
             at C:\Stuff\Sources\rustsrc\src\librustc_driver\lib.rs:1737
  42: std::rt::lang_start::{{closure}}<()>
             at C:\Stuff\Sources\rustsrc\src\libstd\rt.rs:74
  43: std::panicking::try::do_call<closure,i32>
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:310
  44: panic_unwind::__rust_maybe_catch_panic
             at C:\Stuff\Sources\rustsrc\src\libpanic_unwind\lib.rs:102
  45: std::panicking::try<i32,closure>
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:289
  46: std::rt::lang_start_internal
             at C:\Stuff\Sources\rustsrc\src\libstd\rt.rs:58
  47: main
  48: __scrt_common_main_seh
             at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  49: BaseThreadInitThunk
  50: RtlUserThreadStart
query stack during panic:
#0 [check_impl_item_well_formed] processing `Bar::zero`
end of query stack
