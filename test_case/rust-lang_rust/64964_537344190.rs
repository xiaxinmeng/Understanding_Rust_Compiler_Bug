
$ RUST_BACKTRACE=1 /home/user/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc --edition 2018 -Z query-dep-graph a.rs
error: internal compiler error: src/librustc/ich/impls_ty.rs:100: StableHasher: unexpected region '_#0r

thread '<unnamed>' panicked at 'Box<Any>', src/librustc_errors/lib.rs:912:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
             at src/librustc_driver/lib.rs:1187
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::begin_panic
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:407
  13: rustc_errors::HandlerInner::bug
             at src/librustc_errors/lib.rs:912
  14: rustc_errors::Handler::bug
             at src/librustc_errors/lib.rs:684
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
             at src/librustc/util/bug.rs:36
  16: rustc::ty::context::tls::with_opt::{{closure}}
             at src/librustc/ty/context.rs:1982
  17: rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:1932
  18: rustc::ty::context::tls::with_opt
             at src/librustc/ty/context.rs:1982
  19: rustc::util::bug::opt_span_bug_fmt
             at src/librustc/util/bug.rs:32
  20: rustc::util::bug::bug_fmt
             at src/librustc/util/bug.rs:12
  21: rustc::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext> for rustc::ty::sty::RegionKind>::hash_stable
             at src/librustc/ich/impls_ty.rs:100
  22: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures/stable_hasher.rs:421
  23: rustc::ty::sty::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_ich_StableHashingContext_ctx_FOR_TyKind::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext> for rustc::ty::sty::TyKind>::hash_stable
             at src/librustc/ty/sty.rs:89
  24: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures/stable_hasher.rs:421
  25: rustc::ty::context::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_ich_StableHashingContext_ctx_FOR_GeneratorInteriorTypeCause::<impl rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext> for rustc::ty::context::GeneratorInteriorTypeCause>::hash_stable
             at src/librustc/ty/context.rs:309
  26: <[T] as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures/stable_hasher.rs:289
  27: <alloc::vec::Vec<T> as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures/stable_hasher.rs:297
  28: <rustc::ty::context::TypeckTables as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable::{{closure}}
             at src/librustc/ty/context.rs:810
  29: rustc::ich::hcx::StableHashingContext::with_node_id_hashing_mode
             at src/librustc/ich/hcx.rs:129
  30: <rustc::ty::context::TypeckTables as rustc_data_structures::stable_hasher::HashStable<rustc::ich::hcx::StableHashingContext>>::hash_stable
             at src/librustc/ty/context.rs:769
  31: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures/stable_hasher.rs:421
  32: <&T as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures/stable_hasher.rs:421
  33: rustc::dep_graph::graph::hash_result
             at src/librustc/dep_graph/graph.rs:88
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::hash_result
             at src/librustc/ty/query/plumbing.rs:1011
  35: core::ops::function::FnOnce::call_once
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libcore/ops/function.rs:227
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:286
  37: rustc::dep_graph::graph::DepGraph::with_task
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:202
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:565
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:278
  40: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1854
  41: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  42: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1761
  43: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1853
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:277
  45: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1960
  46: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
  47: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1932
  48: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
  49: rustc::ty::context::tls::with_related_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1956
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:266
  51: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:557
  52: rustc::ty::query::plumbing::with_diagnostics
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:211
  53: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:556
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:434
  55: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1003
  56: rustc::ty::query::__query_compute::typeck_tables_of
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:954
  57: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:995
  58: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:277
  59: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1854
  60: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  61: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1761
  62: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1853
  63: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:276
  64: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
  65: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1932
  66: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
  67: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:270
  68: rustc::dep_graph::graph::DepGraph::with_task
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:202
  69: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:565
  70: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:278
  71: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1854
  72: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  73: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1761
  74: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1853
  75: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:277
  76: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1960
  77: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
  78: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1932
  79: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
  80: rustc::ty::context::tls::with_related_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1956
  81: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:266
  82: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:557
  83: rustc::ty::query::plumbing::with_diagnostics
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:211
  84: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:556
  85: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:434
  86: rustc::ty::query::TyCtxtAt::typeck_tables_of
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1080
  87: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::typeck_tables_of
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1072
  88: rustc_typeck::collect::checked_type_of
             at src/librustc_typeck/collect.rs:1362
  89: rustc_typeck::collect::type_of
             at src/librustc_typeck/collect.rs:1144
  90: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1003
  91: rustc::ty::query::__query_compute::type_of
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:954
  92: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:995
  93: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:277
  94: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1854
  95: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
  96: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1761
  97: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1853
  98: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:276
  99: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 100: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1932
 101: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 102: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:270
 103: rustc::dep_graph::graph::DepGraph::with_task
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:202
 104: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:565
 105: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:278
 106: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1854
 107: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 108: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1761
 109: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1853
 110: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:277
 111: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1960
 112: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 113: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1932
 114: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 115: rustc::ty::context::tls::with_related_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1956
 116: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:266
 117: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:557
 118: rustc::ty::query::plumbing::with_diagnostics
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:211
 119: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:556
 120: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:434
 121: rustc::ty::query::TyCtxtAt::type_of
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1080
 122: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::type_of
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1072
 123: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_expr
             at src/librustc_typeck/collect.rs:141
 124: rustc::hir::intravisit::walk_expr
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/<::syntax::visit::walk_list macros>:2
 125: rustc::hir::intravisit::Visitor::visit_fn
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/hir/intravisit.rs:293
 126: rustc::hir::intravisit::walk_impl_item
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/hir/intravisit.rs:921
 127: rustc::hir::map::Map::visit_item_likes_in_module
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/hir/map/mod.rs:586
 128: rustc_typeck::collect::collect_mod_item_types
             at src/librustc_typeck/collect.rs:57
 129: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1003
 130: rustc::ty::query::__query_compute::collect_mod_item_types
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:954
 131: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:995
 132: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:277
 133: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1854
 134: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 135: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1761
 136: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1853
 137: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:276
 138: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 139: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1932
 140: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 141: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:270
 142: rustc::dep_graph::graph::DepGraph::with_task
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:202
 143: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:565
 144: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:278
 145: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1854
 146: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 147: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1761
 148: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1853
 149: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:277
 150: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1960
 151: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 152: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1932
 153: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 154: rustc::ty::context::tls::with_related_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1956
 155: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:266
 156: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:557
 157: rustc::ty::query::plumbing::with_diagnostics
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:211
 158: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:556
 159: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:434
 160: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:619
 161: rustc::ty::query::TyCtxtEnsure::collect_mod_item_types
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1031
 162: rustc_typeck::check_crate::{{closure}}::{{closure}}
             at src/librustc_typeck/lib.rs:306
 163: rustc::util::common::time_ext
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/util/common.rs:116
 164: rustc::util::common::time
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/util/common.rs:110
 165: rustc_typeck::check_crate::{{closure}}
             at src/librustc_typeck/lib.rs:304
 166: rustc::session::Session::track_errors
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/session/mod.rs:334
 167: rustc_typeck::check_crate
             at src/librustc_typeck/lib.rs:303
 168: rustc_interface::passes::analysis
             at src/librustc_interface/passes.rs:915
 169: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1003
 170: rustc::ty::query::__query_compute::analysis
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:954
 171: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:277
 172: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1854
 173: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 174: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1761
 175: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1853
 176: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:276
 177: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 178: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1932
 179: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 180: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:270
 181: rustc::dep_graph::graph::DepGraph::with_eval_always_task
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/dep_graph/graph.rs:387
 182: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:559
 183: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:278
 184: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1854
 185: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 186: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1761
 187: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1853
 188: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:277
 189: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1960
 190: rustc::ty::context::tls::with_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 191: rustc::ty::context::tls::with_context_opt
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1932
 192: rustc::ty::context::tls::with_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1943
 193: rustc::ty::context::tls::with_related_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1956
 194: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:266
 195: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:557
 196: rustc::ty::query::plumbing::with_diagnostics
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:211
 197: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:556
 198: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:434
 199: rustc::ty::query::TyCtxtAt::analysis
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1080
 200: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::analysis
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/query/plumbing.rs:1072
 201: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:377
 202: rustc_interface::passes::BoxedGlobalCtxt::enter::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/passes.rs:809
 203: rustc::ty::context::tls::enter_global::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1886
 204: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1854
 205: rustc_rayon_core::tlv::with
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/tlv.rs:19
 206: rustc::ty::context::tls::set_tlv
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1761
 207: rustc::ty::context::tls::enter_context
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1853
 208: rustc::ty::context::tls::enter_global
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1885
 209: rustc_interface::passes::BoxedGlobalCtxt::enter::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/passes.rs:809
 210: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/<::rustc_data_structures::box_region::declare_box_region_type macros>:21
 211: rustc_interface::passes::create_global_ctxt::{{closure}}
             at src/librustc_interface/passes.rs:873
 212: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::access
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures/box_region.rs:52
 213: rustc_interface::passes::BoxedGlobalCtxt::access
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/<::rustc_data_structures::box_region::declare_box_region_type macros>:24
 214: rustc_interface::passes::BoxedGlobalCtxt::enter
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/passes.rs:809
 215: rustc_driver::run_compiler::{{closure}}
             at src/librustc_driver/lib.rs:377
 216: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/interface.rs:122
 217: rustc_interface::interface::run_compiler::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/interface.rs:141
 218: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:219
 219: rustc_rayon_core::thread_pool::ThreadPool::install::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/thread_pool/mod.rs:160
 220: rustc_rayon_core::registry::Registry::in_worker_cold::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:395
 221: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:121
 222: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:315
 223: std::panicking::try::do_call
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:292
 224: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
 225: std::panicking::try
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:271
 226: std::panic::catch_unwind
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:394
 227: rustc_rayon_core::unwind::halt_unwinding
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/unwind.rs:19
 228: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:121
 229: rustc_rayon_core::job::JobRef::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/job.rs:62
 230: rustc_rayon_core::registry::WorkerThread::execute
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:657
 231: rustc_rayon_core::registry::WorkerThread::wait_until_cold
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:637
 232: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:235
 233: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 234: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:235
 235: rustc::ty::context::tls::with_thread_locals::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1842
 236: std::thread::local::LocalKey<T>::try_with
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:262
 237: std::thread::local::LocalKey<T>::with
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:239
 238: rustc::ty::context::tls::with_thread_locals::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc/ty/context.rs:1834
 239: std::thread::local::LocalKey<T>::try_with
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:262
 240: std::thread::local::LocalKey<T>::with
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/thread/local.rs:239
 241: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:234
 242: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 243: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:230
 244: scoped_tls::ScopedKey<T>::set
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 245: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_interface/util.rs:229
 246: rustc_rayon_core::thread_pool::ThreadPool::scoped_pool::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/thread_pool/mod.rs:104
 247: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
 248: std::panicking::try
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panicking.rs:271
 249: std::panic::catch_unwind
             at /home/user/build/2nonpkgs/rust.stuff/rust/rust/src/libstd/panic.rs:394
 250: rustc_rayon_core::unwind::halt_unwinding
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/unwind.rs:19
 251: rustc_rayon_core::registry::main_loop
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:747
 252: rustc_rayon_core::registry::Registry::new::{{closure}}
             at /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.2.0/src/registry.rs:145
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-dev (7130fc54e 2019-10-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z query-dep-graph

query stack during panic:
#0 [typeck_tables_of] processing `Body::next`
#1 [typeck_tables_of] processing `Body::next::{{closure}}#0`
#2 [type_of] processing `Body::next::{{closure}}#0`
#3 [collect_mod_item_types] collecting item types in top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
