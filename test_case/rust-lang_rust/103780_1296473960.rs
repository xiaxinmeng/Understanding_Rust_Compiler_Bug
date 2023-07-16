plain
   Compiling rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`: index vec had unexpected number of variables', compiler/rustc_borrowck/src/universal_regions.rs:258:9
stack backtrace:
   0:     0x7fd644e85f52 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hdba13059092c04ba
   1:     0x7fd644eed7c8 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7fd644e76631 - std::io::Write::write_fmt::h31de6b2c664bff16
   3:     0x7fd644e85d15 - std::sys_common::backtrace::print::hd0c49335bba07915
   4:     0x7fd644e890f7 - std::panicking::default_hook::{{closure}}::h12df99ff2e9b7214
   5:     0x7fd644e88e55 - std::panicking::default_hook::ha09486f74e1d6522
   Compiling rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
   6:     0x7fd6457ee6a4 - rustc_driver[aa2c3d0cc0fa7243]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fd644e89a03 - std::panicking::rust_panic_with_hook::h580b98aad2fff7cb
   8:     0x7fd644e89737 - std::panicking::begin_panic_handler::{{closure}}::he2d4628496bd86fe
   9:     0x7fd644e864fc - std::sys_common::backtrace::__rust_end_short_backtrace::he8ff653d3d84ef86
  10:     0x7fd644e89402 - rust_begin_unwind
  11:     0x7fd644e3b7b3 - core::panicking::panic_fmt::h3a5b5d72039ab650
  12:     0x7fd644eea171 - core::panicking::assert_failed_inner::hc5f1051b6a11bc96
  13:     0x7fd6456acfcb - core[69c2305d6fa5d54f]::panicking::assert_failed::<usize, usize>
  14:     0x7fd64697dc48 - <rustc_borrowck[87a5392d6181f26a]::universal_regions::UniversalRegions>::closure_mapping
  15:     0x7fd64693bcf5 - <rustc_middle[f414e03c15862bd6]::mir::query::ClosureRegionRequirements as rustc_borrowck[87a5392d6181f26a]::region_infer::ClosureRegionRequirementsExt>::apply_requirements
  16:     0x7fd646a29368 - <rustc_borrowck[87a5392d6181f26a]::type_check::TypeChecker>::prove_closure_bounds
  17:     0x7fd646a372cd - <rustc_borrowck[87a5392d6181f26a]::type_check::TypeChecker>::check_rvalue
  18:     0x7fd646a3d67b - <rustc_borrowck[87a5392d6181f26a]::type_check::TypeChecker>::typeck_mir
  19:     0x7fd646a1fae6 - rustc_borrowck[87a5392d6181f26a]::type_check::type_check
  20:     0x7fd646911434 - rustc_borrowck[87a5392d6181f26a]::nll::compute_regions
  21:     0x7fd646abcb32 - rustc_borrowck[87a5392d6181f26a]::do_mir_borrowck
  22:     0x7fd646aaa009 - rustc_borrowck[87a5392d6181f26a]::mir_borrowck
  23:     0x7fd646a7d11f - <rustc_borrowck[87a5392d6181f26a]::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[f414e03c15862bd6]::ty::context::TyCtxt, rustc_span[6f04c596547a783a]::def_id::LocalDefId)>>::call_once
  24:     0x7fd6473c830e - rustc_query_system[818b688b820e3781]::query::plumbing::try_execute_query::<rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt, rustc_query_system[818b688b820e3781]::query::caches::DefaultCache<rustc_span[6f04c596547a783a]::def_id::LocalDefId, &rustc_middle[f414e03c15862bd6]::mir::query::BorrowCheckResult>>
  25:     0x7fd6474a34fe - rustc_query_system[818b688b820e3781]::query::plumbing::get_query::<rustc_query_impl[bf07a966c3412bd5]::queries::mir_borrowck, rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt>
  26:     0x7fd6470a2564 - <rustc_query_impl[bf07a966c3412bd5]::Queries as rustc_middle[f414e03c15862bd6]::ty::query::QueryEngine>::mir_borrowck
  27:     0x7fd646a292ac - <rustc_borrowck[87a5392d6181f26a]::type_check::TypeChecker>::prove_closure_bounds
  28:     0x7fd646a372cd - <rustc_borrowck[87a5392d6181f26a]::type_check::TypeChecker>::check_rvalue
  29:     0x7fd646a3d67b - <rustc_borrowck[87a5392d6181f26a]::type_check::TypeChecker>::typeck_mir
  30:     0x7fd646a1fae6 - rustc_borrowck[87a5392d6181f26a]::type_check::type_check
  31:     0x7fd646911434 - rustc_borrowck[87a5392d6181f26a]::nll::compute_regions
  32:     0x7fd646abcb32 - rustc_borrowck[87a5392d6181f26a]::do_mir_borrowck
  33:     0x7fd646aaa009 - rustc_borrowck[87a5392d6181f26a]::mir_borrowck
  34:     0x7fd646a7d11f - <rustc_borrowck[87a5392d6181f26a]::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[f414e03c15862bd6]::ty::context::TyCtxt, rustc_span[6f04c596547a783a]::def_id::LocalDefId)>>::call_once
  35:     0x7fd6473c830e - rustc_query_system[818b688b820e3781]::query::plumbing::try_execute_query::<rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt, rustc_query_system[818b688b820e3781]::query::caches::DefaultCache<rustc_span[6f04c596547a783a]::def_id::LocalDefId, &rustc_middle[f414e03c15862bd6]::mir::query::BorrowCheckResult>>
  36:     0x7fd6474a34fe - rustc_query_system[818b688b820e3781]::query::plumbing::get_query::<rustc_query_impl[bf07a966c3412bd5]::queries::mir_borrowck, rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt>
  37:     0x7fd6470a2564 - <rustc_query_impl[bf07a966c3412bd5]::Queries as rustc_middle[f414e03c15862bd6]::ty::query::QueryEngine>::mir_borrowck
  38:     0x7fd646a292ac - <rustc_borrowck[87a5392d6181f26a]::type_check::TypeChecker>::prove_closure_bounds
  39:     0x7fd646a372cd - <rustc_borrowck[87a5392d6181f26a]::type_check::TypeChecker>::check_rvalue
  40:     0x7fd646a3d67b - <rustc_borrowck[87a5392d6181f26a]::type_check::TypeChecker>::typeck_mir
  41:     0x7fd646a1fae6 - rustc_borrowck[87a5392d6181f26a]::type_check::type_check
  42:     0x7fd646911434 - rustc_borrowck[87a5392d6181f26a]::nll::compute_regions
  43:     0x7fd646abcb32 - rustc_borrowck[87a5392d6181f26a]::do_mir_borrowck
  44:     0x7fd646aaa009 - rustc_borrowck[87a5392d6181f26a]::mir_borrowck
  45:     0x7fd646a7d11f - <rustc_borrowck[87a5392d6181f26a]::provide::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[f414e03c15862bd6]::ty::context::TyCtxt, rustc_span[6f04c596547a783a]::def_id::LocalDefId)>>::call_once
  46:     0x7fd6473c830e - rustc_query_system[818b688b820e3781]::query::plumbing::try_execute_query::<rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt, rustc_query_system[818b688b820e3781]::query::caches::DefaultCache<rustc_span[6f04c596547a783a]::def_id::LocalDefId, &rustc_middle[f414e03c15862bd6]::mir::query::BorrowCheckResult>>
  47:     0x7fd6474a34fe - rustc_query_system[818b688b820e3781]::query::plumbing::get_query::<rustc_query_impl[bf07a966c3412bd5]::queries::mir_borrowck, rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt>
  48:     0x7fd6470a2564 - <rustc_query_impl[bf07a966c3412bd5]::Queries as rustc_middle[f414e03c15862bd6]::ty::query::QueryEngine>::mir_borrowck
  49:     0x7fd645999da0 - <core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c7b5c7cf6db03e91]::sync::par_for_each_in<&[rustc_span[6f04c596547a783a]::def_id::LocalDefId], <rustc_middle[f414e03c15862bd6]::hir::map::Map>::par_body_owners<rustc_interface[f8ab63ad520f2673]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once
  50:     0x7fd6458fee8b - rustc_data_structures[c7b5c7cf6db03e91]::sync::par_for_each_in::<&[rustc_span[6f04c596547a783a]::def_id::LocalDefId], <rustc_middle[f414e03c15862bd6]::hir::map::Map>::par_body_owners<rustc_interface[f8ab63ad520f2673]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  51:     0x7fd64590870d - <rustc_session[d359256240218516]::session::Session>::time::<(), rustc_interface[f8ab63ad520f2673]::passes::analysis::{closure#2}>
  52:     0x7fd6459335eb - rustc_interface[f8ab63ad520f2673]::passes::analysis
  53:     0x7fd647410ee0 - rustc_query_system[818b688b820e3781]::query::plumbing::try_execute_query::<rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt, rustc_query_system[818b688b820e3781]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>>
  54:     0x7fd6474f2370 - rustc_query_system[818b688b820e3781]::query::plumbing::get_query::<rustc_query_impl[bf07a966c3412bd5]::queries::analysis, rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt>
  55:     0x7fd64707c07a - <rustc_query_impl[bf07a966c3412bd5]::Queries as rustc_middle[f414e03c15862bd6]::ty::query::QueryEngine>::analysis
  56:     0x7fd6458044f8 - <rustc_interface[f8ab63ad520f2673]::passes::QueryContext>::enter::<rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>
  57:     0x7fd64585ab5b - <rustc_interface[f8ab63ad520f2673]::interface::Compiler>::enter::<rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[f8ab63ad520f2673]::queries::Linker>, rustc_errors[3464920041138443]::ErrorGuaranteed>>
  58:     0x7fd6457efd32 - rustc_span[6f04c596547a783a]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_interface[f8ab63ad520f2673]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  59:     0x7fd64584e93c - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[6f04c596547a783a]::SessionGlobals>>::set::<rustc_interface[f8ab63ad520f2673]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>
  60:     0x7fd64580df9a - std[5f53c15ad22586c0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f8ab63ad520f2673]::util::run_in_thread_pool_with_globals<rustc_interface[f8ab63ad520f2673]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>
  61:     0x7fd645850038 - std[5f53c15ad22586c0]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[5f53c15ad22586c0]::thread::Builder>::spawn_unchecked_<rustc_interface[f8ab63ad520f2673]::util::run_in_thread_pool_with_globals<rustc_interface[f8ab63ad520f2673]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>
  62:     0x7fd6457ff8e9 - <<std[5f53c15ad22586c0]::thread::Builder>::spawn_unchecked_<rustc_interface[f8ab63ad520f2673]::util::run_in_thread_pool_with_globals<rustc_interface[f8ab63ad520f2673]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  63:     0x7fd644e9666e - std::sys::unix::thread::Thread::new::thread_start::h942bfa51de05b60c
  64:     0x7fd644c31b43 - <unknown>
  65:     0x7fd644cc3a00 - <unknown>
  66:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (6b23b63ca 2022-10-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `abi::<impl at compiler/rustc_target/src/abi/mod.rs:82:1: 82:22>::parse::{closure#3}::{closure#0}`
#1 [mir_borrowck] borrow-checking `abi::<impl at compiler/rustc_target/src/abi/mod.rs:82:1: 82:22>::parse::{closure#3}`
#2 [mir_borrowck] borrow-checking `abi::<impl at compiler/rustc_target/src/abi/mod.rs:82:1: 82:22>::parse`
#3 [analysis] running analysis passes on this crate
error: could not compile `rustc_target`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:05:08
