console
error: internal compiler error: compiler/rustc_metadata/src/rmeta/decoder.rs:1189:17: get_optimized_mir: missing MIR for `DefId(18:6 ~ repro[1219]::g::{closure#0})`

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::bug
   2: rustc_errors::Handler::bug
   3: rustc_middle::ty::context::tls::with_opt
   4: rustc_middle::util::bug::opt_span_bug_fmt
   5: rustc_middle::util::bug::bug_fmt
   6: rustc_metadata::rmeta::decoder::cstore_impl::provide_extern::optimized_mir
   7: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
   8: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
   9: rustc_data_structures::stack::ensure_sufficient_stack
  10: rustc_query_system::query::plumbing::force_query_with_job
  11: rustc_query_system::query::plumbing::get_query_impl
  12: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::generator_layout
  13: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
  14: rustc_middle::ty::layout::layout_raw
  15: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::layout_raw>::compute
  16: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  17: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  18: rustc_data_structures::stack::ensure_sufficient_stack
  19: rustc_query_system::query::plumbing::force_query_with_job
  20: rustc_query_system::query::plumbing::get_query_impl
  21: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  22: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  23: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  24: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
  25: rustc_middle::ty::layout::layout_raw
  26: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::layout_raw>::compute
  27: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  29: rustc_data_structures::stack::ensure_sufficient_stack
  30: rustc_query_system::query::plumbing::force_query_with_job
  31: rustc_query_system::query::plumbing::get_query_impl
  32: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  33: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  34: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  35: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
  36: rustc_middle::ty::layout::layout_raw
  37: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::layout_raw>::compute
  38: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  39: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  40: rustc_data_structures::stack::ensure_sufficient_stack
  41: rustc_query_system::query::plumbing::force_query_with_job
  42: rustc_query_system::query::plumbing::get_query_impl
  43: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  44: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  45: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  46: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
  47: rustc_middle::ty::layout::layout_raw
  48: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::layout_raw>::compute
  49: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  50: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  51: rustc_data_structures::stack::ensure_sufficient_stack
  52: rustc_query_system::query::plumbing::force_query_with_job
  53: rustc_query_system::query::plumbing::get_query_impl
  54: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  55: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold
  56: <core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold
  57: <core::iter::adapters::ResultShunt<I,E> as core::iter::traits::iterator::Iterator>::next
  58: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  59: core::iter::adapters::process_results
  60: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
  61: rustc_middle::ty::layout::layout_raw
  62: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::layout_raw>::compute
  63: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  64: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  65: rustc_data_structures::stack::ensure_sufficient_stack
  66: rustc_query_system::query::plumbing::force_query_with_job
  67: rustc_query_system::query::plumbing::get_query_impl
  68: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  69: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  70: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  71: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
  72: rustc_middle::ty::layout::layout_raw
  73: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::layout_raw>::compute
  74: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  75: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  76: rustc_data_structures::stack::ensure_sufficient_stack
  77: rustc_query_system::query::plumbing::force_query_with_job
  78: rustc_query_system::query::plumbing::get_query_impl
  79: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  80: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  81: rustc_mir::transform::run_passes
  82: rustc_mir::transform::optimized_mir
  83: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute
  84: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  85: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  86: rustc_data_structures::stack::ensure_sufficient_stack
  87: rustc_query_system::query::plumbing::force_query_with_job
  88: rustc_query_system::query::plumbing::get_query_impl
  89: clippy_lints::doc::lint_for_missing_headers
  90: <clippy_lints::doc::DocMarkdown as rustc_lint::passes::LateLintPass>::check_item
  91: <rustc_lint::late::LateLintPassObjects as rustc_lint::passes::LateLintPass>::check_item
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.51 (a62a760 2021-01-13)

query stack during panic:
#0 [optimized_mir] optimizing MIR for `repro::g::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@repro::g<()>::{closure#0} {}]`
#2 [layout_raw] computing layout of `std::future::from_generator::GenFuture<[static generator@repro::g<()>::{closure#0} {}]>`
#3 [layout_raw] computing layout of `std::mem::ManuallyDrop<std::future::from_generator::GenFuture<[static generator@repro::g<()>::{closure#0} {}]>>`
#4 [layout_raw] computing layout of `std::mem::MaybeUninit<std::future::from_generator::GenFuture<[static generator@repro::g<()>::{closure#0} {}]>>`
#5 [layout_raw] computing layout of `[static generator@src/main.rs:3:18: 5:2 {std::future::ResumeTy, std::future::from_generator::GenFuture<[static generator@repro::g<()>::{closure#0} {}]>, ()}]`
#6 [layout_raw] computing layout of `std::future::from_generator::GenFuture<[static generator@src/main.rs:3:18: 5:2 {std::future::ResumeTy, std::future::from_generator::GenFuture<[static generator@repro::g<()>::{closure#0} {}]>, ()}]>`
#7 [optimized_mir] optimizing MIR for `f`
#8 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
