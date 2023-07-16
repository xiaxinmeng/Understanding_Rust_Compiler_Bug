plain
ii.i..i.....i......i.ii..i.......i.....iii.....F.....................i..i.............i............. 100/102
..
failures:

---- src/builtin.rs - builtin::MUTABLE_BORROW_RESERVATION_CONFLICT (line 2321) stdout ----
warning: cannot borrow `v` as mutable because it is also borrowed as immutable
  |
  |
4 | let shared = &v;
  |              -- immutable borrow occurs here
5 | v.push(shared.len());
  | |      |
  | |      immutable borrow later used here
  | |      immutable borrow later used here
  | mutable borrow occurs here
  |
  = note: `#[warn(mutable_borrow_reservation_conflict)]` on by default
  = warning: this borrowing pattern was not meant to be accepted, and may become a hard error in the future


thread 'rustc' panicked at 'FIXME(compiler-errors): probably should propagate this?: ErrorReported', compiler/rustc_mir_transform/src/lib.rs:535:10
stack backtrace:
   0:     0x7fe9aa73f2dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc18baf0e5ad62c0e
   1:     0x7fe9aa7ad0de - core::fmt::write::h04f3cb9c5bd3369c
   2:     0x7fe9aa72d1e3 - std::io::Write::write_fmt::h73ff4fde244178ae
   3:     0x7fe9aa73f0fb - std::sys_common::backtrace::print::hc84114397f416d7d
   4:     0x7fe9aa743c51 - std::panicking::default_hook::{{closure}}::hf05675e5828388ac
   5:     0x7fe9aa743816 - std::panicking::default_hook::h05e4c1dbbc633a4e
   6:     0x7fe9ab209961 - rustc_driver[64873b0f470b5bb]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe9aa744381 - std::panicking::rust_panic_with_hook::h96c2153e7808e938
   8:     0x7fe9aa744187 - std::panicking::begin_panic_handler::{{closure}}::hc36987e05b06ca89
   9:     0x7fe9aa73f7f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h30c4ddf35a5c2752
  10:     0x7fe9aa743e59 - rust_begin_unwind
  11:     0x7fe9aa6f8a13 - core::panicking::panic_fmt::h7802ba5043cb2ca5
  12:     0x7fe9aa6f8bc3 - core::result::unwrap_failed::ha8627e166920f885
  13:     0x7fe9ab78ff5d - rustc_mir_transform[a2486eec5704b37f]::optimized_mir
  14:     0x7fe9ac7646fe - rustc_query_system[a30f4d8fc136e373]::query::plumbing::try_execute_query::<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt, rustc_query_system[a30f4d8fc136e373]::query::caches::DefaultCache<rustc_span[519afcfa7470336f]::def_id::DefId, &rustc_middle[8415c423d7cd478b]::mir::Body>>
  15:     0x7fe9ac80d95d - rustc_query_system[a30f4d8fc136e373]::query::plumbing::get_query::<rustc_query_impl[1fa5d22394a75585]::queries::optimized_mir, rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt>
  16:     0x7fe9ad88338a - <rustc_middle[8415c423d7cd478b]::ty::context::TyCtxt>::instance_mir
  17:     0x7fe9ab6083d1 - rustc_monomorphize[903837aa04916361]::collector::collect_neighbours
  18:     0x7fe9ab60d0f6 - rustc_data_structures[1eb754226c267db1]::stack::ensure_sufficient_stack::<(), rustc_monomorphize[903837aa04916361]::collector::collect_items_rec::{closure#0}>
  19:     0x7fe9ab5fee97 - rustc_monomorphize[903837aa04916361]::collector::collect_items_rec
  20:     0x7fe9ab5ff14a - rustc_monomorphize[903837aa04916361]::collector::collect_items_rec
  21:     0x7fe9ab61a232 - <rustc_session[d1341c97f23ce4da]::session::Session>::time::<(), rustc_monomorphize[903837aa04916361]::collector::collect_crate_mono_items::{closure#1}>
  22:     0x7fe9ab5fd79b - rustc_monomorphize[903837aa04916361]::collector::collect_crate_mono_items
  23:     0x7fe9ab638043 - rustc_monomorphize[903837aa04916361]::partitioning::collect_and_partition_mono_items
  24:     0x7fe9ac97597b - rustc_data_structures[1eb754226c267db1]::stack::ensure_sufficient_stack::<(&std[eec5ef45012f6570]::collections::hash::set::HashSet<rustc_span[519afcfa7470336f]::def_id::DefId, core[ba5cb6be30a93795]::hash::BuildHasherDefault<rustc_hash[6f7973c56968f131]::FxHasher>>, &[rustc_middle[8415c423d7cd478b]::mir::mono::CodegenUnit]), rustc_query_system[a30f4d8fc136e373]::query::plumbing::execute_job<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt, (), (&std[eec5ef45012f6570]::collections::hash::set::HashSet<rustc_span[519afcfa7470336f]::def_id::DefId, core[ba5cb6be30a93795]::hash::BuildHasherDefault<rustc_hash[6f7973c56968f131]::FxHasher>>, &[rustc_middle[8415c423d7cd478b]::mir::mono::CodegenUnit])>::{closure#0}>
  25:     0x7fe9ac788d10 - rustc_query_system[a30f4d8fc136e373]::query::plumbing::try_execute_query::<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt, rustc_query_system[a30f4d8fc136e373]::query::caches::DefaultCache<(), (&std[eec5ef45012f6570]::collections::hash::set::HashSet<rustc_span[519afcfa7470336f]::def_id::DefId, core[ba5cb6be30a93795]::hash::BuildHasherDefault<rustc_hash[6f7973c56968f131]::FxHasher>>, &[rustc_middle[8415c423d7cd478b]::mir::mono::CodegenUnit])>>
  26:     0x7fe9ac861c43 - rustc_query_system[a30f4d8fc136e373]::query::plumbing::get_query::<rustc_query_impl[1fa5d22394a75585]::queries::collect_and_partition_mono_items, rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt>
  27:     0x7fe9ac9bb13e - <rustc_query_impl[1fa5d22394a75585]::Queries as rustc_middle[8415c423d7cd478b]::ty::query::QueryEngine>::collect_and_partition_mono_items
  28:     0x7fe9ab44c0a1 - rustc_codegen_ssa[aede23dcece2e3a5]::base::codegen_crate::<rustc_codegen_llvm[c13e9b7dc5abc0a]::LlvmCodegenBackend>
  29:     0x7fe9ab4f538a - <rustc_codegen_llvm[c13e9b7dc5abc0a]::LlvmCodegenBackend as rustc_codegen_ssa[aede23dcece2e3a5]::traits::backend::CodegenBackend>::codegen_crate
  30:     0x7fe9ab358f38 - <rustc_session[d1341c97f23ce4da]::session::Session>::time::<alloc[2897730bb73bbd5c]::boxed::Box<dyn core[ba5cb6be30a93795]::any::Any>, rustc_interface[c90d6bd38ca68993]::passes::start_codegen::{closure#0}>
  31:     0x7fe9ab3f5893 - <rustc_interface[c90d6bd38ca68993]::passes::QueryContext>::enter::<<rustc_interface[c90d6bd38ca68993]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<alloc[2897730bb73bbd5c]::boxed::Box<dyn core[ba5cb6be30a93795]::any::Any>, rustc_errors[5cad097c964025d7]::ErrorReported>>
  32:     0x7fe9ab3a5cce - <rustc_interface[c90d6bd38ca68993]::queries::Queries>::ongoing_codegen
  33:     0x7fe9ab2386e5 - <rustc_interface[c90d6bd38ca68993]::interface::Compiler>::enter::<rustc_driver[64873b0f470b5bb]::run_compiler::{closure#1}::{closure#2}, core[ba5cb6be30a93795]::result::Result<core[ba5cb6be30a93795]::option::Option<rustc_interface[c90d6bd38ca68993]::queries::Linker>, rustc_errors[5cad097c964025d7]::ErrorReported>>
  34:     0x7fe9ab1f2679 - rustc_span[519afcfa7470336f]::with_source_map::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_interface[c90d6bd38ca68993]::interface::create_compiler_and_run<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[64873b0f470b5bb]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7fe9ab236df9 - rustc_interface[c90d6bd38ca68993]::interface::create_compiler_and_run::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[64873b0f470b5bb]::run_compiler::{closure#1}>
  36:     0x7fe9ab274cae - <scoped_tls[9ef8146b5f47b671]::ScopedKey<rustc_span[519afcfa7470336f]::SessionGlobals>>::set::<rustc_interface[c90d6bd38ca68993]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c90d6bd38ca68993]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[64873b0f470b5bb]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>
  37:     0x7fe9ab23e5a5 - std[eec5ef45012f6570]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c90d6bd38ca68993]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c90d6bd38ca68993]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[64873b0f470b5bb]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>
  38:     0x7fe9ab277b61 - std[eec5ef45012f6570]::panicking::try::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, core[ba5cb6be30a93795]::panic::unwind_safe::AssertUnwindSafe<<std[eec5ef45012f6570]::thread::Builder>::spawn_unchecked_<rustc_interface[c90d6bd38ca68993]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c90d6bd38ca68993]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[64873b0f470b5bb]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#1}::{closure#0}>>
  39:     0x7fe9ab234852 - <<std[eec5ef45012f6570]::thread::Builder>::spawn_unchecked_<rustc_interface[c90d6bd38ca68993]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c90d6bd38ca68993]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[64873b0f470b5bb]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#1} as core[ba5cb6be30a93795]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7fe9aa7525d3 - std::sys::unix::thread::Thread::new::thread_start::he83677990b8dc5fd
  41:     0x7fe9a4ac3609 - start_thread
  42:     0x7fe9aa5ba293 - clone
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (0249c759a 2022-02-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C codegen-units=1 -C embed-bitcode=no -C symbol-mangling-version=v0 -Z force-unstable-if-unmarked
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::_doctest_main_src_builtin_rs_2321_0`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
warning: 1 warning emitted

Couldn't compile the test.

