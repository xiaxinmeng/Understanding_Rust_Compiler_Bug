
(gdb) where
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
#1  0x00007ffff3239535 in __GI_abort () at abort.c:79
#2  0x00007ffff34a9ca7 in std::sys::unix::abort_internal ()
    at /home/joshua/rustc2/library/std/src/sys/unix/mod.rs:237
#3  0x00007ffff34743c6 in std::process::abort ()
    at /home/joshua/rustc2/library/std/src/process.rs:1772
#4  0x00007ffff345261e in std::alloc::rust_oom ()
    at /home/joshua/rustc2/library/std/src/alloc.rs:332
#5  0x00007ffff34fac87 in alloc::alloc::__alloc_error_handler::__rg_oom ()
    at /home/joshua/rustc2/library/alloc/src/alloc.rs:397
#6  0x00007ffff34fc767 in alloc::alloc::handle_alloc_error ()
    at /home/joshua/rustc2/library/alloc/src/alloc.rs:366
#7  0x00007ffff554b853 in alloc::raw_vec::RawVec<T,A>::allocate_in ()
    at /home/joshua/rustc2/library/alloc/src/raw_vec.rs:193
#8  0x00007ffff5388c3d in alloc::raw_vec::RawVec<T,A>::with_capacity_zeroed_in ()
    at /home/joshua/rustc2/library/alloc/src/raw_vec.rs:136
#9  <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem ()
    at /home/joshua/rustc2/library/alloc/src/vec/spec_from_elem.rs:39
#10 alloc::vec::from_elem () at /home/joshua/rustc2/library/alloc/src/vec/mod.rs:1963
#11 0x00007ffff52e8ca0 in rustc_middle::mir::interpret::allocation::Allocation<Tag>::uninit ()
    at /home/joshua/rustc2/compiler/rustc_middle/src/mir/interpret/allocation.rs:110
#12 0x00007ffff559cf08 in rustc_mir::interpret::memory::Memory<M>::allocate ()
    at /home/joshua/rustc2/compiler/rustc_mir/src/interpret/memory.rs:197
#13 0x00007ffff5206fd7 in rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::allocate ()
    at /home/joshua/rustc2/compiler/rustc_mir/src/interpret/place.rs:1022
#14 0x00007ffff4edea3f in rustc_mir::const_eval::eval_queries::eval_body_using_ecx ()
    at /home/joshua/rustc2/compiler/rustc_mir/src/const_eval/eval_queries.rs:49
#15 rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider::{{closure}}
    () at /home/joshua/rustc2/compiler/rustc_mir/src/const_eval/eval_queries.rs:305
#16 core::result::Result<T,E>::and_then ()
    at /home/joshua/rustc2/library/core/src/result.rs:704
#17 rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider ()
    at /home/joshua/rustc2/compiler/rustc_mir/src/const_eval/eval_queries.rs:305
#18 0x00007ffff4edacfb in rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_allocation_raw>::compute ()
    at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:377
#19 0x00007ffff4fac027 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl ()
--Type <RET> for more, q to quit, c to continue without paging--c
    at /home/joshua/rustc2/compiler/rustc_query_system/src/dep_graph/graph.rs:362
