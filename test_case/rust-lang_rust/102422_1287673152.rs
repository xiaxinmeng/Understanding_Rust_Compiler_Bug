plain
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [incremental] src/test/incremental/async-lifetimes.rs stdout ----

error in revision `rpass1`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/async-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/async-lifetimes/async-lifetimes.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/async-lifetimes/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/async-lifetimes/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'region variables should not be hashed: '_#1r', /checkout/compiler/rustc_middle/src/ty/sty.rs:1362:9
stack backtrace:
   0:     0x7f27ae44756e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h98fbb72dceaf3818
   1:     0x7f27ae4b0b98 - core::fmt::write::hab2960ba43fd827a
   2:     0x7f27ae438be1 - std::io::Write::write_fmt::h39b608778a03586b
   3:     0x7f27ae447371 - std::sys_common::backtrace::print::h18253e554d2e60f3
   4:     0x7f27ae44a4f4 - std::panicking::default_hook::{{closure}}::hdaa09829cdd1bc1d
   5:     0x7f27ae44a1b9 - std::panicking::default_hook::he65e50de91ab8355
   6:     0x7f27aee36b74 - rustc_driver[5c0ee7d7462d1c9f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f27ae44ac44 - std::panicking::rust_panic_with_hook::h3e5963b42a7f8a2e
   8:     0x7f27ae44a9a7 - std::panicking::begin_panic_handler::{{closure}}::ha393dc5d1b9648af
   9:     0x7f27ae447aa4 - std::sys_common::backtrace::__rust_end_short_backtrace::h8839bb33be53325f
  10:     0x7f27ae44a672 - rust_begin_unwind
  11:     0x7f27ae3fba93 - core::panicking::panic_fmt::hca02b6aaece51c91
  12:     0x7f27b088c78f - <rustc_middle[d02cd6aa03a7ddd6]::ty::sty::RegionVid as rustc_data_structures[8f773674fc6616d0]::stable_hasher::HashStable<rustc_query_system[f47214f64d44928c]::ich::hcx::StableHashingContext>>::hash_stable
  13:     0x7f27b074b841 - <[rustc_middle[d02cd6aa03a7ddd6]::mir::query::ClosureOutlivesRequirement] as rustc_data_structures[8f773674fc6616d0]::stable_hasher::HashStable<rustc_query_system[f47214f64d44928c]::ich::hcx::StableHashingContext>>::hash_stable
  14:     0x7f27b0970dd0 - rustc_query_system[f47214f64d44928c]::dep_graph::graph::hash_result::<&rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>
  15:     0x7f27b09054a1 - <rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>
  16:     0x7f27b0b212ec - rustc_query_system[f47214f64d44928c]::query::plumbing::try_execute_query::<rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt, rustc_query_system[f47214f64d44928c]::query::caches::DefaultCache<rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>>
  17:     0x7f27b0bfdf05 - rustc_query_system[f47214f64d44928c]::query::plumbing::get_query::<rustc_query_impl[4914c0a2da274fd8]::queries::mir_borrowck, rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt>
  18:     0x7f27b07c69d0 - <rustc_query_impl[4914c0a2da274fd8]::Queries as rustc_middle[d02cd6aa03a7ddd6]::ty::query::QueryEngine>::mir_borrowck
  19:     0x7f27b011f320 - <rustc_borrowck[a3034d8a0c2a56cd]::type_check::TypeChecker>::prove_closure_bounds
  20:     0x7f27b012de88 - <rustc_borrowck[a3034d8a0c2a56cd]::type_check::TypeChecker>::check_rvalue
  21:     0x7f27b0133ec1 - <rustc_borrowck[a3034d8a0c2a56cd]::type_check::TypeChecker>::typeck_mir
  22:     0x7f27b011552e - rustc_borrowck[a3034d8a0c2a56cd]::type_check::type_check
  23:     0x7f27afffa9ce - rustc_borrowck[a3034d8a0c2a56cd]::nll::compute_regions
  24:     0x7f27b01a74fa - rustc_borrowck[a3034d8a0c2a56cd]::do_mir_borrowck
  25:     0x7f27b0198802 - rustc_borrowck[a3034d8a0c2a56cd]::mir_borrowck
  26:     0x7f27b016610e - <rustc_borrowck[a3034d8a0c2a56cd]::provide::{closure#0} as core[5ef01f31e7059763]::ops::function::FnOnce<(rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId)>>::call_once
  27:     0x7f27b0700f6d - <rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind as rustc_query_system[f47214f64d44928c]::dep_graph::DepKind>::with_deps::<<rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>::{closure#0}, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>
  28:     0x7f27b09052bc - <rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>
  29:     0x7f27b0b212ec - rustc_query_system[f47214f64d44928c]::query::plumbing::try_execute_query::<rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt, rustc_query_system[f47214f64d44928c]::query::caches::DefaultCache<rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>>
  30:     0x7f27b0bfdf05 - rustc_query_system[f47214f64d44928c]::query::plumbing::get_query::<rustc_query_impl[4914c0a2da274fd8]::queries::mir_borrowck, rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt>
  31:     0x7f27b07c69d0 - <rustc_query_impl[4914c0a2da274fd8]::Queries as rustc_middle[d02cd6aa03a7ddd6]::ty::query::QueryEngine>::mir_borrowck
  32:     0x7f27af64713e - rustc_hir_analysis[e529b11f89f5620a]::collect::type_of::find_opaque_ty_constraints_for_rpit
  33:     0x7f27af64638c - rustc_hir_analysis[e529b11f89f5620a]::collect::type_of::type_of
  34:     0x7f27b0702b58 - <rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind as rustc_query_system[f47214f64d44928c]::dep_graph::DepKind>::with_deps::<<rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::DefId, rustc_middle[d02cd6aa03a7ddd6]::ty::Ty>::{closure#0}, rustc_middle[d02cd6aa03a7ddd6]::ty::Ty>
  35:     0x7f27b09157a8 - <rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::DefId, rustc_middle[d02cd6aa03a7ddd6]::ty::Ty>
  36:     0x7f27b0b3a4bf - rustc_query_system[f47214f64d44928c]::query::plumbing::try_execute_query::<rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt, rustc_query_system[f47214f64d44928c]::query::caches::DefaultCache<rustc_span[5d3150c7eb36bde8]::def_id::DefId, rustc_middle[d02cd6aa03a7ddd6]::ty::Ty>>
  37:     0x7f27b0c4fa07 - rustc_query_system[f47214f64d44928c]::query::plumbing::get_query::<rustc_query_impl[4914c0a2da274fd8]::queries::type_of, rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt>
  38:     0x7f27b0799765 - <rustc_query_impl[4914c0a2da274fd8]::Queries as rustc_middle[d02cd6aa03a7ddd6]::ty::query::QueryEngine>::type_of
  39:     0x7f27af5e02d3 - rustc_hir_analysis[e529b11f89f5620a]::check::check::check_opaque
  40:     0x7f27af5e3e6b - rustc_hir_analysis[e529b11f89f5620a]::check::check::check_item_type
  41:     0x7f27af5ee1ea - rustc_hir_analysis[e529b11f89f5620a]::check::check::check_mod_item_types
  42:     0x7f27b070120d - <rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind as rustc_query_system[f47214f64d44928c]::dep_graph::DepKind>::with_deps::<<rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, ()>::{closure#0}, ()>
  43:     0x7f27b0906c3f - <rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, ()>
  44:     0x7f27b0b23c1c - rustc_query_system[f47214f64d44928c]::query::plumbing::try_execute_query::<rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt, rustc_query_system[f47214f64d44928c]::query::caches::DefaultCache<rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, ()>>
  45:     0x7f27b0c1bc5a - rustc_query_system[f47214f64d44928c]::query::plumbing::get_query::<rustc_query_impl[4914c0a2da274fd8]::queries::check_mod_item_types, rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt>
  46:     0x7f27b07bf3c0 - <rustc_query_impl[4914c0a2da274fd8]::Queries as rustc_middle[d02cd6aa03a7ddd6]::ty::query::QueryEngine>::check_mod_item_types
  47:     0x7f27af61d87a - <rustc_session[c8592ead5602c53f]::session::Session>::time::<(), rustc_hir_analysis[e529b11f89f5620a]::check_crate::{closure#6}>
  48:     0x7f27af5ceed1 - rustc_hir_analysis[e529b11f89f5620a]::check_crate
  49:     0x7f27aef88631 - rustc_interface[4546c02f834b2b89]::passes::analysis
  50:     0x7f27b0708bf4 - <rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind as rustc_query_system[f47214f64d44928c]::dep_graph::DepKind>::with_deps::<<rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, (), core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  51:     0x7f27b094db90 - <rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, (), core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  52:     0x7f27b0b6aac4 - rustc_query_system[f47214f64d44928c]::query::plumbing::try_execute_query::<rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt, rustc_query_system[f47214f64d44928c]::query::caches::DefaultCache<(), core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>>
  53:     0x7f27b0c4fb1b - rustc_query_system[f47214f64d44928c]::query::plumbing::get_query::<rustc_query_impl[4914c0a2da274fd8]::queries::analysis, rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt>
  54:     0x7f27b079a64a - <rustc_query_impl[4914c0a2da274fd8]::Queries as rustc_middle[d02cd6aa03a7ddd6]::ty::query::QueryEngine>::analysis
  55:     0x7f27aee90909 - <rustc_interface[4546c02f834b2b89]::passes::QueryContext>::enter::<rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  56:     0x7f27aeea5f8a - <rustc_interface[4546c02f834b2b89]::interface::Compiler>::enter::<rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}::{closure#2}, core[5ef01f31e7059763]::result::Result<core[5ef01f31e7059763]::option::Option<rustc_interface[4546c02f834b2b89]::queries::Linker>, rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  57:     0x7f27aee382be - rustc_span[5d3150c7eb36bde8]::with_source_map::<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  58:     0x7f27aee9960c - <scoped_tls[429f8f3b9bde313a]::ScopedKey<rustc_span[5d3150c7eb36bde8]::SessionGlobals>>::set::<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  59:     0x7f27aee94159 - std[abe383952cf43c10]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4546c02f834b2b89]::util::run_in_thread_pool_with_globals<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  60:     0x7f27aee9e8d8 - std[abe383952cf43c10]::panic::catch_unwind::<core[5ef01f31e7059763]::panic::unwind_safe::AssertUnwindSafe<<std[abe383952cf43c10]::thread::Builder>::spawn_unchecked_<rustc_interface[4546c02f834b2b89]::util::run_in_thread_pool_with_globals<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  61:     0x7f27aee4a4aa - <<std[abe383952cf43c10]::thread::Builder>::spawn_unchecked_<rustc_interface[4546c02f834b2b89]::util::run_in_thread_pool_with_globals<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#1} as core[5ef01f31e7059763]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7f27ae4574b5 - std::sys::unix::thread::Thread::new::thread_start::h9ac5b9f825d20742
  63:     0x7f27ae1f1b43 - <unknown>
  64:     0x7f27ae283a00 - <unknown>
  65:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (8f7e4fcbc 2022-10-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `<impl at /checkout/src/test/incremental/async-lifetimes.rs:10:1: 10:9>::f::{closure#0}`
#1 [mir_borrowck] borrow-checking `<impl at /checkout/src/test/incremental/async-lifetimes.rs:10:1: 10:9>::f`
#2 [type_of] computing type of `<impl at /checkout/src/test/incremental/async-lifetimes.rs:10:1: 10:9>::f::{opaque#0}`
#3 [check_mod_item_types] checking item types in top-level module
#4 [analysis] running analysis passes on this crate
------------------------------------------


---- [incremental] src/test/incremental/issue-80691-bad-eval-cache.rs stdout ----
---- [incremental] src/test/incremental/issue-80691-bad-eval-cache.rs stdout ----

error in revision `rfail1`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-80691-bad-eval-cache.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-80691-bad-eval-cache/issue-80691-bad-eval-cache.inc" "-Z" "incremental-verify-ich" "-O" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-80691-bad-eval-cache/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-80691-bad-eval-cache/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'region variables should not be hashed: '_#1r', /checkout/compiler/rustc_middle/src/ty/sty.rs:1362:9
stack backtrace:
   0:     0x7f3f92f8956e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h98fbb72dceaf3818
   1:     0x7f3f92ff2b98 - core::fmt::write::hab2960ba43fd827a
   2:     0x7f3f92f7abe1 - std::io::Write::write_fmt::h39b608778a03586b
   3:     0x7f3f92f89371 - std::sys_common::backtrace::print::h18253e554d2e60f3
   4:     0x7f3f92f8c4f4 - std::panicking::default_hook::{{closure}}::hdaa09829cdd1bc1d
   5:     0x7f3f92f8c1b9 - std::panicking::default_hook::he65e50de91ab8355
   6:     0x7f3f93978b74 - rustc_driver[5c0ee7d7462d1c9f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3f92f8cc44 - std::panicking::rust_panic_with_hook::h3e5963b42a7f8a2e
   8:     0x7f3f92f8c9a7 - std::panicking::begin_panic_handler::{{closure}}::ha393dc5d1b9648af
   9:     0x7f3f92f89aa4 - std::sys_common::backtrace::__rust_end_short_backtrace::h8839bb33be53325f
  10:     0x7f3f92f8c672 - rust_begin_unwind
  11:     0x7f3f92f3da93 - core::panicking::panic_fmt::hca02b6aaece51c91
  12:     0x7f3f953ce78f - <rustc_middle[d02cd6aa03a7ddd6]::ty::sty::RegionVid as rustc_data_structures[8f773674fc6616d0]::stable_hasher::HashStable<rustc_query_system[f47214f64d44928c]::ich::hcx::StableHashingContext>>::hash_stable
  13:     0x7f3f9528d841 - <[rustc_middle[d02cd6aa03a7ddd6]::mir::query::ClosureOutlivesRequirement] as rustc_data_structures[8f773674fc6616d0]::stable_hasher::HashStable<rustc_query_system[f47214f64d44928c]::ich::hcx::StableHashingContext>>::hash_stable
  14:     0x7f3f954b2dd0 - rustc_query_system[f47214f64d44928c]::dep_graph::graph::hash_result::<&rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>
  15:     0x7f3f954474a1 - <rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>
  16:     0x7f3f956632ec - rustc_query_system[f47214f64d44928c]::query::plumbing::try_execute_query::<rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt, rustc_query_system[f47214f64d44928c]::query::caches::DefaultCache<rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>>
  17:     0x7f3f9573ff05 - rustc_query_system[f47214f64d44928c]::query::plumbing::get_query::<rustc_query_impl[4914c0a2da274fd8]::queries::mir_borrowck, rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt>
  18:     0x7f3f953089d0 - <rustc_query_impl[4914c0a2da274fd8]::Queries as rustc_middle[d02cd6aa03a7ddd6]::ty::query::QueryEngine>::mir_borrowck
  19:     0x7f3f94c61320 - <rustc_borrowck[a3034d8a0c2a56cd]::type_check::TypeChecker>::prove_closure_bounds
  20:     0x7f3f94c6fe88 - <rustc_borrowck[a3034d8a0c2a56cd]::type_check::TypeChecker>::check_rvalue
  21:     0x7f3f94c75ec1 - <rustc_borrowck[a3034d8a0c2a56cd]::type_check::TypeChecker>::typeck_mir
  22:     0x7f3f94c5752e - rustc_borrowck[a3034d8a0c2a56cd]::type_check::type_check
  23:     0x7f3f94b3c9ce - rustc_borrowck[a3034d8a0c2a56cd]::nll::compute_regions
  24:     0x7f3f94ce94fa - rustc_borrowck[a3034d8a0c2a56cd]::do_mir_borrowck
  25:     0x7f3f94cda802 - rustc_borrowck[a3034d8a0c2a56cd]::mir_borrowck
  26:     0x7f3f94ca810e - <rustc_borrowck[a3034d8a0c2a56cd]::provide::{closure#0} as core[5ef01f31e7059763]::ops::function::FnOnce<(rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId)>>::call_once
  27:     0x7f3f95242f6d - <rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind as rustc_query_system[f47214f64d44928c]::dep_graph::DepKind>::with_deps::<<rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>::{closure#0}, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>
  28:     0x7f3f954472bc - <rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>
  29:     0x7f3f956632ec - rustc_query_system[f47214f64d44928c]::query::plumbing::try_execute_query::<rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt, rustc_query_system[f47214f64d44928c]::query::caches::DefaultCache<rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>>
  30:     0x7f3f9573ff05 - rustc_query_system[f47214f64d44928c]::query::plumbing::get_query::<rustc_query_impl[4914c0a2da274fd8]::queries::mir_borrowck, rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt>
  31:     0x7f3f953089d0 - <rustc_query_impl[4914c0a2da274fd8]::Queries as rustc_middle[d02cd6aa03a7ddd6]::ty::query::QueryEngine>::mir_borrowck
  32:     0x7f3f93b30106 - <core[5ef01f31e7059763]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f773674fc6616d0]::sync::par_for_each_in<&[rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId], <rustc_middle[d02cd6aa03a7ddd6]::hir::map::Map>::par_body_owners<rustc_interface[4546c02f834b2b89]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[5ef01f31e7059763]::ops::function::FnOnce<()>>::call_once
  33:     0x7f3f93a9ebe6 - std[abe383952cf43c10]::panicking::try::<(), core[5ef01f31e7059763]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f773674fc6616d0]::sync::par_for_each_in<&[rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId], <rustc_middle[d02cd6aa03a7ddd6]::hir::map::Map>::par_body_owners<rustc_interface[4546c02f834b2b89]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  34:     0x7f3f93a92afd - rustc_data_structures[8f773674fc6616d0]::sync::par_for_each_in::<&[rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId], <rustc_middle[d02cd6aa03a7ddd6]::hir::map::Map>::par_body_owners<rustc_interface[4546c02f834b2b89]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  35:     0x7f3f93aa7175 - <rustc_session[c8592ead5602c53f]::session::Session>::time::<(), rustc_interface[4546c02f834b2b89]::passes::analysis::{closure#2}>
  36:     0x7f3f93aca66b - rustc_interface[4546c02f834b2b89]::passes::analysis
  37:     0x7f3f9524abf4 - <rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind as rustc_query_system[f47214f64d44928c]::dep_graph::DepKind>::with_deps::<<rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, (), core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  38:     0x7f3f9548fb90 - <rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, (), core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  39:     0x7f3f956acac4 - rustc_query_system[f47214f64d44928c]::query::plumbing::try_execute_query::<rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt, rustc_query_system[f47214f64d44928c]::query::caches::DefaultCache<(), core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>>
  40:     0x7f3f95791b1b - rustc_query_system[f47214f64d44928c]::query::plumbing::get_query::<rustc_query_impl[4914c0a2da274fd8]::queries::analysis, rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt>
  41:     0x7f3f952dc64a - <rustc_query_impl[4914c0a2da274fd8]::Queries as rustc_middle[d02cd6aa03a7ddd6]::ty::query::QueryEngine>::analysis
  42:     0x7f3f939d2909 - <rustc_interface[4546c02f834b2b89]::passes::QueryContext>::enter::<rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  43:     0x7f3f939e7f8a - <rustc_interface[4546c02f834b2b89]::interface::Compiler>::enter::<rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}::{closure#2}, core[5ef01f31e7059763]::result::Result<core[5ef01f31e7059763]::option::Option<rustc_interface[4546c02f834b2b89]::queries::Linker>, rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  44:     0x7f3f9397a2be - rustc_span[5d3150c7eb36bde8]::with_source_map::<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  45:     0x7f3f939db60c - <scoped_tls[429f8f3b9bde313a]::ScopedKey<rustc_span[5d3150c7eb36bde8]::SessionGlobals>>::set::<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  46:     0x7f3f939d6159 - std[abe383952cf43c10]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4546c02f834b2b89]::util::run_in_thread_pool_with_globals<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  47:     0x7f3f939e08d8 - std[abe383952cf43c10]::panic::catch_unwind::<core[5ef01f31e7059763]::panic::unwind_safe::AssertUnwindSafe<<std[abe383952cf43c10]::thread::Builder>::spawn_unchecked_<rustc_interface[4546c02f834b2b89]::util::run_in_thread_pool_with_globals<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  48:     0x7f3f9398c4aa - <<std[abe383952cf43c10]::thread::Builder>::spawn_unchecked_<rustc_interface[4546c02f834b2b89]::util::run_in_thread_pool_with_globals<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#1} as core[5ef01f31e7059763]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7f3f92f994b5 - std::sys::unix::thread::Thread::new::thread_start::h9ac5b9f825d20742
  50:     0x7f3f92d33b43 - <unknown>
  51:     0x7f3f92dc5a00 - <unknown>
  52:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (8f7e4fcbc 2022-10-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `<impl at /checkout/src/test/incremental/issue-80691-bad-eval-cache.rs:145:1: 145:28>::iter_answers::{closure#0}`
#1 [mir_borrowck] borrow-checking `<impl at /checkout/src/test/incremental/issue-80691-bad-eval-cache.rs:145:1: 145:28>::iter_answers`
#2 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'region variables should not be hashed: '_#1r', /checkout/compiler/rustc_middle/src/ty/sty.rs:1362:9
stack backtrace:
   0:     0x7f3f92f8956e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h98fbb72dceaf3818
   1:     0x7f3f92ff2b98 - core::fmt::write::hab2960ba43fd827a
   2:     0x7f3f92f7abe1 - std::io::Write::write_fmt::h39b608778a03586b
   3:     0x7f3f92f89371 - std::sys_common::backtrace::print::h18253e554d2e60f3
   4:     0x7f3f92f8c4f4 - std::panicking::default_hook::{{closure}}::hdaa09829cdd1bc1d
   5:     0x7f3f92f8c1b9 - std::panicking::default_hook::he65e50de91ab8355
   6:     0x7f3f93978b74 - rustc_driver[5c0ee7d7462d1c9f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3f92f8cc44 - std::panicking::rust_panic_with_hook::h3e5963b42a7f8a2e
   8:     0x7f3f92f8c9a7 - std::panicking::begin_panic_handler::{{closure}}::ha393dc5d1b9648af
   9:     0x7f3f92f89aa4 - std::sys_common::backtrace::__rust_end_short_backtrace::h8839bb33be53325f
  10:     0x7f3f92f8c672 - rust_begin_unwind
  11:     0x7f3f92f3da93 - core::panicking::panic_fmt::hca02b6aaece51c91
  12:     0x7f3f953ce78f - <rustc_middle[d02cd6aa03a7ddd6]::ty::sty::RegionVid as rustc_data_structures[8f773674fc6616d0]::stable_hasher::HashStable<rustc_query_system[f47214f64d44928c]::ich::hcx::StableHashingContext>>::hash_stable
  13:     0x7f3f9528d841 - <[rustc_middle[d02cd6aa03a7ddd6]::mir::query::ClosureOutlivesRequirement] as rustc_data_structures[8f773674fc6616d0]::stable_hasher::HashStable<rustc_query_system[f47214f64d44928c]::ich::hcx::StableHashingContext>>::hash_stable
  14:     0x7f3f954b2dd0 - rustc_query_system[f47214f64d44928c]::dep_graph::graph::hash_result::<&rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>
  15:     0x7f3f954474a1 - <rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>
  16:     0x7f3f956632ec - rustc_query_system[f47214f64d44928c]::query::plumbing::try_execute_query::<rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt, rustc_query_system[f47214f64d44928c]::query::caches::DefaultCache<rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId, &rustc_middle[d02cd6aa03a7ddd6]::mir::query::BorrowCheckResult>>
  17:     0x7f3f9573ff05 - rustc_query_system[f47214f64d44928c]::query::plumbing::get_query::<rustc_query_impl[4914c0a2da274fd8]::queries::mir_borrowck, rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt>
  18:     0x7f3f953089d0 - <rustc_query_impl[4914c0a2da274fd8]::Queries as rustc_middle[d02cd6aa03a7ddd6]::ty::query::QueryEngine>::mir_borrowck
  19:     0x7f3f93b30106 - <core[5ef01f31e7059763]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f773674fc6616d0]::sync::par_for_each_in<&[rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId], <rustc_middle[d02cd6aa03a7ddd6]::hir::map::Map>::par_body_owners<rustc_interface[4546c02f834b2b89]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[5ef01f31e7059763]::ops::function::FnOnce<()>>::call_once
  20:     0x7f3f93a9ebe6 - std[abe383952cf43c10]::panicking::try::<(), core[5ef01f31e7059763]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f773674fc6616d0]::sync::par_for_each_in<&[rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId], <rustc_middle[d02cd6aa03a7ddd6]::hir::map::Map>::par_body_owners<rustc_interface[4546c02f834b2b89]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  21:     0x7f3f93a92afd - rustc_data_structures[8f773674fc6616d0]::sync::par_for_each_in::<&[rustc_span[5d3150c7eb36bde8]::def_id::LocalDefId], <rustc_middle[d02cd6aa03a7ddd6]::hir::map::Map>::par_body_owners<rustc_interface[4546c02f834b2b89]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  22:     0x7f3f93aa7175 - <rustc_session[c8592ead5602c53f]::session::Session>::time::<(), rustc_interface[4546c02f834b2b89]::passes::analysis::{closure#2}>
  23:     0x7f3f93aca66b - rustc_interface[4546c02f834b2b89]::passes::analysis
  24:     0x7f3f9524abf4 - <rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind as rustc_query_system[f47214f64d44928c]::dep_graph::DepKind>::with_deps::<<rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, (), core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  25:     0x7f3f9548fb90 - <rustc_query_system[f47214f64d44928c]::dep_graph::graph::DepGraph<rustc_middle[d02cd6aa03a7ddd6]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[d02cd6aa03a7ddd6]::ty::context::TyCtxt, (), core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  26:     0x7f3f956acac4 - rustc_query_system[f47214f64d44928c]::query::plumbing::try_execute_query::<rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt, rustc_query_system[f47214f64d44928c]::query::caches::DefaultCache<(), core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>>
  27:     0x7f3f95791b1b - rustc_query_system[f47214f64d44928c]::query::plumbing::get_query::<rustc_query_impl[4914c0a2da274fd8]::queries::analysis, rustc_query_impl[4914c0a2da274fd8]::plumbing::QueryCtxt>
  28:     0x7f3f952dc64a - <rustc_query_impl[4914c0a2da274fd8]::Queries as rustc_middle[d02cd6aa03a7ddd6]::ty::query::QueryEngine>::analysis
  29:     0x7f3f939d2909 - <rustc_interface[4546c02f834b2b89]::passes::QueryContext>::enter::<rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  30:     0x7f3f939e7f8a - <rustc_interface[4546c02f834b2b89]::interface::Compiler>::enter::<rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}::{closure#2}, core[5ef01f31e7059763]::result::Result<core[5ef01f31e7059763]::option::Option<rustc_interface[4546c02f834b2b89]::queries::Linker>, rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  31:     0x7f3f9397a2be - rustc_span[5d3150c7eb36bde8]::with_source_map::<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  32:     0x7f3f939db60c - <scoped_tls[429f8f3b9bde313a]::ScopedKey<rustc_span[5d3150c7eb36bde8]::SessionGlobals>>::set::<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  33:     0x7f3f939d6159 - std[abe383952cf43c10]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4546c02f834b2b89]::util::run_in_thread_pool_with_globals<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  34:     0x7f3f939e08d8 - std[abe383952cf43c10]::panic::catch_unwind::<core[5ef01f31e7059763]::panic::unwind_safe::AssertUnwindSafe<<std[abe383952cf43c10]::thread::Builder>::spawn_unchecked_<rustc_interface[4546c02f834b2b89]::util::run_in_thread_pool_with_globals<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>
  35:     0x7f3f9398c4aa - <<std[abe383952cf43c10]::thread::Builder>::spawn_unchecked_<rustc_interface[4546c02f834b2b89]::util::run_in_thread_pool_with_globals<rustc_interface[4546c02f834b2b89]::interface::run_compiler<core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>, rustc_driver[5c0ee7d7462d1c9f]::run_compiler::{closure#1}>::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[5ef01f31e7059763]::result::Result<(), rustc_errors[b94fa4278e8f00dd]::ErrorGuaranteed>>::{closure#1} as core[5ef01f31e7059763]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f3f92f994b5 - std::sys::unix::thread::Thread::new::thread_start::h9ac5b9f825d20742
  37:     0x7f3f92d33b43 - <unknown>
  38:     0x7f3f92dc5a00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (8f7e4fcbc 2022-10-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `<impl at /checkout/src/test/incremental/issue-80691-bad-eval-cache.rs:145:1: 145:28>::iter_answers::{closure#2}`
#1 [analysis] running analysis passes on this crate
------------------------------------------



