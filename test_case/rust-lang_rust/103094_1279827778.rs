plain
 finished in 0.721 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 158 tests
FFFFFFFFFFF..F...F..F...........F..FFFFFFFFFFFFF.FFF.FFFFFFFFFFFFFFFFFF.FFFFFFF.FFFFFFFF 88/158
FFFFFFF.FFFFFFFFFFFFF.FFFFFFFFFFF..FFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFF.FF

---- [incremental] src/test/incremental/change_name_of_static_in_fn.rs stdout ----


error in revision `rpass2`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
stack backtrace:
   0:     0x7faeee6f539e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7faeee75eac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7faeee6e6c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7faeee6f833e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7faeee6f7fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7faeef0995b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7faeee6f8bd4 - std::panicking::rust_panic_with_hook::h387fbfdb6a9d6321
   7:     0x7faeee6f88f9 - std::panicking::begin_panic_handler::{{closure}}::he9f84735e433f90d
   8:     0x7faeee6f58d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbccfbc640fce1c0d
   9:     0x7faeee6f8602 - rust_begin_unwind
  10:     0x7faeee6a9a33 - core::panicking::panic_fmt::h143efa9e25241140
  11:     0x7faef007fb30 - <rustc_passes[ae1fd642deb25f9f]::check_attr::CheckAttrVisitor>::check_attributes
  12:     0x7faef0081d9f - <rustc_passes[ae1fd642deb25f9f]::check_attr::CheckAttrVisitor as rustc_hir[9968eec846548fac]::intravisit::Visitor>::visit_item
  13:     0x7faef003aa10 - <rustc_middle[92cbfa92439caac6]::hir::map::Map>::visit_item_likes_in_module::<rustc_passes[ae1fd642deb25f9f]::check_attr::CheckAttrVisitor>
  14:     0x7faef00821f5 - rustc_passes[ae1fd642deb25f9f]::check_attr::check_mod_attrs
  15:     0x7faef093701d - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, rustc_span[2a207b77041a0058]::def_id::LocalDefId, ()>::{closure#0}, ()>
  16:     0x7faef0aaa6cf - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, rustc_span[2a207b77041a0058]::def_id::LocalDefId, ()>
  17:     0x7faef0cda89c - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<rustc_span[2a207b77041a0058]::def_id::LocalDefId, ()>>
  18:     0x7faef0dbb3aa - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::check_mod_attrs, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  19:     0x7faef0c6d2f8 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  20:     0x7faeef25669d - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  21:     0x7faeef1b41a6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  22:     0x7faeef1ac5ad - rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in::<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  23:     0x7faeef25746c - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  24:     0x7faeef1b42c6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}>>
  25:     0x7faeef1eeb8f - rustc_interface[7a6d25e97b1d2994]::passes::analysis
  26:     0x7faef093e9f4 - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  27:     0x7faef0af1600 - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  28:     0x7faef0d216c4 - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<(), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>>
  29:     0x7faef0e09d5b - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::analysis, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  30:     0x7faef0c701f7 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  31:     0x7faeef1070a0 - <rustc_interface[7a6d25e97b1d2994]::passes::QueryContext>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  32:     0x7faeef09b9ba - <rustc_interface[7a6d25e97b1d2994]::interface::Compiler>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}, core[8441253fc0f89e2]::result::Result<core[8441253fc0f89e2]::option::Option<rustc_interface[7a6d25e97b1d2994]::queries::Linker>, rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  33:     0x7faeef0864fe - rustc_span[2a207b77041a0058]::with_source_map::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#1}>
  34:     0x7faeef0b0082 - rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>
  35:     0x7faeef110a2f - <scoped_tls[8a59c43d1c872517]::ScopedKey<rustc_span[2a207b77041a0058]::SessionGlobals>>::set::<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  36:     0x7faeef0c9a9f - std[670518f464012ae8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  37:     0x7faeef088496 - std[670518f464012ae8]::panicking::try::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  38:     0x7faeef0be82a - <<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1} as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7faeee705445 - std::sys::unix::thread::Thread::new::thread_start::h4af8bd76c52c9d81
  40:     0x7faeee49fb43 - <unknown>
  41:     0x7faeee531a00 - <unknown>
  42:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_mod_attrs] checking attributes in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
stack backtrace:
   0:     0x7faeee6f539e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7faeee75eac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7faeee6e6c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7faeee6f833e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7faeee6f7fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7faeef0995b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7faeee6f8bd4 - std::panicking::rust_panic_with_hook::h387fbfdb6a9d6321
   7:     0x7faeee6f88f9 - std::panicking::begin_panic_handler::{{closure}}::he9f84735e433f90d
   8:     0x7faeee6f58d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbccfbc640fce1c0d
   9:     0x7faeee6f8602 - rust_begin_unwind
  10:     0x7faeee6a9a33 - core::panicking::panic_fmt::h143efa9e25241140
  11:     0x7faeef257245 - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#3}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  12:     0x7faeef1b42a6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#3}>>
  13:     0x7faeef1eec62 - rustc_interface[7a6d25e97b1d2994]::passes::analysis
  14:     0x7faef093e9f4 - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  15:     0x7faef0af1600 - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  16:     0x7faef0d216c4 - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<(), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>>
  17:     0x7faef0e09d5b - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::analysis, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  18:     0x7faef0c701f7 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  19:     0x7faeef1070a0 - <rustc_interface[7a6d25e97b1d2994]::passes::QueryContext>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  20:     0x7faeef09b9ba - <rustc_interface[7a6d25e97b1d2994]::interface::Compiler>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}, core[8441253fc0f89e2]::result::Result<core[8441253fc0f89e2]::option::Option<rustc_interface[7a6d25e97b1d2994]::queries::Linker>, rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  21:     0x7faeef0864fe - rustc_span[2a207b77041a0058]::with_source_map::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#1}>
  22:     0x7faeef0b0082 - rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>
  23:     0x7faeef110a2f - <scoped_tls[8a59c43d1c872517]::ScopedKey<rustc_span[2a207b77041a0058]::SessionGlobals>>::set::<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  24:     0x7faeef0c9a9f - std[670518f464012ae8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  25:     0x7faeef088496 - std[670518f464012ae8]::panicking::try::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  26:     0x7faeef0be82a - <<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1} as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7faeee705445 - std::sys::unix::thread::Thread::new::thread_start::h4af8bd76c52c9d81
  28:     0x7faeee49fb43 - <unknown>
  29:     0x7faeee531a00 - <unknown>
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
------------------------------------------


