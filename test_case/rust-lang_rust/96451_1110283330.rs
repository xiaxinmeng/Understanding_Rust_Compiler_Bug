plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: self.reachable_blocks.contains(block)', /checkout/compiler/rustc_mir_dataflow/src/framework/cursor.rs:98:9
   0:     0x7f6131c19c32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   1:     0x7f6131c81638 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f6131c0a171 - std::io::Write::write_fmt::hf3faa85fa7d28190
   2:     0x7f6131c0a171 - std::io::Write::write_fmt::hf3faa85fa7d28190
   3:     0x7f6131c1cf76 - std::panicking::default_hook::{{closure}}::h243e0a014f6b15da
   4:     0x7f6131c1cb6d - std::panicking::default_hook::hdf681f01978f1e20
   5:     0x7f6132736ea1 - rustc_driver[65272f936875c6c1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6131c1d910 - std::panicking::rust_panic_with_hook::h1c127668bc0f49d8
   7:     0x7f6131c1d6e9 - std::panicking::begin_panic_handler::{{closure}}::hdc297c549f81c3b7
   8:     0x7f6131c1a1d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h7b90b067d1e7c19a
   9:     0x7f6131c1d419 - rust_begin_unwind
  10:     0x7f6131bd10b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7f6131bd0f7d - core::panicking::panic::h6966bf306ecf1686
  12:     0x7f6132cabe0f - <rustc_mir_dataflow[5b60fb24e6d11f99]::framework::cursor::ResultsCursor<rustc_mir_dataflow[5b60fb24e6d11f99]::impls::liveness::MaybeLiveLocals>>::seek_to_block_entry
  13:     0x7f6132cabb67 - <rustc_mir_dataflow[5b60fb24e6d11f99]::framework::cursor::ResultsCursor<rustc_mir_dataflow[5b60fb24e6d11f99]::impls::liveness::MaybeLiveLocals>>::seek_after
  14:     0x7f6132dad727 - <rustc_mir_transform[6e5dfdd996c45afc]::dest_prop::DSEVisitor as rustc_middle[e995fb8dc5766eea]::mir::visit::Visitor>::visit_statement
  15:     0x7f6132da9dc4 - <rustc_mir_transform[6e5dfdd996c45afc]::dest_prop::DestinationPropagation as rustc_middle[e995fb8dc5766eea]::mir::MirPass>::run_pass
  16:     0x7f6132cb39c2 - rustc_mir_transform[6e5dfdd996c45afc]::pass_manager::run_passes
  17:     0x7f6132d8b3cb - rustc_mir_transform[6e5dfdd996c45afc]::optimized_mir
  18:     0x7f6133dcc79d - rustc_query_system[989e46076e3dfb40]::query::plumbing::try_execute_query::<rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt, rustc_query_system[989e46076e3dfb40]::query::caches::DefaultCache<rustc_span[7ff2d2023f32a35]::def_id::DefId, &rustc_middle[e995fb8dc5766eea]::mir::Body>>
  19:     0x7f6133e6dd84 - rustc_query_system[989e46076e3dfb40]::query::plumbing::get_query::<rustc_query_impl[757f9843cb5f89aa]::queries::optimized_mir, rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt>
  20:     0x7f61345534ed - <rustc_metadata[acaace744f453e4]::rmeta::encoder::EncodeContext>::encode_crate_root
  21:     0x7f61345682e7 - rustc_metadata[acaace744f453e4]::rmeta::encoder::encode_metadata_impl
  22:     0x7f6134627fd1 - rustc_data_structures[3ce3321229418a76]::sync::join::<rustc_metadata[acaace744f453e4]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[acaace744f453e4]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[acaace744f453e4]::rmeta::encoder::EncodedMetadata, ()>
  23:     0x7f61345678fe - rustc_metadata[acaace744f453e4]::rmeta::encoder::encode_metadata
  24:     0x7f61328427ed - <rustc_interface[ef49b6c417f36e1c]::passes::QueryContext>::enter::<<rustc_interface[ef49b6c417f36e1c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[10878fb91fc84a80]::result::Result<alloc[24f448636cd10183]::boxed::Box<dyn core[10878fb91fc84a80]::any::Any>, rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  25:     0x7f613282597e - <rustc_interface[ef49b6c417f36e1c]::queries::Queries>::ongoing_codegen
  26:     0x7f61326cbd18 - <rustc_interface[ef49b6c417f36e1c]::interface::Compiler>::enter::<rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[ef49b6c417f36e1c]::queries::Linker>, rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  27:     0x7f61326ad726 - rustc_span[7ff2d2023f32a35]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_interface[ef49b6c417f36e1c]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#1}>
  28:     0x7f61326df1f7 - rustc_interface[ef49b6c417f36e1c]::interface::create_compiler_and_run::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>
  29:     0x7f61326e253f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[7ff2d2023f32a35]::SessionGlobals>>::set::<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  30:     0x7f613272a799 - std[38ff3720b7fd637]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ef49b6c417f36e1c]::util::run_in_thread_pool_with_globals<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  31:     0x7f61326e4881 - std[38ff3720b7fd637]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[ef49b6c417f36e1c]::util::run_in_thread_pool_with_globals<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  32:     0x7f61327257a2 - <<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[ef49b6c417f36e1c]::util::run_in_thread_pool_with_globals<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7f6131c2a313 - std::sys::unix::thread::Thread::new::thread_start::h38902d511e7013ce
  34:     0x7f612c17a609 - start_thread
  35:     0x7f6131a8d163 - clone
  36:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (ae48b4758 2022-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `char::convert::from_u32_unchecked`
