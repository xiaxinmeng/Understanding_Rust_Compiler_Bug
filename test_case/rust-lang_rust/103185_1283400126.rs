plain
i....................................................................................... 880/13681
.......i................................................................................ 968/13681
........................................................................................ 1056/13681
........................................................................................ 1144/13681
.......F........................................................F..........F............ 1232/13681
..i..................................................................................... 1408/13681
..................F............................................i........................ 1496/13681
........................................................................................ 1584/13681
........................................................................................ 1672/13681
---
........................................................................................ 6776/13681
................................i............F.......................................... 6864/13681
.ii.ii........i....i...............................................................i.... 6952/13681
........................................................................................ 7040/13681
................................................................i....iF................. 7128/13681
..............F........i..................i.............i............................... 7216/13681
...................................................i.................................... 7392/13681
........................................................................................ 7480/13681
........................................................................................ 7568/13681
..................................................ii.................................... 7656/13681
---
........................................................................................ 10032/13681
....................................................ii...............i.................. 10120/13681
........................................................................................ 10208/13681
........................................................................................ 10296/13681
...........F.....F...F............F..................................................... 10384/13681
........................................................................................ 10560/13681
...............................................................................F........ 10648/13681
........................................................................................ 10736/13681
........................................................................................ 10824/13681
---
failures:

---- [ui] src/test/ui/async-await/async-borrowck-escaping-block-error.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-borrowck-escaping-block-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0373]: async block may outlive the current function, but it borrows `x`, which is owned by the current function
   |
   |
LL |     Box::new(async { x } )
   |                    ^^-^^
   |                    | |
   |                    | `x` is borrowed here
   |                    may outlive borrowed value `x`
note: async block is returned here
  --> /checkout/src/test/ui/async-await/async-borrowck-escaping-block-error.rs:6:5
   |
   |
LL |     Box::new(async { x } )
   |     ^^^^^^^^^^^^^^^^^^^^^^
help: to force the async block to take ownership of `x` (and any other referenced variables), use the `move` keyword
   |
LL |     Box::new(async move { x } )

