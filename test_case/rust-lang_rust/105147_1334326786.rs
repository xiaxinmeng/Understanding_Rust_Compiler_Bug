plain
---- [mir-opt] src/test/mir-opt/building/custom/consts.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/building/custom/consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/consts" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/consts" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/consts/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: /checkout/compiler/rustc_middle/src/mir/mod.rs:615:39: unwrapping cross-crate data
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1575:9
stack backtrace:
stack backtrace:
   0:     0x7ffaf1c87ed5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h197489db1f8041c9
   1:     0x7ffaf1cf7ac8 - core::fmt::write::h0810bb2cf209cb58
   2:     0x7ffaf1c79c81 - std::io::Write::write_fmt::hdfa2531741826335
   3:     0x7ffaf1c87ce1 - std::sys_common::backtrace::print::h005d089afd8691f6
   4:     0x7ffaf1c8b044 - std::panicking::default_hook::{{closure}}::h27d4d6f273ecdfab
   5:     0x7ffaf1c8ad0a - std::panicking::default_hook::hd5fc0b1f5e5bd948
   6:     0x7ffaf26d04c4 - rustc_driver[5df8529dbce8ebc5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ffaf1c8b7b4 - std::panicking::rust_panic_with_hook::hd84df5858a1fbe34
   8:     0x7ffaf53d5c23 - std[4d8c08a1d6cd93f8]::panicking::begin_panic::<rustc_errors[7a3c0cce756dd281]::ExplicitBug>::{closure#0}
   9:     0x7ffaf53ce106 - std[4d8c08a1d6cd93f8]::sys_common::backtrace::__rust_end_short_backtrace::<std[4d8c08a1d6cd93f8]::panicking::begin_panic<rustc_errors[7a3c0cce756dd281]::ExplicitBug>::{closure#0}, !>
  10:     0x7ffaf2670bd6 - std[4d8c08a1d6cd93f8]::panicking::begin_panic::<rustc_errors[7a3c0cce756dd281]::ExplicitBug>
  11:     0x7ffaf53c8e16 - std[4d8c08a1d6cd93f8]::panic::panic_any::<rustc_errors[7a3c0cce756dd281]::ExplicitBug>
  12:     0x7ffaf53c6d30 - <rustc_errors[7a3c0cce756dd281]::HandlerInner>::bug::<&alloc[61e56d2e13f18c62]::string::String>
  13:     0x7ffaf53c68f0 - <rustc_errors[7a3c0cce756dd281]::Handler>::bug::<&alloc[61e56d2e13f18c62]::string::String>
  14:     0x7ffaf556b915 - rustc_middle[114b3b537d7176f2]::ty::context::tls::with_context_opt::<rustc_middle[114b3b537d7176f2]::ty::context::tls::with_opt<rustc_middle[114b3b537d7176f2]::util::bug::opt_span_bug_fmt<rustc_span[e0114d164763424]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7ffaf556d5d9 - rustc_middle[114b3b537d7176f2]::util::bug::opt_span_bug_fmt::<rustc_span[e0114d164763424]::span_encoding::Span>
  16:     0x7ffaf2679908 - rustc_middle[114b3b537d7176f2]::util::bug::bug_fmt
  17:     0x7ffaf32e5812 - <rustc_middle[114b3b537d7176f2]::mir::ClearCrossCrate<&rustc_middle[114b3b537d7176f2]::mir::SourceScopeLocalData>>::assert_crate_local
  18:     0x7ffaf331514d - <rustc_mir_transform[5c4de65d0c4fc280]::check_unsafety::UnsafetyChecker>::register_violations::<&alloc[61e56d2e13f18c62]::vec::Vec<rustc_middle[114b3b537d7176f2]::mir::query::UnsafetyViolation>, core[227267dacc15a5d0]::iter::adapters::copied::Copied<std[4d8c08a1d6cd93f8]::collections::hash::set::Iter<rustc_hir[a42e12c5fc8ec2f6]::hir_id::HirId>>>
  19:     0x7ffaf33143ff - <rustc_mir_transform[5c4de65d0c4fc280]::check_unsafety::UnsafetyChecker as rustc_middle[114b3b537d7176f2]::mir::visit::Visitor>::visit_operand
  20:     0x7ffaf3314097 - <rustc_mir_transform[5c4de65d0c4fc280]::check_unsafety::UnsafetyChecker as rustc_middle[114b3b537d7176f2]::mir::visit::Visitor>::visit_rvalue
  21:     0x7ffaf3315d91 - rustc_mir_transform[5c4de65d0c4fc280]::check_unsafety::unsafety_check_result
  22:     0x7ffaf3311b40 - <rustc_mir_transform[5c4de65d0c4fc280]::check_unsafety::provide::{closure#0} as core[227267dacc15a5d0]::ops::function::FnOnce<(rustc_middle[114b3b537d7176f2]::ty::context::TyCtxt, rustc_span[e0114d164763424]::def_id::LocalDefId)>>::call_once
  23:     0x7ffaf451b232 - rustc_query_system[2b8658e7d9d902a0]::query::plumbing::try_execute_query::<rustc_query_impl[d90372ddb6f5ed6d]::plumbing::QueryCtxt, rustc_query_system[2b8658e7d9d902a0]::query::caches::VecCache<rustc_span[e0114d164763424]::def_id::LocalDefId, &rustc_middle[114b3b537d7176f2]::mir::query::UnsafetyCheckResult>>
  24:     0x7ffaf45f8566 - rustc_query_system[2b8658e7d9d902a0]::query::plumbing::get_query::<rustc_query_impl[d90372ddb6f5ed6d]::queries::unsafety_check_result, rustc_query_impl[d90372ddb6f5ed6d]::plumbing::QueryCtxt>
  25:     0x7ffaf4195b40 - <rustc_query_impl[d90372ddb6f5ed6d]::Queries as rustc_middle[114b3b537d7176f2]::ty::query::QueryEngine>::unsafety_check_result
  26:     0x7ffaf329ec3e - rustc_mir_transform[5c4de65d0c4fc280]::mir_const
  27:     0x7ffaf449815a - rustc_query_system[2b8658e7d9d902a0]::query::plumbing::try_execute_query::<rustc_query_impl[d90372ddb6f5ed6d]::plumbing::QueryCtxt, rustc_query_system[2b8658e7d9d902a0]::query::caches::DefaultCache<rustc_middle[114b3b537d7176f2]::ty::WithOptConstParam<rustc_span[e0114d164763424]::def_id::LocalDefId>, &rustc_data_structures[89239e8ccb8fd3af]::steal::Steal<rustc_middle[114b3b537d7176f2]::mir::Body>>>
  28:     0x7ffaf4636674 - rustc_query_system[2b8658e7d9d902a0]::query::plumbing::get_query::<rustc_query_impl[d90372ddb6f5ed6d]::queries::mir_const, rustc_query_impl[d90372ddb6f5ed6d]::plumbing::QueryCtxt>
  29:     0x7ffaf4181154 - <rustc_query_impl[d90372ddb6f5ed6d]::Queries as rustc_middle[114b3b537d7176f2]::ty::query::QueryEngine>::mir_const
  30:     0x7ffaf329fb8d - rustc_mir_transform[5c4de65d0c4fc280]::mir_promoted
  31:     0x7ffaf45cb2c4 - rustc_query_system[2b8658e7d9d902a0]::query::plumbing::get_query::<rustc_query_impl[d90372ddb6f5ed6d]::queries::mir_promoted, rustc_query_impl[d90372ddb6f5ed6d]::plumbing::QueryCtxt>
  32:     0x7ffaf4183e24 - <rustc_query_impl[d90372ddb6f5ed6d]::Queries as rustc_middle[114b3b537d7176f2]::ty::query::QueryEngine>::mir_promoted
  33:     0x7ffaf3ae2618 - rustc_borrowck[d9d24144480d32de]::mir_borrowck
  34:     0x7ffaf3aad840 - <rustc_borrowck[d9d24144480d32de]::provide::{closure#0} as core[227267dacc15a5d0]::ops::function::FnOnce<(rustc_middle[114b3b537d7176f2]::ty::context::TyCtxt, rustc_span[e0114d164763424]::def_id::LocalDefId)>>::call_once
  35:     0x7ffaf4519d62 - rustc_query_system[2b8658e7d9d902a0]::query::plumbing::try_execute_query::<rustc_query_impl[d90372ddb6f5ed6d]::plumbing::QueryCtxt, rustc_query_system[2b8658e7d9d902a0]::query::caches::VecCache<rustc_span[e0114d164763424]::def_id::LocalDefId, &rustc_middle[114b3b537d7176f2]::mir::query::BorrowCheckResult>>
  36:     0x7ffaf45cabd9 - rustc_query_system[2b8658e7d9d902a0]::query::plumbing::get_query::<rustc_query_impl[d90372ddb6f5ed6d]::queries::mir_borrowck, rustc_query_impl[d90372ddb6f5ed6d]::plumbing::QueryCtxt>
  37:     0x7ffaf41a3210 - <rustc_query_impl[d90372ddb6f5ed6d]::Queries as rustc_middle[114b3b537d7176f2]::ty::query::QueryEngine>::mir_borrowck
  38:     0x7ffaf28a1e41 - <core[227267dacc15a5d0]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[89239e8ccb8fd3af]::sync::par_for_each_in<&[rustc_span[e0114d164763424]::def_id::LocalDefId], <rustc_middle[114b3b537d7176f2]::hir::map::Map>::par_body_owners<rustc_interface[97a011ea2c57c08b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[227267dacc15a5d0]::ops::function::FnOnce<()>>::call_once
  39:     0x7ffaf27f6d26 - std[4d8c08a1d6cd93f8]::panic::catch_unwind::<core[227267dacc15a5d0]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[89239e8ccb8fd3af]::sync::par_for_each_in<&[rustc_span[e0114d164763424]::def_id::LocalDefId], <rustc_middle[114b3b537d7176f2]::hir::map::Map>::par_body_owners<rustc_interface[97a011ea2c57c08b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  40:     0x7ffaf27edacd - rustc_data_structures[89239e8ccb8fd3af]::sync::par_for_each_in::<&[rustc_span[e0114d164763424]::def_id::LocalDefId], <rustc_middle[114b3b537d7176f2]::hir::map::Map>::par_body_owners<rustc_interface[97a011ea2c57c08b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  41:     0x7ffaf2803235 - <rustc_session[c234fdae5de6c3ad]::session::Session>::time::<(), rustc_interface[97a011ea2c57c08b]::passes::analysis::{closure#2}>
  42:     0x7ffaf281b51b - rustc_interface[97a011ea2c57c08b]::passes::analysis
  43:     0x7ffaf44e3e20 - rustc_query_system[2b8658e7d9d902a0]::query::plumbing::try_execute_query::<rustc_query_impl[d90372ddb6f5ed6d]::plumbing::QueryCtxt, rustc_query_system[2b8658e7d9d902a0]::query::caches::DefaultCache<(), core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>>>
  44:     0x7ffaf4633e04 - rustc_query_system[2b8658e7d9d902a0]::query::plumbing::get_query::<rustc_query_impl[d90372ddb6f5ed6d]::queries::analysis, rustc_query_impl[d90372ddb6f5ed6d]::plumbing::QueryCtxt>
  45:     0x7ffaf41774ca - <rustc_query_impl[d90372ddb6f5ed6d]::Queries as rustc_middle[114b3b537d7176f2]::ty::query::QueryEngine>::analysis
  46:     0x7ffaf272b6bc - <rustc_interface[97a011ea2c57c08b]::passes::QueryContext>::enter::<rustc_driver[5df8529dbce8ebc5]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>>
  47:     0x7ffaf2735aaf - <rustc_interface[97a011ea2c57c08b]::interface::Compiler>::enter::<rustc_driver[5df8529dbce8ebc5]::run_compiler::{closure#1}::{closure#2}, core[227267dacc15a5d0]::result::Result<core[227267dacc15a5d0]::option::Option<rustc_interface[97a011ea2c57c08b]::queries::Linker>, rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>>
  48:     0x7ffaf26d1c26 - rustc_span[e0114d164763424]::with_source_map::<core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>, rustc_interface[97a011ea2c57c08b]::interface::run_compiler<core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>, rustc_driver[5df8529dbce8ebc5]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  49:     0x7ffaf273683a - <scoped_tls[849b60d84d59a484]::ScopedKey<rustc_span[e0114d164763424]::SessionGlobals>>::set::<rustc_interface[97a011ea2c57c08b]::interface::run_compiler<core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>, rustc_driver[5df8529dbce8ebc5]::run_compiler::{closure#1}>::{closure#0}, core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>>
  50:     0x7ffaf26f009f - std[4d8c08a1d6cd93f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[97a011ea2c57c08b]::util::run_in_thread_pool_with_globals<rustc_interface[97a011ea2c57c08b]::interface::run_compiler<core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>, rustc_driver[5df8529dbce8ebc5]::run_compiler::{closure#1}>::{closure#0}, core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>>
  51:     0x7ffaf274fb56 - std[4d8c08a1d6cd93f8]::panicking::try::<core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>, core[227267dacc15a5d0]::panic::unwind_safe::AssertUnwindSafe<<std[4d8c08a1d6cd93f8]::thread::Builder>::spawn_unchecked_<rustc_interface[97a011ea2c57c08b]::util::run_in_thread_pool_with_globals<rustc_interface[97a011ea2c57c08b]::interface::run_compiler<core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>, rustc_driver[5df8529dbce8ebc5]::run_compiler::{closure#1}>::{closure#0}, core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  52:     0x7ffaf26e09d5 - <<std[4d8c08a1d6cd93f8]::thread::Builder>::spawn_unchecked_<rustc_interface[97a011ea2c57c08b]::util::run_in_thread_pool_with_globals<rustc_interface[97a011ea2c57c08b]::interface::run_compiler<core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>, rustc_driver[5df8529dbce8ebc5]::run_compiler::{closure#1}>::{closure#0}, core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[227267dacc15a5d0]::result::Result<(), rustc_errors[7a3c0cce756dd281]::ErrorGuaranteed>>::{closure#1} as core[227267dacc15a5d0]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7ffaf1c97dce - std::sys::unix::thread::Thread::new::thread_start::hfb46186e3691734c
  54:     0x7ffaf1a2db43 - <unknown>
  55:     0x7ffaf1abfa00 - <unknown>
  56:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (b19a9586f 2022-12-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/consts -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [unsafety_check_result] unsafety-checking `consts`
#1 [mir_const] preparing `consts` for borrow checking
#2 [mir_promoted] processing MIR for `consts`
#3 [mir_borrowck] borrow-checking `consts`
#4 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


