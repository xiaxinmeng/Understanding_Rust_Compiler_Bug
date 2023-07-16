plain
---- [ui] src/test/ui/consts/precise-drop-with-coverage.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/precise-drop-with-coverage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/precise-drop-with-coverage" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-Cinstrument-coverage" "-Zno-profiler-runtime" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/precise-drop-with-coverage/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_borrowck/src/lib.rs:625:17: Statement not allowed in this MIR phase
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
stack backtrace:
   0:     0x7efd9b6528cc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hdb472832a43477b0
   1:     0x7efd9b6b6c98 - core::fmt::write::h1e03b60556337bee
   2:     0x7efd9b6426d1 - std::io::Write::write_fmt::haab7f98cc9af486e
   3:     0x7efd9b6558be - std::panicking::default_hook::{{closure}}::ha798470602ac908a
   4:     0x7efd9b6555a9 - std::panicking::default_hook::h1cac706f16c643b6
   5:     0x7efd9c184fd4 - rustc_driver[ae9ba73cb05d7f41]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7efd9b656022 - std::panicking::rust_panic_with_hook::h1db18c1ae6d40ff1
   7:     0x7efd9ebe02f3 - std[6bcdd4a558171ccc]::panicking::begin_panic::<rustc_errors[4fb6f9283f45d7ac]::ExplicitBug>::{closure#0}
   8:     0x7efd9ebdf586 - std[6bcdd4a558171ccc]::sys_common::backtrace::__rust_end_short_backtrace::<std[6bcdd4a558171ccc]::panicking::begin_panic<rustc_errors[4fb6f9283f45d7ac]::ExplicitBug>::{closure#0}, !>
   9:     0x7efd9c105306 - std[6bcdd4a558171ccc]::panicking::begin_panic::<rustc_errors[4fb6f9283f45d7ac]::ExplicitBug>
  10:     0x7efd9e9d3556 - std[6bcdd4a558171ccc]::panic::panic_any::<rustc_errors[4fb6f9283f45d7ac]::ExplicitBug>
  11:     0x7efd9e9c9276 - <rustc_errors[4fb6f9283f45d7ac]::HandlerInner>::bug::<&alloc[ac6d57bd3f374455]::string::String>
  12:     0x7efd9e9c8f30 - <rustc_errors[4fb6f9283f45d7ac]::Handler>::bug::<&alloc[ac6d57bd3f374455]::string::String>
  13:     0x7efd9ebc6285 - rustc_middle[11f0dd5f0eec9b27]::util::bug::opt_span_bug_fmt::<rustc_span[9eeca7f23f5ae06f]::span_encoding::Span>::{closure#0}
  14:     0x7efd9ebc472b - rustc_middle[11f0dd5f0eec9b27]::ty::context::tls::with_opt::<rustc_middle[11f0dd5f0eec9b27]::util::bug::opt_span_bug_fmt<rustc_span[9eeca7f23f5ae06f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7efd9ebc46dc - rustc_middle[11f0dd5f0eec9b27]::ty::context::tls::with_opt::<rustc_middle[11f0dd5f0eec9b27]::util::bug::opt_span_bug_fmt<rustc_span[9eeca7f23f5ae06f]::span_encoding::Span>::{closure#0}, ()>
  16:     0x7efd9ebc61c9 - rustc_middle[11f0dd5f0eec9b27]::util::bug::opt_span_bug_fmt::<rustc_span[9eeca7f23f5ae06f]::span_encoding::Span>
  17:     0x7efd9c107405 - rustc_middle[11f0dd5f0eec9b27]::util::bug::bug_fmt
  18:     0x7efd9d39db47 - <rustc_borrowck[3a193089bcfd93a0]::MirBorrowckCtxt as rustc_mir_dataflow[931bb1f7511a123c]::framework::visitor::ResultsVisitor>::visit_statement_before_primary_effect
  19:     0x7efd9d24520d - <rustc_mir_dataflow[931bb1f7511a123c]::framework::direction::Forward as rustc_mir_dataflow[931bb1f7511a123c]::framework::direction::Direction>::visit_results_in_block::<rustc_borrowck[3a193089bcfd93a0]::dataflow::BorrowckAnalyses<rustc_index[427363a202a2bbd9]::bit_set::BitSet<rustc_borrowck[3a193089bcfd93a0]::dataflow::BorrowIndex>, rustc_index[427363a202a2bbd9]::bit_set::ChunkedBitSet<rustc_mir_dataflow[931bb1f7511a123c]::move_paths::MovePathIndex>, rustc_index[427363a202a2bbd9]::bit_set::ChunkedBitSet<rustc_mir_dataflow[931bb1f7511a123c]::move_paths::InitIndex>>, rustc_borrowck[3a193089bcfd93a0]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[931bb1f7511a123c]::framework::engine::Results<rustc_borrowck[3a193089bcfd93a0]::dataflow::Borrows>, rustc_mir_dataflow[931bb1f7511a123c]::framework::engine::Results<rustc_mir_dataflow[931bb1f7511a123c]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[931bb1f7511a123c]::framework::engine::Results<rustc_mir_dataflow[931bb1f7511a123c]::impls::EverInitializedPlaces>>, rustc_borrowck[3a193089bcfd93a0]::MirBorrowckCtxt>
  20:     0x7efd9d3aeddc - rustc_mir_dataflow[931bb1f7511a123c]::framework::visitor::visit_results::<rustc_borrowck[3a193089bcfd93a0]::dataflow::BorrowckAnalyses<rustc_index[427363a202a2bbd9]::bit_set::BitSet<rustc_borrowck[3a193089bcfd93a0]::dataflow::BorrowIndex>, rustc_index[427363a202a2bbd9]::bit_set::ChunkedBitSet<rustc_mir_dataflow[931bb1f7511a123c]::move_paths::MovePathIndex>, rustc_index[427363a202a2bbd9]::bit_set::ChunkedBitSet<rustc_mir_dataflow[931bb1f7511a123c]::move_paths::InitIndex>>, rustc_borrowck[3a193089bcfd93a0]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[931bb1f7511a123c]::framework::engine::Results<rustc_borrowck[3a193089bcfd93a0]::dataflow::Borrows>, rustc_mir_dataflow[931bb1f7511a123c]::framework::engine::Results<rustc_mir_dataflow[931bb1f7511a123c]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[931bb1f7511a123c]::framework::engine::Results<rustc_mir_dataflow[931bb1f7511a123c]::impls::EverInitializedPlaces>>, core[112536468d13b768]::iter::adapters::map::Map<rustc_middle[11f0dd5f0eec9b27]::mir::traversal::ReversePostorderIter, rustc_borrowck[3a193089bcfd93a0]::do_mir_borrowck::{closure#2}>, rustc_borrowck[3a193089bcfd93a0]::MirBorrowckCtxt>
  21:     0x7efd9d3a5909 - rustc_borrowck[3a193089bcfd93a0]::do_mir_borrowck
  22:     0x7efd9d28ee5e - <rustc_infer[553f2b457d9642b0]::infer::InferCtxtBuilder>::enter::<rustc_middle[11f0dd5f0eec9b27]::mir::query::BorrowCheckResult, rustc_borrowck[3a193089bcfd93a0]::mir_borrowck::{closure#0}>
  23:     0x7efd9d39c327 - rustc_borrowck[3a193089bcfd93a0]::mir_borrowck
  24:     0x7efd9d365b46 - <rustc_borrowck[3a193089bcfd93a0]::provide::{closure#0} as core[112536468d13b768]::ops::function::FnOnce<(rustc_middle[11f0dd5f0eec9b27]::ty::context::TyCtxt, rustc_span[9eeca7f23f5ae06f]::def_id::LocalDefId)>>::call_once
  25:     0x7efd9dc3f96c - rustc_query_system[3ca6d130689393e8]::query::plumbing::try_execute_query::<rustc_query_impl[817e4fc1674e3143]::plumbing::QueryCtxt, rustc_query_system[3ca6d130689393e8]::query::caches::DefaultCache<rustc_span[9eeca7f23f5ae06f]::def_id::LocalDefId, &rustc_middle[11f0dd5f0eec9b27]::mir::query::BorrowCheckResult>>
  26:     0x7efd9dd03238 - rustc_query_system[3ca6d130689393e8]::query::plumbing::get_query::<rustc_query_impl[817e4fc1674e3143]::queries::mir_borrowck, rustc_query_impl[817e4fc1674e3143]::plumbing::QueryCtxt>
  27:     0x7efd9d89fe14 - <rustc_query_impl[817e4fc1674e3143]::Queries as rustc_middle[11f0dd5f0eec9b27]::ty::query::QueryEngine>::mir_borrowck
  28:     0x7efd9c33c94a - <rustc_middle[11f0dd5f0eec9b27]::hir::map::Map>::par_body_owners::<rustc_interface[4b9466f196f67b3b]::passes::analysis::{closure#2}::{closure#0}>
  29:     0x7efd9c2c6302 - <rustc_session[2f35fd86cd1a4f2c]::session::Session>::time::<(), rustc_interface[4b9466f196f67b3b]::passes::analysis::{closure#2}>
  30:     0x7efd9c2b490b - rustc_interface[4b9466f196f67b3b]::passes::analysis
  31:     0x7efd9dc7a6e4 - rustc_query_system[3ca6d130689393e8]::query::plumbing::try_execute_query::<rustc_query_impl[817e4fc1674e3143]::plumbing::QueryCtxt, rustc_query_system[3ca6d130689393e8]::query::caches::DefaultCache<(), core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>>>
  32:     0x7efd9dd576b2 - rustc_query_system[3ca6d130689393e8]::query::plumbing::get_query::<rustc_query_impl[817e4fc1674e3143]::queries::analysis, rustc_query_impl[817e4fc1674e3143]::plumbing::QueryCtxt>
  33:     0x7efd9d88179e - <rustc_query_impl[817e4fc1674e3143]::Queries as rustc_middle[11f0dd5f0eec9b27]::ty::query::QueryEngine>::analysis
  34:     0x7efd9c1ec194 - <rustc_interface[4b9466f196f67b3b]::passes::QueryContext>::enter::<rustc_driver[ae9ba73cb05d7f41]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>>
  35:     0x7efd9c1a2af5 - <rustc_interface[4b9466f196f67b3b]::interface::Compiler>::enter::<rustc_driver[ae9ba73cb05d7f41]::run_compiler::{closure#1}::{closure#2}, core[112536468d13b768]::result::Result<core[112536468d13b768]::option::Option<rustc_interface[4b9466f196f67b3b]::queries::Linker>, rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>>
  36:     0x7efd9c1fb3fb - rustc_span[9eeca7f23f5ae06f]::with_source_map::<core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>, rustc_interface[4b9466f196f67b3b]::interface::create_compiler_and_run<core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>, rustc_driver[ae9ba73cb05d7f41]::run_compiler::{closure#1}>::{closure#1}>
  37:     0x7efd9c1a3c91 - <scoped_tls[91d6e69e848962de]::ScopedKey<rustc_span[9eeca7f23f5ae06f]::SessionGlobals>>::set::<rustc_interface[4b9466f196f67b3b]::interface::run_compiler<core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>, rustc_driver[ae9ba73cb05d7f41]::run_compiler::{closure#1}>::{closure#0}, core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>>
  38:     0x7efd9c1f8319 - std[6bcdd4a558171ccc]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4b9466f196f67b3b]::util::run_in_thread_pool_with_globals<rustc_interface[4b9466f196f67b3b]::interface::run_compiler<core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>, rustc_driver[ae9ba73cb05d7f41]::run_compiler::{closure#1}>::{closure#0}, core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>>::{closure#0}, core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>>
  39:     0x7efd9c1b7f51 - std[6bcdd4a558171ccc]::panicking::try::<core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>, core[112536468d13b768]::panic::unwind_safe::AssertUnwindSafe<<std[6bcdd4a558171ccc]::thread::Builder>::spawn_unchecked_<rustc_interface[4b9466f196f67b3b]::util::run_in_thread_pool_with_globals<rustc_interface[4b9466f196f67b3b]::interface::run_compiler<core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>, rustc_driver[ae9ba73cb05d7f41]::run_compiler::{closure#1}>::{closure#0}, core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>>::{closure#0}, core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7efd9c1ef5f2 - <<std[6bcdd4a558171ccc]::thread::Builder>::spawn_unchecked_<rustc_interface[4b9466f196f67b3b]::util::run_in_thread_pool_with_globals<rustc_interface[4b9466f196f67b3b]::interface::run_compiler<core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>, rustc_driver[ae9ba73cb05d7f41]::run_compiler::{closure#1}>::{closure#0}, core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>>::{closure#0}, core[112536468d13b768]::result::Result<(), rustc_errors[4fb6f9283f45d7ac]::ErrorGuaranteed>>::{closure#1} as core[112536468d13b768]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7efd9b661043 - std::sys::unix::thread::Thread::new::thread_start::hea54a37e035df44c
  42:     0x7efd95bb0609 - start_thread
  43:     0x7efd9b4c3133 - clone
  44:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (ac47edf74 2022-06-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib -C instrument-coverage -Z no-profiler-runtime
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `transpose`
#1 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


