
thread 'rustc' panicked at 'region variables should not be hashed: '_#33r', /rustc/001a77fac33f6560ff361ff38f661ff5f1c6bf85/compiler/rustc_type_ir/src/sty.rs:1200:17
stack backtrace:
   0:        0x1038f1766 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h23b23a79382a04a8
   1:        0x10394f16a - core::fmt::write::hf7501ecb7d78dea4
   2:        0x1038e403c - std::io::Write::write_fmt::hdfa018e02d7319db
   3:        0x1038f154a - std::sys_common::backtrace::print::h5c1efba12b7fcf37
   4:        0x1038f4883 - std::panicking::default_hook::{{closure}}::h22bb32170592599f
   5:        0x1038f45d8 - std::panicking::default_hook::h79286b0a2185881e
   6:        0x10d68b3ca - rustc_driver[6e08246ed4576017]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x1038f5079 - std::panicking::rust_panic_with_hook::he9bb4d5357af60c1
   8:        0x1038f4e14 - std::panicking::begin_panic_handler::{{closure}}::h4da5dd0e8bd8bde9
   9:        0x1038f1c08 - std::sys_common::backtrace::__rust_end_short_backtrace::hb81861c731c19fc6
  10:        0x1038f4add - _rust_begin_unwind
  11:        0x10397b873 - core::panicking::panic_fmt::he91b9831c0db757f
  12:        0x10e223f43 - <rustc_data_structures[8e9009ffe2a1157b]::intern::Interned<rustc_type_ir[8757e3438ca4bd52]::sty::RegionKind<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt>> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  13:        0x10eb9fc4c - <std[aaa0cbdc83a63136]::thread::local::LocalKey<core[13decc0b4567ece]::cell::RefCell<std[aaa0cbdc83a63136]::collections::hash::map::HashMap<(usize, usize, rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashingControls), rustc_data_structures[8e9009ffe2a1157b]::fingerprint::Fingerprint, core[13decc0b4567ece]::hash::BuildHasherDefault<rustc_hash[77b8984e3167a1e7]::FxHasher>>>>>::with::<<&rustc_middle[6e5f503c2cf4c9ef]::ty::list::List<rustc_middle[6e5f503c2cf4c9ef]::ty::subst::GenericArg> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable::{closure#0}, rustc_data_structures[8e9009ffe2a1157b]::fingerprint::Fingerprint>
  14:        0x10ebcf10c - <&rustc_middle[6e5f503c2cf4c9ef]::ty::list::List<rustc_middle[6e5f503c2cf4c9ef]::ty::subst::GenericArg> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  15:        0x10ed70a43 - <rustc_type_ir[8757e3438ca4bd52]::ty_info::WithCachedTypeInfo<rustc_type_ir[8757e3438ca4bd52]::sty::TyKind<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt>> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  16:        0x10ed58ffa - <rustc_query_system[2891f53e6206af7a]::dep_graph::dep_node::DepNode<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::construct::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_middle[6e5f503c2cf4c9ef]::ty::Ty>
  17:        0x10eafbeac - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::inhabited_predicate_type, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  18:        0x10ec130bc - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::inhabited_predicate_type
  19:        0x10e1fa54d - <rustc_middle[6e5f503c2cf4c9ef]::ty::Ty>::inhabited_predicate
  20:        0x10cf55bb0 - <rustc_borrowck[a9214d7c38494c38]::type_check::TypeChecker>::typeck_mir
  21:        0x10cf40656 - rustc_borrowck[a9214d7c38494c38]::type_check::type_check
  22:        0x10d0682cd - rustc_borrowck[a9214d7c38494c38]::nll::compute_regions
  23:        0x10d0a8832 - rustc_borrowck[a9214d7c38494c38]::do_mir_borrowck
  24:        0x10d09d1f5 - rustc_borrowck[a9214d7c38494c38]::mir_borrowck
  25:        0x10d073c5f - <rustc_borrowck[a9214d7c38494c38]::provide::{closure#0} as core[13decc0b4567ece]::ops::function::FnOnce<(rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_span[63c358fc09b754ab]::def_id::LocalDefId)>>::call_once
  26:        0x10ed00ff4 - <rustc_query_system[2891f53e6206af7a]::dep_graph::graph::DepGraph<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_span[63c358fc09b754ab]::def_id::LocalDefId, &rustc_middle[6e5f503c2cf4c9ef]::mir::query::BorrowCheckResult>
  27:        0x10ea2ade4 - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::mir_borrowck, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  28:        0x10ec0cb0e - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::mir_borrowck
  29:        0x10defef9b - rustc_data_structures[8e9009ffe2a1157b]::sync::par_for_each_in::<&[rustc_span[63c358fc09b754ab]::def_id::LocalDefId], <rustc_middle[6e5f503c2cf4c9ef]::hir::map::Map>::par_body_owners<rustc_interface[8cd637236d5bc23d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  30:        0x10de898cc - <rustc_session[f7572a986d9f87b6]::session::Session>::time::<(), rustc_interface[8cd637236d5bc23d]::passes::analysis::{closure#2}>
  31:        0x10df08eee - rustc_interface[8cd637236d5bc23d]::passes::analysis
  32:        0x10ed350e9 - <rustc_query_system[2891f53e6206af7a]::dep_graph::graph::DepGraph<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, (), core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  33:        0x10eb3926b - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::analysis, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  34:        0x10ec07dc8 - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::analysis
  35:        0x10d67a2bb - <rustc_interface[8cd637236d5bc23d]::passes::QueryContext>::enter::<rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  36:        0x10d65b544 - <rustc_interface[8cd637236d5bc23d]::interface::Compiler>::enter::<rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}::{closure#2}, core[13decc0b4567ece]::result::Result<core[13decc0b4567ece]::option::Option<rustc_interface[8cd637236d5bc23d]::queries::Linker>, rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  37:        0x10d6786c6 - rustc_span[63c358fc09b754ab]::with_source_map::<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:        0x10d653c4e - <scoped_tls[36a9cf003dd1ba16]::ScopedKey<rustc_span[63c358fc09b754ab]::SessionGlobals>>::set::<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  39:        0x10d61bd37 - std[aaa0cbdc83a63136]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8cd637236d5bc23d]::util::run_in_thread_pool_with_globals<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  40:        0x10d5ff197 - <<std[aaa0cbdc83a63136]::thread::Builder>::spawn_unchecked_<rustc_interface[8cd637236d5bc23d]::util::run_in_thread_pool_with_globals<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#1} as core[13decc0b4567ece]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:        0x1038fe667 - std::sys::unix::thread::Thread::new::thread_start::hbf69f0f715b55c29
  42:     0x7ff803b21259 - __pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (001a77fac 2023-01-30) running on x86_64-apple-darwin

note: compiler flags: --crate-type lib -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `command::<impl at src/command.rs:90:1: 90:39>::over_session`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'region variables should not be hashed: '_#17r', /rustc/001a77fac33f6560ff361ff38f661ff5f1c6bf85/compiler/rustc_type_ir/src/sty.rs:1200:17
stack backtrace:
   0:        0x1038f1766 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h23b23a79382a04a8
   1:        0x10394f16a - core::fmt::write::hf7501ecb7d78dea4
   2:        0x1038e403c - std::io::Write::write_fmt::hdfa018e02d7319db
   3:        0x1038f154a - std::sys_common::backtrace::print::h5c1efba12b7fcf37
   4:        0x1038f4883 - std::panicking::default_hook::{{closure}}::h22bb32170592599f
   5:        0x1038f45d8 - std::panicking::default_hook::h79286b0a2185881e
   6:        0x10d68b3ca - rustc_driver[6e08246ed4576017]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x1038f5079 - std::panicking::rust_panic_with_hook::he9bb4d5357af60c1
   8:        0x1038f4e14 - std::panicking::begin_panic_handler::{{closure}}::h4da5dd0e8bd8bde9
   9:        0x1038f1c08 - std::sys_common::backtrace::__rust_end_short_backtrace::hb81861c731c19fc6
  10:        0x1038f4add - _rust_begin_unwind
  11:        0x10397b873 - core::panicking::panic_fmt::he91b9831c0db757f
  12:        0x10e223f43 - <rustc_data_structures[8e9009ffe2a1157b]::intern::Interned<rustc_type_ir[8757e3438ca4bd52]::sty::RegionKind<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt>> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  13:        0x10eb9fc4c - <std[aaa0cbdc83a63136]::thread::local::LocalKey<core[13decc0b4567ece]::cell::RefCell<std[aaa0cbdc83a63136]::collections::hash::map::HashMap<(usize, usize, rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashingControls), rustc_data_structures[8e9009ffe2a1157b]::fingerprint::Fingerprint, core[13decc0b4567ece]::hash::BuildHasherDefault<rustc_hash[77b8984e3167a1e7]::FxHasher>>>>>::with::<<&rustc_middle[6e5f503c2cf4c9ef]::ty::list::List<rustc_middle[6e5f503c2cf4c9ef]::ty::subst::GenericArg> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable::{closure#0}, rustc_data_structures[8e9009ffe2a1157b]::fingerprint::Fingerprint>
  14:        0x10ebcf10c - <&rustc_middle[6e5f503c2cf4c9ef]::ty::list::List<rustc_middle[6e5f503c2cf4c9ef]::ty::subst::GenericArg> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  15:        0x10ed70a43 - <rustc_type_ir[8757e3438ca4bd52]::ty_info::WithCachedTypeInfo<rustc_type_ir[8757e3438ca4bd52]::sty::TyKind<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt>> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  16:        0x10ed58ffa - <rustc_query_system[2891f53e6206af7a]::dep_graph::dep_node::DepNode<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::construct::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_middle[6e5f503c2cf4c9ef]::ty::Ty>
  17:        0x10eafbeac - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::inhabited_predicate_type, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  18:        0x10ec130bc - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::inhabited_predicate_type
  19:        0x10e1fa54d - <rustc_middle[6e5f503c2cf4c9ef]::ty::Ty>::inhabited_predicate
  20:        0x10cf55bb0 - <rustc_borrowck[a9214d7c38494c38]::type_check::TypeChecker>::typeck_mir
  21:        0x10cf40656 - rustc_borrowck[a9214d7c38494c38]::type_check::type_check
  22:        0x10d0682cd - rustc_borrowck[a9214d7c38494c38]::nll::compute_regions
  23:        0x10d0a8832 - rustc_borrowck[a9214d7c38494c38]::do_mir_borrowck
  24:        0x10d09d1f5 - rustc_borrowck[a9214d7c38494c38]::mir_borrowck
  25:        0x10d073c5f - <rustc_borrowck[a9214d7c38494c38]::provide::{closure#0} as core[13decc0b4567ece]::ops::function::FnOnce<(rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_span[63c358fc09b754ab]::def_id::LocalDefId)>>::call_once
  26:        0x10ed00ff4 - <rustc_query_system[2891f53e6206af7a]::dep_graph::graph::DepGraph<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_span[63c358fc09b754ab]::def_id::LocalDefId, &rustc_middle[6e5f503c2cf4c9ef]::mir::query::BorrowCheckResult>
  27:        0x10ea2ade4 - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::mir_borrowck, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  28:        0x10ec0cb0e - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::mir_borrowck
  29:        0x10defef9b - rustc_data_structures[8e9009ffe2a1157b]::sync::par_for_each_in::<&[rustc_span[63c358fc09b754ab]::def_id::LocalDefId], <rustc_middle[6e5f503c2cf4c9ef]::hir::map::Map>::par_body_owners<rustc_interface[8cd637236d5bc23d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  30:        0x10de898cc - <rustc_session[f7572a986d9f87b6]::session::Session>::time::<(), rustc_interface[8cd637236d5bc23d]::passes::analysis::{closure#2}>
  31:        0x10df08eee - rustc_interface[8cd637236d5bc23d]::passes::analysis
  32:        0x10ed350e9 - <rustc_query_system[2891f53e6206af7a]::dep_graph::graph::DepGraph<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, (), core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  33:        0x10eb3926b - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::analysis, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  34:        0x10ec07dc8 - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::analysis
  35:        0x10d67a2bb - <rustc_interface[8cd637236d5bc23d]::passes::QueryContext>::enter::<rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  36:        0x10d65b544 - <rustc_interface[8cd637236d5bc23d]::interface::Compiler>::enter::<rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}::{closure#2}, core[13decc0b4567ece]::result::Result<core[13decc0b4567ece]::option::Option<rustc_interface[8cd637236d5bc23d]::queries::Linker>, rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  37:        0x10d6786c6 - rustc_span[63c358fc09b754ab]::with_source_map::<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:        0x10d653c4e - <scoped_tls[36a9cf003dd1ba16]::ScopedKey<rustc_span[63c358fc09b754ab]::SessionGlobals>>::set::<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  39:        0x10d61bd37 - std[aaa0cbdc83a63136]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8cd637236d5bc23d]::util::run_in_thread_pool_with_globals<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  40:        0x10d5ff197 - <<std[aaa0cbdc83a63136]::thread::Builder>::spawn_unchecked_<rustc_interface[8cd637236d5bc23d]::util::run_in_thread_pool_with_globals<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#1} as core[13decc0b4567ece]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:        0x1038fe667 - std::sys::unix::thread::Thread::new::thread_start::hbf69f0f715b55c29
  42:     0x7ff803b21259 - __pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (001a77fac 2023-01-30) running on x86_64-apple-darwin

note: compiler flags: --crate-type lib -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `command::<impl at src/command.rs:101:1: 101:41>::over_session`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'region variables should not be hashed: '_#15r', /rustc/001a77fac33f6560ff361ff38f661ff5f1c6bf85/compiler/rustc_type_ir/src/sty.rs:1200:17
stack backtrace:
   0:        0x1038f1766 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h23b23a79382a04a8
   1:        0x10394f16a - core::fmt::write::hf7501ecb7d78dea4
   2:        0x1038e403c - std::io::Write::write_fmt::hdfa018e02d7319db
   3:        0x1038f154a - std::sys_common::backtrace::print::h5c1efba12b7fcf37
   4:        0x1038f4883 - std::panicking::default_hook::{{closure}}::h22bb32170592599f
   5:        0x1038f45d8 - std::panicking::default_hook::h79286b0a2185881e
   6:        0x10d68b3ca - rustc_driver[6e08246ed4576017]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x1038f5079 - std::panicking::rust_panic_with_hook::he9bb4d5357af60c1
   8:        0x1038f4e14 - std::panicking::begin_panic_handler::{{closure}}::h4da5dd0e8bd8bde9
   9:        0x1038f1c08 - std::sys_common::backtrace::__rust_end_short_backtrace::hb81861c731c19fc6
  10:        0x1038f4add - _rust_begin_unwind
  11:        0x10397b873 - core::panicking::panic_fmt::he91b9831c0db757f
  12:        0x10e223f43 - <rustc_data_structures[8e9009ffe2a1157b]::intern::Interned<rustc_type_ir[8757e3438ca4bd52]::sty::RegionKind<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt>> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  13:        0x10eb9fc4c - <std[aaa0cbdc83a63136]::thread::local::LocalKey<core[13decc0b4567ece]::cell::RefCell<std[aaa0cbdc83a63136]::collections::hash::map::HashMap<(usize, usize, rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashingControls), rustc_data_structures[8e9009ffe2a1157b]::fingerprint::Fingerprint, core[13decc0b4567ece]::hash::BuildHasherDefault<rustc_hash[77b8984e3167a1e7]::FxHasher>>>>>::with::<<&rustc_middle[6e5f503c2cf4c9ef]::ty::list::List<rustc_middle[6e5f503c2cf4c9ef]::ty::subst::GenericArg> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable::{closure#0}, rustc_data_structures[8e9009ffe2a1157b]::fingerprint::Fingerprint>
  14:        0x10ebcf10c - <&rustc_middle[6e5f503c2cf4c9ef]::ty::list::List<rustc_middle[6e5f503c2cf4c9ef]::ty::subst::GenericArg> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  15:        0x10ed70a43 - <rustc_type_ir[8757e3438ca4bd52]::ty_info::WithCachedTypeInfo<rustc_type_ir[8757e3438ca4bd52]::sty::TyKind<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt>> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  16:        0x10ed58ffa - <rustc_query_system[2891f53e6206af7a]::dep_graph::dep_node::DepNode<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::construct::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_middle[6e5f503c2cf4c9ef]::ty::Ty>
  17:        0x10eafbeac - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::inhabited_predicate_type, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  18:        0x10ec130bc - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::inhabited_predicate_type
  19:        0x10e1fa54d - <rustc_middle[6e5f503c2cf4c9ef]::ty::Ty>::inhabited_predicate
  20:        0x10cf55bb0 - <rustc_borrowck[a9214d7c38494c38]::type_check::TypeChecker>::typeck_mir
  21:        0x10cf40656 - rustc_borrowck[a9214d7c38494c38]::type_check::type_check
  22:        0x10d0682cd - rustc_borrowck[a9214d7c38494c38]::nll::compute_regions
  23:        0x10d0a8832 - rustc_borrowck[a9214d7c38494c38]::do_mir_borrowck
  24:        0x10d09d1f5 - rustc_borrowck[a9214d7c38494c38]::mir_borrowck
  25:        0x10d073c5f - <rustc_borrowck[a9214d7c38494c38]::provide::{closure#0} as core[13decc0b4567ece]::ops::function::FnOnce<(rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_span[63c358fc09b754ab]::def_id::LocalDefId)>>::call_once
  26:        0x10ed00ff4 - <rustc_query_system[2891f53e6206af7a]::dep_graph::graph::DepGraph<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_span[63c358fc09b754ab]::def_id::LocalDefId, &rustc_middle[6e5f503c2cf4c9ef]::mir::query::BorrowCheckResult>
  27:        0x10ea2ade4 - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::mir_borrowck, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  28:        0x10ec0cb0e - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::mir_borrowck
  29:        0x10defef9b - rustc_data_structures[8e9009ffe2a1157b]::sync::par_for_each_in::<&[rustc_span[63c358fc09b754ab]::def_id::LocalDefId], <rustc_middle[6e5f503c2cf4c9ef]::hir::map::Map>::par_body_owners<rustc_interface[8cd637236d5bc23d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  30:        0x10de898cc - <rustc_session[f7572a986d9f87b6]::session::Session>::time::<(), rustc_interface[8cd637236d5bc23d]::passes::analysis::{closure#2}>
  31:        0x10df08eee - rustc_interface[8cd637236d5bc23d]::passes::analysis
  32:        0x10ed350e9 - <rustc_query_system[2891f53e6206af7a]::dep_graph::graph::DepGraph<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, (), core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  33:        0x10eb3926b - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::analysis, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  34:        0x10ec07dc8 - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::analysis
  35:        0x10d67a2bb - <rustc_interface[8cd637236d5bc23d]::passes::QueryContext>::enter::<rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  36:        0x10d65b544 - <rustc_interface[8cd637236d5bc23d]::interface::Compiler>::enter::<rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}::{closure#2}, core[13decc0b4567ece]::result::Result<core[13decc0b4567ece]::option::Option<rustc_interface[8cd637236d5bc23d]::queries::Linker>, rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  37:        0x10d6786c6 - rustc_span[63c358fc09b754ab]::with_source_map::<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:        0x10d653c4e - <scoped_tls[36a9cf003dd1ba16]::ScopedKey<rustc_span[63c358fc09b754ab]::SessionGlobals>>::set::<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  39:        0x10d61bd37 - std[aaa0cbdc83a63136]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8cd637236d5bc23d]::util::run_in_thread_pool_with_globals<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  40:        0x10d5ff197 - <<std[aaa0cbdc83a63136]::thread::Builder>::spawn_unchecked_<rustc_interface[8cd637236d5bc23d]::util::run_in_thread_pool_with_globals<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#1} as core[13decc0b4567ece]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:        0x1038fe667 - std::sys::unix::thread::Thread::new::thread_start::hbf69f0f715b55c29
  42:     0x7ff803b21259 - __pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (001a77fac 2023-01-30) running on x86_64-apple-darwin

note: compiler flags: --crate-type lib -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `command::<impl at src/command.rs:107:1: 107:23>::over_session`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'region variables should not be hashed: '_#13r', /rustc/001a77fac33f6560ff361ff38f661ff5f1c6bf85/compiler/rustc_type_ir/src/sty.rs:1200:17
stack backtrace:
   0:        0x1038f1766 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h23b23a79382a04a8
   1:        0x10394f16a - core::fmt::write::hf7501ecb7d78dea4
   2:        0x1038e403c - std::io::Write::write_fmt::hdfa018e02d7319db
   3:        0x1038f154a - std::sys_common::backtrace::print::h5c1efba12b7fcf37
   4:        0x1038f4883 - std::panicking::default_hook::{{closure}}::h22bb32170592599f
   5:        0x1038f45d8 - std::panicking::default_hook::h79286b0a2185881e
   6:        0x10d68b3ca - rustc_driver[6e08246ed4576017]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x1038f5079 - std::panicking::rust_panic_with_hook::he9bb4d5357af60c1
   8:        0x1038f4e14 - std::panicking::begin_panic_handler::{{closure}}::h4da5dd0e8bd8bde9
   9:        0x1038f1c08 - std::sys_common::backtrace::__rust_end_short_backtrace::hb81861c731c19fc6
  10:        0x1038f4add - _rust_begin_unwind
  11:        0x10397b873 - core::panicking::panic_fmt::he91b9831c0db757f
  12:        0x10e223f43 - <rustc_data_structures[8e9009ffe2a1157b]::intern::Interned<rustc_type_ir[8757e3438ca4bd52]::sty::RegionKind<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt>> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  13:        0x10eb9fc4c - <std[aaa0cbdc83a63136]::thread::local::LocalKey<core[13decc0b4567ece]::cell::RefCell<std[aaa0cbdc83a63136]::collections::hash::map::HashMap<(usize, usize, rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashingControls), rustc_data_structures[8e9009ffe2a1157b]::fingerprint::Fingerprint, core[13decc0b4567ece]::hash::BuildHasherDefault<rustc_hash[77b8984e3167a1e7]::FxHasher>>>>>::with::<<&rustc_middle[6e5f503c2cf4c9ef]::ty::list::List<rustc_middle[6e5f503c2cf4c9ef]::ty::subst::GenericArg> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable::{closure#0}, rustc_data_structures[8e9009ffe2a1157b]::fingerprint::Fingerprint>
  14:        0x10ebcf10c - <&rustc_middle[6e5f503c2cf4c9ef]::ty::list::List<rustc_middle[6e5f503c2cf4c9ef]::ty::subst::GenericArg> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  15:        0x10ed70a43 - <rustc_type_ir[8757e3438ca4bd52]::ty_info::WithCachedTypeInfo<rustc_type_ir[8757e3438ca4bd52]::sty::TyKind<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt>> as rustc_data_structures[8e9009ffe2a1157b]::stable_hasher::HashStable<rustc_query_system[2891f53e6206af7a]::ich::hcx::StableHashingContext>>::hash_stable
  16:        0x10ed58ffa - <rustc_query_system[2891f53e6206af7a]::dep_graph::dep_node::DepNode<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::construct::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_middle[6e5f503c2cf4c9ef]::ty::Ty>
  17:        0x10eafbeac - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::inhabited_predicate_type, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  18:        0x10ec130bc - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::inhabited_predicate_type
  19:        0x10e1fa54d - <rustc_middle[6e5f503c2cf4c9ef]::ty::Ty>::inhabited_predicate
  20:        0x10cf55bb0 - <rustc_borrowck[a9214d7c38494c38]::type_check::TypeChecker>::typeck_mir
  21:        0x10cf40656 - rustc_borrowck[a9214d7c38494c38]::type_check::type_check
  22:        0x10d0682cd - rustc_borrowck[a9214d7c38494c38]::nll::compute_regions
  23:        0x10d0a8832 - rustc_borrowck[a9214d7c38494c38]::do_mir_borrowck
  24:        0x10d09d1f5 - rustc_borrowck[a9214d7c38494c38]::mir_borrowck
  25:        0x10d073c5f - <rustc_borrowck[a9214d7c38494c38]::provide::{closure#0} as core[13decc0b4567ece]::ops::function::FnOnce<(rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_span[63c358fc09b754ab]::def_id::LocalDefId)>>::call_once
  26:        0x10ed00ff4 - <rustc_query_system[2891f53e6206af7a]::dep_graph::graph::DepGraph<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, rustc_span[63c358fc09b754ab]::def_id::LocalDefId, &rustc_middle[6e5f503c2cf4c9ef]::mir::query::BorrowCheckResult>
  27:        0x10ea2ade4 - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::mir_borrowck, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  28:        0x10ec0cb0e - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::mir_borrowck
  29:        0x10defef9b - rustc_data_structures[8e9009ffe2a1157b]::sync::par_for_each_in::<&[rustc_span[63c358fc09b754ab]::def_id::LocalDefId], <rustc_middle[6e5f503c2cf4c9ef]::hir::map::Map>::par_body_owners<rustc_interface[8cd637236d5bc23d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  30:        0x10de898cc - <rustc_session[f7572a986d9f87b6]::session::Session>::time::<(), rustc_interface[8cd637236d5bc23d]::passes::analysis::{closure#2}>
  31:        0x10df08eee - rustc_interface[8cd637236d5bc23d]::passes::analysis
  32:        0x10ed350e9 - <rustc_query_system[2891f53e6206af7a]::dep_graph::graph::DepGraph<rustc_middle[6e5f503c2cf4c9ef]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6e5f503c2cf4c9ef]::ty::context::TyCtxt, (), core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  33:        0x10eb3926b - rustc_query_system[2891f53e6206af7a]::query::plumbing::try_execute_query::<rustc_query_impl[c2c6d2dd87e81b2b]::queries::analysis, rustc_query_impl[c2c6d2dd87e81b2b]::plumbing::QueryCtxt>
  34:        0x10ec07dc8 - <rustc_query_impl[c2c6d2dd87e81b2b]::Queries as rustc_middle[6e5f503c2cf4c9ef]::ty::query::QueryEngine>::analysis
  35:        0x10d67a2bb - <rustc_interface[8cd637236d5bc23d]::passes::QueryContext>::enter::<rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  36:        0x10d65b544 - <rustc_interface[8cd637236d5bc23d]::interface::Compiler>::enter::<rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}::{closure#2}, core[13decc0b4567ece]::result::Result<core[13decc0b4567ece]::option::Option<rustc_interface[8cd637236d5bc23d]::queries::Linker>, rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  37:        0x10d6786c6 - rustc_span[63c358fc09b754ab]::with_source_map::<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:        0x10d653c4e - <scoped_tls[36a9cf003dd1ba16]::ScopedKey<rustc_span[63c358fc09b754ab]::SessionGlobals>>::set::<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  39:        0x10d61bd37 - std[aaa0cbdc83a63136]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8cd637236d5bc23d]::util::run_in_thread_pool_with_globals<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>
  40:        0x10d5ff197 - <<std[aaa0cbdc83a63136]::thread::Builder>::spawn_unchecked_<rustc_interface[8cd637236d5bc23d]::util::run_in_thread_pool_with_globals<rustc_interface[8cd637236d5bc23d]::interface::run_compiler<core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>, rustc_driver[6e08246ed4576017]::run_compiler::{closure#1}>::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[13decc0b4567ece]::result::Result<(), rustc_errors[983416e9ec6724c2]::ErrorGuaranteed>>::{closure#1} as core[13decc0b4567ece]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:        0x1038fe667 - std::sys::unix::thread::Thread::new::thread_start::hbf69f0f715b55c29
  42:     0x7ff803b21259 - __pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (001a77fac 2023-01-30) running on x86_64-apple-darwin

note: compiler flags: --crate-type lib -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `command::<impl at src/command.rs:116:1: 116:27>::over_session`
#1 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `openssh`
