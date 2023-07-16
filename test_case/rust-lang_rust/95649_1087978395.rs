plain
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `MutatingUse(Store)`,
 right: `NonUse(VarDebugInfo)`', compiler/rustc_mir_transform/src/nrvo.rs:204:13
   0:     0x7f79a4d909bc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h641e0a65e2615a08
   1:     0x7f79a4df9f18 - core::fmt::write::h39191aa5431a5380
   1:     0x7f79a4df9f18 - core::fmt::write::h39191aa5431a5380
   2:     0x7f79a4d810a1 - std::io::Write::write_fmt::hb1861dc9906df921
   3:     0x7f79a4d907eb - std::sys_common::backtrace::print::h0ae42f20033c9262
   4:     0x7f79a4d95164 - std::panicking::default_hook::{{closure}}::hd2976bf86056b49a
   5:     0x7f79a4d94d6f - std::panicking::default_hook::hd02e8d479b982aaa
   6:     0x7f79a589ca01 - rustc_driver[c267567e364ec01c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f79a4d95876 - std::panicking::rust_panic_with_hook::hbbba1d0bbca4f6bf
   8:     0x7f79a4d95687 - std::panicking::begin_panic_handler::{{closure}}::h2d97bb73b3b478f0
   9:     0x7f79a4d90ee4 - std::sys_common::backtrace::__rust_end_short_backtrace::h61f77ae4323c082a
  10:     0x7f79a4d95359 - rust_begin_unwind
  11:     0x7f79a4d4fc03 - core::panicking::panic_fmt::h510f640c0e57f953
  12:     0x7f79a4df69c8 - core::panicking::assert_failed_inner::hd0148e7e424bf90b
  13:     0x7f79a54fcadb - core[8382fcd636289ab6]::panicking::assert_failed::<rustc_middle[632ea405399cdf0e]::mir::visit::PlaceContext, rustc_middle[632ea405399cdf0e]::mir::visit::PlaceContext>
  14:     0x7f79a5eb7b91 - <rustc_mir_transform[b2c67ab7ba24a6fd]::nrvo::RenameToReturnPlace as rustc_middle[632ea405399cdf0e]::mir::visit::MutVisitor>::visit_place
  15:     0x7f79a5ea715b - <rustc_mir_transform[b2c67ab7ba24a6fd]::nrvo::RenameToReturnPlace as rustc_middle[632ea405399cdf0e]::mir::visit::MutVisitor>::visit_statement
  16:     0x7f79a5ea6199 - <rustc_mir_transform[b2c67ab7ba24a6fd]::nrvo::RenameReturnPlace as rustc_middle[632ea405399cdf0e]::mir::MirPass>::run_pass
  17:     0x7f79a5f151b0 - rustc_mir_transform[b2c67ab7ba24a6fd]::pass_manager::run_passes
  18:     0x7f79a5eb0bd9 - rustc_mir_transform[b2c67ab7ba24a6fd]::optimized_mir
  19:     0x7f79a6ee00e4 - rustc_query_system[9b838339c46f0366]::query::plumbing::try_execute_query::<rustc_query_impl[ca3b9df97e0714cd]::plumbing::QueryCtxt, rustc_query_system[9b838339c46f0366]::query::caches::DefaultCache<rustc_span[9284ef26d05fbe21]::def_id::DefId, &rustc_middle[632ea405399cdf0e]::mir::Body>>
  20:     0x7f79a6f86df4 - rustc_query_system[9b838339c46f0366]::query::plumbing::get_query::<rustc_query_impl[ca3b9df97e0714cd]::queries::optimized_mir, rustc_query_impl[ca3b9df97e0714cd]::plumbing::QueryCtxt>
  21:     0x7f79a768153d - <rustc_metadata[c307d9d3388ad280]::rmeta::encoder::EncodeContext>::encode_crate_root
  22:     0x7f79a769722a - rustc_metadata[c307d9d3388ad280]::rmeta::encoder::encode_metadata_impl
  23:     0x7f79a777e641 - rustc_data_structures[7abf5dc379f05b32]::sync::join::<rustc_metadata[c307d9d3388ad280]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[c307d9d3388ad280]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[c307d9d3388ad280]::rmeta::encoder::EncodedMetadata, ()>
  24:     0x7f79a769686e - rustc_metadata[c307d9d3388ad280]::rmeta::encoder::encode_metadata
  25:     0x7f79a599ba49 - <rustc_interface[143c9ff6b1d7c0db]::passes::QueryContext>::enter::<<rustc_interface[143c9ff6b1d7c0db]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[8382fcd636289ab6]::result::Result<alloc[75caa6313e26ebc1]::boxed::Box<dyn core[8382fcd636289ab6]::any::Any>, rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>>
  26:     0x7f79a5985abe - <rustc_interface[143c9ff6b1d7c0db]::queries::Queries>::ongoing_codegen
  27:     0x7f79a58472e7 - <rustc_interface[143c9ff6b1d7c0db]::interface::Compiler>::enter::<rustc_driver[c267567e364ec01c]::run_compiler::{closure#1}::{closure#2}, core[8382fcd636289ab6]::result::Result<core[8382fcd636289ab6]::option::Option<rustc_interface[143c9ff6b1d7c0db]::queries::Linker>, rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>>
  28:     0x7f79a5851079 - rustc_span[9284ef26d05fbe21]::with_source_map::<core[8382fcd636289ab6]::result::Result<(), rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>, rustc_interface[143c9ff6b1d7c0db]::interface::create_compiler_and_run<core[8382fcd636289ab6]::result::Result<(), rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>, rustc_driver[c267567e364ec01c]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7f79a5842b84 - <scoped_tls[6532d0c6e5128321]::ScopedKey<rustc_span[9284ef26d05fbe21]::SessionGlobals>>::set::<rustc_interface[143c9ff6b1d7c0db]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>, rustc_driver[c267567e364ec01c]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>>
  30:     0x7f79a5833d69 - std[9f788053d5631f34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[143c9ff6b1d7c0db]::util::run_in_thread_pool_with_globals<rustc_interface[143c9ff6b1d7c0db]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>, rustc_driver[c267567e364ec01c]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>>
  31:     0x7f79a584eb99 - <<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[143c9ff6b1d7c0db]::util::run_in_thread_pool_with_globals<rustc_interface[143c9ff6b1d7c0db]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>, rustc_driver[c267567e364ec01c]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[ef8891f7d3cbee9a]::ErrorGuaranteed>>::{closure#1} as core[8382fcd636289ab6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f79a4da3e03 - std::sys::unix::thread::Thread::new::thread_start::hd363d8910f104f91
  33:     0x7f799f2fa609 - start_thread
  34:     0x7f79a4c0d163 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (02b60dedd 2022-04-04) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `dep_graph::<impl at compiler/rustc_middle/src/dep_graph/mod.rs:70:1: 139:2>::sess`
error: could not compile `rustc_middle`
Build completed unsuccessfully in 0:05:47
