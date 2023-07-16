
thread 'rustc' panicked at 'index out of bounds: the len is 2 but the index is 2', compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs:450:53
stack backtrace:
   0:        0x1012819c8 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h61a7d3e4c5d884f5
   1:        0x1012ca980 - core::fmt::write::h15b38023853d48c8
   2:        0x1012738a0 - std::io::Write::write_fmt::h8d01a6f9467a53cf
   3:        0x1012847cc - std::panicking::default_hook::{{closure}}::h202969861cea2a40
   4:        0x1012843ac - std::panicking::default_hook::h6075a10f4c58af64
   5:        0x10849ba3c - rustc_driver[35b3b9d305ec1a86]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:        0x101284fa0 - std::panicking::rust_panic_with_hook::h0d2ad4d26e65e971
   7:        0x101284ac0 - std::panicking::begin_panic_handler::{{closure}}::h81487f6587d82c75
   8:        0x101281e94 - std::sys_common::backtrace::__rust_end_short_backtrace::hc543c9309bc89e91
   9:        0x101284a28 - _rust_begin_unwind
  10:        0x1012fa170 - core::panicking::panic_fmt::h61e5a76b978edc22
  11:        0x1012fa144 - core::panicking::panic_bounds_check::hbcdb5a8d91c8ce55
  12:        0x10ad8e170 - <rustc_borrowck[6143a6a723f63a46]::MirBorrowckCtxt>::report_mutability_error
  13:        0x10ad95914 - <rustc_borrowck[6143a6a723f63a46]::MirBorrowckCtxt>::access_place
  14:        0x10ad94244 - <rustc_borrowck[6143a6a723f63a46]::MirBorrowckCtxt as rustc_mir_dataflow[2505fbb23d1ca289]::framework::visitor::ResultsVisitor>::visit_statement_before_primary_effect
  15:        0x10ad2044c - <rustc_mir_dataflow[2505fbb23d1ca289]::framework::direction::Forward as rustc_mir_dataflow[2505fbb23d1ca289]::framework::direction::Direction>::visit_results_in_block::<rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowckAnalyses<rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowIndex>, rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_mir_dataflow[2505fbb23d1ca289]::move_paths::MovePathIndex>, rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_mir_dataflow[2505fbb23d1ca289]::move_paths::InitIndex>>, rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_borrowck[6143a6a723f63a46]::dataflow::Borrows>, rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_mir_dataflow[2505fbb23d1ca289]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_mir_dataflow[2505fbb23d1ca289]::impls::EverInitializedPlaces>>, rustc_borrowck[6143a6a723f63a46]::MirBorrowckCtxt>
  16:        0x10acae320 - rustc_mir_dataflow[2505fbb23d1ca289]::framework::visitor::visit_results::<rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowckAnalyses<rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowIndex>, rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_mir_dataflow[2505fbb23d1ca289]::move_paths::MovePathIndex>, rustc_index[64c13988d092f625]::bit_set::BitSet<rustc_mir_dataflow[2505fbb23d1ca289]::move_paths::InitIndex>>, rustc_borrowck[6143a6a723f63a46]::dataflow::BorrowckAnalyses<rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_borrowck[6143a6a723f63a46]::dataflow::Borrows>, rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_mir_dataflow[2505fbb23d1ca289]::impls::MaybeUninitializedPlaces>, rustc_mir_dataflow[2505fbb23d1ca289]::framework::engine::Results<rustc_mir_dataflow[2505fbb23d1ca289]::impls::EverInitializedPlaces>>, core[489ab2277f19021d]::iter::adapters::map::Map<rustc_middle[29c0f47ba1981f7e]::mir::traversal::ReversePostorder, rustc_borrowck[6143a6a723f63a46]::do_mir_borrowck::{closure#2}>, rustc_borrowck[6143a6a723f63a46]::MirBorrowckCtxt>
  17:        0x10ad9ab04 - rustc_borrowck[6143a6a723f63a46]::do_mir_borrowck
  18:        0x10ad0cc44 - <rustc_infer[233d2e4639a00b1a]::infer::InferCtxtBuilder>::enter::<rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult, rustc_borrowck[6143a6a723f63a46]::mir_borrowck::{closure#0}>
  19:        0x10ad939c0 - rustc_borrowck[6143a6a723f63a46]::mir_borrowck
  20:        0x10ad75d14 - <rustc_borrowck[6143a6a723f63a46]::provide::{closure#0} as core[489ab2277f19021d]::ops::function::FnOnce<(rustc_middle[29c0f47ba1981f7e]::ty::context::TyCtxt, rustc_span[6eae83f828617b72]::def_id::LocalDefId)>>::call_once
  21:        0x10b118a10 - rustc_query_system[2ed05f00384aea15]::query::plumbing::try_execute_query::<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_query_system[2ed05f00384aea15]::query::caches::DefaultCache<rustc_span[6eae83f828617b72]::def_id::LocalDefId, &rustc_middle[29c0f47ba1981f7e]::mir::query::BorrowCheckResult>>
  22:        0x10b1a40c4 - rustc_query_system[2ed05f00384aea15]::query::plumbing::get_query::<rustc_query_impl[1e8f6af660f906f5]::queries::mir_borrowck, rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt>
  23:        0x10863f7e0 - <rustc_session[5f6c487c9c714a47]::session::Session>::time::<(), rustc_interface[3e6b3c7def9e9d82]::passes::analysis::{closure#2}>
  24:        0x1085acbcc - rustc_interface[3e6b3c7def9e9d82]::passes::analysis
  25:        0x10b148548 - rustc_query_system[2ed05f00384aea15]::query::plumbing::try_execute_query::<rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt, rustc_query_system[2ed05f00384aea15]::query::caches::DefaultCache<(), core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>>
  26:        0x10b1dac04 - rustc_query_system[2ed05f00384aea15]::query::plumbing::get_query::<rustc_query_impl[1e8f6af660f906f5]::queries::analysis, rustc_query_impl[1e8f6af660f906f5]::plumbing::QueryCtxt>
  27:        0x1084fad6c - <rustc_interface[3e6b3c7def9e9d82]::passes::QueryContext>::enter::<rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>
  28:        0x1084df010 - <rustc_interface[3e6b3c7def9e9d82]::interface::Compiler>::enter::<rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}::{closure#2}, core[489ab2277f19021d]::result::Result<core[489ab2277f19021d]::option::Option<rustc_interface[3e6b3c7def9e9d82]::queries::Linker>, rustc_errors[1a053f9350564121]::ErrorReported>>
  29:        0x1084a4608 - rustc_span[6eae83f828617b72]::with_source_map::<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_interface[3e6b3c7def9e9d82]::interface::create_compiler_and_run<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}>::{closure#1}>
  30:        0x1084df898 - rustc_interface[3e6b3c7def9e9d82]::interface::create_compiler_and_run::<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}>
  31:        0x1084affc0 - <scoped_tls[34b11748aea080ce]::ScopedKey<rustc_span[6eae83f828617b72]::SessionGlobals>>::set::<rustc_interface[3e6b3c7def9e9d82]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[3e6b3c7def9e9d82]::interface::run_compiler<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}>::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>::{closure#0}::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>
  32:        0x1084ae02c - std[edbc36b44a871839]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3e6b3c7def9e9d82]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[3e6b3c7def9e9d82]::interface::run_compiler<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}>::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>
  33:        0x108500048 - <<std[edbc36b44a871839]::thread::Builder>::spawn_unchecked<rustc_interface[3e6b3c7def9e9d82]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[3e6b3c7def9e9d82]::interface::run_compiler<core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>, rustc_driver[35b3b9d305ec1a86]::run_compiler::{closure#1}>::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>::{closure#0}, core[489ab2277f19021d]::result::Result<(), rustc_errors[1a053f9350564121]::ErrorReported>>::{closure#1} as core[489ab2277f19021d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:        0x10128e318 - std::sys::unix::thread::Thread::new::thread_start::h02ceb9b16148ae76
  35:        0x1bf9754ec - _pthread_from_mach_thread_np

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (936f2600b 2021-11-22) running on aarch64-apple-darwin

query stack during panic:
#0 [mir_borrowck] borrow-checking `solve`
#1 [analysis] running analysis passes on this crate
end of query stack