---- [incremental] src/test/incremental/change_private_fn/struct_point.rs stdout ----
---- [incremental] src/test/incremental/change_private_fn/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
   0:     0x7f5f7f57039e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7f5f7f5d9ac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7f5f7f57333e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7f5f7f572fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7f5f7ff145b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5f7f573bd4 - std::panicking::rust_panic_with_hook::h387fbfdb6a9d6321
   7:     0x7f5f7f5738f9 - std::panicking::begin_panic_handler::{{closure}}::he9f84735e433f90d
   8:     0x7f5f7f5708d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbccfbc640fce1c0d
   9:     0x7f5f7f573602 - rust_begin_unwind
  10:     0x7f5f7f524a33 - core::panicking::panic_fmt::h143efa9e25241140
  11:     0x7f5f800d1a45 - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  12:     0x7f5f8002f1a6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  13:     0x7f5f800275ad - rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in::<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  14:     0x7f5f800d246c - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  15:     0x7f5f8002f2c6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}>>
  16:     0x7f5f80069b8f - rustc_interface[7a6d25e97b1d2994]::passes::analysis
  17:     0x7f5f817b99f4 - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  18:     0x7f5f8196c600 - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  19:     0x7f5f81b9c6c4 - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<(), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>>
  20:     0x7f5f81c84d5b - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::analysis, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  21:     0x7f5f81aeb1f7 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  22:     0x7f5f7ff820a0 - <rustc_interface[7a6d25e97b1d2994]::passes::QueryContext>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  23:     0x7f5f7ff169ba - <rustc_interface[7a6d25e97b1d2994]::interface::Compiler>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}, core[8441253fc0f89e2]::result::Result<core[8441253fc0f89e2]::option::Option<rustc_interface[7a6d25e97b1d2994]::queries::Linker>, rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  24:     0x7f5f7ff014fe - rustc_span[2a207b77041a0058]::with_source_map::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7f5f7ff2b082 - rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>
  26:     0x7f5f7ff8ba2f - <scoped_tls[8a59c43d1c872517]::ScopedKey<rustc_span[2a207b77041a0058]::SessionGlobals>>::set::<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  27:     0x7f5f7ff44a9f - std[670518f464012ae8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  28:     0x7f5f7ff03496 - std[670518f464012ae8]::panicking::try::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x7f5f7ff3982a - <<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1} as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f5f7f580445 - std::sys::unix::thread::Thread::new::thread_start::h4af8bd76c52c9d81
  31:     0x7f5f7f31ab43 - <unknown>
  32:     0x7f5f7f3aca00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
   0:     0x7f5f7f57039e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7f5f7f5d9ac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7f5f7f57333e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7f5f7f572fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7f5f7ff145b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5f7f573bd4 - std::panicking::rust_panic_with_hook::h387fbfdb6a9d6321
   7:     0x7f5f7f5738f9 - std::panicking::begin_panic_handler::{{closure}}::he9f84735e433f90d
   8:     0x7f5f7f5708d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbccfbc640fce1c0d
   9:     0x7f5f7f573602 - rust_begin_unwind
  10:     0x7f5f7f524a33 - core::panicking::panic_fmt::h143efa9e25241140
  11:     0x7f5f80efab30 - <rustc_passes[ae1fd642deb25f9f]::check_attr::CheckAttrVisitor>::check_attributes
  12:     0x7f5f80efcfc7 - <rustc_passes[ae1fd642deb25f9f]::check_attr::CheckAttrVisitor as rustc_hir[9968eec846548fac]::intravisit::Visitor>::visit_impl_item
  13:     0x7f5f80eb5ad0 - <rustc_middle[92cbfa92439caac6]::hir::map::Map>::visit_item_likes_in_module::<rustc_passes[ae1fd642deb25f9f]::check_attr::CheckAttrVisitor>
  14:     0x7f5f80efd1f5 - rustc_passes[ae1fd642deb25f9f]::check_attr::check_mod_attrs
  15:     0x7f5f817b201d - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, rustc_span[2a207b77041a0058]::def_id::LocalDefId, ()>::{closure#0}, ()>
  16:     0x7f5f819256cf - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, rustc_span[2a207b77041a0058]::def_id::LocalDefId, ()>
  17:     0x7f5f81b5589c - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<rustc_span[2a207b77041a0058]::def_id::LocalDefId, ()>>
  18:     0x7f5f81c363aa - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::check_mod_attrs, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  19:     0x7f5f81ae82f8 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  20:     0x7f5f800d169d - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  21:     0x7f5f8002f1a6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  22:     0x7f5f800275ad - rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in::<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  23:     0x7f5f800d246c - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  24:     0x7f5f8002f2c6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}>>
  25:     0x7f5f80069b8f - rustc_interface[7a6d25e97b1d2994]::passes::analysis
  26:     0x7f5f817b99f4 - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  27:     0x7f5f8196c600 - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  28:     0x7f5f81b9c6c4 - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<(), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>>
  29:     0x7f5f81c84d5b - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::analysis, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  30:     0x7f5f81aeb1f7 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  31:     0x7f5f7ff820a0 - <rustc_interface[7a6d25e97b1d2994]::passes::QueryContext>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  32:     0x7f5f7ff169ba - <rustc_interface[7a6d25e97b1d2994]::interface::Compiler>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}, core[8441253fc0f89e2]::result::Result<core[8441253fc0f89e2]::option::Option<rustc_interface[7a6d25e97b1d2994]::queries::Linker>, rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  33:     0x7f5f7ff014fe - rustc_span[2a207b77041a0058]::with_source_map::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#1}>
  34:     0x7f5f7ff2b082 - rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>
  35:     0x7f5f7ff8ba2f - <scoped_tls[8a59c43d1c872517]::ScopedKey<rustc_span[2a207b77041a0058]::SessionGlobals>>::set::<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  36:     0x7f5f7ff44a9f - std[670518f464012ae8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  37:     0x7f5f7ff03496 - std[670518f464012ae8]::panicking::try::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  38:     0x7f5f7ff3982a - <<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1} as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f5f7f580445 - std::sys::unix::thread::Thread::new::thread_start::h4af8bd76c52c9d81
  40:     0x7f5f7f31ab43 - <unknown>
  41:     0x7f5f7f3aca00 - <unknown>
  42:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [check_mod_attrs] checking attributes in module `point`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
   0:     0x7f5f7f57039e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7f5f7f5d9ac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7f5f7f57333e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7f5f7f572fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7f5f7ff145b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5f7f573bd4 - std::panicking::rust_panic_with_hook::h387fbfdb6a9d6321
   7:     0x7f5f7f5738f9 - std::panicking::begin_panic_handler::{{closure}}::he9f84735e433f90d
   8:     0x7f5f7f5708d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbccfbc640fce1c0d
   9:     0x7f5f7f573602 - rust_begin_unwind
  10:     0x7f5f7f524a33 - core::panicking::panic_fmt::h143efa9e25241140
  11:     0x7f5f800d1a45 - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  12:     0x7f5f8002f1a6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  13:     0x7f5f800275ad - rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in::<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  14:     0x7f5f800d246c - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  15:     0x7f5f8002f2c6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}>>
  16:     0x7f5f80069b8f - rustc_interface[7a6d25e97b1d2994]::passes::analysis
  17:     0x7f5f817b99f4 - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  18:     0x7f5f8196c600 - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  19:     0x7f5f81b9c6c4 - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<(), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>>
  20:     0x7f5f81c84d5b - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::analysis, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  21:     0x7f5f81aeb1f7 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  22:     0x7f5f7ff820a0 - <rustc_interface[7a6d25e97b1d2994]::passes::QueryContext>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  23:     0x7f5f7ff169ba - <rustc_interface[7a6d25e97b1d2994]::interface::Compiler>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}, core[8441253fc0f89e2]::result::Result<core[8441253fc0f89e2]::option::Option<rustc_interface[7a6d25e97b1d2994]::queries::Linker>, rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  24:     0x7f5f7ff014fe - rustc_span[2a207b77041a0058]::with_source_map::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7f5f7ff2b082 - rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>
  26:     0x7f5f7ff8ba2f - <scoped_tls[8a59c43d1c872517]::ScopedKey<rustc_span[2a207b77041a0058]::SessionGlobals>>::set::<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  27:     0x7f5f7ff44a9f - std[670518f464012ae8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  28:     0x7f5f7ff03496 - std[670518f464012ae8]::panicking::try::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x7f5f7ff3982a - <<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1} as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f5f7f580445 - std::sys::unix::thread::Thread::new::thread_start::h4af8bd76c52c9d81
  31:     0x7f5f7f31ab43 - <unknown>
  32:     0x7f5f7f3aca00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
   0:     0x7f5f7f57039e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7f5f7f5d9ac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7f5f7f57333e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7f5f7f572fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7f5f7ff145b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5f7f573bd4 - std::panicking::rust_panic_with_hook::h387fbfdb6a9d6321
   7:     0x7f5f7f5738f9 - std::panicking::begin_panic_handler::{{closure}}::he9f84735e433f90d
   8:     0x7f5f7f5708d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbccfbc640fce1c0d
   9:     0x7f5f7f573602 - rust_begin_unwind
  10:     0x7f5f7f524a33 - core::panicking::panic_fmt::h143efa9e25241140
  11:     0x7f5f800d1a45 - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  12:     0x7f5f8002f1a6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  13:     0x7f5f800275ad - rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in::<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  14:     0x7f5f800d246c - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  15:     0x7f5f8002f2c6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}>>
  16:     0x7f5f80069b8f - rustc_interface[7a6d25e97b1d2994]::passes::analysis
  17:     0x7f5f817b99f4 - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  18:     0x7f5f8196c600 - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  19:     0x7f5f81b9c6c4 - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<(), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>>
  20:     0x7f5f81c84d5b - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::analysis, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  21:     0x7f5f81aeb1f7 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  22:     0x7f5f7ff820a0 - <rustc_interface[7a6d25e97b1d2994]::passes::QueryContext>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  23:     0x7f5f7ff169ba - <rustc_interface[7a6d25e97b1d2994]::interface::Compiler>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}, core[8441253fc0f89e2]::result::Result<core[8441253fc0f89e2]::option::Option<rustc_interface[7a6d25e97b1d2994]::queries::Linker>, rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  24:     0x7f5f7ff014fe - rustc_span[2a207b77041a0058]::with_source_map::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7f5f7ff2b082 - rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>
  26:     0x7f5f7ff8ba2f - <scoped_tls[8a59c43d1c872517]::ScopedKey<rustc_span[2a207b77041a0058]::SessionGlobals>>::set::<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  27:     0x7f5f7ff44a9f - std[670518f464012ae8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  28:     0x7f5f7ff03496 - std[670518f464012ae8]::panicking::try::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x7f5f7ff3982a - <<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1} as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f5f7f580445 - std::sys::unix::thread::Thread::new::thread_start::h4af8bd76c52c9d81
  31:     0x7f5f7f31ab43 - <unknown>
  32:     0x7f5f7f3aca00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
   0:     0x7f5f7f57039e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7f5f7f5d9ac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7f5f7f57333e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7f5f7f572fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7f5f7ff145b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5f7f573bd4 - std::panicking::rust_panic_with_hook::h387fbfdb6a9d6321
   7:     0x7f5f7f5738f9 - std::panicking::begin_panic_handler::{{closure}}::he9f84735e433f90d
   8:     0x7f5f7f5708d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbccfbc640fce1c0d
   9:     0x7f5f7f573602 - rust_begin_unwind
  10:     0x7f5f7f524a33 - core::panicking::panic_fmt::h143efa9e25241140
  11:     0x7f5f800d1a45 - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  12:     0x7f5f8002f1a6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  13:     0x7f5f800275ad - rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in::<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  14:     0x7f5f800d246c - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  15:     0x7f5f8002f2c6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}>>
  16:     0x7f5f80069b8f - rustc_interface[7a6d25e97b1d2994]::passes::analysis
  17:     0x7f5f817b99f4 - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  18:     0x7f5f8196c600 - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  19:     0x7f5f81b9c6c4 - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<(), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>>
  20:     0x7f5f81c84d5b - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::analysis, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  21:     0x7f5f81aeb1f7 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  22:     0x7f5f7ff820a0 - <rustc_interface[7a6d25e97b1d2994]::passes::QueryContext>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  23:     0x7f5f7ff169ba - <rustc_interface[7a6d25e97b1d2994]::interface::Compiler>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}, core[8441253fc0f89e2]::result::Result<core[8441253fc0f89e2]::option::Option<rustc_interface[7a6d25e97b1d2994]::queries::Linker>, rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  24:     0x7f5f7ff014fe - rustc_span[2a207b77041a0058]::with_source_map::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7f5f7ff2b082 - rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>
  26:     0x7f5f7ff8ba2f - <scoped_tls[8a59c43d1c872517]::ScopedKey<rustc_span[2a207b77041a0058]::SessionGlobals>>::set::<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  27:     0x7f5f7ff44a9f - std[670518f464012ae8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  28:     0x7f5f7ff03496 - std[670518f464012ae8]::panicking::try::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x7f5f7ff3982a - <<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1} as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f5f7f580445 - std::sys::unix::thread::Thread::new::thread_start::h4af8bd76c52c9d81
  31:     0x7f5f7f31ab43 - <unknown>
  32:     0x7f5f7f3aca00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
   0:     0x7f5f7f57039e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7f5f7f5d9ac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7f5f7f57333e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7f5f7f572fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7f5f7ff145b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5f7f573bd4 - std::panicking::rust_panic_with_hook::h387fbfdb6a9d6321
   7:     0x7f5f7f5738f9 - std::panicking::begin_panic_handler::{{closure}}::he9f84735e433f90d
   8:     0x7f5f7f5708d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbccfbc640fce1c0d
   9:     0x7f5f7f573602 - rust_begin_unwind
  10:     0x7f5f7f524a33 - core::panicking::panic_fmt::h143efa9e25241140
  11:     0x7f5f800d1a45 - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  12:     0x7f5f8002f1a6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  13:     0x7f5f800275ad - rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in::<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  14:     0x7f5f800d246c - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  15:     0x7f5f8002f2c6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}>>
  16:     0x7f5f80069b8f - rustc_interface[7a6d25e97b1d2994]::passes::analysis
  17:     0x7f5f817b99f4 - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  18:     0x7f5f8196c600 - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  19:     0x7f5f81b9c6c4 - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<(), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>>
  20:     0x7f5f81c84d5b - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::analysis, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  21:     0x7f5f81aeb1f7 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  22:     0x7f5f7ff820a0 - <rustc_interface[7a6d25e97b1d2994]::passes::QueryContext>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  23:     0x7f5f7ff169ba - <rustc_interface[7a6d25e97b1d2994]::interface::Compiler>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}, core[8441253fc0f89e2]::result::Result<core[8441253fc0f89e2]::option::Option<rustc_interface[7a6d25e97b1d2994]::queries::Linker>, rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  24:     0x7f5f7ff014fe - rustc_span[2a207b77041a0058]::with_source_map::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7f5f7ff2b082 - rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>
  26:     0x7f5f7ff8ba2f - <scoped_tls[8a59c43d1c872517]::ScopedKey<rustc_span[2a207b77041a0058]::SessionGlobals>>::set::<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  27:     0x7f5f7ff44a9f - std[670518f464012ae8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  28:     0x7f5f7ff03496 - std[670518f464012ae8]::panicking::try::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x7f5f7ff3982a - <<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1} as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f5f7f580445 - std::sys::unix::thread::Thread::new::thread_start::h4af8bd76c52c9d81
  31:     0x7f5f7f31ab43 - <unknown>
  32:     0x7f5f7f3aca00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
   0:     0x7f5f7f57039e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7f5f7f5d9ac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7f5f7f57333e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7f5f7f572fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7f5f7ff145b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5f7f573bd4 - std::panicking::rust_panic_with_hook::h387fbfdb6a9d6321
   7:     0x7f5f7f5738f9 - std::panicking::begin_panic_handler::{{closure}}::he9f84735e433f90d
   8:     0x7f5f7f5708d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbccfbc640fce1c0d
   9:     0x7f5f7f573602 - rust_begin_unwind
  10:     0x7f5f7f524a33 - core::panicking::panic_fmt::h143efa9e25241140
  11:     0x7f5f800d1a45 - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  12:     0x7f5f8002f1a6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  13:     0x7f5f800275ad - rustc_data_structures[57598c19cab507ed]::sync::par_for_each_in::<&[rustc_hir[9968eec846548fac]::hir_id::OwnerId], <rustc_middle[92cbfa92439caac6]::hir::map::Map>::par_for_each_module<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  14:     0x7f5f800d246c - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  15:     0x7f5f8002f2c6 - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#1}>>
  16:     0x7f5f80069b8f - rustc_interface[7a6d25e97b1d2994]::passes::analysis
  17:     0x7f5f817b99f4 - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  18:     0x7f5f8196c600 - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  19:     0x7f5f81b9c6c4 - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<(), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>>
  20:     0x7f5f81c84d5b - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::analysis, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  21:     0x7f5f81aeb1f7 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  22:     0x7f5f7ff820a0 - <rustc_interface[7a6d25e97b1d2994]::passes::QueryContext>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  23:     0x7f5f7ff169ba - <rustc_interface[7a6d25e97b1d2994]::interface::Compiler>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}, core[8441253fc0f89e2]::result::Result<core[8441253fc0f89e2]::option::Option<rustc_interface[7a6d25e97b1d2994]::queries::Linker>, rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  24:     0x7f5f7ff014fe - rustc_span[2a207b77041a0058]::with_source_map::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7f5f7ff2b082 - rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>
  26:     0x7f5f7ff8ba2f - <scoped_tls[8a59c43d1c872517]::ScopedKey<rustc_span[2a207b77041a0058]::SessionGlobals>>::set::<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  27:     0x7f5f7ff44a9f - std[670518f464012ae8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  28:     0x7f5f7ff03496 - std[670518f464012ae8]::panicking::try::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  29:     0x7f5f7ff3982a - <<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1} as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f5f7f580445 - std::sys::unix::thread::Thread::new::thread_start::h4af8bd76c52c9d81
  31:     0x7f5f7f31ab43 - <unknown>
  32:     0x7f5f7f3aca00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
   0:     0x7f5f7f57039e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7f5f7f5d9ac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   2:     0x7f5f7f561c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7f5f7f57333e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7f5f7f572fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7f5f7ff145b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
