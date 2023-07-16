plain
 finished in 0.561 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 157 tests
FFFFFFFFFFF...F...F...F.F..........FFFFFFFFFFFFFF..FFFFFFFFFFFFFFFFFFFF.FFFF.FFFFFFFFFFF 88/157
FFFFFFFFFFF.FFFFF.FFFFFFFFFFFFFF.FF.FFFF.FFFFFFFFFFFFFFFFFFFFFFFFFF.F

---- [incremental] src/test/incremental/async-lifetimes.rs stdout ----


error in revision `rpass2`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/async-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/async-lifetimes/async-lifetimes.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/async-lifetimes/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/async-lifetimes/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_middle/src/ty/query.rs:130:5
stack backtrace:
   0:     0x7faf8544212c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   1:     0x7faf854aaca8 - core::fmt::write::h66d344d627acacfa
   2:     0x7faf85432901 - std::io::Write::write_fmt::hb82622afec1d37f6
   3:     0x7faf8544511e - std::panicking::default_hook::{{closure}}::h5f74af02800c5fab
   4:     0x7faf85444de7 - std::panicking::default_hook::h5a871b697f3d2d92
   5:     0x7faf85dc2fa4 - rustc_driver[a33928266507259a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7faf854458d1 - std::panicking::rust_panic_with_hook::h0fb060d1c9b166af
   7:     0x7faf854456b9 - std::panicking::begin_panic_handler::{{closure}}::hf55cdf2d233786e7
   8:     0x7faf854426a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h45d7ed344138a257
   9:     0x7faf854453c2 - rust_begin_unwind
  10:     0x7faf853f5e43 - core::panicking::panic_fmt::hfd8aedf5b9da29a1
  11:     0x7faf853f5d0d - core::panicking::panic::hb35f88cac0b12d00
  12:     0x7faf86c198e6 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::ArenaCache<rustc_span[16f708bed991f38b]::def_id::DefId, rustc_middle[b8568a7b9c9324a7]::middle::codegen_fn_attrs::CodegenFnAttrs>>
  13:     0x7faf86c2e6c9 - <rustc_passes[fed4cc23c6721870]::check_attr::CheckAttrVisitor>::check_attributes
  14:     0x7faf86c30937 - <rustc_passes[fed4cc23c6721870]::check_attr::CheckAttrVisitor as rustc_hir[2d670f5e51690485]::intravisit::Visitor>::visit_item
  15:     0x7faf86c8dde0 - <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::visit_item_likes_in_module::<rustc_passes[fed4cc23c6721870]::check_attr::CheckAttrVisitor>
  16:     0x7faf86c30d7d - rustc_passes[fed4cc23c6721870]::check_attr::check_mod_attrs
  17:     0x7faf874bb61d - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>::{closure#0}, ()>
  18:     0x7faf8766012d - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>
  19:     0x7faf878cd106 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>>
  20:     0x7faf879a1f46 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::check_mod_attrs, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  21:     0x7faf87822fc1 - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::check_mod_attrs
  22:     0x7faf85f5b1c9 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>>
  23:     0x7faf85f4ec3d - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a7fa772e30dbdc6e]::sync::par_for_each_in<&[rustc_span[16f708bed991f38b]::def_id::LocalDefId], <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::par_for_each_module<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  24:     0x7faf85ec1e3b - rustc_data_structures[a7fa772e30dbdc6e]::sync::par_for_each_in::<&[rustc_span[16f708bed991f38b]::def_id::LocalDefId], <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::par_for_each_module<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  25:     0x7faf85f4ee52 - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  26:     0x7faf85ec0856 - std[2ad72f44f058fdd3]::panic::catch_unwind::<core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}>, ()>
  27:     0x7faf85ee7642 - rustc_interface[c397efb26ff26c9d]::passes::analysis
  28:     0x7faf874c26c4 - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  29:     0x7faf8769f8dd - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  30:     0x7faf8790d447 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  31:     0x7faf879efc94 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::analysis, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  32:     0x7faf8780bdad - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::analysis
  33:     0x7faf85e27075 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  34:     0x7faf85e2e507 - <rustc_interface[c397efb26ff26c9d]::passes::QueryContext>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  35:     0x7faf85dcb291 - <rustc_interface[c397efb26ff26c9d]::interface::Compiler>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}, core[3334471bab16b0e9]::result::Result<core[3334471bab16b0e9]::option::Option<rustc_interface[c397efb26ff26c9d]::queries::Linker>, rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  36:     0x7faf85dc4561 - rustc_span[16f708bed991f38b]::with_source_map::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#1}>
  37:     0x7faf85de6861 - rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>
  38:     0x7faf85daeef2 - <scoped_tls[838c36787839aa63]::ScopedKey<rustc_span[16f708bed991f38b]::SessionGlobals>>::set::<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  39:     0x7faf85e2630f - std[2ad72f44f058fdd3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  40:     0x7faf85daf4de - std[2ad72f44f058fdd3]::panicking::try::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7faf85e29730 - <<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1} as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7faf85452645 - std::sys::unix::thread::Thread::new::thread_start::h8714a0d2d89391e6
  43:     0x7faf851eeb43 - <unknown>
  44:     0x7faf85280a00 - <unknown>
  45:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (77fe4bc1d 2022-08-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_mod_attrs] checking attributes in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_middle/src/ty/query.rs:130:5
