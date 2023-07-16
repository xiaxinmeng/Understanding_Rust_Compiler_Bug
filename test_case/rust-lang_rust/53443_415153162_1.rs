
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
  10: rustc::middle::resolve_lifetime::{{impl}}::visit_ty
             at C:\Stuff\Sources\rustsrc\src\librustc\middle\resolve_lifetime.rs:626
  11: rustc::hir::intravisit::walk_item<rustc::middle::resolve_lifetime::LifetimeContext>
             at C:\Stuff\Sources\rustsrc\src\librustc\hir\intravisit.rs:539
  12: rustc::middle::resolve_lifetime::{{impl}}::visit_item
             at C:\Stuff\Sources\rustsrc\src\librustc\middle\resolve_lifetime.rs:539
  13: rustc::middle::resolve_lifetime::resolve_lifetimes
             at C:\Stuff\Sources\rustsrc\src\librustc\middle\resolve_lifetime.rs:392
  14: rustc::ty::query::{{impl}}::compute
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:820
  15: rustc::ty::context::tls::with_context<closure,alloc::rc::Rc<rustc::middle::privacy::AccessLevels<syntax::ast::NodeId>>>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1953
  16: rustc::dep_graph::graph::DepGraph::with_task_impl<rustc::ty::context::TyCtxt,rustc::hir::def_id::CrateNum,alloc::rc::Rc<rustc::middle::resolve_lifetime::ResolveLifetimes>>
             at C:\Stuff\Sources\rustsrc\src\librustc\dep_graph\graph.rs:266
  17: rustc::ty::context::tls::with_related_context<closure,(alloc::rc::Rc<rustc::middle::resolve_lifetime::ResolveLifetimes>, rustc::dep_graph::graph::DepNodeIndex)>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1964
  18: rustc::ty::context::TyCtxt::force_query_with_job<rustc::ty::query::queries::resolve_lifetimes>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:543
  19: rustc::ty::context::TyCtxt::get_query<rustc::ty::query::queries::resolve_lifetimes>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:640
  20: core::ops::function::FnOnce::call_once<closure,(rustc::ty::context::TyCtxt, rustc::hir::def_id::DefIndex)>
             at C:\Stuff\Sources\rustsrc\src\libcore\ops\function.rs:238
  21: rustc::ty::query::{{impl}}::compute
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:820
  22: rustc::ty::context::tls::with_context<closure,alloc::rc::Rc<rustc::middle::privacy::AccessLevels<syntax::ast::NodeId>>>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1953
  23: rustc::dep_graph::graph::DepGraph::with_task_impl<rustc::ty::context::TyCtxt,rustc::hir::def_id::DefIndex,core::option::Option<alloc::rc::Rc<std::collections::hash::map::HashMap<rustc::hir::ItemLocalId, alloc::rc::Rc<alloc::vec::Vec<rustc::middle::resolve_lifetime::Set1<rustc::middle::resolve_lifetime::Region>>>, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>>
             at C:\Stuff\Sources\rustsrc\src\librustc\dep_graph\graph.rs:266
  24: rustc::ty::context::tls::with_related_context<closure,(core::option::Option<alloc::rc::Rc<std::collections::hash::map::HashMap<rustc::hir::ItemLocalId, alloc::rc::Rc<alloc::vec::Vec<rustc::middle::resolve_lifetime::Set1<rustc::middle::resolve_lifetime::Region>>>, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>, rustc::dep_graph::graph::DepNodeIndex)>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1964
  25: rustc::ty::context::TyCtxt::force_query_with_job<rustc::ty::query::queries::object_lifetime_defaults_map>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:543
  26: rustc::ty::context::TyCtxt::get_query<rustc::ty::query::queries::object_lifetime_defaults_map>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:640
  27: rustc::ty::context::TyCtxt::object_lifetime_defaults
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:2787
  28: rustc_typeck::collect::generics_of
             at C:\Stuff\Sources\rustsrc\src\librustc_typeck\collect.rs:915
  29: rustc::ty::query::__query_compute::generics_of<closure,rustc::ty::Generics*>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:790
  30: rustc::ty::query::{{impl}}::compute
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:820
  31: rustc::ty::context::tls::with_context<closure,alloc::rc::Rc<rustc::traits::specialize::specialization_graph::Graph>>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1953
  32: rustc::dep_graph::graph::DepGraph::with_task_impl<rustc::ty::context::TyCtxt,rustc::hir::def_id::DefId,rustc::ty::Generics*>
             at C:\Stuff\Sources\rustsrc\src\librustc\dep_graph\graph.rs:266
  33: rustc::ty::context::tls::with_related_context<closure,(rustc::ty::Generics*, rustc::dep_graph::graph::DepNodeIndex)>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1964
  34: rustc::ty::context::TyCtxt::force_query_with_job<rustc::ty::query::queries::generics_of>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:543
  35: rustc::ty::context::TyCtxt::get_query<rustc::ty::query::queries::generics_of>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:640
  36: rustc::ty::context::TyCtxt::generics_of
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\query\plumbing.rs:869
  37: rustc_typeck::collect::{{impl}}::visit_item
             at C:\Stuff\Sources\rustsrc\src\librustc_typeck\collect.rs:113
  38: rustc::hir::Crate::visit_all_item_likes<rustc::hir::itemlikevisit::DeepVisitor<rustc_typeck::collect::CollectItemTypesVisitor>>
             at C:\Stuff\Sources\rustsrc\src\librustc\hir\mod.rs:719
  39: rustc::util::common::time<(),closure>
             at C:\Stuff\Sources\rustsrc\src\librustc\util\common.rs:157
  40: rustc_typeck::check_crate
             at C:\Stuff\Sources\rustsrc\src\librustc_typeck\lib.rs:342
  41: rustc::ty::context::tls::enter_context<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, alloc::boxed::Box<Any>, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1868
  42: std::thread::local::LocalKey<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::result::Result<(), core::fmt::Error>>>::with<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::result::Result<(), core::fmt::Error>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, alloc::boxed
