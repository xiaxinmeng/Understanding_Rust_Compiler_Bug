
PS D:\code\cargo> cargo +stage1 build  
   Compiling cargo v0.58.0 (D:\code\cargo)
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', D:\code\rust\compiler\rustc_middle\src\ty\sty.rs:974:9
stack backtrace:
   0: std::panicking::begin_panic_handler
             at D:\code\rust\library\std\src\panicking.rs:517
   1: core::panicking::panic_fmt
             at D:\code\rust\library\core\src\panicking.rs:100
   2: core::panicking::panic
             at D:\code\rust\library\core\src\panicking.rs:50
   3: rustc_middle::ty::sty::Binder::dummy<rustc_middle::ty::sty::TraitRef>
             at D:\code\rust\compiler\rustc_middle\src\ty\sty.rs:974
   4: rustc_trait_selection::traits::type_known_to_meet_bound_modulo_regions
             at D:\code\rust\compiler\rustc_trait_selection\src\traits\mod.rs:146
   5: rustc_ty_utils::common_traits::is_item_raw::closure$0
             at D:\code\rust\compiler\rustc_ty_utils\src\common_traits.rs:33
   6: rustc_infer::infer::InferCtxtBuilder::enter<bool,rustc_ty_utils::common_traits::is_item_raw::closure$0>
             at D:\code\rust\compiler\rustc_infer\src\infer\mod.rs:630
   7: rustc_ty_utils::common_traits::is_item_raw
             at D:\code\rust\compiler\rustc_ty_utils\src\common_traits.rs:32
   8: rustc_ty_utils::common_traits::is_sized_raw
             at D:\code\rust\compiler\rustc_ty_utils\src\common_traits.rs:14
   9: rustc_query_system::dep_graph::graph::impl$3::with_task_impl::closure$0
             at D:\code\rust\compiler\rustc_query_system\src\dep_graph\graph.rs:267
  10: rustc_middle::dep_graph::impl$0::with_deps::closure$0::closure$0
             at D:\code\rust\compiler\rustc_middle\src\dep_graph\mod.rs:76
  11: rustc_middle::ty::context::tls::enter_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1765
  12: rustc_middle::ty::context::tls::set_tlv
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1749
  13: rustc_middle::ty::context::tls::enter_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1765
  14: rustc_middle::dep_graph::impl$0::with_deps::closure$0
             at D:\code\rust\compiler\rustc_middle\src\dep_graph\mod.rs:76
  15: rustc_middle::ty::context::tls::with_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1793
  16: rustc_middle::ty::context::tls::with_context_opt
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1782
  17: rustc_middle::ty::context::tls::with_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1793
  18: rustc_middle::dep_graph::impl$0::with_deps<rustc_query_system::dep_graph::graph::impl$3::with_task_impl::closure$0,bool>
             at D:\code\rust\compiler\rustc_middle\src\dep_graph\mod.rs:73
  19: rustc_query_system::dep_graph::graph::DepGraph::with_task_impl
             at D:\code\rust\compiler\rustc_query_system\src\dep_graph\graph.rs:267
  20: rustc_query_system::dep_graph::graph::DepGraph::with_task<enum$<rustc_middle::dep_graph::dep_node::DepKind>,rustc_middle::ty::context::TyCtxt,rustc_middle::ty::ParamEnvAnd<ref$<rustc_middle::ty::TyS> >,bool>
             at D:\code\rust\compiler\rustc_query_system\src\dep_graph\graph.rs:221
  21: rustc_query_system::query::plumbing::execute_job::closure$3
             at D:\code\rust\compiler\rustc_query_system\src\query\plumbing.rs:477
  22: stacker::maybe_grow
             at C:\Users\wesleywiser\.cargo\registry\src\github.com-1ecc6299db9ec823\stacker-0.1.14\src\lib.rs:55
  23: rustc_data_structures::stack::ensure_sufficient_stack<tuple$<bool,rustc_query_system::dep_graph::graph::DepNodeIndex>,rustc_query_system::query::plumbing::execute_job::closure$3>
             at D:\code\rust\compiler\rustc_data_structures\src\stack.rs:17
  24: rustc_query_impl::plumbing::impl$2::start_query::closure$0::closure$0
             at D:\code\rust\compiler\rustc_query_impl\src\plumbing.rs:136
  25: rustc_middle::ty::context::tls::enter_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1765
  26: rustc_middle::ty::context::tls::set_tlv
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1749
  27: rustc_middle::ty::context::tls::enter_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1765
  28: rustc_query_impl::plumbing::impl$2::start_query::closure$0
             at D:\code\rust\compiler\rustc_query_impl\src\plumbing.rs:135
  29: rustc_middle::ty::context::tls::with_related_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1809
  30: rustc_middle::ty::context::tls::with_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1793
  31: rustc_middle::ty::context::tls::with_context_opt
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1782
  32: rustc_middle::ty::context::tls::with_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1793
  33: rustc_middle::ty::context::tls::with_related_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1806
  34: rustc_query_impl::plumbing::impl$2::start_query
             at D:\code\rust\compiler\rustc_query_impl\src\plumbing.rs:124
  35: rustc_query_system::query::plumbing::execute_job
             at D:\code\rust\compiler\rustc_query_system\src\query\plumbing.rs:467
  36: rustc_query_system::query::plumbing::try_execute_query<rustc_query_impl::plumbing::QueryCtxt,rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::ParamEnvAnd<ref$<rustc_middle::ty::TyS> >,bool> >
             at D:\code\rust\compiler\rustc_query_system\src\query\plumbing.rs:401
  37: rustc_query_system::query::plumbing::get_query<rustc_query_impl::queries::is_sized_raw,rustc_query_impl::plumbing::QueryCtxt>
             at D:\code\rust\compiler\rustc_query_system\src\query\plumbing.rs:733
  38: rustc_middle::ty::query::TyCtxtAt::is_sized_raw
             at D:\code\rust\compiler\rustc_middle\src\ty\query.rs:204
  39: rustc_middle::ty::TyS::is_sized
             at D:\code\rust\compiler\rustc_middle\src\ty\util.rs:669
  40: rustc_middle::ty::layout::LayoutCx::layout_of_uncached
             at D:\code\rust\compiler\rustc_middle\src\ty\layout.rs:588
  41: rustc_middle::ty::layout::layout_of::closure$0::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\layout.rs:240
  42: rustc_middle::ty::context::tls::enter_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1765
  43: rustc_middle::ty::context::tls::set_tlv
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1749
  44: rustc_middle::ty::context::tls::enter_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1765
  45: rustc_middle::ty::layout::layout_of::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\layout.rs:229
  46: rustc_middle::ty::context::tls::with_related_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1809
  47: rustc_middle::ty::context::tls::with_context::closure$0<rustc_middle::ty::context::tls::with_related_context::closure$0,enum$<core::result::Result<rustc_target::abi::TyAndLayout<ref$<rustc_middle::ty::TyS> >,enum$<rustc_middle::ty::layout::LayoutError> >
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1793
  48: rustc_middle::ty::layout::layout_of
             at D:\code\rust\compiler\rustc_middle\src\ty\layout.rs:253
  49: rustc_query_system::query::plumbing::execute_job::closure$3::closure$0
             at D:\code\rust\compiler\rustc_query_system\src\query\plumbing.rs:470
  50: rustc_middle::dep_graph::impl$0::with_deps::closure$0::closure$0
             at D:\code\rust\compiler\rustc_middle\src\dep_graph\mod.rs:76
  51: rustc_middle::ty::context::tls::enter_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1765
  52: rustc_middle::ty::context::tls::set_tlv
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1749
  53: rustc_middle::ty::context::tls::enter_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1765
  54: rustc_middle::dep_graph::impl$0::with_deps::closure$0
             at D:\code\rust\compiler\rustc_middle\src\dep_graph\mod.rs:76
  55: rustc_middle::ty::context::tls::with_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1793
  56: rustc_middle::ty::context::tls::with_context_opt
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1782
  57: rustc_middle::ty::context::tls::with_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1793
  58: rustc_middle::dep_graph::impl$0::with_deps<rustc_query_system::query::plumbing::execute_job::closure$3::closure$0,rustc_middle::mir::query::DestructuredConst>
             at D:\code\rust\compiler\rustc_middle\src\dep_graph\mod.rs:73
  59: rustc_query_system::dep_graph::graph::DepGraph::with_task_impl
             at D:\code\rust\compiler\rustc_query_system\src\dep_graph\graph.rs:267
  60: rustc_query_system::dep_graph::graph::DepGraph::with_task<enum$<rustc_middle::dep_graph::dep_node::DepKind>,rustc_middle::ty::context::TyCtxt,rustc_middle::ty::ParamEnvAnd<ref$<rustc_middle::ty::TyS> >,enum$<core::result::Result<rustc_target::abi::TyAndLa
             at D:\code\rust\compiler\rustc_query_system\src\dep_graph\graph.rs:221
  61: rustc_query_system::query::plumbing::execute_job::closure$3
             at D:\code\rust\compiler\rustc_query_system\src\query\plumbing.rs:477
  62: stacker::maybe_grow
             at C:\Users\wesleywiser\.cargo\registry\src\github.com-1ecc6299db9ec823\stacker-0.1.14\src\lib.rs:55
  63: rustc_data_structures::stack::ensure_sufficient_stack<tuple$<enum$<core::result::Result<rustc_target::abi::TyAndLayout<ref$<rustc_middle::ty::TyS> >,enum$<rustc_middle::ty::layout::LayoutError> > >,rustc_query_system::dep_graph::graph::DepNodeIndex>,rustc
             at D:\code\rust\compiler\rustc_data_structures\src\stack.rs:17
  64: rustc_query_impl::plumbing::impl$2::start_query::closure$0::closure$0
             at D:\code\rust\compiler\rustc_query_impl\src\plumbing.rs:136
  65: rustc_middle::ty::context::tls::enter_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1765
  66: rustc_middle::ty::context::tls::set_tlv
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1749
  67: rustc_middle::ty::context::tls::enter_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1765
  68: rustc_query_impl::plumbing::impl$2::start_query::closure$0
             at D:\code\rust\compiler\rustc_query_impl\src\plumbing.rs:135
  69: rustc_middle::ty::context::tls::with_related_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1809
  70: rustc_middle::ty::context::tls::with_context::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1793
  71: rustc_middle::ty::context::tls::with_context_opt
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1782
  72: rustc_middle::ty::context::tls::with_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1793
  73: rustc_middle::ty::context::tls::with_related_context
             at D:\code\rust\compiler\rustc_middle\src\ty\context.rs:1806
  74: rustc_query_impl::plumbing::impl$2::start_query
             at D:\code\rust\compiler\rustc_query_impl\src\plumbing.rs:124
  75: rustc_query_system::query::plumbing::execute_job
             at D:\code\rust\compiler\rustc_query_system\src\query\plumbing.rs:467
  76: rustc_query_system::query::plumbing::try_execute_query
             at D:\code\rust\compiler\rustc_query_system\src\query\plumbing.rs:401
  77: rustc_query_system::query::plumbing::get_query<rustc_query_impl::queries::layout_of,rustc_query_impl::plumbing::QueryCtxt>
             at D:\code\rust\compiler\rustc_query_system\src\query\plumbing.rs:733
  78: rustc_query_impl::impl$530::layout_of
             at D:\code\rust\compiler\rustc_query_impl\src\plumbing.rs:573
  79: rustc_middle::ty::query::TyCtxtAt::layout_of
             at D:\code\rust\compiler\rustc_middle\src\ty\query.rs:204
  80: rustc_middle::ty::layout::LayoutOf::spanned_layout_of
             at D:\code\rust\compiler\rustc_middle\src\ty\layout.rs:2176
  81: rustc_middle::ty::layout::LayoutOf::layout_of<rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> > 
             at D:\code\rust\compiler\rustc_middle\src\ty\layout.rs:2163
  82: rustc_middle::ty::layout::impl$3::layout_of_uncached::closure$5::closure$0
             at D:\code\rust\compiler\rustc_middle\src\ty\layout.rs:833
  83: core::iter::adapters::map::map_try_fold::closure$0
             at D:\code\rust\library\core\src\iter\adapters\map.rs:91
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-dev running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [is_sized_raw] computing whether `dyn core::ops::function::Fn() -> core::result::Result<alloc::string::String, anyhow::Error>` is `Sized`
#1 [layout_of] computing layout of `&dyn core::ops::function::Fn() -> core::result::Result<alloc::string::String, anyhow::Error>`
#2 [layout_of] computing layout of `core::option::Option<&dyn core::ops::function::Fn() -> core::result::Result<alloc::string::String, anyhow::Error>>`
end of query stack
error: could not compile `cargo`
