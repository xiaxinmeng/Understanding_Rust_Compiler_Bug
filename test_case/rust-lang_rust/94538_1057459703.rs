plain
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
thread 'rustc' panicked at 'assertion failed: count <= chunk_domain_size', /checkout/compiler/rustc_index/src/bit_set.rs:754:9
   0:     0x7f9d129aa5ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h641e0a65e2615a08
   1:     0x7f9d12a174ae - core::fmt::write::h39191aa5431a5380
   1:     0x7f9d12a174ae - core::fmt::write::h39191aa5431a5380
   2:     0x7f9d129997a1 - std::io::Write::write_fmt::hb1861dc9906df921
   3:     0x7f9d129aa3db - std::sys_common::backtrace::print::h0ae42f20033c9262
   4:     0x7f9d129aebf4 - std::panicking::default_hook::{{closure}}::hd2976bf86056b49a
   5:     0x7f9d129ae7d6 - std::panicking::default_hook::hd02e8d479b982aaa
   6:     0x7f9d133ffc31 - rustc_driver[1687b6f54ec85a2d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f9d129af303 - std::panicking::rust_panic_with_hook::hbbba1d0bbca4f6bf
   8:     0x7f9d129af0d9 - std::panicking::begin_panic_handler::{{closure}}::h2d97bb73b3b478f0
   9:     0x7f9d129aaad4 - std::sys_common::backtrace::__rust_end_short_backtrace::h61f77ae4323c082a
  10:     0x7f9d129aede9 - rust_begin_unwind
  11:     0x7f9d12966a73 - core::panicking::panic_fmt::h510f640c0e57f953
  12:     0x7f9d1296693d - core::panicking::panic::h5aad8a7117313b35
  13:     0x7f9d143a38cf - <rustc_mir_dataflow[2201aea7f3b0edd]::framework::GenKillSet<rustc_mir_dataflow[2201aea7f3b0edd]::move_paths::MovePathIndex>>::apply::<rustc_index[90e18c305638fcb4]::bit_set::ChunkedBitSet<rustc_mir_dataflow[2201aea7f3b0edd]::move_paths::MovePathIndex>>
  14:     0x7f9d14334ba9 - <rustc_mir_dataflow[2201aea7f3b0edd]::framework::engine::Engine<rustc_mir_dataflow[2201aea7f3b0edd]::impls::MaybeUninitializedPlaces>>::iterate_to_fixpoint
  15:     0x7f9d144b54f3 - rustc_borrowck[85fe006c0f8dd3a2]::do_mir_borrowck
  16:     0x7f9d143e8bd1 - <rustc_infer[39a903d0f366d8c7]::infer::InferCtxtBuilder>::enter::<rustc_middle[31ee61178343d15]::mir::query::BorrowCheckResult, rustc_borrowck[85fe006c0f8dd3a2]::mir_borrowck::{closure#0}>
  17:     0x7f9d144ac0c3 - rustc_borrowck[85fe006c0f8dd3a2]::mir_borrowck
  18:     0x7f9d1447978f - <rustc_borrowck[85fe006c0f8dd3a2]::provide::{closure#0} as core[8382fcd636289ab6]::ops::function::FnOnce<(rustc_middle[31ee61178343d15]::ty::context::TyCtxt, rustc_span[7a273c255f7631f3]::def_id::LocalDefId)>>::call_once
  19:     0x7f9d149181b2 - rustc_query_system[7de27dcf8c6c63f]::query::plumbing::try_execute_query::<rustc_query_impl[48e294c0af17776d]::plumbing::QueryCtxt, rustc_query_system[7de27dcf8c6c63f]::query::caches::DefaultCache<rustc_span[7a273c255f7631f3]::def_id::LocalDefId, &rustc_middle[31ee61178343d15]::mir::query::BorrowCheckResult>>
  20:     0x7f9d149dd750 - rustc_query_system[7de27dcf8c6c63f]::query::plumbing::get_query::<rustc_query_impl[48e294c0af17776d]::queries::mir_borrowck, rustc_query_impl[48e294c0af17776d]::plumbing::QueryCtxt>
  21:     0x7f9d1356a61e - <rustc_middle[31ee61178343d15]::hir::map::Map>::par_body_owners::<rustc_interface[e9a32af1e3f110c9]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f9d1359b670 - <rustc_session[ff90c9fddf4ef87c]::session::Session>::time::<(), rustc_interface[e9a32af1e3f110c9]::passes::analysis::{closure#2}>
  23:     0x7f9d13632c8b - rustc_interface[e9a32af1e3f110c9]::passes::analysis
  24:     0x7f9d14951fcb - rustc_query_system[7de27dcf8c6c63f]::query::plumbing::try_execute_query::<rustc_query_impl[48e294c0af17776d]::plumbing::QueryCtxt, rustc_query_system[7de27dcf8c6c63f]::query::caches::DefaultCache<(), core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>>>
  25:     0x7f9d14a46d09 - rustc_query_system[7de27dcf8c6c63f]::query::plumbing::get_query::<rustc_query_impl[48e294c0af17776d]::queries::analysis, rustc_query_impl[48e294c0af17776d]::plumbing::QueryCtxt>
  26:     0x7f9d133f3fd5 - <rustc_interface[e9a32af1e3f110c9]::passes::QueryContext>::enter::<rustc_driver[1687b6f54ec85a2d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>>
  27:     0x7f9d1346b330 - rustc_interface[e9a32af1e3f110c9]::interface::create_compiler_and_run::<core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>, rustc_driver[1687b6f54ec85a2d]::run_compiler::{closure#1}>
  28:     0x7f9d13425a39 - std[9f788053d5631f34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e9a32af1e3f110c9]::util::run_in_thread_pool_with_globals<rustc_interface[e9a32af1e3f110c9]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>, rustc_driver[1687b6f54ec85a2d]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>>
  29:     0x7f9d13478abe - std[9f788053d5631f34]::panic::catch_unwind::<core[8382fcd636289ab6]::panic::unwind_safe::AssertUnwindSafe<<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[e9a32af1e3f110c9]::util::run_in_thread_pool_with_globals<rustc_interface[e9a32af1e3f110c9]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>, rustc_driver[1687b6f54ec85a2d]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>>
  30:     0x7f9d1341b8f0 - <<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[e9a32af1e3f110c9]::util::run_in_thread_pool_with_globals<rustc_interface[e9a32af1e3f110c9]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>, rustc_driver[1687b6f54ec85a2d]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorGuaranteed>>::{closure#1} as core[8382fcd636289ab6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7f9d129be293 - std::sys::unix::thread::Thread::new::thread_start::hd363d8910f104f91
  32:     0x7f9d0cd30609 - start_thread
  33:     0x7f9d12827293 - clone
  34:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (a0299ac7b 2022-03-02) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `interpret::intrinsics::<impl at compiler/rustc_const_eval/src/interpret/intrinsics.rs:108:1: 608:2>::emulate_intrinsic`
#1 [analysis] running analysis passes on this crate
error: could not compile `rustc_const_eval`
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:08:20