stack backtrace:
   0:     0x7faf8544212c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   1:     0x7faf854aaca8 - core::fmt::write::h66d344d627acacfa
   2:     0x7faf85432901 - std::io::Write::write_fmt::hb82622afec1d37f6
   3:     0x7faf8544511e - std::panicking::default_hook::{{closure}}::h5f74af02800c5fab
   4:     0x7faf85444de7 - std::panicking::default_hook::h5a871b697f3d2d92
   5:     0x7faf85dc2fa4 - rustc_driver[a33928266507259a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7faf854458d1 - std::panicking::rust_panic_with_hook::h0fb060d1c9b166af
   7:     0x7faf854456b9 - std::panicking::begin_panic_handler::{{closure}}::hf55cdf2d233786e7
   8:     0x7faf854426a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h45d7ed344138a257
   9:     0x7faf854453c2 - rust_begin_unwind
  10:     0x7faf853f5e43 - core::panicking::panic_fmt::hfd8aedf5b9da29a1
  11:     0x7faf853f5d0d - core::panicking::panic::hb35f88cac0b12d00
  12:     0x7faf85f5c0a8 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), rustc_session[938d711f099bcbe5]::session::Limits>>
  13:     0x7faf85f4edfa - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#3}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  14:     0x7faf85ec0836 - std[2ad72f44f058fdd3]::panic::catch_unwind::<core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#3}>, ()>
  15:     0x7faf85ee7713 - rustc_interface[c397efb26ff26c9d]::passes::analysis
  16:     0x7faf874c26c4 - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  17:     0x7faf8769f8dd - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  18:     0x7faf8790d447 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  19:     0x7faf879efc94 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::analysis, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  20:     0x7faf8780bdad - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::analysis
  21:     0x7faf85e27075 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  22:     0x7faf85e2e507 - <rustc_interface[c397efb26ff26c9d]::passes::QueryContext>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  23:     0x7faf85dcb291 - <rustc_interface[c397efb26ff26c9d]::interface::Compiler>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}, core[3334471bab16b0e9]::result::Result<core[3334471bab16b0e9]::option::Option<rustc_interface[c397efb26ff26c9d]::queries::Linker>, rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  24:     0x7faf85dc4561 - rustc_span[16f708bed991f38b]::with_source_map::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7faf85de6861 - rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>
  26:     0x7faf85daeef2 - <scoped_tls[838c36787839aa63]::ScopedKey<rustc_span[16f708bed991f38b]::SessionGlobals>>::set::<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  27:     0x7faf85e2630f - std[2ad72f44f058fdd3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  28:     0x7faf85daf4de - std[2ad72f44f058fdd3]::panicking::try::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x7faf85e29730 - <<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1} as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7faf85452645 - std::sys::unix::thread::Thread::new::thread_start::h8714a0d2d89391e6
  31:     0x7faf851eeb43 - <unknown>
  32:     0x7faf85280a00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (77fe4bc1d 2022-08-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
------------------------------------------


---- [incremental] src/test/incremental/change_symbol_export_status.rs stdout ----
---- [incremental] src/test/incremental/change_symbol_export_status.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_middle/src/ty/query.rs:130:5
   0:     0x7f07e392312c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   0:     0x7f07e392312c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   1:     0x7f07e398bca8 - core::fmt::write::h66d344d627acacfa
   2:     0x7f07e3913901 - std::io::Write::write_fmt::hb82622afec1d37f6
   3:     0x7f07e392611e - std::panicking::default_hook::{{closure}}::h5f74af02800c5fab
   4:     0x7f07e3925de7 - std::panicking::default_hook::h5a871b697f3d2d92
   5:     0x7f07e42a3fa4 - rustc_driver[a33928266507259a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f07e39268d1 - std::panicking::rust_panic_with_hook::h0fb060d1c9b166af
   7:     0x7f07e39266b9 - std::panicking::begin_panic_handler::{{closure}}::hf55cdf2d233786e7
   8:     0x7f07e39236a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h45d7ed344138a257
   9:     0x7f07e39263c2 - rust_begin_unwind
  10:     0x7f07e38d6e43 - core::panicking::panic_fmt::hfd8aedf5b9da29a1
  11:     0x7f07e38d6d0d - core::panicking::panic::hb35f88cac0b12d00
  12:     0x7f07e443ca9a - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::option::Option<rustc_span[16f708bed991f38b]::def_id::LocalDefId>>>
  13:     0x7f07e43ba865 - <rustc_session[938d711f099bcbe5]::session::Session>::time::<(), rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#0}::{closure#1}>
  14:     0x7f07e442fd84 - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#0}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  15:     0x7f07e43a17dc - std[2ad72f44f058fdd3]::panic::catch_unwind::<core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#0}>, ()>
  16:     0x7f07e43c8618 - rustc_interface[c397efb26ff26c9d]::passes::analysis
  17:     0x7f07e59a36c4 - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  18:     0x7f07e5b808dd - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  19:     0x7f07e5dee447 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  20:     0x7f07e5ed0c94 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::analysis, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  21:     0x7f07e5cecdad - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::analysis
  22:     0x7f07e4308075 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  23:     0x7f07e430f507 - <rustc_interface[c397efb26ff26c9d]::passes::QueryContext>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  24:     0x7f07e42ac291 - <rustc_interface[c397efb26ff26c9d]::interface::Compiler>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}, core[3334471bab16b0e9]::result::Result<core[3334471bab16b0e9]::option::Option<rustc_interface[c397efb26ff26c9d]::queries::Linker>, rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  25:     0x7f07e42a5561 - rustc_span[16f708bed991f38b]::with_source_map::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#1}>
  26:     0x7f07e42c7861 - rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>
  27:     0x7f07e428fef2 - <scoped_tls[838c36787839aa63]::ScopedKey<rustc_span[16f708bed991f38b]::SessionGlobals>>::set::<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  28:     0x7f07e430730f - std[2ad72f44f058fdd3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  29:     0x7f07e42904de - std[2ad72f44f058fdd3]::panicking::try::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  30:     0x7f07e430a730 - <<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1} as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7f07e3933645 - std::sys::unix::thread::Thread::new::thread_start::h8714a0d2d89391e6
  32:     0x7f07e36cfb43 - <unknown>
  33:     0x7f07e3761a00 - <unknown>
  34:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (77fe4bc1d 2022-08-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_middle/src/ty/query.rs:130:5
   0:     0x7f07e392312c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   0:     0x7f07e392312c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   1:     0x7f07e398bca8 - core::fmt::write::h66d344d627acacfa
   2:     0x7f07e3913901 - std::io::Write::write_fmt::hb82622afec1d37f6
   3:     0x7f07e392611e - std::panicking::default_hook::{{closure}}::h5f74af02800c5fab
   4:     0x7f07e3925de7 - std::panicking::default_hook::h5a871b697f3d2d92
   5:     0x7f07e42a3fa4 - rustc_driver[a33928266507259a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f07e39268d1 - std::panicking::rust_panic_with_hook::h0fb060d1c9b166af
   7:     0x7f07e39266b9 - std::panicking::begin_panic_handler::{{closure}}::hf55cdf2d233786e7
   8:     0x7f07e39236a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h45d7ed344138a257
   9:     0x7f07e39263c2 - rust_begin_unwind
  10:     0x7f07e38d6e43 - core::panicking::panic_fmt::hfd8aedf5b9da29a1
  11:     0x7f07e38d6d0d - core::panicking::panic::hb35f88cac0b12d00
  12:     0x7f07e443c1e6 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>>
  13:     0x7f07e442fc1c - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a7fa772e30dbdc6e]::sync::par_for_each_in<&[rustc_span[16f708bed991f38b]::def_id::LocalDefId], <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::par_for_each_module<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  14:     0x7f07e43a2e3b - rustc_data_structures[a7fa772e30dbdc6e]::sync::par_for_each_in::<&[rustc_span[16f708bed991f38b]::def_id::LocalDefId], <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::par_for_each_module<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  15:     0x7f07e442fe52 - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  16:     0x7f07e43a1856 - std[2ad72f44f058fdd3]::panic::catch_unwind::<core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}>, ()>
  17:     0x7f07e43c8642 - rustc_interface[c397efb26ff26c9d]::passes::analysis
  18:     0x7f07e59a36c4 - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  19:     0x7f07e5b808dd - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  20:     0x7f07e5dee447 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  21:     0x7f07e5ed0c94 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::analysis, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  22:     0x7f07e5cecdad - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::analysis
  23:     0x7f07e4308075 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  24:     0x7f07e430f507 - <rustc_interface[c397efb26ff26c9d]::passes::QueryContext>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  25:     0x7f07e42ac291 - <rustc_interface[c397efb26ff26c9d]::interface::Compiler>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}, core[3334471bab16b0e9]::result::Result<core[3334471bab16b0e9]::option::Option<rustc_interface[c397efb26ff26c9d]::queries::Linker>, rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  26:     0x7f07e42a5561 - rustc_span[16f708bed991f38b]::with_source_map::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#1}>
  27:     0x7f07e42c7861 - rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>
  28:     0x7f07e428fef2 - <scoped_tls[838c36787839aa63]::ScopedKey<rustc_span[16f708bed991f38b]::SessionGlobals>>::set::<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  29:     0x7f07e430730f - std[2ad72f44f058fdd3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  30:     0x7f07e42904de - std[2ad72f44f058fdd3]::panicking::try::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  31:     0x7f07e430a730 - <<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1} as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f07e3933645 - std::sys::unix::thread::Thread::new::thread_start::h8714a0d2d89391e6
  33:     0x7f07e36cfb43 - <unknown>
  34:     0x7f07e3761a00 - <unknown>
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (77fe4bc1d 2022-08-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_middle/src/ty/query.rs:130:5
   0:     0x7f07e392312c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   0:     0x7f07e392312c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   1:     0x7f07e398bca8 - core::fmt::write::h66d344d627acacfa
   2:     0x7f07e3913901 - std::io::Write::write_fmt::hb82622afec1d37f6
   3:     0x7f07e392611e - std::panicking::default_hook::{{closure}}::h5f74af02800c5fab
   4:     0x7f07e3925de7 - std::panicking::default_hook::h5a871b697f3d2d92
   5:     0x7f07e42a3fa4 - rustc_driver[a33928266507259a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f07e39268d1 - std::panicking::rust_panic_with_hook::h0fb060d1c9b166af
   7:     0x7f07e39266b9 - std::panicking::begin_panic_handler::{{closure}}::hf55cdf2d233786e7
   8:     0x7f07e39236a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h45d7ed344138a257
   9:     0x7f07e39263c2 - rust_begin_unwind
  10:     0x7f07e38d6e43 - core::panicking::panic_fmt::hfd8aedf5b9da29a1
  11:     0x7f07e38d6d0d - core::panicking::panic::hb35f88cac0b12d00
  12:     0x7f07e443c1e6 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>>
  13:     0x7f07e442fc1c - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a7fa772e30dbdc6e]::sync::par_for_each_in<&[rustc_span[16f708bed991f38b]::def_id::LocalDefId], <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::par_for_each_module<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  14:     0x7f07e43a2e3b - rustc_data_structures[a7fa772e30dbdc6e]::sync::par_for_each_in::<&[rustc_span[16f708bed991f38b]::def_id::LocalDefId], <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::par_for_each_module<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  15:     0x7f07e442fe52 - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  16:     0x7f07e43a1856 - std[2ad72f44f058fdd3]::panic::catch_unwind::<core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}>, ()>
  17:     0x7f07e43c8642 - rustc_interface[c397efb26ff26c9d]::passes::analysis
  18:     0x7f07e59a36c4 - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  19:     0x7f07e5b808dd - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  20:     0x7f07e5dee447 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  21:     0x7f07e5ed0c94 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::analysis, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  22:     0x7f07e5cecdad - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::analysis
  23:     0x7f07e4308075 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  24:     0x7f07e430f507 - <rustc_interface[c397efb26ff26c9d]::passes::QueryContext>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  25:     0x7f07e42ac291 - <rustc_interface[c397efb26ff26c9d]::interface::Compiler>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}, core[3334471bab16b0e9]::result::Result<core[3334471bab16b0e9]::option::Option<rustc_interface[c397efb26ff26c9d]::queries::Linker>, rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  26:     0x7f07e42a5561 - rustc_span[16f708bed991f38b]::with_source_map::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#1}>
  27:     0x7f07e42c7861 - rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>
  28:     0x7f07e428fef2 - <scoped_tls[838c36787839aa63]::ScopedKey<rustc_span[16f708bed991f38b]::SessionGlobals>>::set::<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  29:     0x7f07e430730f - std[2ad72f44f058fdd3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  30:     0x7f07e42904de - std[2ad72f44f058fdd3]::panicking::try::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  31:     0x7f07e430a730 - <<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1} as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f07e3933645 - std::sys::unix::thread::Thread::new::thread_start::h8714a0d2d89391e6
  33:     0x7f07e36cfb43 - <unknown>
  34:     0x7f07e3761a00 - <unknown>
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (77fe4bc1d 2022-08-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_middle/src/ty/query.rs:130:5
   0:     0x7f07e392312c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   0:     0x7f07e392312c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   1:     0x7f07e398bca8 - core::fmt::write::h66d344d627acacfa
   2:     0x7f07e3913901 - std::io::Write::write_fmt::hb82622afec1d37f6
   3:     0x7f07e392611e - std::panicking::default_hook::{{closure}}::h5f74af02800c5fab
   4:     0x7f07e3925de7 - std::panicking::default_hook::h5a871b697f3d2d92
   5:     0x7f07e42a3fa4 - rustc_driver[a33928266507259a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f07e39268d1 - std::panicking::rust_panic_with_hook::h0fb060d1c9b166af
   7:     0x7f07e39266b9 - std::panicking::begin_panic_handler::{{closure}}::hf55cdf2d233786e7
   8:     0x7f07e39236a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h45d7ed344138a257
   9:     0x7f07e39263c2 - rust_begin_unwind
  10:     0x7f07e38d6e43 - core::panicking::panic_fmt::hfd8aedf5b9da29a1
  11:     0x7f07e38d6d0d - core::panicking::panic::hb35f88cac0b12d00
  12:     0x7f07e443c1e6 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>>
  13:     0x7f07e442fc1c - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a7fa772e30dbdc6e]::sync::par_for_each_in<&[rustc_span[16f708bed991f38b]::def_id::LocalDefId], <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::par_for_each_module<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  14:     0x7f07e43a2e3b - rustc_data_structures[a7fa772e30dbdc6e]::sync::par_for_each_in::<&[rustc_span[16f708bed991f38b]::def_id::LocalDefId], <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::par_for_each_module<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  15:     0x7f07e442fe52 - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  16:     0x7f07e43a1856 - std[2ad72f44f058fdd3]::panic::catch_unwind::<core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}>, ()>
  17:     0x7f07e43c8642 - rustc_interface[c397efb26ff26c9d]::passes::analysis
  18:     0x7f07e59a36c4 - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  19:     0x7f07e5b808dd - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  20:     0x7f07e5dee447 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  21:     0x7f07e5ed0c94 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::analysis, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  22:     0x7f07e5cecdad - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::analysis
  23:     0x7f07e4308075 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  24:     0x7f07e430f507 - <rustc_interface[c397efb26ff26c9d]::passes::QueryContext>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  25:     0x7f07e42ac291 - <rustc_interface[c397efb26ff26c9d]::interface::Compiler>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}, core[3334471bab16b0e9]::result::Result<core[3334471bab16b0e9]::option::Option<rustc_interface[c397efb26ff26c9d]::queries::Linker>, rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  26:     0x7f07e42a5561 - rustc_span[16f708bed991f38b]::with_source_map::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#1}>
  27:     0x7f07e42c7861 - rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>
  28:     0x7f07e428fef2 - <scoped_tls[838c36787839aa63]::ScopedKey<rustc_span[16f708bed991f38b]::SessionGlobals>>::set::<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  29:     0x7f07e430730f - std[2ad72f44f058fdd3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  30:     0x7f07e42904de - std[2ad72f44f058fdd3]::panicking::try::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  31:     0x7f07e430a730 - <<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1} as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f07e3933645 - std::sys::unix::thread::Thread::new::thread_start::h8714a0d2d89391e6
  33:     0x7f07e36cfb43 - <unknown>
  34:     0x7f07e3761a00 - <unknown>
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (77fe4bc1d 2022-08-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_middle/src/ty/query.rs:130:5
   0:     0x7f07e392312c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   0:     0x7f07e392312c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   1:     0x7f07e398bca8 - core::fmt::write::h66d344d627acacfa
   2:     0x7f07e3913901 - std::io::Write::write_fmt::hb82622afec1d37f6
   3:     0x7f07e392611e - std::panicking::default_hook::{{closure}}::h5f74af02800c5fab
   4:     0x7f07e3925de7 - std::panicking::default_hook::h5a871b697f3d2d92
   5:     0x7f07e42a3fa4 - rustc_driver[a33928266507259a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f07e39268d1 - std::panicking::rust_panic_with_hook::h0fb060d1c9b166af
   7:     0x7f07e39266b9 - std::panicking::begin_panic_handler::{{closure}}::hf55cdf2d233786e7
   8:     0x7f07e39236a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h45d7ed344138a257
   9:     0x7f07e39263c2 - rust_begin_unwind
  10:     0x7f07e38d6e43 - core::panicking::panic_fmt::hfd8aedf5b9da29a1
  11:     0x7f07e38d6d0d - core::panicking::panic::hb35f88cac0b12d00
  12:     0x7f07e443d0a8 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), rustc_session[938d711f099bcbe5]::session::Limits>>
  13:     0x7f07e442fdfa - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#3}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  14:     0x7f07e43a1836 - std[2ad72f44f058fdd3]::panic::catch_unwind::<core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#3}>, ()>
  15:     0x7f07e43c8713 - rustc_interface[c397efb26ff26c9d]::passes::analysis
  16:     0x7f07e59a36c4 - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  17:     0x7f07e5b808dd - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  18:     0x7f07e5dee447 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  19:     0x7f07e5ed0c94 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::analysis, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  20:     0x7f07e5cecdad - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::analysis
  21:     0x7f07e4308075 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  22:     0x7f07e430f507 - <rustc_interface[c397efb26ff26c9d]::passes::QueryContext>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  23:     0x7f07e42ac291 - <rustc_interface[c397efb26ff26c9d]::interface::Compiler>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}, core[3334471bab16b0e9]::result::Result<core[3334471bab16b0e9]::option::Option<rustc_interface[c397efb26ff26c9d]::queries::Linker>, rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  24:     0x7f07e42a5561 - rustc_span[16f708bed991f38b]::with_source_map::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7f07e42c7861 - rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>
  26:     0x7f07e428fef2 - <scoped_tls[838c36787839aa63]::ScopedKey<rustc_span[16f708bed991f38b]::SessionGlobals>>::set::<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  27:     0x7f07e430730f - std[2ad72f44f058fdd3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  28:     0x7f07e42904de - std[2ad72f44f058fdd3]::panicking::try::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x7f07e430a730 - <<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1} as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f07e3933645 - std::sys::unix::thread::Thread::new::thread_start::h8714a0d2d89391e6
  31:     0x7f07e36cfb43 - <unknown>
  32:     0x7f07e3761a00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (77fe4bc1d 2022-08-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
------------------------------------------


---- [incremental] src/test/incremental/change_crate_dep_kind.rs stdout ----
---- [incremental] src/test/incremental/change_crate_dep_kind.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_dep_kind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/change_crate_dep_kind.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Cpanic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_middle/src/ty/query.rs:130:5
stack backtrace:
   0:     0x7f3cd028212c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   1:     0x7f3cd02eaca8 - core::fmt::write::h66d344d627acacfa
   2:     0x7f3cd0272901 - std::io::Write::write_fmt::hb82622afec1d37f6
   3:     0x7f3cd028511e - std::panicking::default_hook::{{closure}}::h5f74af02800c5fab
   4:     0x7f3cd0284de7 - std::panicking::default_hook::h5a871b697f3d2d92
   5:     0x7f3cd0c02fa4 - rustc_driver[a33928266507259a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f3cd02858d1 - std::panicking::rust_panic_with_hook::h0fb060d1c9b166af
   7:     0x7f3cd02856b9 - std::panicking::begin_panic_handler::{{closure}}::hf55cdf2d233786e7
   8:     0x7f3cd02826a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h45d7ed344138a257
   9:     0x7f3cd02853c2 - rust_begin_unwind
  10:     0x7f3cd0235e43 - core::panicking::panic_fmt::hfd8aedf5b9da29a1
  11:     0x7f3cd0235d0d - core::panicking::panic::hb35f88cac0b12d00
  12:     0x7f3cd1a598e6 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::ArenaCache<rustc_span[16f708bed991f38b]::def_id::DefId, rustc_middle[b8568a7b9c9324a7]::middle::codegen_fn_attrs::CodegenFnAttrs>>
  13:     0x7f3cd1a6e6c9 - <rustc_passes[fed4cc23c6721870]::check_attr::CheckAttrVisitor>::check_attributes
  14:     0x7f3cd1a70937 - <rustc_passes[fed4cc23c6721870]::check_attr::CheckAttrVisitor as rustc_hir[2d670f5e51690485]::intravisit::Visitor>::visit_item
  15:     0x7f3cd1acdde0 - <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::visit_item_likes_in_module::<rustc_passes[fed4cc23c6721870]::check_attr::CheckAttrVisitor>
  16:     0x7f3cd1a70d7d - rustc_passes[fed4cc23c6721870]::check_attr::check_mod_attrs
  17:     0x7f3cd22fb61d - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>::{closure#0}, ()>
  18:     0x7f3cd24a012d - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>
  19:     0x7f3cd270d106 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>>
  20:     0x7f3cd27e1f46 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::check_mod_attrs, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  21:     0x7f3cd2662fc1 - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::check_mod_attrs
  22:     0x7f3cd0d9b1c9 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<rustc_span[16f708bed991f38b]::def_id::LocalDefId, ()>>
  23:     0x7f3cd0d8ec3d - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a7fa772e30dbdc6e]::sync::par_for_each_in<&[rustc_span[16f708bed991f38b]::def_id::LocalDefId], <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::par_for_each_module<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  24:     0x7f3cd0d01e3b - rustc_data_structures[a7fa772e30dbdc6e]::sync::par_for_each_in::<&[rustc_span[16f708bed991f38b]::def_id::LocalDefId], <rustc_middle[b8568a7b9c9324a7]::hir::map::Map>::par_for_each_module<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  25:     0x7f3cd0d8ee52 - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  26:     0x7f3cd0d00856 - std[2ad72f44f058fdd3]::panic::catch_unwind::<core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#1}>, ()>
  27:     0x7f3cd0d27642 - rustc_interface[c397efb26ff26c9d]::passes::analysis
  28:     0x7f3cd23026c4 - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  29:     0x7f3cd24df8dd - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  30:     0x7f3cd274d447 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  31:     0x7f3cd282fc94 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::analysis, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  32:     0x7f3cd264bdad - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::analysis
  33:     0x7f3cd0c67075 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  34:     0x7f3cd0c6e507 - <rustc_interface[c397efb26ff26c9d]::passes::QueryContext>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  35:     0x7f3cd0c0b291 - <rustc_interface[c397efb26ff26c9d]::interface::Compiler>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}, core[3334471bab16b0e9]::result::Result<core[3334471bab16b0e9]::option::Option<rustc_interface[c397efb26ff26c9d]::queries::Linker>, rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  36:     0x7f3cd0c04561 - rustc_span[16f708bed991f38b]::with_source_map::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#1}>
  37:     0x7f3cd0c26861 - rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>
  38:     0x7f3cd0beeef2 - <scoped_tls[838c36787839aa63]::ScopedKey<rustc_span[16f708bed991f38b]::SessionGlobals>>::set::<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  39:     0x7f3cd0c6630f - std[2ad72f44f058fdd3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  40:     0x7f3cd0bef4de - std[2ad72f44f058fdd3]::panicking::try::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7f3cd0c69730 - <<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1} as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7f3cd0292645 - std::sys::unix::thread::Thread::new::thread_start::h8714a0d2d89391e6
  43:     0x7f3cd002eb43 - <unknown>
  44:     0x7f3cd00c0a00 - <unknown>
  45:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (77fe4bc1d 2022-08-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -C panic=unwind
query stack during panic:
query stack during panic:
#0 [check_mod_attrs] checking attributes in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_middle/src/ty/query.rs:130:5
stack backtrace:
   0:     0x7f3cd028212c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   1:     0x7f3cd02eaca8 - core::fmt::write::h66d344d627acacfa
   2:     0x7f3cd0272901 - std::io::Write::write_fmt::hb82622afec1d37f6
   3:     0x7f3cd028511e - std::panicking::default_hook::{{closure}}::h5f74af02800c5fab
   4:     0x7f3cd0284de7 - std::panicking::default_hook::h5a871b697f3d2d92
   5:     0x7f3cd0c02fa4 - rustc_driver[a33928266507259a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f3cd02858d1 - std::panicking::rust_panic_with_hook::h0fb060d1c9b166af
   7:     0x7f3cd02856b9 - std::panicking::begin_panic_handler::{{closure}}::hf55cdf2d233786e7
   8:     0x7f3cd02826a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h45d7ed344138a257
   9:     0x7f3cd02853c2 - rust_begin_unwind
  10:     0x7f3cd0235e43 - core::panicking::panic_fmt::hfd8aedf5b9da29a1
  11:     0x7f3cd0235d0d - core::panicking::panic::hb35f88cac0b12d00
  12:     0x7f3cd0d9c0a8 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), rustc_session[938d711f099bcbe5]::session::Limits>>
  13:     0x7f3cd0d8edfa - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#3}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  14:     0x7f3cd0d00836 - std[2ad72f44f058fdd3]::panic::catch_unwind::<core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#3}>, ()>
  15:     0x7f3cd0d27713 - rustc_interface[c397efb26ff26c9d]::passes::analysis
  16:     0x7f3cd23026c4 - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  17:     0x7f3cd24df8dd - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  18:     0x7f3cd274d447 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  19:     0x7f3cd282fc94 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::analysis, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  20:     0x7f3cd264bdad - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::analysis
  21:     0x7f3cd0c67075 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  22:     0x7f3cd0c6e507 - <rustc_interface[c397efb26ff26c9d]::passes::QueryContext>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  23:     0x7f3cd0c0b291 - <rustc_interface[c397efb26ff26c9d]::interface::Compiler>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}, core[3334471bab16b0e9]::result::Result<core[3334471bab16b0e9]::option::Option<rustc_interface[c397efb26ff26c9d]::queries::Linker>, rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  24:     0x7f3cd0c04561 - rustc_span[16f708bed991f38b]::with_source_map::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7f3cd0c26861 - rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>
  26:     0x7f3cd0beeef2 - <scoped_tls[838c36787839aa63]::ScopedKey<rustc_span[16f708bed991f38b]::SessionGlobals>>::set::<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  27:     0x7f3cd0c6630f - std[2ad72f44f058fdd3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  28:     0x7f3cd0bef4de - std[2ad72f44f058fdd3]::panicking::try::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x7f3cd0c69730 - <<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1} as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
---
  hash: c628127c5cafbcfd
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-a842d6fd47d5c6b1.rlib
  name: alloc
  cnum: 5
  hash: 19990ca4079666ba
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-ca31ea02f2bb12b4.rlib
  name: libc
  cnum: 6
  hash: a40c75a57807bd07
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-257e8d8a301ed47a.rlib
  name: unwind
  cnum: 7
  hash: ce3b9c8dae676ace
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c366f5512eee754d.rlib
  name: cfg_if
  cnum: 8
  hash: 40b4b4b251f2bd98
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-b73ecbdf3f87ab5e.rlib
  cnum: 9
  hash: 582d74d6a34c93ce
  reqd: Explicit
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-b6c4bc679ce255e1.rlib
  name: rustc_std_workspace_alloc
  cnum: 10
  hash: db16277c824d35dd
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-d8275cfba3af12c2.rlib
  name: adler
  cnum: 11
  hash: 26d9df0e685cbb3a
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-ddbb2255472ce6b7.rlib
  name: hashbrown
  cnum: 12
  hash: 2b4847d2b76cee73
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-6038fbd57ac26997.rlib
  cnum: 13
  hash: 9b93b89cda213af7
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-81a3a2d8faa634d4.rlib
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-81a3a2d8faa634d4.rlib
  name: rustc_demangle
  cnum: 14
  hash: e33add80611c87a0
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-6961d49bae28f9a7.rlib
  name: addr2line
  cnum: 15
  hash: 3a73602dd03e409a
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-b55e3601a4b2b0f7.rlib
  name: gimli
  cnum: 16
  hash: b218186145ff2ac8
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-06a027d6fba66955.rlib
  name: object
  cnum: 17
  hash: 0a14a33fc0ef805a
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-55fdfa36da332923.rlib
  name: memchr
  cnum: 18
  hash: 25e169b88e2aeb19
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-07ef2bbcd63eace9.rlib
  cnum: 19
  hash: 8d9016919ac751c2
  reqd: Implicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-fe9c1edff4105948.rlib
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-fe9c1edff4105948.rlib

DEBUG rustc_ast_passes::feature_gate gate_feature(feature = "prelude_import", span = no-location (#1)); has? false
INFO rustc_interface::passes 0 parse sess buffered_lints
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(3), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early early context: exit_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(3), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early early context: enter_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(8), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(2), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early early context: exit_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(8), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(2), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early early context: enter_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(10), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, TokenStream([Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }, Alone)])), tokens: None }, tokens: Some(LazyTokenStream(AttrAnnotatedTokenStream([(Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:2 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:2: 5:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:14: 5:15 (#0) }, Bracket, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }), Alone)])), Alone)])), Alone)]))) }), id: AttrId(1), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:15 (#0) }])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(10), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, TokenStream([Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }, Alone)])), tokens: None }, tokens: Some(LazyTokenStream(AttrAnnotatedTokenStream([(Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:2 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:2: 5:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:14: 5:15 (#0) }, Bracket, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }), Alone)])), Alone)])), Alone)]))) }), id: AttrId(1), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:15 (#0) }])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6])) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6])) --- trying to force dependency hir_crate(0-0)
DEBUG rustc_middle::dep_graph try_force_from_dep_node(hir_crate(0-0)) --- trying to force
┐rustc_ast_lowering::item::lower_crate 
├─┐rustc_ast_lowering::with_hir_id_owner owner=NodeId(0)
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─┐rustc_ast_lowering::index::index_hir item=Crate(Mod { spans: ModSpans { inner_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 8:2 (#0), inject_use_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:1 (#0) }, item_ids: [ItemId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, ItemId { def_id: DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std) }, ItemId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }] })
│ │ ├─0ms DEBUG rustc_ast_lowering::index visit_nested_item: ItemId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }
│ │ ├─0ms DEBUG rustc_ast_lowering::index visit_nested_item: ItemId { def_id: DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std) }
│ │ ├─0ms DEBUG rustc_ast_lowering::index visit_nested_item: ItemId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }
├─┘
┘
┘
┐rustc_ast_lowering::item::lower_item item=Item { attrs: ThinVec(Some([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(3), style: Outer, span: no-location (#1) }])), id: NodeId(1), span: no-location (#1), vis: Visibility { kind: Inherited, span: no-location (#1), tokens: None }, ident: #0, kind: Use(UseTree { prefix: Path { span: no-location (#1), segments: [PathSegment { ident: {{root}}#1, id: NodeId(3), args: None }, PathSegment { ident: std#1, id: NodeId(4), args: None }, PathSegment { ident: prelude#1, id: NodeId(5), args: None }, PathSegment { ident: rust_2015#1, id: NodeId(6), args: None }], tokens: None }, kind: Glob, span: no-location (#1) }), tokens: None }
├─┐rustc_ast_lowering::with_hir_id_owner owner=NodeId(1)
│ ├─┐rustc_ast_lowering::item::lower_use_tree tree=UseTree { prefix: Path { span: no-location (#1), segments: [PathSegment { ident: {{root}}#1, id: NodeId(3), args: None }, PathSegment { ident: std#1, id: NodeId(4), args: None }, PathSegment { ident: prelude#1, id: NodeId(5), args: None }, PathSegment { ident: rust_2015#1, id: NodeId(6), args: None }], tokens: None }, kind: Glob, span: no-location (#1) }, prefix=Path { span: no-location (#1), segments: [], tokens: None }, id=NodeId(1), vis_span=no-location (#1), ident=#0, attrs=Some([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(3), style: Outer, span: no-location (#1) }])
│ │ ├─0ms DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: {{root}}#1, id: NodeId(3), args: None })
│ │ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident={{root}}#1 original-id=NodeId(3) new-id=HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 1 }
│ │ ├─0ms DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: std#1, id: NodeId(4), args: None })
│ │ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=std#1 original-id=NodeId(4) new-id=HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 2 }
│ │ ├─0ms DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: prelude#1, id: NodeId(5), args: None })
│ │ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=prelude#1 original-id=NodeId(5) new-id=HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 3 }
│ │ ├─0ms DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: rust_2015#1, id: NodeId(6), args: None })
│ │ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=rust_2015#1 original-id=NodeId(6) new-id=HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 4 }
│ ├─┘
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=328896
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(69) len=328896
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(283) len=328896
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─┐rustc_ast_lowering::index::index_hir item=Item(Item { ident: #0, def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), kind: Use(Path { span: no-location (#1), res: Err, segments: [PathSegment { ident: {{root}}#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 1 }), res: Some(Err), args: None, infer_args: false }, PathSegment { ident: std#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 2 }), res: Some(Def(Mod, DefId(1:0 ~ std[2ad7]))), args: None, infer_args: false }, PathSegment { ident: prelude#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 3 }), res: Some(Def(Mod, DefId(1:69 ~ std[2ad7]::prelude))), args: None, infer_args: false }, PathSegment { ident: rust_2015#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 4 }), res: Some(Def(Mod, DefId(1:283 ~ std[2ad7]::prelude::rust_2015))), args: None, infer_args: false }] }, Glob), span: no-location (#1), vis_span: no-location (#1) })
│ │ ├─┐rustc_ast_lowering::index::visit_item i=Item { ident: #0, def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), kind: Use(Path { span: no-location (#1), res: Err, segments: [PathSegment { ident: {{root}}#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 1 }), res: Some(Err), args: None, infer_args: false }, PathSegment { ident: std#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 2 }), res: Some(Def(Mod, DefId(1:0 ~ std[2ad7]))), args: None, infer_args: false }, PathSegment { ident: prelude#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 3 }), res: Some(Def(Mod, DefId(1:69 ~ std[2ad7]::prelude))), args: None, infer_args: false }, PathSegment { ident: rust_2015#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 4 }), res: Some(Def(Mod, DefId(1:283 ~ std[2ad7]::prelude::rust_2015))), args: None, infer_args: false }] }, Glob), span: no-location (#1), vis_span: no-location (#1) }
│ │ │ ├─┐rustc_ast_lowering::index::insert span=no-location (#1), hir_id=HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 1 }, node=PathSegment(PathSegment { ident: {{root}}#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 1 }), res: Some(Err), args: None, infer_args: false })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=no-location (#1), hir_id=HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 2 }, node=PathSegment(PathSegment { ident: std#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 2 }), res: Some(Def(Mod, DefId(1:0 ~ std[2ad7]))), args: None, infer_args: false })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=no-location (#1), hir_id=HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 3 }, node=PathSegment(PathSegment { ident: prelude#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 3 }), res: Some(Def(Mod, DefId(1:69 ~ std[2ad7]::prelude))), args: None, infer_args: false })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=no-location (#1), hir_id=HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 4 }, node=PathSegment(PathSegment { ident: rust_2015#1, hir_id: Some(HirId { owner: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}), local_id: 4 }), res: Some(Def(Mod, DefId(1:283 ~ std[2ad7]::prelude::rust_2015))), args: None, infer_args: false })
│ │ ├─┘
│ ├─┘
├─┘
┘
┘
┐rustc_ast_lowering::item::lower_item item=Item { attrs: ThinVec(Some([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(8), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(2), style: Outer, span: no-location (#1) }])), id: NodeId(7), span: no-location (#1), vis: Visibility { kind: Inherited, span: no-location (#1), tokens: None }, ident: std#2, kind: ExternCrate(None), tokens: None }
├─┐rustc_ast_lowering::with_hir_id_owner owner=NodeId(7)
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
│ ├─┐rustc_ast_lowering::index::index_hir item=Item(Item { ident: std#2, def_id: DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std), kind: ExternCrate(None), span: no-location (#1), vis_span: no-location (#1) })
│ │ ├─┐rustc_ast_lowering::index::visit_item i=Item { ident: std#2, def_id: DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std), kind: ExternCrate(None), span: no-location (#1), vis_span: no-location (#1) }
│ ├─┘
├─┘
┘
┘
┐rustc_ast_lowering::item::lower_item item=Item { attrs: ThinVec(Some([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(10), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, TokenStream([Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }, Alone)])), tokens: None }, tokens: Some(LazyTokenStream(AttrAnnotatedTokenStream([(Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:2 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:2: 5:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:14: 5:15 (#0) }, Bracket, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0) }), Alone), (Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, AttrAnnotatedTokenStream([(Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }), Alone)])), Alone)])), Alone)]))) }), id: AttrId(1), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:15 (#0) }])), id: NodeId(9), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 8:2 (#0), vis: Visibility { kind: Public, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:4 (#0), tokens: None }, ident: foo#0, kind: Fn(Fn { defaultness: Final, generics: Generics { params: [], where_clause: WhereClause { has_where_token: false, predicates: [], span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:13: 6:13 (#0) }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:11: 6:11 (#0) }, sig: FnSig { header: FnHeader { unsafety: No, asyncness: No, constness: No, ext: None }, decl: FnDecl { inputs: [], output: Default(/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 6:14 (#0)) }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:13 (#0) }, body: Some(Block { stmts: [Stmt { id: NodeId(12), kind: Semi(Expr { id: NodeId(13), kind: Block(Block { stmts: [Stmt { id: NodeId(15), kind: Semi(Expr { id: NodeId(16), kind: Call(Expr { id: NodeId(17), kind: Path(None, Path { span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), segments: [PathSegment { ident: $crate#5, id: NodeId(18), args: None }, PathSegment { ident: io#5, id: NodeId(19), args: None }, PathSegment { ident: _print#5, id: NodeId(20), args: None }], tokens: None }), span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), attrs: ThinVec(None), tokens: None }, [Expr { id: NodeId(21), kind: Call(Expr { id: NodeId(22), kind: Path(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6), segments: [PathSegment { ident: $crate#6, id: NodeId(23), args: None }, PathSegment { ident: fmt#0, id: NodeId(24), args: None }, PathSegment { ident: Arguments#0, id: NodeId(25), args: None }, PathSegment { ident: new_v1#0, id: NodeId(26), args: None }], tokens: None }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6), attrs: ThinVec(None), tokens: None }, [Expr { id: NodeId(27), kind: AddrOf(Ref, Not, Expr { id: NodeId(28), kind: Array([Expr { id: NodeId(29), kind: Lit(Lit { token_lit: Lit { kind: Str, symbol: "\\n", suffix: None }, kind: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4), attrs: ThinVec(None), tokens: None }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4), attrs: ThinVec(None), tokens: None }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4), attrs: ThinVec(None), tokens: None }, Expr { id: NodeId(30), kind: AddrOf(Ref, Not, Expr { id: NodeId(31), kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6), attrs: ThinVec(None), tokens: None }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6), attrs: ThinVec(None), tokens: None }]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6), attrs: ThinVec(None), tokens: None }]), span: /checkout/library/std/src/macros.rs:77:9: 77:59 (#5), attrs: ThinVec(None), tokens: None }), span: /checkout/library/std/src/macros.rs:77:9: 77:60 (#5) }], id: NodeId(14), rules: Default, span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5), tokens: None, could_be_bare_literal: false }, None), span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5), attrs: ThinVec(None), tokens: None }), span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5) }], id: NodeId(11), rules: Default, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0), tokens: None, could_be_bare_literal: false }) }), tokens: None }
├─┐rustc_ast_lowering::with_hir_id_owner owner=NodeId(9)
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), lower_path_segment(segment: PathSegment { ident: $crate#5, id: NodeId(18), args: None })
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=$crate#5 original-id=NodeId(18) new-id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), lower_path_segment(segment: PathSegment { ident: io#5, id: NodeId(19), args: None })
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=io#5 original-id=NodeId(19) new-id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), lower_path_segment(segment: PathSegment { ident: _print#5, id: NodeId(20), args: None })
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=_print#5 original-id=NodeId(20) new-id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6), lower_path_segment(segment: PathSegment { ident: $crate#6, id: NodeId(23), args: None })
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=$crate#6 original-id=NodeId(23) new-id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6), lower_path_segment(segment: PathSegment { ident: fmt#0, id: NodeId(24), args: None })
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=fmt#0 original-id=NodeId(24) new-id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6), lower_path_segment(segment: PathSegment { ident: Arguments#0, id: NodeId(25), args: None })
│ ├─0ms DEBUG rustc_ast_lowering::path expected_lifetimes=1
│ ├─┐rustc_ast_lowering::new_named_lifetime id=NodeId(32), new_id=NodeId(32), span=/checkout/library/std/src/macros.rs:77:28: 77:58 (#0), ident='_#0
│ │ ├─┐rustc_ast_lowering::new_named_lifetime_with_res id=NodeId(32), span=/checkout/library/std/src/macros.rs:77:28: 77:58 (#0), ident='_#0, res=Infer
│ │ │ ├─0ms DEBUG rustc_ast_lowering name=Infer
│ ├─┘
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=Arguments#0 original-id=NodeId(25) new-id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6), lower_path_segment(segment: PathSegment { ident: new_v1#0, id: NodeId(26), args: None })
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=new_v1#0 original-id=NodeId(26) new-id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }
│ ├─┐rustc_ast_lowering::item::lower_generics generics=Generics { params: [], where_clause: WhereClause { has_where_token: false, predicates: [], span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:13: 6:13 (#0) }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:11: 6:11 (#0) }, parent_node_id=NodeId(9), itctx=Universal
│ │ ├─┐rustc_ast_lowering::lower_fn_decl decl=FnDecl { inputs: [], output: Default(/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 6:14 (#0)) }, fn_node_id=Some(NodeId(9)), kind=Fn, make_ret_async=None
│ ├─┘
│ ├─┘
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(4304) len=328896
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(3085) len=328896
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(48903) len=1198928
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(10176) len=1198928
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─┐rustc_ast_lowering::index::index_hir item=Item(Item { ident: foo#0, def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), kind: Fn(FnSig { header: FnHeader { unsafety: Normal, constness: NotConst, asyncness: NotAsync, abi: Rust }, decl: FnDecl { inputs: [], output: DefaultReturn(/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 6:14 (#0)), c_variadic: false, implicit_self: None }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:13 (#0) }, Generics { params: [], predicates: [], has_where_clause_predicates: false, where_clause_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:13: 6:13 (#0), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:11: 6:11 (#0) }, BodyId { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 24 } }), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 8:2 (#0), vis_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:4 (#0) })
│ │ ├─┐rustc_ast_lowering::index::visit_item i=Item { ident: foo#0, def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), kind: Fn(FnSig { header: FnHeader { unsafety: Normal, constness: NotConst, asyncness: NotAsync, abi: Rust }, decl: FnDecl { inputs: [], output: DefaultReturn(/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 6:14 (#0)), c_variadic: false, implicit_self: None }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:13 (#0) }, Generics { params: [], predicates: [], has_where_clause_predicates: false, where_clause_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:13: 6:13 (#0), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:11: 6:11 (#0) }, BodyId { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 24 } }), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 8:2 (#0), vis_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:4 (#0) }
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 24 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 24 }, kind: Block(Block { stmts: [Stmt { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 22 }, kind: Semi(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 21 }, kind: Block(Block { stmts: [Stmt { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), res: Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }), res: Some(Def(Mod, DefId(1:3085 ~ std[2ad7]::io))), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }), res: Some(Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print))), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6), res: Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }), res: Some(Err), args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:9: 77:59 (#5) }), span: /checkout/library/std/src/macros.rs:77:9: 77:60 (#5) }], expr: None, hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 20 }, rules: DefaultBlock, span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5), targeted_by_break: false }, None), span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5) }), span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5) }], expr: None, hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 23 }, rules: DefaultBlock, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0), targeted_by_break: false }, None), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 23 }, node=Block(Block { stmts: [Stmt { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 22 }, kind: Semi(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 21 }, kind: Block(Block { stmts: [Stmt { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), res: Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }), res: Some(Def(Mod, DefId(1:3085 ~ std[2ad7]::io))), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }), res: Some(Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print))), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6), res: Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }), res: Some(Err), args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:9: 77:59 (#5) }), span: /checkout/library/std/src/macros.rs:77:9: 77:60 (#5) }], expr: None, hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 20 }, rules: DefaultBlock, span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5), targeted_by_break: false }, None), span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5) }), span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5) }], expr: None, hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 23 }, rules: DefaultBlock, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0), targeted_by_break: false })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:76:23: 78:6 (#5), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 22 }, node=Stmt(Stmt { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 22 }, kind: Semi(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 21 }, kind: Block(Block { stmts: [Stmt { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), res: Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }), res: Some(Def(Mod, DefId(1:3085 ~ std[2ad7]::io))), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }), res: Some(Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print))), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6), res: Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }), res: Some(Err), args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:9: 77:59 (#5) }), span: /checkout/library/std/src/macros.rs:77:9: 77:60 (#5) }], expr: None, hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 20 }, rules: DefaultBlock, span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5), targeted_by_break: false }, None), span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5) }), span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:76:23: 78:6 (#5), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 21 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 21 }, kind: Block(Block { stmts: [Stmt { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), res: Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }), res: Some(Def(Mod, DefId(1:3085 ~ std[2ad7]::io))), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }), res: Some(Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print))), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6), res: Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }), res: Some(Err), args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:9: 77:59 (#5) }), span: /checkout/library/std/src/macros.rs:77:9: 77:60 (#5) }], expr: None, hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 20 }, rules: DefaultBlock, span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5), targeted_by_break: false }, None), span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:76:23: 78:6 (#5), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 20 }, node=Block(Block { stmts: [Stmt { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), res: Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }), res: Some(Def(Mod, DefId(1:3085 ~ std[2ad7]::io))), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }), res: Some(Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print))), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6), res: Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }), res: Some(Err), args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:9: 77:59 (#5) }), span: /checkout/library/std/src/macros.rs:77:9: 77:60 (#5) }], expr: None, hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 20 }, rules: DefaultBlock, span: /checkout/library/std/src/macros.rs:76:23: 78:6 (#5), targeted_by_break: false })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:9: 77:60 (#5), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 19 }, node=Stmt(Stmt { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), res: Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }), res: Some(Def(Mod, DefId(1:3085 ~ std[2ad7]::io))), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }), res: Some(Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print))), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6), res: Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }), res: Some(Err), args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:9: 77:59 (#5) }), span: /checkout/library/std/src/macros.rs:77:9: 77:60 (#5) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:9: 77:59 (#5), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 18 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), res: Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }), res: Some(Def(Mod, DefId(1:3085 ~ std[2ad7]::io))), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }), res: Some(Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print))), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6), res: Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }), res: Some(Err), args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:9: 77:59 (#5) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:9: 77:27 (#5), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 4 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5), res: Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }), res: Some(Def(Mod, DefId(1:3085 ~ std[2ad7]::io))), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }), res: Some(Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print))), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:9: 77:27 (#5) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:9: 77:27 (#5), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }, node=PathSegment(PathSegment { ident: $crate#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 1 }), res: Some(Err), args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:9: 77:27 (#5), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }, node=PathSegment(PathSegment { ident: io#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 2 }), res: Some(Def(Mod, DefId(1:3085 ~ std[2ad7]::io))), args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:9: 77:27 (#5), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }, node=PathSegment(PathSegment { ident: _print#5, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 3 }), res: Some(Def(Fn, DefId(1:4304 ~ std[2ad7]::io::stdio::_print))), args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:28: 77:58 (#6), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 17 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6), res: Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }), res: Some(Err), args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }, [Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:28: 77:58 (#6), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 11 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6), res: Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }), res: Some(Err), args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:28: 77:28 (#6), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, node=Ty(Ty { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6), res: Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:77:28: 77:28 (#6) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:28: 77:28 (#6), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }, node=PathSegment(PathSegment { ident: $crate#6, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 5 }), res: Some(Err), args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:28: 77:28 (#6), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }, node=PathSegment(PathSegment { ident: fmt#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 6 }), res: Some(Def(Mod, DefId(2:10176 ~ core[3334]::fmt))), args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:28: 77:28 (#6), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }, node=PathSegment(PathSegment { ident: Arguments#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 8 }), res: Some(Def(Struct, DefId(2:48903 ~ core[3334]::fmt::Arguments))), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:77:58: 77:58 (#6) }), infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:28: 77:58 (#0), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, node=Lifetime(Lifetime { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 7 }, span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#0), name: Infer })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:28: 77:58 (#6), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }, node=PathSegment(PathSegment { ident: new_v1#0, hir_id: Some(HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 10 }), res: Some(Err), args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:127:24: 127:28 (#4), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 14 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:127:24: 127:28 (#4), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }]), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:127:24: 127:28 (#4), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) }), span: /checkout/library/std/src/macros.rs:127:24: 127:28 (#4) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:28: 77:58 (#6), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 16 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) }), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:77:28: 77:58 (#6), hir_id=HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, node=Expr(Expr { hir_id: HirId { owner: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo), local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:77:28: 77:58 (#6) })
│ │ ├─┘
│ ├─┘
├─┘
┘
┘
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6])) - END - dependency hir_crate(0-0) was red after forcing
DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6]::{use#0})) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6]::{use#0})) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6]::std)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6]::std)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6]::foo)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner_nodes(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner_nodes(rustc_rust_log_aux[8cb6]::foo)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_module_items(rustc_rust_log_aux[8cb6])) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_module_items(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_module_items(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]::{use#0}) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_module_items(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]::std) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_module_items(rustc_rust_log_aux[8cb6])) - END - dependency hir_owner(rustc_rust_log_aux[8cb6]::foo) was immediately red
DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(entry_fn(0-0)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(entry_fn(0-0)) - END - successfully marked as green
DEBUG rustc_query_system::query::plumbing BEGIN verify_ich(entry_fn(0-0))
DEBUG rustc_query_system::query::plumbing END verify_ich(entry_fn(0-0))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- found dependency hir_crate_items(0-0) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- state of dependency hir_attrs(rustc_rust_log_aux[8cb6]::{use#0}) (8cb6324e8d20b84d-bf87887a1d96ec29) is unknown, trying to mark it green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::{use#0})) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::{use#0})) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- trying to force dependency hir_attrs(rustc_rust_log_aux[8cb6]::{use#0})
DEBUG rustc_middle::dep_graph try_force_from_dep_node(hir_attrs(rustc_rust_log_aux[8cb6]::{use#0})) --- trying to force
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::{use#0})) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::{use#0})) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- managed to FORCE dependency hir_attrs(rustc_rust_log_aux[8cb6]::{use#0}) to green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- state of dependency hir_attrs(rustc_rust_log_aux[8cb6]::std) (8cb6324e8d20b84d-a4ebeafe30299a14) is unknown, trying to mark it green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::std)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::std)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- trying to force dependency hir_attrs(rustc_rust_log_aux[8cb6]::std)
DEBUG rustc_middle::dep_graph try_force_from_dep_node(hir_attrs(rustc_rust_log_aux[8cb6]::std)) --- trying to force
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::std)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::std)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- managed to FORCE dependency hir_attrs(rustc_rust_log_aux[8cb6]::std) to green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- state of dependency hir_attrs(rustc_rust_log_aux[8cb6]::foo) (8cb6324e8d20b84d-7ac63b91867badbd) is unknown, trying to mark it green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::foo)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- trying to force dependency hir_attrs(rustc_rust_log_aux[8cb6]::foo)
DEBUG rustc_middle::dep_graph try_force_from_dep_node(hir_attrs(rustc_rust_log_aux[8cb6]::foo)) --- trying to force
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::foo)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- managed to FORCE dependency hir_attrs(rustc_rust_log_aux[8cb6]::foo) to green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) - END - successfully marked as green
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_middle/src/ty/query.rs:130:5
   0:     0x7f2c8b41512c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   0:     0x7f2c8b41512c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde0dd739297a4170
   1:     0x7f2c8b47dca8 - core::fmt::write::h66d344d627acacfa
   2:     0x7f2c8b405901 - std::io::Write::write_fmt::hb82622afec1d37f6
   3:     0x7f2c8b41811e - std::panicking::default_hook::{{closure}}::h5f74af02800c5fab
   4:     0x7f2c8b417de7 - std::panicking::default_hook::h5a871b697f3d2d92
   5:     0x7f2c8bd95fa4 - rustc_driver[a33928266507259a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2c8b4188d1 - std::panicking::rust_panic_with_hook::h0fb060d1c9b166af
   7:     0x7f2c8b4186b9 - std::panicking::begin_panic_handler::{{closure}}::hf55cdf2d233786e7
   8:     0x7f2c8b4156a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h45d7ed344138a257
   9:     0x7f2c8b4183c2 - rust_begin_unwind
  10:     0x7f2c8b3c8e43 - core::panicking::panic_fmt::hfd8aedf5b9da29a1
  11:     0x7f2c8b3c8d0d - core::panicking::panic::hb35f88cac0b12d00
  12:     0x7f2c8bf2ea9a - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::option::Option<rustc_span[16f708bed991f38b]::def_id::LocalDefId>>>
  13:     0x7f2c8beac865 - <rustc_session[938d711f099bcbe5]::session::Session>::time::<(), rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#0}::{closure#1}>
  14:     0x7f2c8bf21d84 - <core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#0}> as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once
  15:     0x7f2c8be937dc - std[2ad72f44f058fdd3]::panic::catch_unwind::<core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c397efb26ff26c9d]::passes::analysis::{closure#0}::{closure#0}>, ()>
  16:     0x7f2c8beba618 - rustc_interface[c397efb26ff26c9d]::passes::analysis
  17:     0x7f2c8d4956c4 - <rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind as rustc_query_system[e31828d683db36a2]::dep_graph::DepKind>::with_deps::<<rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  18:     0x7f2c8d6728dd - <rustc_query_system[e31828d683db36a2]::dep_graph::graph::DepGraph<rustc_middle[b8568a7b9c9324a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b8568a7b9c9324a7]::ty::context::TyCtxt, (), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  19:     0x7f2c8d8e0447 - rustc_query_system[e31828d683db36a2]::query::plumbing::try_execute_query::<rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt, rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  20:     0x7f2c8d9c2c94 - rustc_query_system[e31828d683db36a2]::query::plumbing::get_query::<rustc_query_impl[fdf912c10b8aed29]::queries::analysis, rustc_query_impl[fdf912c10b8aed29]::plumbing::QueryCtxt>
  21:     0x7f2c8d7dedad - <rustc_query_impl[fdf912c10b8aed29]::Queries as rustc_middle[b8568a7b9c9324a7]::ty::query::QueryEngine>::analysis
  22:     0x7f2c8bdfa075 - rustc_middle[b8568a7b9c9324a7]::ty::query::evaluate_query::<rustc_query_system[e31828d683db36a2]::query::caches::DefaultCache<(), core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>>
  23:     0x7f2c8be01507 - <rustc_interface[c397efb26ff26c9d]::passes::QueryContext>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  24:     0x7f2c8bd9e291 - <rustc_interface[c397efb26ff26c9d]::interface::Compiler>::enter::<rustc_driver[a33928266507259a]::run_compiler::{closure#1}::{closure#2}, core[3334471bab16b0e9]::result::Result<core[3334471bab16b0e9]::option::Option<rustc_interface[c397efb26ff26c9d]::queries::Linker>, rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  25:     0x7f2c8bd97561 - rustc_span[16f708bed991f38b]::with_source_map::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#1}>
  26:     0x7f2c8bdb9861 - rustc_interface[c397efb26ff26c9d]::interface::create_compiler_and_run::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>
  27:     0x7f2c8bd81ef2 - <scoped_tls[838c36787839aa63]::ScopedKey<rustc_span[16f708bed991f38b]::SessionGlobals>>::set::<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  28:     0x7f2c8bdf930f - std[2ad72f44f058fdd3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>
  29:     0x7f2c8bd824de - std[2ad72f44f058fdd3]::panicking::try::<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, core[3334471bab16b0e9]::panic::unwind_safe::AssertUnwindSafe<<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  30:     0x7f2c8bdfc730 - <<std[2ad72f44f058fdd3]::thread::Builder>::spawn_unchecked_<rustc_interface[c397efb26ff26c9d]::util::run_in_thread_pool_with_globals<rustc_interface[c397efb26ff26c9d]::interface::run_compiler<core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>, rustc_driver[a33928266507259a]::run_compiler::{closure#1}>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#0}, core[3334471bab16b0e9]::result::Result<(), rustc_errors[a0c1c5ded8bf3409]::ErrorGuaranteed>>::{closure#1} as core[3334471bab16b0e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7f2c8b425645 - std::sys::unix::thread::Thread::new::thread_start::h8714a0d2d89391e6
  32:     0x7f2c8b1c1b43 - <unknown>
  33:     0x7f2c8b253a00 - <unknown>
  34:                0x0 - <unknown>

DEBUG rustc_errors diagnostic=Diagnostic { level: Bug, message: [(Str("unexpected panic"), NoStyle)], code: None, span: MultiSpan { primary_spans: [], span_labels: [] }, children: [], suggestions: Ok([]), args: [], sort_span: no-location (#0), is_lint: false }
DEBUG rustc_errors self.emitted_diagnostics={}
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]

DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors diagnostic=Diagnostic { level: Note, message: [(Str("the compiler unexpectedly panicked. this is a bug."), NoStyle)], code: None, span: MultiSpan { primary_spans: [], span_labels: [] }, children: [], suggestions: Ok([]), args: [], sort_span: no-location (#0), is_lint: false }
DEBUG rustc_errors self.emitted_diagnostics={}
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]

DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors diagnostic=Diagnostic { level: Note, message: [(Str("we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md"), NoStyle)], code: None, span: MultiSpan { primary_spans: [], span_labels: [] }, children: [], suggestions: Ok([]), args: [], sort_span: no-location (#0), is_lint: false }
DEBUG rustc_errors self.emitted_diagnostics={}
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]

DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors diagnostic=Diagnostic { level: Note, message: [(Str("rustc 1.65.0-nightly (77fe4bc1d 2022-08-30) running on x86_64-unknown-linux-gnu"), NoStyle)], code: None, span: MultiSpan { primary_spans: [], span_labels: [] }, children: [], suggestions: Ok([]), args: [], sort_span: no-location (#0), is_lint: false }
DEBUG rustc_errors self.emitted_diagnostics={}
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]
note: rustc 1.65.0-nightly (77fe4bc1d 2022-08-30) running on x86_64-unknown-linux-gnu
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors diagnostic=Diagnostic { level: Note, message: [(Str("compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib"), NoStyle)], code: None, span: MultiSpan { primary_spans: [], span_labels: [] }, children: [], suggestions: Ok([]), args: [], sort_span: no-location (#0), is_lint: false }
DEBUG rustc_errors self.emitted_diagnostics={}
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]
note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]
#0 [analysis] running analysis passes on this crate
end of query stack
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) --- found dependency hir_module_items(rustc_rust_log_aux[8cb6]) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]::{use#0}) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]::std) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) - END - dependency hir_owner(rustc_rust_log_aux[8cb6]::foo) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) --- found dependency hir_module_items(rustc_rust_log_aux[8cb6]) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]::{use#0}) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]::std) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) - END - dependency hir_owner(rustc_rust_log_aux[8cb6]::foo) was immediately red
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) --- found dependency hir_module_items(rustc_rust_log_aux[8cb6]) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]::{use#0}) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) --- found dependency hir_attrs(rustc_rust_log_aux[8cb6]::{use#0}) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]::std) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) --- found dependency hir_attrs(rustc_rust_log_aux[8cb6]::std) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) - END - dependency hir_owner(rustc_rust_log_aux[8cb6]::foo) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) --- found dependency hir_module_items(rustc_rust_log_aux[8cb6]) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]::{use#0}) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) --- found dependency hir_attrs(rustc_rust_log_aux[8cb6]::{use#0}) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) --- found dependency hir_owner(rustc_rust_log_aux[8cb6]::std) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) --- found dependency hir_attrs(rustc_rust_log_aux[8cb6]::std) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_attrs(rustc_rust_log_aux[8cb6])) - END - dependency hir_owner(rustc_rust_log_aux[8cb6]::foo) was immediately red
DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(codegen_fn_attrs(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(codegen_fn_attrs(rustc_rust_log_aux[8cb6]::foo)) --- state of dependency opt_def_kind(rustc_rust_log_aux[8cb6]::foo) (8cb6324e8d20b84d-7ac63b91867badbd) is unknown, trying to mark it green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(opt_def_kind(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(opt_def_kind(rustc_rust_log_aux[8cb6]::foo)) --- state of dependency local_def_id_to_hir_id(rustc_rust_log_aux[8cb6]::foo) (8cb6324e8d20b84d-7ac63b91867badbd) is unknown, trying to mark it green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(local_def_id_to_hir_id(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(local_def_id_to_hir_id(rustc_rust_log_aux[8cb6]::foo)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(opt_def_kind(rustc_rust_log_aux[8cb6]::foo)) --- trying to force dependency local_def_id_to_hir_id(rustc_rust_log_aux[8cb6]::foo)
DEBUG rustc_middle::dep_graph try_force_from_dep_node(local_def_id_to_hir_id(rustc_rust_log_aux[8cb6]::foo)) --- trying to force
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(local_def_id_to_hir_id(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(local_def_id_to_hir_id(rustc_rust_log_aux[8cb6]::foo)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(opt_def_kind(rustc_rust_log_aux[8cb6]::foo)) --- managed to FORCE dependency local_def_id_to_hir_id(rustc_rust_log_aux[8cb6]::foo) to green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(opt_def_kind(rustc_rust_log_aux[8cb6]::foo)) - END - dependency hir_owner(rustc_rust_log_aux[8cb6]::foo) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(codegen_fn_attrs(rustc_rust_log_aux[8cb6]::foo)) --- trying to force dependency opt_def_kind(rustc_rust_log_aux[8cb6]::foo)
DEBUG rustc_middle::dep_graph try_force_from_dep_node(opt_def_kind(rustc_rust_log_aux[8cb6]::foo)) --- trying to force
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(opt_def_kind(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(opt_def_kind(rustc_rust_log_aux[8cb6]::foo)) --- found dependency local_def_id_to_hir_id(rustc_rust_log_aux[8cb6]::foo) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(opt_def_kind(rustc_rust_log_aux[8cb6]::foo)) - END - dependency hir_owner(rustc_rust_log_aux[8cb6]::foo) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(codegen_fn_attrs(rustc_rust_log_aux[8cb6]::foo)) --- managed to FORCE dependency opt_def_kind(rustc_rust_log_aux[8cb6]::foo) to green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(codegen_fn_attrs(rustc_rust_log_aux[8cb6]::foo)) --- found dependency local_def_id_to_hir_id(rustc_rust_log_aux[8cb6]::foo) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(codegen_fn_attrs(rustc_rust_log_aux[8cb6]::foo)) --- found dependency hir_attrs(rustc_rust_log_aux[8cb6]::foo) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(codegen_fn_attrs(rustc_rust_log_aux[8cb6]::foo)) --- state of dependency should_inherit_track_caller(rustc_rust_log_aux[8cb6]::foo) (8cb6324e8d20b84d-7ac63b91867badbd) is unknown, trying to mark it green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(should_inherit_track_caller(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(should_inherit_track_caller(rustc_rust_log_aux[8cb6]::foo)) --- found dependency opt_def_kind(rustc_rust_log_aux[8cb6]::foo) to be immediately green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(should_inherit_track_caller(rustc_rust_log_aux[8cb6]::foo)) - END - successfully marked as green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(codegen_fn_attrs(rustc_rust_log_aux[8cb6]::foo)) --- managed to MARK dependency should_inherit_track_caller(rustc_rust_log_aux[8cb6]::foo) as green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(codegen_fn_attrs(rustc_rust_log_aux[8cb6]::foo)) --- trying to force dependency get_lang_items(0-0)
DEBUG rustc_middle::dep_graph try_force_from_dep_node(get_lang_items(0-0)) --- trying to force
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=1198928
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=24608
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=32
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=172048
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=115024
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=1920
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=80
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=21856
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=48
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=1280
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=48992
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=4272
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=5472
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=6624
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=161760
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=263344
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=20032
DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=640
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(defined_lang_items(std[2ad7])) - BEGIN
