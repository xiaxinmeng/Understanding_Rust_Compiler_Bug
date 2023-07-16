plain
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling hashbrown v0.12.0
   Compiling object v0.26.2
   Compiling addr2line v0.16.0
error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:774:54: collection encountered polymorphic constant: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:9705 ~ std[7e43]::sys_common::thread_info::{impl#0}::with), const_param_did: None }, substs: [core::option::Option<core::ops::Range<usize>>, [closure@library/std/src/sys_common/thread_info.rs:38:22: 38:53]], promoted: Some(promoted[0]) }, &thread::local::LocalKey<core::cell::RefCell<core::option::Option<sys_common::thread_info::ThreadInfo>>>)
   |
20 | /         THREAD_INFO
20 | /         THREAD_INFO
21 | |             .try_with(move |thread_info| {
22 | |                 let mut thread_info = thread_info.borrow_mut();
23 | |                 let thread_info = thread_info.get_or_insert_with(|| ThreadInfo {
27 | |                 f(thread_info)
28 | |             })
   | |______________^


thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
   0:     0x7f06a57e7e12 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7f06a584fa98 - core::fmt::write::ha01458c252ca8e28
   2:     0x7f06a57d80e1 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7f06a57eb129 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7f06a57eadca - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7f06a62e9aa4 - rustc_driver[48e10e3c4f250537]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f06a57eb98f - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7f06a678d6c3 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[7c07a63214684621]::ExplicitBug>::{closure#0}
   8:     0x7f06a678bee6 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[7c07a63214684621]::ExplicitBug>::{closure#0}, !>
   9:     0x7f06a5f8fb96 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[7c07a63214684621]::ExplicitBug>
  10:     0x7f06a6765546 - std[836a811975e52724]::panic::panic_any::<rustc_errors[7c07a63214684621]::ExplicitBug>
  11:     0x7f06a6764f23 - <rustc_errors[7c07a63214684621]::HandlerInner>::span_bug::<rustc_span[d6a89a0bfce8bb5]::span_encoding::Span, &alloc[f55ce12b9f25f528]::string::String>
  12:     0x7f06a6764d60 - <rustc_errors[7c07a63214684621]::Handler>::span_bug::<rustc_span[d6a89a0bfce8bb5]::span_encoding::Span, &alloc[f55ce12b9f25f528]::string::String>
  13:     0x7f06a676ad22 - rustc_middle[c9f99ab0490ddc12]::ty::context::tls::with_opt::<rustc_middle[c9f99ab0490ddc12]::util::bug::opt_span_bug_fmt<rustc_span[d6a89a0bfce8bb5]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f06a676ab49 - rustc_middle[c9f99ab0490ddc12]::util::bug::opt_span_bug_fmt::<rustc_span[d6a89a0bfce8bb5]::span_encoding::Span>
  15:     0x7f06a5f8fe27 - rustc_middle[c9f99ab0490ddc12]::util::bug::span_bug_fmt::<rustc_span[d6a89a0bfce8bb5]::span_encoding::Span>
  16:     0x7f06a67555ee - <rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::MirNeighborCollector as rustc_middle[c9f99ab0490ddc12]::mir::visit::Visitor>::visit_constant
  17:     0x7f06a674d5df - <rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::MirNeighborCollector as rustc_middle[c9f99ab0490ddc12]::mir::visit::Visitor>::visit_operand
  18:     0x7f06a674c755 - <rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::MirNeighborCollector as rustc_middle[c9f99ab0490ddc12]::mir::visit::Visitor>::visit_rvalue
  19:     0x7f06a6756f5b - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_neighbours
  20:     0x7f06a6753514 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  21:     0x7f06a6753a46 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  22:     0x7f06a6753a46 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  23:     0x7f06a6753a46 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  24:     0x7f06a6753a46 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  25:     0x7f06a6753a46 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  26:     0x7f06a6753a46 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  27:     0x7f06a6753a46 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  28:     0x7f06a6753a46 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  29:     0x7f06a6753a46 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  30:     0x7f06a6753a46 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_items_rec
  31:     0x7f06a676a152 - <rustc_session[6823c539bda58cf9]::session::Session>::time::<(), rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_crate_mono_items::{closure#1}>
  32:     0x7f06a674fa81 - rustc_monomorphize[cbc2f6cf3f6eefe8]::collector::collect_crate_mono_items
  33:     0x7f06a676378d - rustc_monomorphize[cbc2f6cf3f6eefe8]::partitioning::collect_and_partition_mono_items
  34:     0x7f06a7d915f5 - rustc_query_system[8cd1849d215b6e18]::query::plumbing::try_execute_query::<rustc_query_impl[be7419aac166b395]::plumbing::QueryCtxt, rustc_query_system[8cd1849d215b6e18]::query::caches::DefaultCache<(), (&std[836a811975e52724]::collections::hash::set::HashSet<rustc_span[d6a89a0bfce8bb5]::def_id::DefId, core[6d9550a4e960c99f]::hash::BuildHasherDefault<rustc_hash[e8a2bbde91638579]::FxHasher>>, &[rustc_middle[c9f99ab0490ddc12]::mir::mono::CodegenUnit])>>
  35:     0x7f06a7e5110a - rustc_query_system[8cd1849d215b6e18]::query::plumbing::get_query::<rustc_query_impl[be7419aac166b395]::queries::collect_and_partition_mono_items, rustc_query_impl[be7419aac166b395]::plumbing::QueryCtxt>
  36:     0x7f06a79f6909 - <rustc_query_impl[be7419aac166b395]::Queries as rustc_middle[c9f99ab0490ddc12]::ty::query::QueryEngine>::collect_and_partition_mono_items
  37:     0x7f06a65669f3 - rustc_codegen_ssa[aca223f6fa342053]::base::codegen_crate::<rustc_codegen_llvm[5193eac684f0f511]::LlvmCodegenBackend>
  38:     0x7f06a65ad4fc - <rustc_codegen_llvm[5193eac684f0f511]::LlvmCodegenBackend as rustc_codegen_ssa[aca223f6fa342053]::traits::backend::CodegenBackend>::codegen_crate
  39:     0x7f06a642ffa8 - <rustc_session[6823c539bda58cf9]::session::Session>::time::<alloc[f55ce12b9f25f528]::boxed::Box<dyn core[6d9550a4e960c99f]::any::Any>, rustc_interface[2ec13e7f8be7b8f4]::passes::start_codegen::{closure#0}>
  40:     0x7f06a641ad3c - <rustc_interface[2ec13e7f8be7b8f4]::passes::QueryContext>::enter::<<rustc_interface[2ec13e7f8be7b8f4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6d9550a4e960c99f]::result::Result<alloc[f55ce12b9f25f528]::boxed::Box<dyn core[6d9550a4e960c99f]::any::Any>, rustc_errors[7c07a63214684621]::ErrorGuaranteed>>
  41:     0x7f06a6405bce - <rustc_interface[2ec13e7f8be7b8f4]::queries::Queries>::ongoing_codegen
  42:     0x7f06a63081ff - <rustc_interface[2ec13e7f8be7b8f4]::interface::Compiler>::enter::<rustc_driver[48e10e3c4f250537]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[2ec13e7f8be7b8f4]::queries::Linker>, rustc_errors[7c07a63214684621]::ErrorGuaranteed>>
  43:     0x7f06a62eaff6 - rustc_span[d6a89a0bfce8bb5]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>, rustc_interface[2ec13e7f8be7b8f4]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>, rustc_driver[48e10e3c4f250537]::run_compiler::{closure#1}>::{closure#1}>
  44:     0x7f06a630941e - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[d6a89a0bfce8bb5]::SessionGlobals>>::set::<rustc_interface[2ec13e7f8be7b8f4]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>, rustc_driver[48e10e3c4f250537]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>>
  45:     0x7f06a63608a9 - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2ec13e7f8be7b8f4]::util::run_in_thread_pool_with_globals<rustc_interface[2ec13e7f8be7b8f4]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>, rustc_driver[48e10e3c4f250537]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>>
  46:     0x7f06a63586f9 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[2ec13e7f8be7b8f4]::util::run_in_thread_pool_with_globals<rustc_interface[2ec13e7f8be7b8f4]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>, rustc_driver[48e10e3c4f250537]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7f06a57f8353 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  48:     0x7f069fd48609 - start_thread
  49:     0x7f06a565b133 - clone
  50:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (e1f0917d0 2022-07-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: could not compile `std`
Build completed unsuccessfully in 0:03:57
