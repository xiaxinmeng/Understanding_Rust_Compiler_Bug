
   Compiling playground v0.0.1 (/playground)
error[E0658]: trait aliases are experimental
  --> src/lib.rs:11:1
   |
11 | trait Foo<Ix, OnSet> = where OnSet: Callback0;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: for more information, see https://github.com/rust-lang/rust/issues/41517
   = help: add `#![feature(trait_alias)]` to the crate attributes to enable

warning: trait objects without an explicit `dyn` are deprecated
  --> src/lib.rs:14:16
   |
14 |     type Ctx = Foo<Ix, OnSet>;
   |                ^^^^^^^^^^^^^^ help: use `dyn`: `dyn Foo<Ix, OnSet>`
   |
   = note: `#[warn(bare_trait_objects)]` on by default

error: internal compiler error: src/librustc_typeck/astconv.rs:1216: trait_ref_to_existential called on <OnSet as Callback0> with non-dummy Self

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:925:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:189
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:206
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:473
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_opt
  19: rustc::util::bug::opt_span_bug_fmt
  20: rustc::util::bug::bug_fmt
  21: <dyn rustc_typeck::astconv::AstConv>::trait_ref_to_existential
  22: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  23: <dyn rustc_typeck::astconv::AstConv>::conv_object_ty_poly_trait_ref
  24: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty
  25: rustc_typeck::collect::checked_type_of
  26: rustc_typeck::collect::type_of
  27: rustc::ty::query::__query_compute::type_of
  28: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  31: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_impl_item
  32: rustc::hir::map::Map::visit_item_likes_in_module
  33: rustc_typeck::collect::collect_mod_item_types
  34: rustc::ty::query::__query_compute::collect_mod_item_types
  35: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  39: rustc_typeck::check_crate::{{closure}}::{{closure}}
  40: rustc::util::common::time
  41: rustc_typeck::check_crate
  42: rustc_interface::passes::analysis
  43: rustc::ty::query::__query_compute::analysis
  44: rustc::dep_graph::graph::DepGraph::with_task_impl
  45: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  46: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  47: rustc_interface::passes::create_global_ctxt::{{closure}}
  48: rustc_interface::passes::BoxedGlobalCtxt::enter
  49: rustc_interface::interface::run_compiler_in_existing_thread_pool
  50: std::thread::local::LocalKey<T>::with
  51: scoped_tls::ScopedKey<T>::set
  52: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (7979016af 2019-10-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] processing `<SharedRange<Ix, OnSet> as HasCtx>::Ctx`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `playground`.

To learn more, run the command again with --verbose