---
  hash: 1364b66339341803
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-a842d6fd47d5c6b1.rlib
  name: alloc
  cnum: 5
  hash: 9c0082f4df5d2f76
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-e0aaedc8933e1018.rlib
  name: libc
  cnum: 6
  hash: dc38ba56a8d371ea
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-257e8d8a301ed47a.rlib
  name: unwind
  cnum: 7
  hash: 02d49b8048d04963
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-32e4723cdc2ea263.rlib
  name: cfg_if
  cnum: 8
  hash: bb0af58971166b3c
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-7b5db433717c7b42.rlib
  cnum: 9
  hash: bc9f377689d288f6
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-ec4691244a9466fe.rlib
---
  hash: fa1e467170bd710b
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-454e2d3fa9c292eb.rlib
  name: std_detect
  cnum: 13
  hash: 565fc02ca6be3a71
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-e3a3d45bfe1a57de.rlib
  name: cfg_if
  cnum: 14
  hash: d4e6c5a10e4faabd
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-7dd89bdff87f7722.rlib
  name: rustc_demangle
  cnum: 15
  hash: 1ca72ee1f4ef8c2b
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-d1c0ae91db3515bb.rlib
  cnum: 16
  hash: af753b48467262e7
  reqd: Explicit
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-aa21f095e02ffecc.rlib
  name: gimli
  cnum: 17
  hash: 20ee847aa4b2b487
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-b0365ac5d240746c.rlib
  name: object
  cnum: 18
  hash: dbfd9f09b9492133
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-9b8426ee63623af8.rlib
  name: memchr
  cnum: 19
  hash: b607a330e0b6622d
  reqd: Explicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-25faf4e8d9bf7bbb.rlib
  cnum: 20
  hash: 63190e25d349ba74
  reqd: Implicit
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-035e66f62371ff1e.rlib
   rlib: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-035e66f62371ff1e.rlib