thread 'rustc' panicked at 'attempt to subtract with overflow', compiler/rustc_span/src/source_map.rs:935:41
stack backtrace:
stack backtrace:
   0:     0x7fd5c2eb659e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9c7bc3008d4753a1
   1:     0x7fd5c2f1fb68 - core::fmt::write::h1372e916b4c89768
   2:     0x7fd5c2ea7cb1 - std::io::Write::write_fmt::hf60dae036630dd2a
   3:     0x7fd5c2eb63a1 - std::sys_common::backtrace::print::hba316f9dedc1f4d4
   4:     0x7fd5c2eb9524 - std::panicking::default_hook::{{closure}}::h377d822b7482aed0
   5:     0x7fd5c2eb91e9 - std::panicking::default_hook::h74f45b104d7cdcb8
   6:     0x7fd5c3896344 - rustc_driver[4f321b30148f513d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fd5c2eb9c74 - std::panicking::rust_panic_with_hook::h5d346fceab4c6d09
   8:     0x7fd5c2eb9999 - std::panicking::begin_panic_handler::{{closure}}::h8c9caaba91e6b8d5
   9:     0x7fd5c2eb6ad4 - std::sys_common::backtrace::__rust_end_short_backtrace::heafcf7edf95f5fdc
  10:     0x7fd5c2eb96a2 - rust_begin_unwind
  11:     0x7fd5c2e6aa93 - core::panicking::panic_fmt::he457b2b30a462006
  12:     0x7fd5c2e6ab6d - core::panicking::panic::hb2f6a2d4688e3999
  13:     0x7fd5c6943990 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::find_width_of_character_at_span
  14:     0x7fd5c6942161 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::end_point
  15:     0x7fd5c4ba9d65 - <rustc_borrowck[cae355615d4a7533]::MirBorrowckCtxt as rustc_mir_dataflow[40da318b522903ad]::framework::visitor::ResultsVisitor>::visit_terminator_after_primary_effect
  16:     0x7fd5c4a4f4ee - rustc_mir_dataflow[40da318b522903ad]::framework::visitor::visit_results::<rustc_borrowck[cae355615d4a7533]::dataflow::BorrowckAnalyses<rustc_index[a72efc4009e621fb]::bit_set::BitSet<rustc_borrowck[cae355615d4a7533]::dataflow::BorrowIndex>, rustc_index[a72efc4009e621fb]::bit_set::ChunkedBitSet<rustc_mir_dataflow[40da318b522903ad]::move_paths::MovePathIndex>, rustc_index[a72efc4009e621fb]::bit_set::ChunkedBitSet<rustc_mir_dataflow[40da318b522903ad]::move_paths::InitIndex>>, rustc_borrowck[cae355615d4a7533]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_borrowck[cae355615d4a7533]::dataflow::Borrows>, rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_mir_dataflow[40da318b522903ad]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_mir_dataflow[40da318b522903ad]::impls::EverInitializedPlaces>>, core[b9cd8aca94731106]::iter::adapters::map::Map<rustc_middle[ebe8ea6241d0cae4]::mir::traversal::ReversePostorderIter, rustc_borrowck[cae355615d4a7533]::do_mir_borrowck::{closure#2}>, rustc_borrowck[cae355615d4a7533]::MirBorrowckCtxt>
  17:     0x7fd5c4bb7649 - rustc_borrowck[cae355615d4a7533]::do_mir_borrowck
  18:     0x7fd5c4ba7832 - rustc_borrowck[cae355615d4a7533]::mir_borrowck
  19:     0x7fd5c4b74e8e - <rustc_borrowck[cae355615d4a7533]::provide::{closure#0} as core[b9cd8aca94731106]::ops::function::FnOnce<(rustc_middle[ebe8ea6241d0cae4]::ty::context::TyCtxt, rustc_span[2882602c202d4b9d]::def_id::LocalDefId)>>::call_once
  20:     0x7fd5c5537839 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<rustc_span[2882602c202d4b9d]::def_id::LocalDefId, &rustc_middle[ebe8ea6241d0cae4]::mir::query::BorrowCheckResult>>
  21:     0x7fd5c5614cc5 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::mir_borrowck, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  22:     0x7fd5c51d6f10 - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::mir_borrowck
  23:     0x7fd5c3f30b3e - rustc_hir_analysis[992278d4339648d7]::collect::type_of::find_opaque_ty_constraints_for_rpit
  24:     0x7fd5c3f2fd8c - rustc_hir_analysis[992278d4339648d7]::collect::type_of::type_of
  25:     0x7fd5c5554330 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<rustc_span[2882602c202d4b9d]::def_id::DefId, rustc_middle[ebe8ea6241d0cae4]::ty::Ty>>
  26:     0x7fd5c5667a77 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::type_of, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  27:     0x7fd5c51a9ca5 - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::type_of
  28:     0x7fd5c40ec50a - rustc_hir_analysis[992278d4339648d7]::check::check::check_opaque
  29:     0x7fd5c40efb1f - rustc_hir_analysis[992278d4339648d7]::check::check::check_item_type
  30:     0x7fd5c40fc72a - rustc_hir_analysis[992278d4339648d7]::check::check::check_mod_item_types
  31:     0x7fd5c553a172 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<rustc_span[2882602c202d4b9d]::def_id::LocalDefId, ()>>
  32:     0x7fd5c5632a0a - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::check_mod_item_types, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  33:     0x7fd5c51cf900 - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::check_mod_item_types
  34:     0x7fd5c3f0c2ca - <rustc_session[d1ae2edc16765877]::session::Session>::time::<(), rustc_hir_analysis[992278d4339648d7]::check_crate::{closure#6}>
  35:     0x7fd5c418c121 - rustc_hir_analysis[992278d4339648d7]::check_crate
  36:     0x7fd5c39f0d65 - rustc_interface[c95f1242d2b864cb]::passes::analysis
  37:     0x7fd5c5580fef - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<(), core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>>
  38:     0x7fd5c5667b8b - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::analysis, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  39:     0x7fd5c51aab8a - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::analysis
  40:     0x7fd5c38f24a9 - <rustc_interface[c95f1242d2b864cb]::passes::QueryContext>::enter::<rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  41:     0x7fd5c3907e4a - <rustc_interface[c95f1242d2b864cb]::interface::Compiler>::enter::<rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}::{closure#2}, core[b9cd8aca94731106]::result::Result<core[b9cd8aca94731106]::option::Option<rustc_interface[c95f1242d2b864cb]::queries::Linker>, rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  42:     0x7fd5c388499e - rustc_span[2882602c202d4b9d]::with_source_map::<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  43:     0x7fd5c38f7ab2 - std[f68e0b14c4fa0719]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  44:     0x7fd5c38ff5f8 - std[f68e0b14c4fa0719]::panic::catch_unwind::<core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<<std[f68e0b14c4fa0719]::thread::Builder>::spawn_unchecked_<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  45:     0x7fd5c38ab81a - <<std[f68e0b14c4fa0719]::thread::Builder>::spawn_unchecked_<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#1} as core[b9cd8aca94731106]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:     0x7fd5c2ec64e5 - std::sys::unix::thread::Thread::new::thread_start::h952c3210da94aa1b
  47:     0x7fd5c2c60b43 - <unknown>
  48:     0x7fd5c2cf2a00 - <unknown>
  49:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (ff7bd22bd 2022-10-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `test_ref`
#1 [type_of] computing type of `test_ref::{opaque#0}`
#2 [check_mod_item_types] checking item types in top-level module
#3 [analysis] running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0373`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-local-borrow-outlives-fn.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-local-borrow-outlives-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow-outlives-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow-outlives-fn/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempt to subtract with overflow', compiler/rustc_span/src/source_map.rs:935:41
stack backtrace:
   0:     0x7f6611c1b59e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9c7bc3008d4753a1
   1:     0x7f6611c84b68 - core::fmt::write::h1372e916b4c89768
   2:     0x7f6611c0ccb1 - std::io::Write::write_fmt::hf60dae036630dd2a
   3:     0x7f6611c1b3a1 - std::sys_common::backtrace::print::hba316f9dedc1f4d4
   4:     0x7f6611c1e524 - std::panicking::default_hook::{{closure}}::h377d822b7482aed0
   5:     0x7f6611c1e1e9 - std::panicking::default_hook::h74f45b104d7cdcb8
   6:     0x7f66125fb344 - rustc_driver[4f321b30148f513d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f6611c1ec74 - std::panicking::rust_panic_with_hook::h5d346fceab4c6d09
   8:     0x7f6611c1e999 - std::panicking::begin_panic_handler::{{closure}}::h8c9caaba91e6b8d5
   9:     0x7f6611c1bad4 - std::sys_common::backtrace::__rust_end_short_backtrace::heafcf7edf95f5fdc
  10:     0x7f6611c1e6a2 - rust_begin_unwind
  11:     0x7f6611bcfa93 - core::panicking::panic_fmt::he457b2b30a462006
  12:     0x7f6611bcfb6d - core::panicking::panic::hb2f6a2d4688e3999
  13:     0x7f66156a8990 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::find_width_of_character_at_span
  14:     0x7f66156a7161 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::end_point
  15:     0x7f661390ed65 - <rustc_borrowck[cae355615d4a7533]::MirBorrowckCtxt as rustc_mir_dataflow[40da318b522903ad]::framework::visitor::ResultsVisitor>::visit_terminator_after_primary_effect
  16:     0x7f66137b44ee - rustc_mir_dataflow[40da318b522903ad]::framework::visitor::visit_results::<rustc_borrowck[cae355615d4a7533]::dataflow::BorrowckAnalyses<rustc_index[a72efc4009e621fb]::bit_set::BitSet<rustc_borrowck[cae355615d4a7533]::dataflow::BorrowIndex>, rustc_index[a72efc4009e621fb]::bit_set::ChunkedBitSet<rustc_mir_dataflow[40da318b522903ad]::move_paths::MovePathIndex>, rustc_index[a72efc4009e621fb]::bit_set::ChunkedBitSet<rustc_mir_dataflow[40da318b522903ad]::move_paths::InitIndex>>, rustc_borrowck[cae355615d4a7533]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_borrowck[cae355615d4a7533]::dataflow::Borrows>, rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_mir_dataflow[40da318b522903ad]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_mir_dataflow[40da318b522903ad]::impls::EverInitializedPlaces>>, core[b9cd8aca94731106]::iter::adapters::map::Map<rustc_middle[ebe8ea6241d0cae4]::mir::traversal::ReversePostorderIter, rustc_borrowck[cae355615d4a7533]::do_mir_borrowck::{closure#2}>, rustc_borrowck[cae355615d4a7533]::MirBorrowckCtxt>
  17:     0x7f661391c649 - rustc_borrowck[cae355615d4a7533]::do_mir_borrowck
  18:     0x7f661390c832 - rustc_borrowck[cae355615d4a7533]::mir_borrowck
  19:     0x7f66138d9e8e - <rustc_borrowck[cae355615d4a7533]::provide::{closure#0} as core[b9cd8aca94731106]::ops::function::FnOnce<(rustc_middle[ebe8ea6241d0cae4]::ty::context::TyCtxt, rustc_span[2882602c202d4b9d]::def_id::LocalDefId)>>::call_once
  20:     0x7f661429c839 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<rustc_span[2882602c202d4b9d]::def_id::LocalDefId, &rustc_middle[ebe8ea6241d0cae4]::mir::query::BorrowCheckResult>>
  21:     0x7f6614379cc5 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::mir_borrowck, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  22:     0x7f6613f3bf10 - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::mir_borrowck
  23:     0x7f66127bcb26 - <core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[b9cd8aca94731106]::ops::function::FnOnce<()>>::call_once
  24:     0x7f661272e036 - std[f68e0b14c4fa0719]::panicking::try::<(), core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  25:     0x7f661271356d - rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in::<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  26:     0x7f6612756060 - rustc_interface[c95f1242d2b864cb]::passes::analysis
  27:     0x7f66142e5fef - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<(), core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>>
  28:     0x7f66143ccb8b - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::analysis, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  29:     0x7f6613f0fb8a - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::analysis
  30:     0x7f66126574a9 - <rustc_interface[c95f1242d2b864cb]::passes::QueryContext>::enter::<rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  31:     0x7f661266ce4a - <rustc_interface[c95f1242d2b864cb]::interface::Compiler>::enter::<rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}::{closure#2}, core[b9cd8aca94731106]::result::Result<core[b9cd8aca94731106]::option::Option<rustc_interface[c95f1242d2b864cb]::queries::Linker>, rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  32:     0x7f66125e999e - rustc_span[2882602c202d4b9d]::with_source_map::<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7f661265cab2 - std[f68e0b14c4fa0719]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  34:     0x7f66126645f8 - std[f68e0b14c4fa0719]::panic::catch_unwind::<core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<<std[f68e0b14c4fa0719]::thread::Builder>::spawn_unchecked_<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  35:     0x7f661261081a - <<std[f68e0b14c4fa0719]::thread::Builder>::spawn_unchecked_<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#1} as core[b9cd8aca94731106]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f6611c2b4e5 - std::sys::unix::thread::Thread::new::thread_start::h952c3210da94aa1b
  37:     0x7f66119c5b43 - <unknown>
  38:     0x7f6611a57a00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (ff7bd22bd 2022-10-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `cplusplus_mode`
#1 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-return-variable-on-stack-via-clone.rs stdout ----
---- [ui] src/test/ui/borrowck/borrowck-return-variable-on-stack-via-clone.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-return-variable-on-stack-via-clone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-return-variable-on-stack-via-clone" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-return-variable-on-stack-via-clone/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempt to subtract with overflow', compiler/rustc_span/src/source_map.rs:935:41
stack backtrace:
   0:     0x7f7307a0159e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9c7bc3008d4753a1
   1:     0x7f7307a6ab68 - core::fmt::write::h1372e916b4c89768
   2:     0x7f73079f2cb1 - std::io::Write::write_fmt::hf60dae036630dd2a
   3:     0x7f7307a013a1 - std::sys_common::backtrace::print::hba316f9dedc1f4d4
   4:     0x7f7307a04524 - std::panicking::default_hook::{{closure}}::h377d822b7482aed0
   5:     0x7f7307a041e9 - std::panicking::default_hook::h74f45b104d7cdcb8
   6:     0x7f73083e1344 - rustc_driver[4f321b30148f513d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f7307a04c74 - std::panicking::rust_panic_with_hook::h5d346fceab4c6d09
   8:     0x7f7307a04999 - std::panicking::begin_panic_handler::{{closure}}::h8c9caaba91e6b8d5
   9:     0x7f7307a01ad4 - std::sys_common::backtrace::__rust_end_short_backtrace::heafcf7edf95f5fdc
  10:     0x7f7307a046a2 - rust_begin_unwind
  11:     0x7f73079b5a93 - core::panicking::panic_fmt::he457b2b30a462006
  12:     0x7f73079b5b6d - core::panicking::panic::hb2f6a2d4688e3999
  13:     0x7f730b48e990 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::find_width_of_character_at_span
  14:     0x7f730b48d161 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::end_point
  15:     0x7f73096f4d65 - <rustc_borrowck[cae355615d4a7533]::MirBorrowckCtxt as rustc_mir_dataflow[40da318b522903ad]::framework::visitor::ResultsVisitor>::visit_terminator_after_primary_effect
  16:     0x7f730959a4ee - rustc_mir_dataflow[40da318b522903ad]::framework::visitor::visit_results::<rustc_borrowck[cae355615d4a7533]::dataflow::BorrowckAnalyses<rustc_index[a72efc4009e621fb]::bit_set::BitSet<rustc_borrowck[cae355615d4a7533]::dataflow::BorrowIndex>, rustc_index[a72efc4009e621fb]::bit_set::ChunkedBitSet<rustc_mir_dataflow[40da318b522903ad]::move_paths::MovePathIndex>, rustc_index[a72efc4009e621fb]::bit_set::ChunkedBitSet<rustc_mir_dataflow[40da318b522903ad]::move_paths::InitIndex>>, rustc_borrowck[cae355615d4a7533]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_borrowck[cae355615d4a7533]::dataflow::Borrows>, rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_mir_dataflow[40da318b522903ad]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_mir_dataflow[40da318b522903ad]::impls::EverInitializedPlaces>>, core[b9cd8aca94731106]::iter::adapters::map::Map<rustc_middle[ebe8ea6241d0cae4]::mir::traversal::ReversePostorderIter, rustc_borrowck[cae355615d4a7533]::do_mir_borrowck::{closure#2}>, rustc_borrowck[cae355615d4a7533]::MirBorrowckCtxt>
  17:     0x7f7309702649 - rustc_borrowck[cae355615d4a7533]::do_mir_borrowck
  18:     0x7f73096f2832 - rustc_borrowck[cae355615d4a7533]::mir_borrowck
  19:     0x7f73096bfe8e - <rustc_borrowck[cae355615d4a7533]::provide::{closure#0} as core[b9cd8aca94731106]::ops::function::FnOnce<(rustc_middle[ebe8ea6241d0cae4]::ty::context::TyCtxt, rustc_span[2882602c202d4b9d]::def_id::LocalDefId)>>::call_once
  20:     0x7f730a082839 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<rustc_span[2882602c202d4b9d]::def_id::LocalDefId, &rustc_middle[ebe8ea6241d0cae4]::mir::query::BorrowCheckResult>>
  21:     0x7f730a15fcc5 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::mir_borrowck, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  22:     0x7f7309d21f10 - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::mir_borrowck
  23:     0x7f73085a2b26 - <core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[b9cd8aca94731106]::ops::function::FnOnce<()>>::call_once
  24:     0x7f7308514036 - std[f68e0b14c4fa0719]::panicking::try::<(), core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  25:     0x7f73084f956d - rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in::<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  26:     0x7f730853c060 - rustc_interface[c95f1242d2b864cb]::passes::analysis
  27:     0x7f730a0cbfef - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<(), core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>>
  28:     0x7f730a1b2b8b - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::analysis, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  29:     0x7f7309cf5b8a - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::analysis
  30:     0x7f730843d4a9 - <rustc_interface[c95f1242d2b864cb]::passes::QueryContext>::enter::<rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  31:     0x7f7308452e4a - <rustc_interface[c95f1242d2b864cb]::interface::Compiler>::enter::<rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}::{closure#2}, core[b9cd8aca94731106]::result::Result<core[b9cd8aca94731106]::option::Option<rustc_interface[c95f1242d2b864cb]::queries::Linker>, rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  32:     0x7f73083cf99e - rustc_span[2882602c202d4b9d]::with_source_map::<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7f7308442ab2 - std[f68e0b14c4fa0719]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  34:     0x7f730844a5f8 - std[f68e0b14c4fa0719]::panic::catch_unwind::<core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<<std[f68e0b14c4fa0719]::thread::Builder>::spawn_unchecked_<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  35:     0x7f73083f681a - <<std[f68e0b14c4fa0719]::thread::Builder>::spawn_unchecked_<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#1} as core[b9cd8aca94731106]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f7307a114e5 - std::sys::unix::thread::Thread::new::thread_start::h952c3210da94aa1b
  37:     0x7f73077abb43 - <unknown>
  38:     0x7f730783da00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (ff7bd22bd 2022-10-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `leak`
#1 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-thread-local-static-borrow-outlives-fn.rs stdout ----
---- [ui] src/test/ui/borrowck/borrowck-thread-local-static-borrow-outlives-fn.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-thread-local-static-borrow-outlives-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-thread-local-static-borrow-outlives-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-thread-local-static-borrow-outlives-fn/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempt to subtract with overflow', compiler/rustc_span/src/source_map.rs:935:41
stack backtrace:
   0:     0x7fcf64ea059e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9c7bc3008d4753a1
   1:     0x7fcf64f09b68 - core::fmt::write::h1372e916b4c89768
   2:     0x7fcf64e91cb1 - std::io::Write::write_fmt::hf60dae036630dd2a
   3:     0x7fcf64ea03a1 - std::sys_common::backtrace::print::hba316f9dedc1f4d4
   4:     0x7fcf64ea3524 - std::panicking::default_hook::{{closure}}::h377d822b7482aed0
   5:     0x7fcf64ea31e9 - std::panicking::default_hook::h74f45b104d7cdcb8
   6:     0x7fcf65880344 - rustc_driver[4f321b30148f513d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fcf64ea3c74 - std::panicking::rust_panic_with_hook::h5d346fceab4c6d09
   8:     0x7fcf64ea3999 - std::panicking::begin_panic_handler::{{closure}}::h8c9caaba91e6b8d5
   9:     0x7fcf64ea0ad4 - std::sys_common::backtrace::__rust_end_short_backtrace::heafcf7edf95f5fdc
  10:     0x7fcf64ea36a2 - rust_begin_unwind
  11:     0x7fcf64e54a93 - core::panicking::panic_fmt::he457b2b30a462006
  12:     0x7fcf64e54b6d - core::panicking::panic::hb2f6a2d4688e3999
  13:     0x7fcf6892d990 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::find_width_of_character_at_span
  14:     0x7fcf6892c161 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::end_point
  15:     0x7fcf66b93d65 - <rustc_borrowck[cae355615d4a7533]::MirBorrowckCtxt as rustc_mir_dataflow[40da318b522903ad]::framework::visitor::ResultsVisitor>::visit_terminator_after_primary_effect
  16:     0x7fcf66a394ee - rustc_mir_dataflow[40da318b522903ad]::framework::visitor::visit_results::<rustc_borrowck[cae355615d4a7533]::dataflow::BorrowckAnalyses<rustc_index[a72efc4009e621fb]::bit_set::BitSet<rustc_borrowck[cae355615d4a7533]::dataflow::BorrowIndex>, rustc_index[a72efc4009e621fb]::bit_set::ChunkedBitSet<rustc_mir_dataflow[40da318b522903ad]::move_paths::MovePathIndex>, rustc_index[a72efc4009e621fb]::bit_set::ChunkedBitSet<rustc_mir_dataflow[40da318b522903ad]::move_paths::InitIndex>>, rustc_borrowck[cae355615d4a7533]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_borrowck[cae355615d4a7533]::dataflow::Borrows>, rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_mir_dataflow[40da318b522903ad]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_mir_dataflow[40da318b522903ad]::impls::EverInitializedPlaces>>, core[b9cd8aca94731106]::iter::adapters::map::Map<rustc_middle[ebe8ea6241d0cae4]::mir::traversal::ReversePostorderIter, rustc_borrowck[cae355615d4a7533]::do_mir_borrowck::{closure#2}>, rustc_borrowck[cae355615d4a7533]::MirBorrowckCtxt>
  17:     0x7fcf66ba1649 - rustc_borrowck[cae355615d4a7533]::do_mir_borrowck
  18:     0x7fcf66b91832 - rustc_borrowck[cae355615d4a7533]::mir_borrowck
  19:     0x7fcf66b5ee8e - <rustc_borrowck[cae355615d4a7533]::provide::{closure#0} as core[b9cd8aca94731106]::ops::function::FnOnce<(rustc_middle[ebe8ea6241d0cae4]::ty::context::TyCtxt, rustc_span[2882602c202d4b9d]::def_id::LocalDefId)>>::call_once
  20:     0x7fcf67521839 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<rustc_span[2882602c202d4b9d]::def_id::LocalDefId, &rustc_middle[ebe8ea6241d0cae4]::mir::query::BorrowCheckResult>>
  21:     0x7fcf675fecc5 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::mir_borrowck, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  22:     0x7fcf671c0f10 - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::mir_borrowck
  23:     0x7fcf65a41b26 - <core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[b9cd8aca94731106]::ops::function::FnOnce<()>>::call_once
  24:     0x7fcf659b3036 - std[f68e0b14c4fa0719]::panicking::try::<(), core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  25:     0x7fcf6599856d - rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in::<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  26:     0x7fcf659db060 - rustc_interface[c95f1242d2b864cb]::passes::analysis
  27:     0x7fcf6756afef - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<(), core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>>
  28:     0x7fcf67651b8b - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::analysis, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  29:     0x7fcf67194b8a - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::analysis
  30:     0x7fcf658dc4a9 - <rustc_interface[c95f1242d2b864cb]::passes::QueryContext>::enter::<rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  31:     0x7fcf658f1e4a - <rustc_interface[c95f1242d2b864cb]::interface::Compiler>::enter::<rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}::{closure#2}, core[b9cd8aca94731106]::result::Result<core[b9cd8aca94731106]::option::Option<rustc_interface[c95f1242d2b864cb]::queries::Linker>, rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  32:     0x7fcf6586e99e - rustc_span[2882602c202d4b9d]::with_source_map::<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7fcf658e1ab2 - std[f68e0b14c4fa0719]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  34:     0x7fcf658e95f8 - std[f68e0b14c4fa0719]::panic::catch_unwind::<core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<<std[f68e0b14c4fa0719]::thread::Builder>::spawn_unchecked_<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  35:     0x7fcf6589581a - <<std[f68e0b14c4fa0719]::thread::Builder>::spawn_unchecked_<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#1} as core[b9cd8aca94731106]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fcf64eb04e5 - std::sys::unix::thread::Thread::new::thread_start::h952c3210da94aa1b
  37:     0x7fcf64c4ab43 - <unknown>
  38:     0x7fcf64cdca00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (ff7bd22bd 2022-10-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main`
#1 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/c-variadic/variadic-ffi-4.rs stdout ----
---- [ui] src/test/ui/c-variadic/variadic-ffi-4.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'f`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:8:5
   |
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ function was supposed to return data with lifetime `'f` but it is returning data with lifetime `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:14:5
   |
   |
LL | pub unsafe extern "C" fn no_escape1(_: usize, ap: ...) -> VaListImpl<'static> {
   |                                               -- has type `VaListImpl<'1>`
LL |     ap //~ ERROR: lifetime may not live long enough
   |     ^^ returning this value requires that `'1` must outlive `'static`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:18:31
   |
   |
LL |     let _ = ap.with_copy(|ap| ap); //~ ERROR: lifetime may not live long enough
   |                           --- ^^ returning this value requires that `'1` must outlive `'2`
   |                           | |
   |                           | return type of closure is VaList<'2, '_>
   |                           has type `VaList<'1, '_>`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
LL |     *ap0 = ap1;
   |     ^^^^ assignment requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
LL |     *ap0 = ap1;
   |     ^^^^ assignment requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
thread 'rustc' panicked at 'attempt to subtract with overflow', compiler/rustc_span/src/source_map.rs:935:41
stack backtrace:
stack backtrace:
   0:     0x7fdef506359e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9c7bc3008d4753a1
   1:     0x7fdef50ccb68 - core::fmt::write::h1372e916b4c89768
   2:     0x7fdef5054cb1 - std::io::Write::write_fmt::hf60dae036630dd2a
   3:     0x7fdef50633a1 - std::sys_common::backtrace::print::hba316f9dedc1f4d4
   4:     0x7fdef5066524 - std::panicking::default_hook::{{closure}}::h377d822b7482aed0
   5:     0x7fdef50661e9 - std::panicking::default_hook::h74f45b104d7cdcb8
   6:     0x7fdef5a43344 - rustc_driver[4f321b30148f513d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fdef5066c74 - std::panicking::rust_panic_with_hook::h5d346fceab4c6d09
   8:     0x7fdef5066999 - std::panicking::begin_panic_handler::{{closure}}::h8c9caaba91e6b8d5
   9:     0x7fdef5063ad4 - std::sys_common::backtrace::__rust_end_short_backtrace::heafcf7edf95f5fdc
  10:     0x7fdef50666a2 - rust_begin_unwind
  11:     0x7fdef5017a93 - core::panicking::panic_fmt::he457b2b30a462006
  12:     0x7fdef5017b6d - core::panicking::panic::hb2f6a2d4688e3999
  13:     0x7fdef8af0990 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::find_width_of_character_at_span
  14:     0x7fdef8aef161 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::end_point
  15:     0x7fdef6d56d65 - <rustc_borrowck[cae355615d4a7533]::MirBorrowckCtxt as rustc_mir_dataflow[40da318b522903ad]::framework::visitor::ResultsVisitor>::visit_terminator_after_primary_effect
  16:     0x7fdef6bfc4ee - rustc_mir_dataflow[40da318b522903ad]::framework::visitor::visit_results::<rustc_borrowck[cae355615d4a7533]::dataflow::BorrowckAnalyses<rustc_index[a72efc4009e621fb]::bit_set::BitSet<rustc_borrowck[cae355615d4a7533]::dataflow::BorrowIndex>, rustc_index[a72efc4009e621fb]::bit_set::ChunkedBitSet<rustc_mir_dataflow[40da318b522903ad]::move_paths::MovePathIndex>, rustc_index[a72efc4009e621fb]::bit_set::ChunkedBitSet<rustc_mir_dataflow[40da318b522903ad]::move_paths::InitIndex>>, rustc_borrowck[cae355615d4a7533]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_borrowck[cae355615d4a7533]::dataflow::Borrows>, rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_mir_dataflow[40da318b522903ad]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[40da318b522903ad]::framework::engine::Results<rustc_mir_dataflow[40da318b522903ad]::impls::EverInitializedPlaces>>, core[b9cd8aca94731106]::iter::adapters::map::Map<rustc_middle[ebe8ea6241d0cae4]::mir::traversal::ReversePostorderIter, rustc_borrowck[cae355615d4a7533]::do_mir_borrowck::{closure#2}>, rustc_borrowck[cae355615d4a7533]::MirBorrowckCtxt>
  17:     0x7fdef6d64649 - rustc_borrowck[cae355615d4a7533]::do_mir_borrowck
  18:     0x7fdef6d54832 - rustc_borrowck[cae355615d4a7533]::mir_borrowck
  19:     0x7fdef6d21e8e - <rustc_borrowck[cae355615d4a7533]::provide::{closure#0} as core[b9cd8aca94731106]::ops::function::FnOnce<(rustc_middle[ebe8ea6241d0cae4]::ty::context::TyCtxt, rustc_span[2882602c202d4b9d]::def_id::LocalDefId)>>::call_once
  20:     0x7fdef76e4839 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<rustc_span[2882602c202d4b9d]::def_id::LocalDefId, &rustc_middle[ebe8ea6241d0cae4]::mir::query::BorrowCheckResult>>
  21:     0x7fdef77c1cc5 - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::mir_borrowck, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  22:     0x7fdef7383f10 - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::mir_borrowck
  23:     0x7fdef5c04b26 - <core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[b9cd8aca94731106]::ops::function::FnOnce<()>>::call_once
  24:     0x7fdef5b76036 - std[f68e0b14c4fa0719]::panicking::try::<(), core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  25:     0x7fdef5b5b56d - rustc_data_structures[3088c7ced20d72f9]::sync::par_for_each_in::<&[rustc_span[2882602c202d4b9d]::def_id::LocalDefId], <rustc_middle[ebe8ea6241d0cae4]::hir::map::Map>::par_body_owners<rustc_interface[c95f1242d2b864cb]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  26:     0x7fdef5b9e060 - rustc_interface[c95f1242d2b864cb]::passes::analysis
  27:     0x7fdef772dfef - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::try_execute_query::<rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt, rustc_query_system[b41cf83b9f6b383d]::query::caches::DefaultCache<(), core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>>
  28:     0x7fdef7814b8b - rustc_query_system[b41cf83b9f6b383d]::query::plumbing::get_query::<rustc_query_impl[1ff86fb58015f018]::queries::analysis, rustc_query_impl[1ff86fb58015f018]::plumbing::QueryCtxt>
  29:     0x7fdef7357b8a - <rustc_query_impl[1ff86fb58015f018]::Queries as rustc_middle[ebe8ea6241d0cae4]::ty::query::QueryEngine>::analysis
  30:     0x7fdef5a9f4a9 - <rustc_interface[c95f1242d2b864cb]::passes::QueryContext>::enter::<rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  31:     0x7fdef5ab4e4a - <rustc_interface[c95f1242d2b864cb]::interface::Compiler>::enter::<rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}::{closure#2}, core[b9cd8aca94731106]::result::Result<core[b9cd8aca94731106]::option::Option<rustc_interface[c95f1242d2b864cb]::queries::Linker>, rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  32:     0x7fdef5a3199e - rustc_span[2882602c202d4b9d]::with_source_map::<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7fdef5aa4ab2 - std[f68e0b14c4fa0719]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  34:     0x7fdef5aac5f8 - std[f68e0b14c4fa0719]::panic::catch_unwind::<core[b9cd8aca94731106]::panic::unwind_safe::AssertUnwindSafe<<std[f68e0b14c4fa0719]::thread::Builder>::spawn_unchecked_<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>
  35:     0x7fdef5a5881a - <<std[f68e0b14c4fa0719]::thread::Builder>::spawn_unchecked_<rustc_interface[c95f1242d2b864cb]::util::run_in_thread_pool_with_globals<rustc_interface[c95f1242d2b864cb]::interface::run_compiler<core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>, rustc_driver[4f321b30148f513d]::run_compiler::{closure#1}>::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b9cd8aca94731106]::result::Result<(), rustc_errors[1a60a4a5e621f0c4]::ErrorGuaranteed>>::{closure#1} as core[b9cd8aca94731106]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fdef50734e5 - std::sys::unix::thread::Thread::new::thread_start::h952c3210da94aa1b
  37:     0x7fdef4e0db43 - <unknown>
  38:     0x7fdef4e9fa00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (ff7bd22bd 2022-10-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `no_escape4`
#1 [analysis] running analysis passes on this crate
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:35:12
   |
   |
LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1.clone();
   |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:35:12
   |
   |
LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1.clone();
   |            ^^^^^^^^^^^ argument requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to 8 previous errors
------------------------------------------



---- [ui] src/test/ui/closures/closure-bounds-static-cant-capture-borrowed.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-bounds-static-cant-capture-borrowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-bounds-static-cant-capture-borrowed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-bounds-static-cant-capture-borrowed/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempt to subtract with overflow', compiler/rustc_span/src/source_map.rs:935:41
stack backtrace:
   0:     0x7f1581f6459e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9c7bc3008d4753a1
   1:     0x7f1581fcdb68 - core::fmt::write::h1372e916b4c89768
   2:     0x7f1581f55cb1 - std::io::Write::write_fmt::hf60dae036630dd2a
   3:     0x7f1581f643a1 - std::sys_common::backtrace::print::hba316f9dedc1f4d4
   4:     0x7f1581f67524 - std::panicking::default_hook::{{closure}}::h377d822b7482aed0
   5:     0x7f1581f671e9 - std::panicking::default_hook::h74f45b104d7cdcb8
   6:     0x7f1582944344 - rustc_driver[4f321b30148f513d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1581f67c74 - std::panicking::rust_panic_with_hook::h5d346fceab4c6d09
   8:     0x7f1581f67999 - std::panicking::begin_panic_handler::{{closure}}::h8c9caaba91e6b8d5
   9:     0x7f1581f64ad4 - std::sys_common::backtrace::__rust_end_short_backtrace::heafcf7edf95f5fdc
  10:     0x7f1581f676a2 - rust_begin_unwind
  11:     0x7f1581f18a93 - core::panicking::panic_fmt::he457b2b30a462006
  12:     0x7f1581f18b6d - core::panicking::panic::hb2f6a2d4688e3999
  13:     0x7f15859f1990 - <rustc_span[2882602c202d4b9d]::source_map::SourceMap>::find_width_of_character_at_span