::Box<Any>, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
             at C:\Stuff\Sources\rustsrc\src\libstd\thread\local.rs:248
  43: rustc::ty::context::TyCtxt::create_and_enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, alloc::boxed::Box<Any>, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
             at C:\Stuff\Sources\rustsrc\src\librustc\ty\context.rs:1186
  44: rustc_driver::driver::compile_input
             at C:\Stuff\Sources\rustsrc\src\librustc_driver\driver.rs:285
  45: rustc_driver::run_compiler_with_pool
             at C:\Stuff\Sources\rustsrc\src\librustc_driver\lib.rs:562
  46: scoped_tls::ScopedKey<syntax::Globals>::set<syntax::Globals,closure,(core::result::Result<(), rustc::session::CompileIncomplete>, core::option::Option<rustc::session::Session>)>
             at C:\Users\Arnavion\.cargo\registry\src\github.com-1ecc6299db9ec823\scoped-tls-0.1.2\src\lib.rs:155
  47: std::panic::{{impl}}::call_once<(),closure>
             at C:\Stuff\Sources\rustsrc\src\libstd\panic.rs:313
  48: panic_unwind::__rust_maybe_catch_panic
             at C:\Stuff\Sources\rustsrc\src\libpanic_unwind\lib.rs:102
  49: std::panicking::try<(),std::panic::AssertUnwindSafe<closure>>
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:289
  50: rustc_driver::run<closure>
             at C:\Stuff\Sources\rustsrc\src\librustc_driver\lib.rs:187
  51: rustc_driver::main
             at C:\Stuff\Sources\rustsrc\src\librustc_driver\lib.rs:1737
  52: std::rt::lang_start::{{closure}}<()>
             at C:\Stuff\Sources\rustsrc\src\libstd\rt.rs:74
  53: std::panicking::try::do_call<closure,i32>
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:310
  54: panic_unwind::__rust_maybe_catch_panic
             at C:\Stuff\Sources\rustsrc\src\libpanic_unwind\lib.rs:102
  55: std::panicking::try<i32,closure>
             at C:\Stuff\Sources\rustsrc\src\libstd\panicking.rs:289
  56: std::rt::lang_start_internal
             at C:\Stuff\Sources\rustsrc\src\libstd\rt.rs:58
  57: main
  58: __scrt_common_main_seh
             at f:\dd\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  59: BaseThreadInitThunk
  60: RtlUserThreadStart
query stack during panic:
#0 [resolve_lifetimes] resolving lifetimes
#1 [object_lifetime_defaults_map] looking up lifetime defaults for a region
#2 [generics_of] processing `Bar`
end of query stack
