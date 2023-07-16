plain
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
thread 'rustc' panicked at '(*(_1.0: &mut [generator@compiler/rustc_metadata/src/rmeta/encoder.rs:1136:73: 1155:14]))', compiler/rustc_const_eval/src/transform/validate.rs:314:13
stack backtrace:
   0:     0x7f596fbd0da2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7f596fc386c8 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f596fbc10f1 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7f596fbd4106 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7f596fbd3cfd - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7f59707288e1 - rustc_driver[1a777b00ee158e36]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f596fbd4aa0 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7f596fbd48b7 - std::panicking::begin_panic_handler::{{closure}}::h79b0ac1d2b9c8b15
   8:     0x7f596fbd1344 - std::sys_common::backtrace::__rust_end_short_backtrace::h72d3f8515fc7966b
   9:     0x7f596fbd45a9 - rust_begin_unwind
  10:     0x7f596fb880b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7f5971c942cb - <rustc_const_eval[463eeed51a022010]::transform::validate::TypeChecker as rustc_middle[8d4dc3708b593ac1]::mir::visit::Visitor>::visit_place
  12:     0x7f5971c9528d - <rustc_const_eval[463eeed51a022010]::transform::validate::TypeChecker as rustc_middle[8d4dc3708b593ac1]::mir::visit::Visitor>::visit_rvalue
  13:     0x7f5971c9581a - <rustc_const_eval[463eeed51a022010]::transform::validate::TypeChecker as rustc_middle[8d4dc3708b593ac1]::mir::visit::Visitor>::visit_statement
  14:     0x7f5971c92c2f - <rustc_const_eval[463eeed51a022010]::transform::validate::Validator as rustc_middle[8d4dc3708b593ac1]::mir::MirPass>::run_pass
  15:     0x7f5970cc1867 - rustc_mir_transform[8cd1ea75711a0041]::pass_manager::run_passes
  16:     0x7f5970dace7b - rustc_mir_transform[8cd1ea75711a0041]::optimized_mir
  17:     0x7f5971e2eead - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, &rustc_middle[8d4dc3708b593ac1]::mir::Body>>
  18:     0x7f5971ecbe42 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::optimized_mir, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  19:     0x7f5972270ad9 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::optimized_mir
  20:     0x7f59726139fd - <rustc_metadata[6387b7782e9ee88c]::rmeta::encoder::EncodeContext>::encode_crate_root
  21:     0x7f597262874b - rustc_metadata[6387b7782e9ee88c]::rmeta::encoder::encode_metadata_impl
  22:     0x7f59726ed831 - rustc_data_structures[db21f27220b58c22]::sync::join::<rustc_metadata[6387b7782e9ee88c]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[6387b7782e9ee88c]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[6387b7782e9ee88c]::rmeta::encoder::EncodedMetadata, ()>
  23:     0x7f5972627d4e - rustc_metadata[6387b7782e9ee88c]::rmeta::encoder::encode_metadata
  24:     0x7f5970838c7c - <rustc_interface[b11680d0c9f2606a]::passes::QueryContext>::enter::<<rustc_interface[b11680d0c9f2606a]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[10878fb91fc84a80]::result::Result<alloc[24f448636cd10183]::boxed::Box<dyn core[10878fb91fc84a80]::any::Any>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  25:     0x7f597081bbfe - <rustc_interface[b11680d0c9f2606a]::queries::Queries>::ongoing_codegen
  26:     0x7f59706be378 - <rustc_interface[b11680d0c9f2606a]::interface::Compiler>::enter::<rustc_driver[1a777b00ee158e36]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[b11680d0c9f2606a]::queries::Linker>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  27:     0x7f59706a0306 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[b11680d0c9f2606a]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[1a777b00ee158e36]::run_compiler::{closure#1}>::{closure#1}>
  28:     0x7f59706bf60f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[b11680d0c9f2606a]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[1a777b00ee158e36]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  29:     0x7f59707142a9 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b11680d0c9f2606a]::util::run_in_thread_pool_with_globals<rustc_interface[b11680d0c9f2606a]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[1a777b00ee158e36]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  30:     0x7f59706d3051 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[b11680d0c9f2606a]::util::run_in_thread_pool_with_globals<rustc_interface[b11680d0c9f2606a]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[1a777b00ee158e36]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  31:     0x7f5970716522 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[b11680d0c9f2606a]::util::run_in_thread_pool_with_globals<rustc_interface[b11680d0c9f2606a]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[1a777b00ee158e36]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f596fbe1403 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
  33:     0x7f596a131609 - start_thread
  34:     0x7f596fa44163 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (8290b4cb6 2022-05-13) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `rmeta::encoder::<impl at compiler/rustc_metadata/src/rmeta/encoder.rs:987:1: 1964:2>::encode_info_for_mod::{closure#0}`
error: could not compile `rustc_metadata`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:08:15
