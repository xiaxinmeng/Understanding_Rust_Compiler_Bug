plain
failures:

---- [ui] src/test/ui/generator/yield-in-box.rs stdout ----

error: test run failed!
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-in-box/a"
stdout: none
stderr: none

---- [ui] src/test/ui/mir/ssa-analysis-regression-50041.rs stdout ----

error: test compilation failed although it shouldn't!
error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/ssa-analysis-regression-50041.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_mir_transform/src/elaborate_box_derefs.rs:81:54
stack backtrace:
   0:     0x7f507a0989ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc84ef39a90e53929
   1:     0x7f507a0feff8 - core::fmt::write::h242f3131f164e830
   2:     0x7f507a088771 - std::io::Write::write_fmt::hbfef5ad44a295871
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   3:     0x7f507a09b9de - std::panicking::default_hook::{{closure}}::hae753f29448da830
   4:     0x7f507a09b60c - std::panicking::default_hook::h71f5cd75d58256a0
   5:     0x7f507abf78e1 - rustc_driver[691bc896c45f7d96]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f507a09c23e - std::panicking::rust_panic_with_hook::ha7b67fb0f9cc78c3
   7:     0x7f507a09bff9 - std::panicking::begin_panic_handler::{{closure}}::hcc6e5ba716b77417
   8:     0x7f507a098ec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h3d0c7415eb62a49a
   9:     0x7f507a09bd19 - rust_begin_unwind
  10:     0x7f507a050073 - core::panicking::panic_fmt::haf9aac53b6f0649c
  11:     0x7f507a04ff3d - core::panicking::panic::h37ac28709cb24f8c
  12:     0x7f507b137321 - <rustc_mir_transform[ff339bf59aa38738]::elaborate_box_derefs::ElaborateBoxDerefs as rustc_middle[99c12c0726164538]::mir::MirPass>::run_pass
  13:     0x7f507b172592 - rustc_mir_transform[ff339bf59aa38738]::pass_manager::run_passes
  14:     0x7f507b264d50 - rustc_mir_transform[ff339bf59aa38738]::run_post_borrowck_cleanup_passes
  15:     0x7f507b264609 - rustc_mir_transform[ff339bf59aa38738]::mir_drops_elaborated_and_const_checked
  16:     0x7f507c298a88 - rustc_query_system[ba2e98e81c5080c4]::query::plumbing::try_execute_query::<rustc_query_impl[281ef18973befe4]::plumbing::QueryCtxt, rustc_query_system[ba2e98e81c5080c4]::query::caches::DefaultCache<rustc_middle[99c12c0726164538]::ty::WithOptConstParam<rustc_span[ac0bc6d32e291b06]::def_id::LocalDefId>, &rustc_data_structures[89b2f884db6d167c]::steal::Steal<rustc_middle[99c12c0726164538]::mir::Body>>>
  17:     0x7f507c3da1da - rustc_query_system[ba2e98e81c5080c4]::query::plumbing::get_query::<rustc_query_impl[281ef18973befe4]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[281ef18973befe4]::plumbing::QueryCtxt>
  18:     0x7f507c7247c6 - <rustc_query_impl[281ef18973befe4]::Queries as rustc_middle[99c12c0726164538]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  19:     0x7f507b2652e8 - rustc_mir_transform[ff339bf59aa38738]::optimized_mir
  20:     0x7f507c2cced8 - rustc_query_system[ba2e98e81c5080c4]::query::plumbing::try_execute_query::<rustc_query_impl[281ef18973befe4]::plumbing::QueryCtxt, rustc_query_system[ba2e98e81c5080c4]::query::caches::DefaultCache<rustc_span[ac0bc6d32e291b06]::def_id::DefId, &rustc_middle[99c12c0726164538]::mir::Body>>
  21:     0x7f507c374703 - rustc_query_system[ba2e98e81c5080c4]::query::plumbing::get_query::<rustc_query_impl[281ef18973befe4]::queries::optimized_mir, rustc_query_impl[281ef18973befe4]::plumbing::QueryCtxt>
  22:     0x7f507ca8bcb8 - <rustc_metadata[ae525c484fd7d6a9]::rmeta::encoder::EncodeContext>::encode_crate_root
  23:     0x7f507caa25a6 - rustc_metadata[ae525c484fd7d6a9]::rmeta::encoder::encode_metadata_impl
  24:     0x7f507cb5d781 - rustc_data_structures[89b2f884db6d167c]::sync::join::<rustc_metadata[ae525c484fd7d6a9]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[ae525c484fd7d6a9]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[ae525c484fd7d6a9]::rmeta::encoder::EncodedMetadata, ()>
  25:     0x7f507caa1abe - rustc_metadata[ae525c484fd7d6a9]::rmeta::encoder::encode_metadata
  26:     0x7f507acf2c16 - <rustc_interface[199c53ef3796a23]::passes::QueryContext>::enter::<<rustc_interface[199c53ef3796a23]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[e440ea95e42423cc]::result::Result<alloc[c893676b29ec05e1]::boxed::Box<dyn core[e440ea95e42423cc]::any::Any>, rustc_errors[4c730e405383d456]::ErrorGuaranteed>>
  27:     0x7f507acde5de - <rustc_interface[199c53ef3796a23]::queries::Queries>::ongoing_codegen
  28:     0x7f507ab86e10 - <rustc_interface[199c53ef3796a23]::interface::Compiler>::enter::<rustc_driver[691bc896c45f7d96]::run_compiler::{closure#1}::{closure#2}, core[e440ea95e42423cc]::result::Result<core[e440ea95e42423cc]::option::Option<rustc_interface[199c53ef3796a23]::queries::Linker>, rustc_errors[4c730e405383d456]::ErrorGuaranteed>>
  29:     0x7f507abe49db - rustc_span[ac0bc6d32e291b06]::with_source_map::<core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>, rustc_interface[199c53ef3796a23]::interface::create_compiler_and_run<core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>, rustc_driver[691bc896c45f7d96]::run_compiler::{closure#1}>::{closure#1}>
  30:     0x7f507ab87f49 - <scoped_tls[70624b53fb37079b]::ScopedKey<rustc_span[ac0bc6d32e291b06]::SessionGlobals>>::set::<rustc_interface[199c53ef3796a23]::interface::run_compiler<core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>, rustc_driver[691bc896c45f7d96]::run_compiler::{closure#1}>::{closure#0}, core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>>
  31:     0x7f507abe19c9 - std[7e4e60dccf76a23c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[199c53ef3796a23]::util::run_in_thread_pool_with_globals<rustc_interface[199c53ef3796a23]::interface::run_compiler<core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>, rustc_driver[691bc896c45f7d96]::run_compiler::{closure#1}>::{closure#0}, core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>>::{closure#0}, core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>>
  32:     0x7f507ab89441 - std[7e4e60dccf76a23c]::panicking::try::<core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>, core[e440ea95e42423cc]::panic::unwind_safe::AssertUnwindSafe<<std[7e4e60dccf76a23c]::thread::Builder>::spawn_unchecked_<rustc_interface[199c53ef3796a23]::util::run_in_thread_pool_with_globals<rustc_interface[199c53ef3796a23]::interface::run_compiler<core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>, rustc_driver[691bc896c45f7d96]::run_compiler::{closure#1}>::{closure#0}, core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>>::{closure#0}, core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7f507abdcc82 - <<std[7e4e60dccf76a23c]::thread::Builder>::spawn_unchecked_<rustc_interface[199c53ef3796a23]::util::run_in_thread_pool_with_globals<rustc_interface[199c53ef3796a23]::interface::run_compiler<core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>, rustc_driver[691bc896c45f7d96]::run_compiler::{closure#1}>::{closure#0}, core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>>::{closure#0}, core[e440ea95e42423cc]::result::Result<(), rustc_errors[4c730e405383d456]::ErrorGuaranteed>>::{closure#1} as core[e440ea95e42423cc]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f507a0a7593 - std::sys::unix::thread::Thread::new::thread_start::h966c6c29bb14b8b3
  35:     0x7f50745fa609 - start_thread
  36:     0x7f5079f0d163 - clone
  37:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (f3391d93a 2022-04-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=4
query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `<impl at /checkout/src/test/ui/mir/ssa-analysis-regression-50041.rs:11:1: 13:2>::drop`
#1 [optimized_mir] optimizing MIR for `<impl at /checkout/src/test/ui/mir/ssa-analysis-regression-50041.rs:11:1: 13:2>::drop`
------------------------------------------



