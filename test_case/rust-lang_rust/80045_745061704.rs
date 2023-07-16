
> RUST_BACKTRACE=1 rustc +stage1 -Zinstrument-coverage src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs
warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:14:19
   |
14 | enum E where i32: Foo { V } //~ WARNING trivial_bounds
   |                   ^^^
   |
   = note: `#[warn(trivial_bounds)]` on by default

warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:16:21
   |
16 | struct S where i32: Foo; //~ WARNING trivial_bounds
   |                     ^^^

warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:18:20
   |
18 | trait T where i32: Foo {} //~ WARNING trivial_bounds
   |                    ^^^

warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:20:20
   |
20 | union U where i32: Foo { f: i32 } //~ WARNING trivial_bounds
   |                    ^^^

warning: where clauses are not enforced in type aliases
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:22:14
   |
22 | type Y where i32: Foo = ();
   |              ^^^^^^^^
   |
   = note: `#[warn(type_alias_bounds)]` on by default
help: the clause will not be checked when the type alias is used, and should be removed
   |
22 | type Y  = ();
   |       --

warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:22:19
   |
22 | type Y where i32: Foo = ();
   |                   ^^^

warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:26:28
   |
26 | impl Foo for () where i32: Foo { //~ WARNING trivial_bounds
   |                            ^^^

warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:34:19
   |
34 | fn f() where i32: Foo { //~ WARNING trivial_bounds
   |                   ^^^

warning: Trait bound &'static str: Foo does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:41:28
   |
41 | fn g() where &'static str: Foo { //~ WARNING trivial_bounds
   |                            ^^^

warning: Trait bound str: Sized does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:55:37
   |
55 | struct TwoStrs(str, str) where str: Sized; //~ WARNING trivial_bounds
   |                                     ^^^^^

warning: Trait bound for<'a> Dst<(dyn A + 'a)>: Sized does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:57:51
   |
57 | fn unsized_local() where for<'a> Dst<dyn A + 'a>: Sized { //~ WARNING trivial_bounds
   |                                                   ^^^^^

warning: Trait bound str: Sized does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:61:35
   |
61 | fn return_str() -> str where str: Sized { //~ WARNING trivial_bounds
   |                                   ^^^^^

warning: Trait bound String: Neg does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:65:46
   |
65 | fn use_op(s: String) -> String where String: ::std::ops::Neg<Output=String> {
   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: Trait bound i32: Iterator does not depend on any type or lifetime parameters
  --> src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs:70:25
   |
70 | fn use_for() where i32: Iterator { //~ WARNING trivial_bounds
   |                         ^^^^^^^^

error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:43:32: could not fully normalize `std::option::Option<<i32 as std::iter::Iterator>::Item>`

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
stack backtrace:
   0: std::panicking::begin_panic
             at ./library/std/src/panicking.rs:519:12
   1: rustc_errors::HandlerInner::bug
             at ./compiler/rustc_errors/src/lib.rs:958:9
   2: rustc_errors::Handler::bug
             at ./compiler/rustc_errors/src/lib.rs:675:9
   3: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
             at ./compiler/rustc_middle/src/util/bug.rs:33:34
   4: rustc_middle::ty::context::tls::with_opt::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1798:40
   5: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1750:22
   6: rustc_middle::ty::context::tls::with_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1798:9
   7: rustc_middle::util::bug::opt_span_bug_fmt
             at ./compiler/rustc_middle/src/util/bug.rs:29:5
   8: rustc_middle::util::bug::bug_fmt
             at ./compiler/rustc_middle/src/util/bug.rs:14:5
   9: rustc_traits::normalize_erasing_regions::normalize_generic_arg_after_erasing_regions::{{closure}}
             at ./compiler/rustc_traits/src/normalize_erasing_regions.rs:43:32
  10: rustc_infer::infer::InferCtxtBuilder::enter
             at ./compiler/rustc_infer/src/infer/mod.rs:583:9
  11: rustc_traits::normalize_erasing_regions::normalize_generic_arg_after_erasing_regions
             at ./compiler/rustc_traits/src/normalize_erasing_regions.rs:22:5
  12: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
  13: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
  14: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  15: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  16: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  17: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  18: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
  19: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
  20: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
  21: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  22: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1777:13
  23: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1761:40
  24: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1750:22
  25: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
  26: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1774:9
  27: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  28: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
  29: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
  30: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
  31: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
  32: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
  33: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
  34: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
  35: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
  36: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
  37: rustc_middle::ty::query::TyCtxtAt::normalize_generic_arg_after_erasing_regions
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
  38: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::normalize_generic_arg_after_erasing_regions
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
  39: <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty
             at ./compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:97:9
  40: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::fold_with
             at ./compiler/rustc_middle/src/ty/structural_impls.rs:969:9
  41: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
             at ./compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:37:13
  42: rustc_middle::ty::util::<impl rustc_middle::ty::TyS>::needs_drop
             at ./compiler/rustc_middle/src/ty/util.rs:766:30
  43: rustc_mir::dataflow::drop_flag_effects::on_all_drop_children_bits::{{closure}}
             at ./compiler/rustc_mir/src/dataflow/drop_flag_effects.rs:156:12
  44: rustc_mir::dataflow::drop_flag_effects::on_all_children_bits::on_all_children_bits
             at ./compiler/rustc_mir/src/dataflow/drop_flag_effects.rs:126:9
  45: rustc_mir::dataflow::drop_flag_effects::on_all_children_bits
             at ./compiler/rustc_mir/src/dataflow/drop_flag_effects.rs:138:5
  46: rustc_mir::dataflow::drop_flag_effects::on_all_drop_children_bits
             at ./compiler/rustc_mir/src/dataflow/drop_flag_effects.rs:150:5
  47: rustc_mir::transform::elaborate_drops::find_dead_unwinds
             at ./compiler/rustc_mir/src/transform/elaborate_drops.rs:116:9
  48: <rustc_mir::transform::elaborate_drops::ElaborateDrops as rustc_mir::transform::MirPass>::run_pass
             at ./compiler/rustc_mir/src/transform/elaborate_drops.rs:41:32
  49: rustc_mir::transform::run_passes::{{closure}}
             at ./compiler/rustc_mir/src/transform/mod.rs:186:9
  50: rustc_mir::transform::run_passes
             at ./compiler/rustc_mir/src/transform/mod.rs:202:13
  51: rustc_mir::transform::run_post_borrowck_cleanup_passes
             at ./compiler/rustc_mir/src/transform/mod.rs:372:5
  52: rustc_mir::transform::mir_drops_elaborated_and_const_checked
             at ./compiler/rustc_mir/src/transform/mod.rs:340:5
  53: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_drops_elaborated_and_const_checked>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  54: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
  55: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:200:9
  56: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:608:17
  57: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  58: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  59: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  60: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  61: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
  62: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
  63: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
  64: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  65: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1777:13
  66: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1761:40
  67: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1750:22
  68: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
  69: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1774:9
  70: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  71: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
  72: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
  73: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
  74: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
  75: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
  76: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
  77: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
  78: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
  79: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
  80: rustc_middle::ty::query::TyCtxtAt::mir_drops_elaborated_and_const_checked
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
  81: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::mir_drops_elaborated_and_const_checked
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
  82: rustc_mir::transform::inner_optimized_mir
             at ./compiler/rustc_mir/src/transform/mod.rs:485:20
  83: rustc_mir::transform::optimized_mir
             at ./compiler/rustc_mir/src/transform/mod.rs:462:25
  84: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  85: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
  86: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
  87: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  88: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  89: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  90: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  91: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
  92: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
  93: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
  94: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  95: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1777:13
  96: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1761:40
  97: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1750:22
  98: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
  99: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1774:9
 100: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
 101: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
 102: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
 103: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
 104: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
 105: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
 106: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
 107: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
 108: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
 109: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
 110: rustc_middle::ty::query::TyCtxtAt::optimized_mir
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
 111: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::optimized_mir
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
 112: rustc_mir::transform::coverage::query::covered_file_name
             at ./compiler/rustc_mir/src/transform/coverage/query.rs:132:20
 113: rustc_mir::transform::coverage::query::provide::{{closure}}
             at ./compiler/rustc_mir/src/transform/coverage/query.rs:14:49
 114: core::ops::function::FnOnce::call_once
             at ./library/core/src/ops/function.rs:227:5
 115: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::covered_file_name>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
 116: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
 117: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
 118: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
 119: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
 120: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
 121: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
 122: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
 123: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
 124: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
 125: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
 126: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1777:13
 127: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1761:40
 128: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1750:22
 129: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
 130: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1774:9
 131: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
 132: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
 133: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
 134: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
 135: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
 136: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
 137: <rustc_query_system::query::caches::ArenaCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:198:13
 138: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
 139: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
 140: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
 141: rustc_middle::ty::query::TyCtxtAt::covered_file_name
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
 142: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::covered_file_name
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
 143: rustc_codegen_llvm::coverageinfo::mapgen::add_unreachable_coverage
             at ./compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs:292:49
 144: rustc_codegen_llvm::coverageinfo::mapgen::finalize
             at ./compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs:51:5
 145: rustc_codegen_llvm::coverageinfo::<impl rustc_codegen_ssa::traits::coverageinfo::CoverageInfoMethods for rustc_codegen_llvm::context::CodegenCx>::coverageinfo_finalize
             at ./compiler/rustc_codegen_llvm/src/coverageinfo/mod.rs:46:9
 146: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
             at ./compiler/rustc_codegen_llvm/src/base.rs:149:17
 147: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
 148: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:200:9
 149: rustc_codegen_llvm::base::compile_codegen_unit
             at ./compiler/rustc_codegen_llvm/src/base.rs:104:9
 150: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::compile_codegen_unit
             at ./compiler/rustc_codegen_llvm/src/lib.rs:106:9
 151: rustc_codegen_ssa::base::codegen_crate
             at ./compiler/rustc_codegen_ssa/src/base.rs:642:38
 152: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             at ./compiler/rustc_codegen_llvm/src/lib.rs:267:18
 153: rustc_interface::passes::start_codegen::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:996:9
 154: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:570:9
 155: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:9:9
 156: rustc_interface::passes::start_codegen
             at ./compiler/rustc_interface/src/passes.rs:995:19
 157: rustc_interface::queries::Queries::ongoing_codegen::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/queries.rs:282:20
 158: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:725:42
 159: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
 160: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
 161: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
 162: rustc_interface::passes::QueryContext::enter
             at ./compiler/rustc_interface/src/passes.rs:725:9
 163: rustc_interface::queries::Queries::ongoing_codegen::{{closure}}
             at ./compiler/rustc_interface/src/queries.rs:273:13
 164: rustc_interface::queries::Query<T>::compute
             at ./compiler/rustc_interface/src/queries.rs:35:28
 165: rustc_interface::queries::Queries::ongoing_codegen
             at ./compiler/rustc_interface/src/queries.rs:271:9
 166: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:446:13
 167: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:413:19
 168: rustc_driver::run_compiler::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:341:22
 169: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:196:13
 170: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:765:5
 171: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:190:5
 172: rustc_interface::interface::run_compiler::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:212:12
 173: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:152:13
 174: scoped_tls::ScopedKey<T>::set
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 175: rustc_span::with_session_globals
             at ./compiler/rustc_span/src/lib.rs:94:5
 176: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:150:9
 177: rustc_interface::util::scoped_thread::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:125:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z instrument-coverage

query stack during panic:
#0 [normalize_generic_arg_after_erasing_regions] normalizing `std::option::Option<<i32 as std::iter::Iterator>::Item>`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `use_for`
#2 [optimized_mir] optimizing MIR for `use_for`
#3 [covered_file_name] retrieving the covered file name, if instrumented, for `use_for`
end of query stack
error: aborting due to previous error; 14 warnings emitted