DEBUG rustc_ast_passes::feature_gate gate_feature(feature = "prelude_import", span = no-location (#1)); has? false
INFO rustc_interface::passes 0 parse sess buffered_lints
DEBUG rustc_lint::early id=NodeId(0)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early id=NodeId(1)
DEBUG rustc_lint::early early context: enter_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(3), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early early context: exit_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(3), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early id=NodeId(7)
DEBUG rustc_lint::early early context: enter_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(8), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(2), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early early context: exit_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(8), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(2), style: Outer, span: no-location (#1) }])
DEBUG rustc_lint::early id=NodeId(9)
DEBUG rustc_lint::early early context: enter_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(10), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, TokenStream([Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }, Alone)])), tokens: None }, tokens: Some(LazyAttrTokenStream(AttrTokenStream([Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:2 (#0) }, Alone), Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:2: 5:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:14: 5:15 (#0) }, Bracket, AttrTokenStream([Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0) }, Alone), Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, AttrTokenStream([Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }, Alone)]))]))]))) }), id: AttrId(1), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:15 (#0) }])
DEBUG rustc_lint::early id=NodeId(12)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early id=NodeId(13)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early id=NodeId(15)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early id=NodeId(16)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early id=NodeId(17)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early id=NodeId(21)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early id=NodeId(22)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early id=NodeId(27)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early id=NodeId(28)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early id=NodeId(29)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early id=NodeId(30)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early id=NodeId(31)
DEBUG rustc_lint::early early context: enter_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_lint::early early context: exit_attrs([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(10), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, TokenStream([Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }, Alone)])), tokens: None }, tokens: Some(LazyAttrTokenStream(AttrTokenStream([Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:2 (#0) }, Alone), Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:2: 5:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:14: 5:15 (#0) }, Bracket, AttrTokenStream([Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0) }, Alone), Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, AttrTokenStream([Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }, Alone)]))]))]))) }), id: AttrId(1), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:15 (#0) }])
DEBUG rustc_lint::early early context: exit_attrs([])
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6])) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_owner(rustc_rust_log_aux[8cb6])) --- trying to force dependency hir_crate(0-0)
DEBUG rustc_query_system::dep_graph try_force_from_dep_node(hir_crate(0-0)) --- trying to force
┐rustc_ast_lowering::item::lower_crate 
├─┐rustc_ast_lowering::with_hir_id_owner owner=NodeId(0)
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─┐rustc_ast_lowering::index::index_hir item=Crate(Mod { spans: ModSpans { inner_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 8:2 (#0), inject_use_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:2:1: 2:1 (#0) }, item_ids: [ItemId { def_id: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) } }, ItemId { def_id: OwnerId { def_id: DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std) } }, ItemId { def_id: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) } }] })
│ │ ├─0ms DEBUG rustc_ast_lowering::index visit_nested_item: ItemId { def_id: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) } }
│ │ ├─0ms DEBUG rustc_ast_lowering::index visit_nested_item: ItemId { def_id: OwnerId { def_id: DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std) } }
│ │ ├─0ms DEBUG rustc_ast_lowering::index visit_nested_item: ItemId { def_id: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) } }
├─┘
┘
┘
┐rustc_ast_lowering::item::lower_item item=Item { attrs: [Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(3), style: Outer, span: no-location (#1) }], id: NodeId(1), span: no-location (#1), vis: Visibility { kind: Inherited, span: no-location (#1), tokens: None }, ident: #0, kind: Use(UseTree { prefix: Path { span: no-location (#1), segments: [PathSegment { ident: {{root}}#1, id: NodeId(3), args: None }, PathSegment { ident: std#1, id: NodeId(4), args: None }, PathSegment { ident: prelude#1, id: NodeId(5), args: None }, PathSegment { ident: rust_2015#1, id: NodeId(6), args: None }], tokens: None }, kind: Glob, span: no-location (#1) }), tokens: None }
├─┐rustc_ast_lowering::with_hir_id_owner owner=NodeId(1)
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(1)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 0 }
│ ├─┘
│ ├─┐rustc_ast_lowering::item::lower_use_tree tree=UseTree { prefix: Path { span: no-location (#1), segments: [PathSegment { ident: {{root}}#1, id: NodeId(3), args: None }, PathSegment { ident: std#1, id: NodeId(4), args: None }, PathSegment { ident: prelude#1, id: NodeId(5), args: None }, PathSegment { ident: rust_2015#1, id: NodeId(6), args: None }], tokens: None }, kind: Glob, span: no-location (#1) }, prefix=Path { span: no-location (#1), segments: [], tokens: None }, id=NodeId(1), vis_span=no-location (#1), ident=#0, attrs=Some([Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: prelude_import#1, id: NodeId(2), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(3), style: Outer, span: no-location (#1) }])
│ │ ├─0ms DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: {{root}}#1, id: NodeId(3), args: None })
│ │ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(3)
│ │ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 1 }
│ │ ├─┘
│ │ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident={{root}}#1 original-id=NodeId(3) new-id=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 1 }
│ │ ├─0ms DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: std#1, id: NodeId(4), args: None })
│ │ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(4)
│ │ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 2 }
│ │ ├─┘
│ │ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=std#1 original-id=NodeId(4) new-id=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 2 }
│ │ ├─0ms DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: prelude#1, id: NodeId(5), args: None })
│ │ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(5)
│ │ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 3 }
│ │ ├─┘
│ │ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=prelude#1 original-id=NodeId(5) new-id=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 3 }
│ │ ├─0ms DEBUG rustc_ast_lowering::path path_span: no-location (#1), lower_path_segment(segment: PathSegment { ident: rust_2015#1, id: NodeId(6), args: None })
│ │ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(6)
│ │ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 4 }
│ │ ├─┘
│ │ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=rust_2015#1 original-id=NodeId(6) new-id=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 4 }
│ ├─┘
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(0) len=331440
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(69) len=331440
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(283) len=331440
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(1)) = DefPathHash(Fingerprint(10139346924027820109, 13801149641163992105))
│ ├─┐rustc_ast_lowering::index::index_hir item=Item(Item { ident: #0, def_id: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, kind: Use(Path { span: no-location (#1), res: Err, segments: [PathSegment { ident: {{root}}#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 1 }, res: Err, args: None, infer_args: false }, PathSegment { ident: std#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 2 }, res: Def(Mod, DefId(1:0 ~ std[6705])), args: None, infer_args: false }, PathSegment { ident: prelude#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 3 }, res: Def(Mod, DefId(1:69 ~ std[6705]::prelude)), args: None, infer_args: false }, PathSegment { ident: rust_2015#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 4 }, res: Def(Mod, DefId(1:283 ~ std[6705]::prelude::rust_2015)), args: None, infer_args: false }] }, Glob), span: no-location (#1), vis_span: no-location (#1) })
│ │ ├─┐rustc_ast_lowering::index::visit_item i=Item { ident: #0, def_id: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, kind: Use(Path { span: no-location (#1), res: Err, segments: [PathSegment { ident: {{root}}#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 1 }, res: Err, args: None, infer_args: false }, PathSegment { ident: std#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 2 }, res: Def(Mod, DefId(1:0 ~ std[6705])), args: None, infer_args: false }, PathSegment { ident: prelude#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 3 }, res: Def(Mod, DefId(1:69 ~ std[6705]::prelude)), args: None, infer_args: false }, PathSegment { ident: rust_2015#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 4 }, res: Def(Mod, DefId(1:283 ~ std[6705]::prelude::rust_2015)), args: None, infer_args: false }] }, Glob), span: no-location (#1), vis_span: no-location (#1) }
│ │ │ ├─┐rustc_ast_lowering::index::insert span=no-location (#1), hir_id=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 1 }, node=PathSegment(PathSegment { ident: {{root}}#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 1 }, res: Err, args: None, infer_args: false })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=no-location (#1), hir_id=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 2 }, node=PathSegment(PathSegment { ident: std#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 2 }, res: Def(Mod, DefId(1:0 ~ std[6705])), args: None, infer_args: false })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=no-location (#1), hir_id=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 3 }, node=PathSegment(PathSegment { ident: prelude#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 3 }, res: Def(Mod, DefId(1:69 ~ std[6705]::prelude)), args: None, infer_args: false })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=no-location (#1), hir_id=HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 4 }, node=PathSegment(PathSegment { ident: rust_2015#1, hir_id: HirId { owner: OwnerId { def_id: DefId(0:1 ~ rustc_rust_log_aux[8cb6]::{use#0}) }, local_id: 4 }, res: Def(Mod, DefId(1:283 ~ std[6705]::prelude::rust_2015)), args: None, infer_args: false })
│ │ ├─┘
│ ├─┘
├─┘
┘
┘
┐rustc_ast_lowering::item::lower_item item=Item { attrs: [Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: no-location (#1), segments: [PathSegment { ident: macro_use#1, id: NodeId(8), args: None }], tokens: None }, args: Empty, tokens: None }, tokens: None }), id: AttrId(2), style: Outer, span: no-location (#1) }], id: NodeId(7), span: no-location (#1), vis: Visibility { kind: Inherited, span: no-location (#1), tokens: None }, ident: std#2, kind: ExternCrate(None), tokens: None }
├─┐rustc_ast_lowering::with_hir_id_owner owner=NodeId(7)
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(7)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std) }, local_id: 0 }
│ ├─┘
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(2)) = DefPathHash(Fingerprint(10139346924027820109, 11883850419197811220))
│ ├─┐rustc_ast_lowering::index::index_hir item=Item(Item { ident: std#2, def_id: OwnerId { def_id: DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std) }, kind: ExternCrate(None), span: no-location (#1), vis_span: no-location (#1) })
│ │ ├─┐rustc_ast_lowering::index::visit_item i=Item { ident: std#2, def_id: OwnerId { def_id: DefId(0:2 ~ rustc_rust_log_aux[8cb6]::std) }, kind: ExternCrate(None), span: no-location (#1), vis_span: no-location (#1) }
│ ├─┘
├─┘
┘
┘
┐rustc_ast_lowering::item::lower_item item=Item { attrs: [Attribute { kind: Normal(NormalAttr { item: AttrItem { path: Path { span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0), segments: [PathSegment { ident: cfg#0, id: NodeId(10), args: None }], tokens: None }, args: Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, TokenStream([Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }, Alone)])), tokens: None }, tokens: Some(LazyAttrTokenStream(AttrTokenStream([Token(Token { kind: Pound, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:2 (#0) }, Alone), Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:2: 5:3 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:14: 5:15 (#0) }, Bracket, AttrTokenStream([Token(Token { kind: Ident("cfg", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:3: 5:6 (#0) }, Alone), Delimited(DelimSpan { open: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:6: 5:7 (#0), close: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:13: 5:14 (#0) }, Parenthesis, AttrTokenStream([Token(Token { kind: Ident("rpass2", false), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:7: 5:13 (#0) }, Alone)]))]))]))) }), id: AttrId(1), style: Outer, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:5:1: 5:15 (#0) }], id: NodeId(9), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 8:2 (#0), vis: Visibility { kind: Public, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:4 (#0), tokens: None }, ident: foo#0, kind: Fn(Fn { defaultness: Final, generics: Generics { params: [], where_clause: WhereClause { has_where_token: false, predicates: [], span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:13: 6:13 (#0) }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:11: 6:11 (#0) }, sig: FnSig { header: FnHeader { unsafety: No, asyncness: No, constness: No, ext: None }, decl: FnDecl { inputs: [], output: Default(/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 6:14 (#0)) }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:13 (#0) }, body: Some(Block { stmts: [Stmt { id: NodeId(12), kind: Semi(Expr { id: NodeId(13), kind: Block(Block { stmts: [Stmt { id: NodeId(15), kind: Semi(Expr { id: NodeId(16), kind: Call(Expr { id: NodeId(17), kind: Path(None, Path { span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), segments: [PathSegment { ident: $crate#5, id: NodeId(18), args: None }, PathSegment { ident: io#5, id: NodeId(19), args: None }, PathSegment { ident: _print#5, id: NodeId(20), args: None }], tokens: None }), span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), attrs: [], tokens: None }, [Expr { id: NodeId(21), kind: Call(Expr { id: NodeId(22), kind: Path(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6), segments: [PathSegment { ident: $crate#6, id: NodeId(23), args: None }, PathSegment { ident: fmt#0, id: NodeId(24), args: None }, PathSegment { ident: Arguments#0, id: NodeId(25), args: None }, PathSegment { ident: new_v1#0, id: NodeId(26), args: None }], tokens: None }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6), attrs: [], tokens: None }, [Expr { id: NodeId(27), kind: AddrOf(Ref, Not, Expr { id: NodeId(28), kind: Array([Expr { id: NodeId(29), kind: Lit(Lit { token_lit: Lit { kind: Str, symbol: "\\n", suffix: None }, kind: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4), attrs: [], tokens: None }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4), attrs: [], tokens: None }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4), attrs: [], tokens: None }, Expr { id: NodeId(30), kind: AddrOf(Ref, Not, Expr { id: NodeId(31), kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6), attrs: [], tokens: None }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6), attrs: [], tokens: None }]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6), attrs: [], tokens: None }]), span: /checkout/library/std/src/macros.rs:80:9: 80:59 (#5), attrs: [], tokens: None }), span: /checkout/library/std/src/macros.rs:80:9: 80:60 (#5) }], id: NodeId(14), rules: Default, span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5), tokens: None, could_be_bare_literal: false }, None), span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5), attrs: [], tokens: None }), span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5) }], id: NodeId(11), rules: Default, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0), tokens: None, could_be_bare_literal: false }) }), tokens: None }
├─┐rustc_ast_lowering::with_hir_id_owner owner=NodeId(9)
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(9)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 0 }
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), lower_path_segment(segment: PathSegment { ident: $crate#5, id: NodeId(18), args: None })
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(18)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=$crate#5 original-id=NodeId(18) new-id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), lower_path_segment(segment: PathSegment { ident: io#5, id: NodeId(19), args: None })
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(19)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=io#5 original-id=NodeId(19) new-id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), lower_path_segment(segment: PathSegment { ident: _print#5, id: NodeId(20), args: None })
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(20)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=_print#5 original-id=NodeId(20) new-id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(17)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 4 }
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6), lower_path_segment(segment: PathSegment { ident: $crate#6, id: NodeId(23), args: None })
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(23)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=$crate#6 original-id=NodeId(23) new-id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6), lower_path_segment(segment: PathSegment { ident: fmt#0, id: NodeId(24), args: None })
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(24)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=fmt#0 original-id=NodeId(24) new-id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6), lower_path_segment(segment: PathSegment { ident: Arguments#0, id: NodeId(25), args: None })
│ ├─0ms DEBUG rustc_ast_lowering::path expected_lifetimes=1
│ ├─┐rustc_ast_lowering::new_named_lifetime id=NodeId(32), new_id=NodeId(32), span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#0), ident='_#0
│ │ ├─┐rustc_ast_lowering::new_named_lifetime_with_res id=NodeId(32), span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#0), ident='_#0, res=Infer
│ │ │ ├─0ms DEBUG rustc_ast_lowering name=Infer
│ │ │ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(32)
│ │ │ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }
│ │ ├─┘
│ ├─┘
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(25)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=Arguments#0 original-id=NodeId(25) new-id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }
│ ├─┐rustc_ast_lowering::next_id 
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path path_span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6), lower_path_segment(segment: PathSegment { ident: new_v1#0, id: NodeId(26), args: None })
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(26)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }
│ ├─┘
│ ├─0ms DEBUG rustc_ast_lowering::path lower_path_segment: ident=new_v1#0 original-id=NodeId(26) new-id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(22)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(29)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(28)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(27)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(31)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(30)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(21)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 17 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(16)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 18 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(15)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 19 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(14)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 20 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(13)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 21 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(12)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 22 }
│ ├─┘
│ ├─┐rustc_ast_lowering::lower_node_id ast_node_id=NodeId(11)
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 23 }
│ ├─┐rustc_ast_lowering::next_id 
│ ├─┐rustc_ast_lowering::next_id 
│ │ ├─0ms DEBUG rustc_ast_lowering return=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 24 }
│ ├─┘
│ ├─┐rustc_ast_lowering::item::lower_generics generics=Generics { params: [], where_clause: WhereClause { has_where_token: false, predicates: [], span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:13: 6:13 (#0) }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:11: 6:11 (#0) }, parent_node_id=NodeId(9), itctx=Universal
│ │ ├─┐rustc_ast_lowering::lower_fn_decl decl=FnDecl { inputs: [], output: Default(/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 6:14 (#0)) }, fn_node_id=Some(NodeId(9)), fn_span=/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:13 (#0), kind=Fn, make_ret_async=None
│ ├─┘
│ ├─┘
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(4334) len=331440
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(3104) len=331440
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(49005) len=1206624
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_metadata::rmeta::table LazyTable::lookup: index=DefIndex(10077) len=1206624
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─0ms DEBUG rustc_hir::definitions def_path_hash(DefIndex(3)) = DefPathHash(Fingerprint(10139346924027820109, 8846824014228467133))
│ ├─┐rustc_ast_lowering::index::index_hir item=Item(Item { ident: foo#0, def_id: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, kind: Fn(FnSig { header: FnHeader { unsafety: Normal, constness: NotConst, asyncness: NotAsync, abi: Rust }, decl: FnDecl { inputs: [], output: DefaultReturn(/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 6:14 (#0)), c_variadic: false, implicit_self: None }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:13 (#0) }, Generics { params: [], predicates: [], has_where_clause_predicates: false, where_clause_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:13: 6:13 (#0), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:11: 6:11 (#0) }, BodyId { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 24 } }), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 8:2 (#0), vis_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:4 (#0) })
│ │ ├─┐rustc_ast_lowering::index::visit_item i=Item { ident: foo#0, def_id: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, kind: Fn(FnSig { header: FnHeader { unsafety: Normal, constness: NotConst, asyncness: NotAsync, abi: Rust }, decl: FnDecl { inputs: [], output: DefaultReturn(/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 6:14 (#0)), c_variadic: false, implicit_self: None }, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:13 (#0) }, Generics { params: [], predicates: [], has_where_clause_predicates: false, where_clause_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:13: 6:13 (#0), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:11: 6:11 (#0) }, BodyId { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 24 } }), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 8:2 (#0), vis_span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:1: 6:4 (#0) }
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 24 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 24 }, kind: Block(Block { stmts: [Stmt { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 22 }, kind: Semi(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 21 }, kind: Block(Block { stmts: [Stmt { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }, res: Err, args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }, res: Def(Mod, DefId(1:3104 ~ std[6705]::io)), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }, res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6), res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, res: Err, args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:9: 80:59 (#5) }), span: /checkout/library/std/src/macros.rs:80:9: 80:60 (#5) }], expr: None, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 20 }, rules: DefaultBlock, span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5), targeted_by_break: false }, None), span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5) }), span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5) }], expr: None, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 23 }, rules: DefaultBlock, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0), targeted_by_break: false }, None), span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 23 }, node=Block(Block { stmts: [Stmt { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 22 }, kind: Semi(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 21 }, kind: Block(Block { stmts: [Stmt { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }, res: Err, args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }, res: Def(Mod, DefId(1:3104 ~ std[6705]::io)), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }, res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6), res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, res: Err, args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:9: 80:59 (#5) }), span: /checkout/library/std/src/macros.rs:80:9: 80:60 (#5) }], expr: None, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 20 }, rules: DefaultBlock, span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5), targeted_by_break: false }, None), span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5) }), span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5) }], expr: None, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 23 }, rules: DefaultBlock, span: /checkout/src/test/incremental/auxiliary/rustc-rust-log-aux.rs:6:14: 8:2 (#0), targeted_by_break: false })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:79:23: 81:6 (#5), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 22 }, node=Stmt(Stmt { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 22 }, kind: Semi(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 21 }, kind: Block(Block { stmts: [Stmt { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }, res: Err, args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }, res: Def(Mod, DefId(1:3104 ~ std[6705]::io)), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }, res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6), res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, res: Err, args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:9: 80:59 (#5) }), span: /checkout/library/std/src/macros.rs:80:9: 80:60 (#5) }], expr: None, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 20 }, rules: DefaultBlock, span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5), targeted_by_break: false }, None), span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5) }), span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:79:23: 81:6 (#5), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 21 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 21 }, kind: Block(Block { stmts: [Stmt { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }, res: Err, args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }, res: Def(Mod, DefId(1:3104 ~ std[6705]::io)), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }, res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6), res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, res: Err, args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:9: 80:59 (#5) }), span: /checkout/library/std/src/macros.rs:80:9: 80:60 (#5) }], expr: None, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 20 }, rules: DefaultBlock, span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5), targeted_by_break: false }, None), span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:79:23: 81:6 (#5), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 20 }, node=Block(Block { stmts: [Stmt { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }, res: Err, args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }, res: Def(Mod, DefId(1:3104 ~ std[6705]::io)), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }, res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6), res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, res: Err, args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:9: 80:59 (#5) }), span: /checkout/library/std/src/macros.rs:80:9: 80:60 (#5) }], expr: None, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 20 }, rules: DefaultBlock, span: /checkout/library/std/src/macros.rs:79:23: 81:6 (#5), targeted_by_break: false })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:9: 80:60 (#5), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 19 }, node=Stmt(Stmt { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 19 }, kind: Semi(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }, res: Err, args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }, res: Def(Mod, DefId(1:3104 ~ std[6705]::io)), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }, res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6), res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, res: Err, args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:9: 80:59 (#5) }), span: /checkout/library/std/src/macros.rs:80:9: 80:60 (#5) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:9: 80:59 (#5), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 18 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 18 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }, res: Err, args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }, res: Def(Mod, DefId(1:3104 ~ std[6705]::io)), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }, res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6), res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, res: Err, args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:9: 80:59 (#5) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:9: 80:27 (#5), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 4 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 4 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5), res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), segments: [PathSegment { ident: $crate#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }, res: Err, args: None, infer_args: true }, PathSegment { ident: io#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }, res: Def(Mod, DefId(1:3104 ~ std[6705]::io)), args: None, infer_args: true }, PathSegment { ident: _print#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }, res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), args: None, infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:9: 80:27 (#5) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:9: 80:15 (#5), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }, node=PathSegment(PathSegment { ident: $crate#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 1 }, res: Err, args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:17: 80:19 (#5), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }, node=PathSegment(PathSegment { ident: io#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 2 }, res: Def(Mod, DefId(1:3104 ~ std[6705]::io)), args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:21: 80:27 (#5), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }, node=PathSegment(PathSegment { ident: _print#5, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 3 }, res: Def(Fn, DefId(1:4334 ~ std[6705]::io::stdio::_print)), args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#6), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 17 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 17 }, kind: Call(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6), res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, res: Err, args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }, [Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#6), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 11 }, kind: Path(TypeRelative(Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6), res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6) }, PathSegment { ident: new_v1#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, res: Err, args: None, infer_args: true })), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:28: 80:28 (#6), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, node=Ty(Ty { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 9 }, kind: Path(Resolved(None, Path { span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6), res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), segments: [PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true }, PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true }, PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true }] })), span: /checkout/library/std/src/macros.rs:80:28: 80:28 (#6) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#6), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, node=PathSegment(PathSegment { ident: $crate#6, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 5 }, res: Err, args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#0), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, node=PathSegment(PathSegment { ident: fmt#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 6 }, res: Def(Mod, DefId(2:10077 ~ core[0844]::fmt)), args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#0), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, node=PathSegment(PathSegment { ident: Arguments#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 8 }, res: Def(Struct, DefId(2:49005 ~ core[0844]::fmt::Arguments)), args: Some(GenericArgs { args: [Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })], bindings: [], parenthesized: false, span_ext: /checkout/library/std/src/macros.rs:80:58: 80:58 (#6) }), infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#0), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, node=Lifetime(Lifetime { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 7 }, span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#0), name: Infer })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#0), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, node=PathSegment(PathSegment { ident: new_v1#0, hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 10 }, res: Err, args: None, infer_args: true })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:133:24: 133:28 (#4), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 14 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:133:24: 133:28 (#4), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 13 }, kind: Array([Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }]), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:133:24: 133:28 (#4), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 12 }, kind: Lit(Spanned { node: Str("\n", Cooked), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) }), span: /checkout/library/std/src/macros.rs:133:24: 133:28 (#4) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#6), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 16 }, kind: AddrOf(Ref, Not, Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) }), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) })
│ │ │ ├─┘
│ │ │ ├─┐rustc_ast_lowering::index::insert span=/checkout/library/std/src/macros.rs:80:28: 80:58 (#6), hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, node=Expr(Expr { hir_id: HirId { owner: OwnerId { def_id: DefId(0:3 ~ rustc_rust_log_aux[8cb6]::foo) }, local_id: 15 }, kind: Array([]), span: /checkout/library/std/src/macros.rs:80:28: 80:58 (#6) })
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
DEBUG rustc_query_system::dep_graph try_force_from_dep_node(hir_attrs(rustc_rust_log_aux[8cb6]::{use#0})) --- trying to force
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::{use#0})) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::{use#0})) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- managed to FORCE dependency hir_attrs(rustc_rust_log_aux[8cb6]::{use#0}) to green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- state of dependency hir_attrs(rustc_rust_log_aux[8cb6]::std) (8cb6324e8d20b84d-a4ebeafe30299a14) is unknown, trying to mark it green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::std)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::std)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- trying to force dependency hir_attrs(rustc_rust_log_aux[8cb6]::std)
DEBUG rustc_query_system::dep_graph try_force_from_dep_node(hir_attrs(rustc_rust_log_aux[8cb6]::std)) --- trying to force
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::std)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::std)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- managed to FORCE dependency hir_attrs(rustc_rust_log_aux[8cb6]::std) to green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- state of dependency hir_attrs(rustc_rust_log_aux[8cb6]::foo) (8cb6324e8d20b84d-7ac63b91867badbd) is unknown, trying to mark it green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::foo)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- trying to force dependency hir_attrs(rustc_rust_log_aux[8cb6]::foo)
DEBUG rustc_query_system::dep_graph try_force_from_dep_node(hir_attrs(rustc_rust_log_aux[8cb6]::foo)) --- trying to force
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::foo)) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(hir_attrs(rustc_rust_log_aux[8cb6]::foo)) - END - dependency hir_crate(0-0) was immediately red
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) --- managed to FORCE dependency hir_attrs(rustc_rust_log_aux[8cb6]::foo) to green
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(proc_macro_decls_static(0-0)) - END - successfully marked as green
thread 'rustc' panicked at 'Failed to executed query', /checkout/compiler/rustc_middle/src/ty/query.rs:383:1
   0:     0x7f8c2415739e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   0:     0x7f8c2415739e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66599dcf9f596ce3
   1:     0x7f8c241c0ac8 - core::fmt::write::hb7fe745282c03ea1
   2:     0x7f8c24148c11 - std::io::Write::write_fmt::hc7de14f5f597cf44
   3:     0x7f8c2415a33e - std::panicking::default_hook::{{closure}}::hcba55d18ff4c9330
   4:     0x7f8c24159fe9 - std::panicking::default_hook::h3eed926b05923398
   5:     0x7f8c24afb5b4 - rustc_driver[79d2ddbc50f08d9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f8c2415abd4 - std::panicking::rust_panic_with_hook::h387fbfdb6a9d6321
   7:     0x7f8c2415a8f9 - std::panicking::begin_panic_handler::{{closure}}::he9f84735e433f90d
   8:     0x7f8c241578d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbccfbc640fce1c0d
   9:     0x7f8c2415a602 - rust_begin_unwind
  10:     0x7f8c2410ba33 - core::panicking::panic_fmt::h143efa9e25241140
  11:     0x7f8c24c37fba - <rustc_session[6c99766a50e8c377]::session::Session>::time::<(), rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#0}::{closure#1}>
  12:     0x7f8c24cb8fd4 - <core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#0}> as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once
  13:     0x7f8c24c1624c - std[670518f464012ae8]::panicking::try::<(), core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[7a6d25e97b1d2994]::passes::analysis::{closure#0}::{closure#0}>>
  14:     0x7f8c24c50b65 - rustc_interface[7a6d25e97b1d2994]::passes::analysis
  15:     0x7f8c263a09f4 - <rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind as rustc_query_system[9892bac3bd80dc24]::dep_graph::DepKind>::with_deps::<<rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  16:     0x7f8c26553600 - <rustc_query_system[9892bac3bd80dc24]::dep_graph::graph::DepGraph<rustc_middle[92cbfa92439caac6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[92cbfa92439caac6]::ty::context::TyCtxt, (), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  17:     0x7f8c267836c4 - rustc_query_system[9892bac3bd80dc24]::query::plumbing::try_execute_query::<rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt, rustc_query_system[9892bac3bd80dc24]::query::caches::DefaultCache<(), core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>>
  18:     0x7f8c2686bd5b - rustc_query_system[9892bac3bd80dc24]::query::plumbing::get_query::<rustc_query_impl[475ebef8c48f8323]::queries::analysis, rustc_query_impl[475ebef8c48f8323]::plumbing::QueryCtxt>
  19:     0x7f8c266d21f7 - <rustc_query_impl[475ebef8c48f8323]::Queries as rustc_middle[92cbfa92439caac6]::ty::query::QueryEngine>::execute
  20:     0x7f8c24b690a0 - <rustc_interface[7a6d25e97b1d2994]::passes::QueryContext>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  21:     0x7f8c24afd9ba - <rustc_interface[7a6d25e97b1d2994]::interface::Compiler>::enter::<rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}::{closure#2}, core[8441253fc0f89e2]::result::Result<core[8441253fc0f89e2]::option::Option<rustc_interface[7a6d25e97b1d2994]::queries::Linker>, rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  22:     0x7f8c24ae84fe - rustc_span[2a207b77041a0058]::with_source_map::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#1}>
  23:     0x7f8c24b12082 - rustc_interface[7a6d25e97b1d2994]::interface::create_compiler_and_run::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>
  24:     0x7f8c24b72a2f - <scoped_tls[8a59c43d1c872517]::ScopedKey<rustc_span[2a207b77041a0058]::SessionGlobals>>::set::<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  25:     0x7f8c24b2ba9f - std[670518f464012ae8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>
  26:     0x7f8c24aea496 - std[670518f464012ae8]::panicking::try::<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, core[8441253fc0f89e2]::panic::unwind_safe::AssertUnwindSafe<<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  27:     0x7f8c24b2082a - <<std[670518f464012ae8]::thread::Builder>::spawn_unchecked_<rustc_interface[7a6d25e97b1d2994]::util::run_in_thread_pool_with_globals<rustc_interface[7a6d25e97b1d2994]::interface::run_compiler<core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>, rustc_driver[79d2ddbc50f08d9c]::run_compiler::{closure#1}>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#0}, core[8441253fc0f89e2]::result::Result<(), rustc_errors[a045e816fee36253]::ErrorGuaranteed>>::{closure#1} as core[8441253fc0f89e2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  28:     0x7f8c24167445 - std::sys::unix::thread::Thread::new::thread_start::h4af8bd76c52c9d81
  29:     0x7f8c23f01b43 - <unknown>
  30:     0x7f8c23f93a00 - <unknown>
  31:                0x0 - <unknown>

DEBUG rustc_errors diagnostic=Diagnostic { level: Bug, message: [(Str("unexpected panic"), NoStyle)], code: None, span: MultiSpan { primary_spans: [], span_labels: [] }, children: [], suggestions: Ok([]), args: {}, sort_span: no-location (#0), is_lint: false }
DEBUG rustc_errors self.emitted_diagnostics={}
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]

DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors diagnostic=Diagnostic { level: Note, message: [(Str("the compiler unexpectedly panicked. this is a bug."), NoStyle)], code: None, span: MultiSpan { primary_spans: [], span_labels: [] }, children: [], suggestions: Ok([]), args: {}, sort_span: no-location (#0), is_lint: false }
DEBUG rustc_errors self.emitted_diagnostics={}
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]

DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors diagnostic=Diagnostic { level: Note, message: [(Str("we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md"), NoStyle)], code: None, span: MultiSpan { primary_spans: [], span_labels: [] }, children: [], suggestions: Ok([]), args: {}, sort_span: no-location (#0), is_lint: false }
DEBUG rustc_errors self.emitted_diagnostics={}
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]

DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors diagnostic=Diagnostic { level: Note, message: [(Str("rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu"), NoStyle)], code: None, span: MultiSpan { primary_spans: [], span_labels: [] }, children: [], suggestions: Ok([]), args: {}, sort_span: no-location (#0), is_lint: false }
DEBUG rustc_errors self.emitted_diagnostics={}
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]
note: rustc 1.66.0-nightly (117eba639 2022-10-15) running on x86_64-unknown-linux-gnu
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors::diagnostic_builder Created new diagnostic
DEBUG rustc_errors diagnostic=Diagnostic { level: Note, message: [(Str("compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib"), NoStyle)], code: None, span: MultiSpan { primary_spans: [], span_labels: [] }, children: [], suggestions: Ok([]), args: {}, sort_span: no-location (#0), is_lint: false }
DEBUG rustc_errors self.emitted_diagnostics={}
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]
note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
DEBUG rustc_errors::emitter emit_diagnostic: suggestions=[]
#0 [analysis] running analysis passes on this crate
end of query stack
DEBUG rustc_hir::definitions def_path_hash(DefIndex(0)) = DefPathHash(Fingerprint(10139346924027820109, 2504381839606093412))
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) - BEGIN
DEBUG rustc_query_system::dep_graph::graph try_mark_previous_green(check_mod_loops(rustc_rust_log_aux[8cb6])) --- found dependency hir_module_items(rustc_rust_log_aux[8cb6]) to be immediately green
