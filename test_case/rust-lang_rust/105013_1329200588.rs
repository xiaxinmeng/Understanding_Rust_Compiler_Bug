plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.84
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Binder(<[closure@library/core/src/slice/sort.rs:694:37: 694:52] as ops::function::FnMut<(&mut usize,)>>, [])`,
 right: `Binder(<[closure@library/core/src/slice/sort.rs:694:37: 694:52] as ops::function::FnMut<(&mut usize,)>>, [])`', compiler/rustc_trait_selection/src/traits/codegen.rs:28:5
   0:     0x7f2b392c5512 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h76efda83803f7c88
   0:     0x7f2b392c5512 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h76efda83803f7c88
   1:     0x7f2b39333638 - core::fmt::write::h0cd9f5419c66d611
   2:     0x7f2b392b5fe1 - std::io::Write::write_fmt::h115d78592252bf9d
   3:     0x7f2b392c52d5 - std::sys_common::backtrace::print::hd9ebfd55eb3388eb
   4:     0x7f2b392c85e7 - std::panicking::default_hook::{{closure}}::h98850a6457a8782d
   5:     0x7f2b392c8345 - std::panicking::default_hook::hef750a0d73d5e8e9
   6:     0x7f2b39ca8984 - rustc_driver[ab3ba77cd7534cfb]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f2b392c8ef3 - std::panicking::rust_panic_with_hook::ha62dd9a4e901383e
   8:     0x7f2b392c8c27 - std::panicking::begin_panic_handler::{{closure}}::h30577589484d4d4f
   9:     0x7f2b392c5abc - std::sys_common::backtrace::__rust_end_short_backtrace::h8641e565b807163f
  10:     0x7f2b392c88f2 - rust_begin_unwind
  11:     0x7f2b3927a023 - core::panicking::panic_fmt::ha29375e42f9e82fb
  12:     0x7f2b3932fefb - core::panicking::assert_failed_inner::hc36543c12f11956e
  13:     0x7f2b39c217bb - core[867cfca19013d5a]::panicking::assert_failed::<rustc_middle[4f81a7ac8c79b4c8]::ty::sty::Binder<rustc_middle[4f81a7ac8c79b4c8]::ty::sty::TraitRef>, rustc_middle[4f81a7ac8c79b4c8]::ty::sty::Binder<rustc_middle[4f81a7ac8c79b4c8]::ty::sty::TraitRef>>
  14:     0x7f2b3c5b37c3 - rustc_trait_selection[684cd2d7a7441411]::traits::codegen::codegen_select_candidate
  15:     0x7f2b3bb391be - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::codegen_select_candidate, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  16:     0x7f2b3b7100c8 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::codegen_select_candidate
  17:     0x7f2b3a1539ff - rustc_ty_utils[40d190a702156b7]::instance::inner_resolve_instance
  18:     0x7f2b3a152708 - rustc_ty_utils[40d190a702156b7]::instance::resolve_instance
  19:     0x7f2b3bb11bef - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::resolve_instance, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  20:     0x7f2b3b73fa18 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::resolve_instance
  21:     0x7f2b3c9a6614 - <rustc_middle[4f81a7ac8c79b4c8]::ty::instance::Instance>::resolve_opt_const_arg
  22:     0x7f2b3c9a6294 - <rustc_middle[4f81a7ac8c79b4c8]::ty::instance::Instance>::resolve
  23:     0x7f2b3ae51082 - <rustc_mir_build[6ecd4054b0fed128]::lints::Search>::is_recursive_call
  24:     0x7f2b3ae69586 - <rustc_data_structures[d4cd6911de3a1e89]::graph::iterate::TriColorDepthFirstSearch<rustc_middle[4f81a7ac8c79b4c8]::mir::basic_blocks::BasicBlocks>>::run_from_start::<rustc_mir_build[6ecd4054b0fed128]::lints::Search>
  25:     0x7f2b3ae509b5 - rustc_mir_build[6ecd4054b0fed128]::lints::check
  26:     0x7f2b3ad8f77f - rustc_mir_build[6ecd4054b0fed128]::build::mir_built
  27:     0x7f2b3b9f8522 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_middle[4f81a7ac8c79b4c8]::ty::WithOptConstParam<rustc_span[84eb59eb28be3601]::def_id::LocalDefId>, &rustc_data_structures[d4cd6911de3a1e89]::steal::Steal<rustc_middle[4f81a7ac8c79b4c8]::mir::Body>>>
  28:     0x7f2b3bb582fe - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::mir_built, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  29:     0x7f2b3b6df6a3 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::mir_built
  30:     0x7f2b3a8ab324 - rustc_mir_transform[dba757bcdc012a24]::check_unsafety::unsafety_check_result
  31:     0x7f2b3a8a67cc - <rustc_mir_transform[dba757bcdc012a24]::check_unsafety::provide::{closure#0} as core[867cfca19013d5a]::ops::function::FnOnce<(rustc_middle[4f81a7ac8c79b4c8]::ty::context::TyCtxt, rustc_span[84eb59eb28be3601]::def_id::LocalDefId)>>::call_once
  32:     0x7f2b3ba63d95 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::VecCache<rustc_span[84eb59eb28be3601]::def_id::LocalDefId, &rustc_middle[4f81a7ac8c79b4c8]::mir::query::UnsafetyCheckResult>>
  33:     0x7f2b3bb26568 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::unsafety_check_result, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  34:     0x7f2b3b6f1a44 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::unsafety_check_result
  35:     0x7f2b3a827f5d - rustc_mir_transform[dba757bcdc012a24]::mir_const
  36:     0x7f2b3b9f8522 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_middle[4f81a7ac8c79b4c8]::ty::WithOptConstParam<rustc_span[84eb59eb28be3601]::def_id::LocalDefId>, &rustc_data_structures[d4cd6911de3a1e89]::steal::Steal<rustc_middle[4f81a7ac8c79b4c8]::mir::Body>>>
  37:     0x7f2b3bb58421 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::mir_const, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  38:     0x7f2b3b6dfd13 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::mir_const
  39:     0x7f2b3a828f2b - rustc_mir_transform[dba757bcdc012a24]::mir_promoted
  40:     0x7f2b3bb01bc9 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::mir_promoted, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  41:     0x7f2b3b6e23d3 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::mir_promoted
  42:     0x7f2b3b0698f0 - rustc_borrowck[700c5cd42141f1e3]::mir_borrowck
  43:     0x7f2b3b03bb41 - <rustc_borrowck[700c5cd42141f1e3]::provide::{closure#0} as core[867cfca19013d5a]::ops::function::FnOnce<(rustc_middle[4f81a7ac8c79b4c8]::ty::context::TyCtxt, rustc_span[84eb59eb28be3601]::def_id::LocalDefId)>>::call_once
  44:     0x7f2b3ba62e05 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::VecCache<rustc_span[84eb59eb28be3601]::def_id::LocalDefId, &rustc_middle[4f81a7ac8c79b4c8]::mir::query::BorrowCheckResult>>
  45:     0x7f2b3bb011be - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::mir_borrowck, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  46:     0x7f2b3b6fd3f4 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::mir_borrowck
  47:     0x7f2b39dc7b41 - std[65b05b99172e1c7e]::panic::catch_unwind::<core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d4cd6911de3a1e89]::sync::par_for_each_in<&[rustc_span[84eb59eb28be3601]::def_id::LocalDefId], <rustc_middle[4f81a7ac8c79b4c8]::hir::map::Map>::par_body_owners<rustc_interface[f9336dc587154158]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  48:     0x7f2b39dc0d77 - rustc_data_structures[d4cd6911de3a1e89]::sync::par_for_each_in::<&[rustc_span[84eb59eb28be3601]::def_id::LocalDefId], <rustc_middle[4f81a7ac8c79b4c8]::hir::map::Map>::par_body_owners<rustc_interface[f9336dc587154158]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  49:     0x7f2b39dd732f - <rustc_session[26948ece6d29333b]::session::Session>::time::<(), rustc_interface[f9336dc587154158]::passes::analysis::{closure#2}>
  50:     0x7f2b39df76ab - rustc_interface[f9336dc587154158]::passes::analysis
  51:     0x7f2b3ba31f55 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<(), core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>>
  52:     0x7f2b3bb56210 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::analysis, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  53:     0x7f2b3b6d75da - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::analysis
  54:     0x7f2b39cffda8 - <rustc_interface[f9336dc587154158]::passes::QueryContext>::enter::<rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  55:     0x7f2b39d09cce - <rustc_interface[f9336dc587154158]::interface::Compiler>::enter::<rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}::{closure#2}, core[867cfca19013d5a]::result::Result<core[867cfca19013d5a]::option::Option<rustc_interface[f9336dc587154158]::queries::Linker>, rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  56:     0x7f2b39caa042 - rustc_span[84eb59eb28be3601]::with_source_map::<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  57:     0x7f2b39d0a7c8 - <scoped_tls[b64ea83672690cf8]::ScopedKey<rustc_span[84eb59eb28be3601]::SessionGlobals>>::set::<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  58:     0x7f2b39cc7560 - std[65b05b99172e1c7e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  59:     0x7f2b39d0bb76 - std[65b05b99172e1c7e]::panicking::try::<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<<std[65b05b99172e1c7e]::thread::Builder>::spawn_unchecked_<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  60:     0x7f2b39cb85c9 - <<std[65b05b99172e1c7e]::thread::Builder>::spawn_unchecked_<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#1} as core[867cfca19013d5a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7f2b392d5e0e - std::sys::unix::thread::Thread::new::thread_start::h0d9480603bdbd4eb
  62:     0x7f2b3906ab43 - <unknown>
  63:     0x7f2b390fca00 - <unknown>
  64:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (0675db779 2022-11-28) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [codegen_select_candidate] computing candidate for `<[closure@library/core/src/slice/sort.rs:694:37: 694:52] as ops::function::FnMut<(&mut usize,)>>`
#1 [resolve_instance] resolving instance `<[closure@library/core/src/slice/sort.rs:694:37: 694:52] as ops::function::FnMut<(&mut usize,)>>::call_mut`
#2 [mir_built] building MIR for `slice::sort::choose_pivot`
#3 [unsafety_check_result] unsafety-checking `slice::sort::choose_pivot`
#4 [mir_const] preparing `slice::sort::choose_pivot` for borrow checking
#5 [mir_promoted] processing MIR for `slice::sort::choose_pivot`
#6 [mir_borrowck] borrow-checking `slice::sort::choose_pivot`
#7 [analysis] running analysis passes on this crate
thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Binder(<iter::adapters::filter::Filter<slice::iter::Iter<'_, u8>, [closure@library/core/src/str/count.rs:135:21: 135:29]> as iter::traits::iterator::Iterator>, [])`,
 right: `Binder(<iter::adapters::filter::Filter<slice::iter::Iter<'_, u8>, [closure@library/core/src/str/count.rs:135:21: 135:29]> as iter::traits::iterator::Iterator>, [])`', compiler/rustc_trait_selection/src/traits/codegen.rs:28:5
   0:     0x7f2b392c5512 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h76efda83803f7c88
   0:     0x7f2b392c5512 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h76efda83803f7c88
   1:     0x7f2b39333638 - core::fmt::write::h0cd9f5419c66d611
   2:     0x7f2b392b5fe1 - std::io::Write::write_fmt::h115d78592252bf9d
   3:     0x7f2b392c52d5 - std::sys_common::backtrace::print::hd9ebfd55eb3388eb
   4:     0x7f2b392c85e7 - std::panicking::default_hook::{{closure}}::h98850a6457a8782d
   5:     0x7f2b392c8345 - std::panicking::default_hook::hef750a0d73d5e8e9
   6:     0x7f2b39ca8984 - rustc_driver[ab3ba77cd7534cfb]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f2b392c8ef3 - std::panicking::rust_panic_with_hook::ha62dd9a4e901383e
   8:     0x7f2b392c8c27 - std::panicking::begin_panic_handler::{{closure}}::h30577589484d4d4f
   9:     0x7f2b392c5abc - std::sys_common::backtrace::__rust_end_short_backtrace::h8641e565b807163f
  10:     0x7f2b392c88f2 - rust_begin_unwind
  11:     0x7f2b3927a023 - core::panicking::panic_fmt::ha29375e42f9e82fb
  12:     0x7f2b3932fefb - core::panicking::assert_failed_inner::hc36543c12f11956e
  13:     0x7f2b39c217bb - core[867cfca19013d5a]::panicking::assert_failed::<rustc_middle[4f81a7ac8c79b4c8]::ty::sty::Binder<rustc_middle[4f81a7ac8c79b4c8]::ty::sty::TraitRef>, rustc_middle[4f81a7ac8c79b4c8]::ty::sty::Binder<rustc_middle[4f81a7ac8c79b4c8]::ty::sty::TraitRef>>
  14:     0x7f2b3c5b37c3 - rustc_trait_selection[684cd2d7a7441411]::traits::codegen::codegen_select_candidate
  15:     0x7f2b3bb391be - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::codegen_select_candidate, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  16:     0x7f2b3b7100c8 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::codegen_select_candidate
  17:     0x7f2b3a1539ff - rustc_ty_utils[40d190a702156b7]::instance::inner_resolve_instance
  18:     0x7f2b3a152708 - rustc_ty_utils[40d190a702156b7]::instance::resolve_instance
  19:     0x7f2b3bb11bef - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::resolve_instance, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  20:     0x7f2b3b73fa18 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::resolve_instance
  21:     0x7f2b3c9a6614 - <rustc_middle[4f81a7ac8c79b4c8]::ty::instance::Instance>::resolve_opt_const_arg
  22:     0x7f2b3c9a6294 - <rustc_middle[4f81a7ac8c79b4c8]::ty::instance::Instance>::resolve
  23:     0x7f2b3ae51082 - <rustc_mir_build[6ecd4054b0fed128]::lints::Search>::is_recursive_call
  24:     0x7f2b3ae69586 - <rustc_data_structures[d4cd6911de3a1e89]::graph::iterate::TriColorDepthFirstSearch<rustc_middle[4f81a7ac8c79b4c8]::mir::basic_blocks::BasicBlocks>>::run_from_start::<rustc_mir_build[6ecd4054b0fed128]::lints::Search>
  25:     0x7f2b3ae509b5 - rustc_mir_build[6ecd4054b0fed128]::lints::check
  26:     0x7f2b3ad8f77f - rustc_mir_build[6ecd4054b0fed128]::build::mir_built
  27:     0x7f2b3b9f8522 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_middle[4f81a7ac8c79b4c8]::ty::WithOptConstParam<rustc_span[84eb59eb28be3601]::def_id::LocalDefId>, &rustc_data_structures[d4cd6911de3a1e89]::steal::Steal<rustc_middle[4f81a7ac8c79b4c8]::mir::Body>>>
  28:     0x7f2b3bb582fe - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::mir_built, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  29:     0x7f2b3b6df6a3 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::mir_built
  30:     0x7f2b3a8ab324 - rustc_mir_transform[dba757bcdc012a24]::check_unsafety::unsafety_check_result
  31:     0x7f2b3a8a67cc - <rustc_mir_transform[dba757bcdc012a24]::check_unsafety::provide::{closure#0} as core[867cfca19013d5a]::ops::function::FnOnce<(rustc_middle[4f81a7ac8c79b4c8]::ty::context::TyCtxt, rustc_span[84eb59eb28be3601]::def_id::LocalDefId)>>::call_once
  32:     0x7f2b3ba63d95 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::VecCache<rustc_span[84eb59eb28be3601]::def_id::LocalDefId, &rustc_middle[4f81a7ac8c79b4c8]::mir::query::UnsafetyCheckResult>>
  33:     0x7f2b3bb26568 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::unsafety_check_result, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  34:     0x7f2b3b6f1a44 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::unsafety_check_result
  35:     0x7f2b3a827f5d - rustc_mir_transform[dba757bcdc012a24]::mir_const
  36:     0x7f2b3b9f8522 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<rustc_middle[4f81a7ac8c79b4c8]::ty::WithOptConstParam<rustc_span[84eb59eb28be3601]::def_id::LocalDefId>, &rustc_data_structures[d4cd6911de3a1e89]::steal::Steal<rustc_middle[4f81a7ac8c79b4c8]::mir::Body>>>
  37:     0x7f2b3bb58421 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::mir_const, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  38:     0x7f2b3b6dfd13 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::mir_const
  39:     0x7f2b3a828f2b - rustc_mir_transform[dba757bcdc012a24]::mir_promoted
  40:     0x7f2b3bb01bc9 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::mir_promoted, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  41:     0x7f2b3b6e23d3 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::mir_promoted
  42:     0x7f2b3b0698f0 - rustc_borrowck[700c5cd42141f1e3]::mir_borrowck
  43:     0x7f2b3b03bb41 - <rustc_borrowck[700c5cd42141f1e3]::provide::{closure#0} as core[867cfca19013d5a]::ops::function::FnOnce<(rustc_middle[4f81a7ac8c79b4c8]::ty::context::TyCtxt, rustc_span[84eb59eb28be3601]::def_id::LocalDefId)>>::call_once
  44:     0x7f2b3ba62e05 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::VecCache<rustc_span[84eb59eb28be3601]::def_id::LocalDefId, &rustc_middle[4f81a7ac8c79b4c8]::mir::query::BorrowCheckResult>>
  45:     0x7f2b3bb011be - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::mir_borrowck, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  46:     0x7f2b3b6fd3f4 - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::mir_borrowck
  47:     0x7f2b39dc7b41 - std[65b05b99172e1c7e]::panic::catch_unwind::<core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d4cd6911de3a1e89]::sync::par_for_each_in<&[rustc_span[84eb59eb28be3601]::def_id::LocalDefId], <rustc_middle[4f81a7ac8c79b4c8]::hir::map::Map>::par_body_owners<rustc_interface[f9336dc587154158]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  48:     0x7f2b39dc0d77 - rustc_data_structures[d4cd6911de3a1e89]::sync::par_for_each_in::<&[rustc_span[84eb59eb28be3601]::def_id::LocalDefId], <rustc_middle[4f81a7ac8c79b4c8]::hir::map::Map>::par_body_owners<rustc_interface[f9336dc587154158]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  49:     0x7f2b39dd732f - <rustc_session[26948ece6d29333b]::session::Session>::time::<(), rustc_interface[f9336dc587154158]::passes::analysis::{closure#2}>
  50:     0x7f2b39df76ab - rustc_interface[f9336dc587154158]::passes::analysis
  51:     0x7f2b3ba31f55 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::try_execute_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt, rustc_query_system[b7532c72bbf7e115]::query::caches::DefaultCache<(), core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>>
  52:     0x7f2b3bb56210 - rustc_query_system[b7532c72bbf7e115]::query::plumbing::get_query::<rustc_query_impl[b2ca3a4d56ef3bb1]::queries::analysis, rustc_query_impl[b2ca3a4d56ef3bb1]::plumbing::QueryCtxt>
  53:     0x7f2b3b6d75da - <rustc_query_impl[b2ca3a4d56ef3bb1]::Queries as rustc_middle[4f81a7ac8c79b4c8]::ty::query::QueryEngine>::analysis
  54:     0x7f2b39cffda8 - <rustc_interface[f9336dc587154158]::passes::QueryContext>::enter::<rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  55:     0x7f2b39d09cce - <rustc_interface[f9336dc587154158]::interface::Compiler>::enter::<rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}::{closure#2}, core[867cfca19013d5a]::result::Result<core[867cfca19013d5a]::option::Option<rustc_interface[f9336dc587154158]::queries::Linker>, rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  56:     0x7f2b39caa042 - rustc_span[84eb59eb28be3601]::with_source_map::<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  57:     0x7f2b39d0a7c8 - <scoped_tls[b64ea83672690cf8]::ScopedKey<rustc_span[84eb59eb28be3601]::SessionGlobals>>::set::<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  58:     0x7f2b39cc7560 - std[65b05b99172e1c7e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>
  59:     0x7f2b39d0bb76 - std[65b05b99172e1c7e]::panicking::try::<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<<std[65b05b99172e1c7e]::thread::Builder>::spawn_unchecked_<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  60:     0x7f2b39cb85c9 - <<std[65b05b99172e1c7e]::thread::Builder>::spawn_unchecked_<rustc_interface[f9336dc587154158]::util::run_in_thread_pool_with_globals<rustc_interface[f9336dc587154158]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>, rustc_driver[ab3ba77cd7534cfb]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[6d9bb464f3a9d055]::ErrorGuaranteed>>::{closure#1} as core[867cfca19013d5a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7f2b392d5e0e - std::sys::unix::thread::Thread::new::thread_start::h0d9480603bdbd4eb
  62:     0x7f2b3906ab43 - <unknown>
  63:     0x7f2b390fca00 - <unknown>
  64:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (0675db779 2022-11-28) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [codegen_select_candidate] computing candidate for `<iter::adapters::filter::Filter<slice::iter::Iter<'_, u8>, [closure@library/core/src/str/count.rs:135:21: 135:29]> as iter::traits::iterator::Iterator>`
#1 [resolve_instance] resolving instance `<iter::adapters::filter::Filter<slice::iter::Iter<'_, u8>, [closure@library/core/src/str/count.rs:135:21: 135:29]> as iter::traits::iterator::Iterator>::count`
#2 [mir_built] building MIR for `str::count::char_count_general_case`
#3 [unsafety_check_result] unsafety-checking `str::count::char_count_general_case`
#4 [mir_const] preparing `str::count::char_count_general_case` for borrow checking
#5 [mir_promoted] processing MIR for `str::count::char_count_general_case`
#6 [mir_borrowck] borrow-checking `str::count::char_count_general_case`
#7 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:03:25