#20 0x00007ffff4fb209f in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task () at /home/joshua/rustc2/compiler/rustc_query_system/src/dep_graph/graph.rs:245
#21 0x00007ffff5038e63 in rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:607
#22 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:73
#23 stacker::maybe_grow () at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55
#24 rustc_data_structures::stack::ensure_sufficient_stack () at /home/joshua/rustc2/compiler/rustc_data_structures/src/stack.rs:16
#25 0x00007ffff500b4a2 in rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:73
#26 rustc_middle::ty::context::tls::enter_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1712
#27 rustc_middle::ty::context::tls::set_tlv () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1696
#28 rustc_middle::ty::context::tls::enter_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1712
#29 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:72
#30 rustc_middle::ty::context::tls::with_related_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1756
#31 rustc_middle::ty::context::tls::with_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1740
#32 rustc_middle::ty::context::tls::with_context_opt () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1729
#33 rustc_middle::ty::context::tls::with_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1740
#34 rustc_middle::ty::context::tls::with_related_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1753
#35 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:61
#36 rustc_query_system::query::plumbing::force_query_with_job::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:597
#37 rustc_query_system::query::plumbing::with_diagnostics () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:302
#38 rustc_query_system::query::plumbing::force_query_with_job () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:596
#39 0x00007ffff4fc8046 in rustc_query_system::query::plumbing::try_execute_query () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:426
#40 rustc_query_system::query::plumbing::get_query_impl::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:644
#41 <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/caches.rs:114
#42 rustc_query_system::query::plumbing::try_get_cached () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:379
#43 rustc_query_system::query::plumbing::get_query_impl () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:636
#44 0x00007ffff4edd0a5 in rustc_query_system::query::plumbing::get_query () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:738
#45 rustc_middle::ty::query::TyCtxtAt::eval_to_allocation_raw () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:487
#46 rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::eval_to_allocation_raw () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:448
#47 rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider () at /home/joshua/rustc2/compiler/rustc_mir/src/const_eval/eval_queries.rs:238
#48 0x00007ffff4edadfb in rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_const_value_raw>::compute () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:377
#49 0x00007ffff4fa4452 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl () at /home/joshua/rustc2/compiler/rustc_query_system/src/dep_graph/graph.rs:362
#50 0x00007ffff4fb188f in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task () at /home/joshua/rustc2/compiler/rustc_query_system/src/dep_graph/graph.rs:245
#51 0x00007ffff5037023 in rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:607
#52 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:73
#53 stacker::maybe_grow () at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55
#54 rustc_data_structures::stack::ensure_sufficient_stack () at /home/joshua/rustc2/compiler/rustc_data_structures/src/stack.rs:16
#55 0x00007ffff5004218 in rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:73
#56 rustc_middle::ty::context::tls::enter_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1712
#57 rustc_middle::ty::context::tls::set_tlv () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1696
#58 rustc_middle::ty::context::tls::enter_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1712
#59 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:72
#60 rustc_middle::ty::context::tls::with_related_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1756
#61 rustc_middle::ty::context::tls::with_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1740
#62 rustc_middle::ty::context::tls::with_context_opt () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1729
#63 rustc_middle::ty::context::tls::with_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1740
#64 rustc_middle::ty::context::tls::with_related_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1753
#65 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:61
#66 rustc_query_system::query::plumbing::force_query_with_job::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:597
#67 rustc_query_system::query::plumbing::with_diagnostics () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:302
#68 rustc_query_system::query::plumbing::force_query_with_job () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:596
#69 0x00007ffff4fc4e56 in rustc_query_system::query::plumbing::try_execute_query () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:426
#70 rustc_query_system::query::plumbing::get_query_impl::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:644
#71 <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/caches.rs:114
#72 rustc_query_system::query::plumbing::try_get_cached () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:379
#73 rustc_query_system::query::plumbing::get_query_impl () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:636
#74 0x00007ffff4edcdf1 in rustc_query_system::query::plumbing::get_query () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:738
#75 rustc_middle::ty::query::TyCtxtAt::eval_to_const_value_raw () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:487
#76 rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::eval_to_const_value_raw () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:448
#77 rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider () at /home/joshua/rustc2/compiler/rustc_mir/src/const_eval/eval_queries.rs:215
#78 0x00007ffff67cb8ec in rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_const_value_raw>::compute () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:377
#79 0x00007ffff65aa4b5 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl () at /home/joshua/rustc2/compiler/rustc_query_system/src/dep_graph/graph.rs:362
#80 0x00007ffff65d6adf in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task () at /home/joshua/rustc2/compiler/rustc_query_system/src/dep_graph/graph.rs:245
#81 0x00007ffff6895e63 in rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:607
#82 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:73
#83 stacker::maybe_grow () at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55
#84 rustc_data_structures::stack::ensure_sufficient_stack () at /home/joshua/rustc2/compiler/rustc_data_structures/src/stack.rs:16
#85 0x00007ffff699e365 in rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:73
#86 rustc_middle::ty::context::tls::enter_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1712
#87 rustc_middle::ty::context::tls::set_tlv () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1696
#88 rustc_middle::ty::context::tls::enter_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1712
#89 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:72
#90 rustc_middle::ty::context::tls::with_related_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1756
#91 rustc_middle::ty::context::tls::with_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1740
#92 rustc_middle::ty::context::tls::with_context_opt () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1729
#93 rustc_middle::ty::context::tls::with_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1740
#94 rustc_middle::ty::context::tls::with_related_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1753
#95 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:61
#96 rustc_query_system::query::plumbing::force_query_with_job::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:597
#97 rustc_query_system::query::plumbing::with_diagnostics () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:302
#98 rustc_query_system::query::plumbing::force_query_with_job () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:596
#99 0x00007ffff69467c0 in rustc_query_system::query::plumbing::try_execute_query () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:426
#100 rustc_query_system::query::plumbing::get_query_impl::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:644
#101 <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/caches.rs:114
#102 rustc_query_system::query::plumbing::try_get_cached () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:379
#103 rustc_query_system::query::plumbing::get_query_impl () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:636
#104 0x00007ffff67ab9c7 in rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:487
#105 0x00007ffff67ab594 in rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_poly () at /home/joshua/rustc2/compiler/rustc_middle/src/mir/interpret/queries.rs:22
#106 0x00007ffff5e7b6b2 in <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item () at /home/joshua/rustc2/compiler/rustc_lint/src/builtin.rs:1497
#107 0x00007ffff5e94894 in <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item () at /home/joshua/rustc2/compiler/rustc_lint/src/passes.rs:115
#108 0x00007ffff3e53343 in <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:145
#109 rustc_lint::late::LateContextAndPass<T>::with_param_env () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:75
#110 <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}} () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:144
#111 rustc_lint::late::LateContextAndPass<T>::with_lint_attrs () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:63
#112 <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:143
#113 rustc_hir::intravisit::Visitor::visit_nested_item () at /home/joshua/rustc2/compiler/rustc_hir/src/intravisit.rs:273
#114 0x00007ffff3e4513d in rustc_hir::intravisit::walk_mod () at /home/joshua/rustc2/compiler/rustc_hir/src/intravisit.rs:500
#115 0x00007ffff3e52e2c in rustc_lint::late::LateContextAndPass<T>::process_mod () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:81
#116 <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_mod () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:251
#117 0x00007ffff3e414ef in rustc_hir::intravisit::walk_crate () at /home/joshua/rustc2/compiler/rustc_hir/src/intravisit.rs:486
#118 0x00007ffff3e50a2f in rustc_lint::late::late_lint_pass_crate::{{closure}} () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:444
#119 rustc_lint::late::LateContextAndPass<T>::with_lint_attrs () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:63
#120 rustc_lint::late::late_lint_pass_crate () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:439
#121 rustc_lint::late::late_lint_crate () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:458
#122 0x00007ffff3e6fe38 in rustc_lint::late::check_crate::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:488
#123 rustc_data_structures::profiling::VerboseTimingGuard::run () at /home/joshua/rustc2/compiler/rustc_data_structures/src/profiling.rs:570
#124 rustc_session::utils::<impl rustc_session::session::Session>::time () at /home/joshua/rustc2/compiler/rustc_session/src/utils.rs:9
#125 0x00007ffff3e6a8c2 in rustc_lint::late::check_crate::{{closure}} () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:486
#126 rustc_data_structures::sync::join () at /home/joshua/rustc2/compiler/rustc_data_structures/src/sync.rs:159
#127 0x00007ffff3e72b20 in rustc_lint::late::check_crate () at /home/joshua/rustc2/compiler/rustc_lint/src/late.rs:484
#128 rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_interface/src/passes.rs:912
#129 rustc_data_structures::profiling::VerboseTimingGuard::run () at /home/joshua/rustc2/compiler/rustc_data_structures/src/profiling.rs:570
#130 rustc_session::utils::<impl rustc_session::session::Session>::time () at /home/joshua/rustc2/compiler/rustc_session/src/utils.rs:9
#131 0x00007ffff3de0c48 in rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_interface/src/passes.rs:911
#132 core::ops::function::FnOnce::call_once () at /home/joshua/rustc2/library/core/src/ops/function.rs:227
#133 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () at /home/joshua/rustc2/library/std/src/panic.rs:322
#134 std::panicking::try::do_call () at /home/joshua/rustc2/library/std/src/panicking.rs:379
#135 std::panicking::try () at /home/joshua/rustc2/library/std/src/panicking.rs:343
#136 0x00007ffff3ebe4bd in std::panic::catch_unwind () at /home/joshua/rustc2/library/std/src/panic.rs:396
#137 rustc_interface::passes::analysis::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_interface/src/passes.rs:898
#138 core::ops::function::FnOnce::call_once () at /home/joshua/rustc2/library/core/src/ops/function.rs:227
#139 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () at /home/joshua/rustc2/library/std/src/panic.rs:322
#140 0x00007ffff3de0c16 in std::panicking::try::do_call () at /home/joshua/rustc2/library/std/src/panicking.rs:379
#141 std::panicking::try () at /home/joshua/rustc2/library/std/src/panicking.rs:343
#142 0x00007ffff3e719ba in std::panic::catch_unwind () at /home/joshua/rustc2/library/std/src/panic.rs:396
#143 rustc_interface::passes::analysis::{{closure}} () at /home/joshua/rustc2/compiler/rustc_interface/src/passes.rs:894
#144 rustc_data_structures::profiling::VerboseTimingGuard::run () at /home/joshua/rustc2/compiler/rustc_data_structures/src/profiling.rs:570
#145 rustc_session::utils::<impl rustc_session::session::Session>::time () at /home/joshua/rustc2/compiler/rustc_session/src/utils.rs:9
#146 0x00007ffff3e5a0be in rustc_interface::passes::analysis () at /home/joshua/rustc2/compiler/rustc_interface/src/passes.rs:893
#147 0x00007ffff3b6d232 in rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:377
#148 0x00007ffff3b75833 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl () at /home/joshua/rustc2/compiler/rustc_query_system/src/dep_graph/graph.rs:362
#149 0x00007ffff3b77088 in rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task () at /home/joshua/rustc2/compiler/rustc_query_system/src/dep_graph/graph.rs:418
#150 0x00007ffff3b94222 in rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:599
#151 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:73
#152 stacker::maybe_grow () at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55
#153 rustc_data_structures::stack::ensure_sufficient_stack () at /home/joshua/rustc2/compiler/rustc_data_structures/src/stack.rs:16
#154 0x00007ffff3b8f5cd in rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:73
#155 rustc_middle::ty::context::tls::enter_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1712
#156 rustc_middle::ty::context::tls::set_tlv () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1696
#157 rustc_middle::ty::context::tls::enter_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1712
#158 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:72
#159 rustc_middle::ty::context::tls::with_related_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1756
#160 rustc_middle::ty::context::tls::with_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1740
#161 rustc_middle::ty::context::tls::with_context_opt () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1729
#162 rustc_middle::ty::context::tls::with_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1740
#163 rustc_middle::ty::context::tls::with_related_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1753
#164 rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:61
#165 rustc_query_system::query::plumbing::force_query_with_job::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:597
#166 rustc_query_system::query::plumbing::with_diagnostics () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:302
#167 rustc_query_system::query::plumbing::force_query_with_job () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:596
#168 0x00007ffff3b8c955 in rustc_query_system::query::plumbing::try_execute_query () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:426
#169 rustc_query_system::query::plumbing::get_query_impl::{{closure}} () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:644
#170 <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/caches.rs:114
#171 rustc_query_system::query::plumbing::try_get_cached () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:379
#172 rustc_query_system::query::plumbing::get_query_impl () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:636
#173 0x00007ffff3b6d8a7 in rustc_query_system::query::plumbing::get_query () at /home/joshua/rustc2/compiler/rustc_query_system/src/query/plumbing.rs:738
#174 rustc_middle::ty::query::TyCtxtAt::analysis () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:487
#175 rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::analysis () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/query/plumbing.rs:448
#176 rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_driver/src/lib.rs:440
#177 rustc_interface::passes::QueryContext::enter::{{closure}} () at /home/joshua/rustc2/compiler/rustc_interface/src/passes.rs:742
#178 rustc_middle::ty::context::tls::enter_context::{{closure}} () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1712
#179 rustc_middle::ty::context::tls::set_tlv () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1696
#180 rustc_middle::ty::context::tls::enter_context () at /home/joshua/rustc2/compiler/rustc_middle/src/ty/context.rs:1712
#181 rustc_interface::passes::QueryContext::enter () at /home/joshua/rustc2/compiler/rustc_interface/src/passes.rs:742
#182 0x00007ffff3b48413 in rustc_driver::run_compiler::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_driver/src/lib.rs:440
#183 rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter () at /home/joshua/rustc2/compiler/rustc_interface/src/queries.rs:418
#184 0x00007ffff3b7d000 in rustc_driver::run_compiler::{{closure}} () at /home/joshua/rustc2/compiler/rustc_driver/src/lib.rs:341
#185 rustc_interface::interface::create_compiler_and_run::{{closure}} () at /home/joshua/rustc2/compiler/rustc_interface/src/interface.rs:197
#186 rustc_span::with_source_map () at /home/joshua/rustc2/compiler/rustc_span/src/lib.rs:787
#187 0x00007ffff3b49523 in rustc_interface::interface::create_compiler_and_run () at /home/joshua/rustc2/compiler/rustc_interface/src/interface.rs:191
#188 0x00007ffff3b7206b in rustc_interface::interface::run_compiler::{{closure}} () at /home/joshua/rustc2/compiler/rustc_interface/src/interface.rs:213
#189 rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}} () at /home/joshua/rustc2/compiler/rustc_interface/src/util.rs:152
#190 scoped_tls::ScopedKey<T>::set () at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
#191 0x00007ffff3b7d773 in rustc_span::with_session_globals () at /home/joshua/rustc2/compiler/rustc_span/src/lib.rs:103
#192 0x00007ffff3b72f20 in rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}} () at /home/joshua/rustc2/compiler/rustc_interface/src/util.rs:150
#193 rustc_interface::util::scoped_thread::{{closure}} () at /home/joshua/rustc2/compiler/rustc_interface/src/util.rs:125
#194 std::sys_common::backtrace::__rust_begin_short_backtrace () at /home/joshua/rustc2/library/std/src/sys_common/backtrace.rs:125
#195 0x00007ffff3b9a1c6 in std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}} () at /home/joshua/rustc2/library/std/src/thread/mod.rs:474
#196 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () at /home/joshua/rustc2/library/std/src/panic.rs:322
#197 std::panicking::try::do_call () at /home/joshua/rustc2/library/std/src/panicking.rs:379
#198 std::panicking::try () at /home/joshua/rustc2/library/std/src/panicking.rs:343
#199 0x00007ffff3bca18a in std::panic::catch_unwind () at /home/joshua/rustc2/library/std/src/panic.rs:396
#200 std::thread::Builder::spawn_unchecked::{{closure}} () at /home/joshua/rustc2/library/std/src/thread/mod.rs:473
#201 core::ops::function::FnOnce::call_once{{vtable-shim}} () at /home/joshua/rustc2/library/core/src/ops/function.rs:227
#202 0x00007ffff3473368 in <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once () at /home/joshua/rustc2/library/alloc/src/boxed.rs:1484
#203 <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once () at /home/joshua/rustc2/library/alloc/src/boxed.rs:1484
#204 0x00007ffff344e72a in std::sys::unix::thread::Thread::new::thread_start () at /home/joshua/rustc2/library/std/src/sys/unix/thread.rs:71
#205 0x00007fffee796fa3 in start_thread (arg=<optimized out>) at pthread_create.c:486
#206 0x00007ffff33104cf in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
