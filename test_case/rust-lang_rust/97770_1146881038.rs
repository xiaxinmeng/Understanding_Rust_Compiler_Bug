plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
touch /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/invalid-library/invalid-library/lib.rmeta
ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/invalid-library/invalid-library/libfoo-ffffffff-1.0.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/invalid-library/invalid-library/lib.rmeta
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/invalid-library/invalid-library:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/invalid-library/invalid-library -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/invalid-library/invalid-library  foo.rs 2>&1 | "/checkout/src/etc/cat-and-grep.sh" "found invalid metadata"
[[[ begin stdout ]]]
thread 'rustc' panicked at 'assertion failed: slice_ptr < base_end_ptr', /checkout/compiler/rustc_data_structures/src/owned_slice.rs:83:9
stack backtrace:
   0:     0x7f625a3bd8ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4e27c81c96c0d15d
   1:     0x7f625a4246e8 - core::fmt::write::h868aeaa9fef3f186
   2:     0x7f625a3ad711 - std::io::Write::write_fmt::hdac65482bb67cd2c
   3:     0x7f625a3c08ae - std::panicking::default_hook::{{closure}}::h1ee05ab12f4f0b32
   4:     0x7f625a3c0599 - std::panicking::default_hook::h5838e8a1dc33800f
   5:     0x7f625aed8a31 - rustc_driver[e1b2f228a85077b4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f625a3c1012 - std::panicking::rust_panic_with_hook::h68cfbe64a1d210fe
   7:     0x7f625a3c0e09 - std::panicking::begin_panic_handler::{{closure}}::h58920ebe1551a2cd
   8:     0x7f625a3bddc4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf3536da450daeb7b
   9:     0x7f625a3c0b29 - rust_begin_unwind
  10:     0x7f625a375013 - core::panicking::panic_fmt::h194e7e3ed25faa00
  11:     0x7f625a374edd - core::panicking::panic::h71351e52bd0305b7
  12:     0x7f625cbc9a7c - <rustc_data_structures[ac205f1e1902ffe3]::owned_slice::OwnedSlice<alloc[2cf378c07dada30b]::boxed::Box<dyn core[b1d560ef9a788be6]::ops::deref::Deref<Target = [u8]>>, u8>>::try_map::<<rustc_codegen_ssa[22113314f3216882]::back::metadata::DefaultMetadataLoader as rustc_session[cdd9dfbd94faf50a]::cstore::MetadataLoader>::get_rlib_metadata::{closure#0}, alloc[2cf378c07dada30b]::string::String>
  13:     0x7f625cc91243 - <rustc_codegen_ssa[22113314f3216882]::back::metadata::DefaultMetadataLoader as rustc_session[cdd9dfbd94faf50a]::cstore::MetadataLoader>::get_rlib_metadata
  14:     0x7f625cf7df45 - rustc_metadata[10eef8a79b5649a1]::locator::get_metadata_section
  15:     0x7f625cf7b5c8 - <rustc_metadata[10eef8a79b5649a1]::locator::CrateLocator>::extract_one
  16:     0x7f625cf7ac33 - <rustc_metadata[10eef8a79b5649a1]::locator::CrateLocator>::extract_lib
  17:     0x7f625cf7a198 - <rustc_metadata[10eef8a79b5649a1]::locator::CrateLocator>::find_library_crate
  18:     0x7f625cf78746 - <rustc_metadata[10eef8a79b5649a1]::locator::CrateLocator>::maybe_load_library_crate
  19:     0x7f625cfdcf7a - <rustc_metadata[10eef8a79b5649a1]::creader::CrateLoader>::load
  20:     0x7f625cfd8b92 - <rustc_metadata[10eef8a79b5649a1]::creader::CrateLoader>::maybe_resolve_crate
  21:     0x7f625cfd7cd2 - <rustc_metadata[10eef8a79b5649a1]::creader::CrateLoader>::resolve_crate
  22:     0x7f625cfdf681 - <rustc_metadata[10eef8a79b5649a1]::creader::CrateLoader>::process_extern_crate
  23:     0x7f625bbb28ae - <rustc_resolve[100e92d6aa913809]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[7dd9ee820844db4]::visit::Visitor>::visit_item
  24:     0x7f625bbe67ed - rustc_ast[7dd9ee820844db4]::visit::walk_crate::<rustc_resolve[100e92d6aa913809]::build_reduced_graph::BuildReducedGraphVisitor>
  25:     0x7f625bbb5141 - <rustc_resolve[100e92d6aa913809]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[7dd9ee820844db4]::visit::Visitor>::visit_crate
  26:     0x7f625bc1d389 - <rustc_resolve[100e92d6aa913809]::Resolver as rustc_expand[b77185d66c9d60d2]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  27:     0x7f625d128cc4 - <rustc_expand[b77185d66c9d60d2]::expand::MacroExpander>::collect_invocations
  28:     0x7f625d124109 - <rustc_expand[b77185d66c9d60d2]::expand::MacroExpander>::fully_expand_fragment
  29:     0x7f625d123d62 - <rustc_expand[b77185d66c9d60d2]::expand::MacroExpander>::expand_crate
  30:     0x7f625affedbc - <rustc_session[cdd9dfbd94faf50a]::session::Session>::time::<core[b1d560ef9a788be6]::result::Result<rustc_ast[7dd9ee820844db4]::ast::Crate, rustc_errors[63528033c844095f]::ErrorGuaranteed>, rustc_interface[ab3bf0f9bc89f837]::passes::configure_and_expand::{closure#1}>
  31:     0x7f625aff22e3 - rustc_interface[ab3bf0f9bc89f837]::passes::configure_and_expand
  32:     0x7f625b025d14 - <rustc_interface[ab3bf0f9bc89f837]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[ab3bf0f9bc89f837]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[b1d560ef9a788be6]::result::Result<rustc_ast[7dd9ee820844db4]::ast::Crate, rustc_errors[63528033c844095f]::ErrorGuaranteed>>
  33:     0x7f625b098bf5 - <rustc_interface[ab3bf0f9bc89f837]::queries::Queries>::expansion
  34:     0x7f625aef6330 - <rustc_interface[ab3bf0f9bc89f837]::interface::Compiler>::enter::<rustc_driver[e1b2f228a85077b4]::run_compiler::{closure#1}::{closure#2}, core[b1d560ef9a788be6]::result::Result<core[b1d560ef9a788be6]::option::Option<rustc_interface[ab3bf0f9bc89f837]::queries::Linker>, rustc_errors[63528033c844095f]::ErrorGuaranteed>>
  35:     0x7f625aeda2fb - rustc_span[e54008d3e749a0d7]::with_source_map::<core[b1d560ef9a788be6]::result::Result<(), rustc_errors[63528033c844095f]::ErrorGuaranteed>, rustc_interface[ab3bf0f9bc89f837]::interface::create_compiler_and_run<core[b1d560ef9a788be6]::result::Result<(), rustc_errors[63528033c844095f]::ErrorGuaranteed>, rustc_driver[e1b2f228a85077b4]::run_compiler::{closure#1}>::{closure#1}>
  36:     0x7f625aef7604 - <scoped_tls[b30a3cf53b5292e3]::ScopedKey<rustc_span[e54008d3e749a0d7]::SessionGlobals>>::set::<rustc_interface[ab3bf0f9bc89f837]::interface::run_compiler<core[b1d560ef9a788be6]::result::Result<(), rustc_errors[63528033c844095f]::ErrorGuaranteed>, rustc_driver[e1b2f228a85077b4]::run_compiler::{closure#1}>::{closure#0}, core[b1d560ef9a788be6]::result::Result<(), rustc_errors[63528033c844095f]::ErrorGuaranteed>>
  37:     0x7f625af555c9 - std[a4e4726f53690811]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab3bf0f9bc89f837]::util::run_in_thread_pool_with_globals<rustc_interface[ab3bf0f9bc89f837]::interface::run_compiler<core[b1d560ef9a788be6]::result::Result<(), rustc_errors[63528033c844095f]::ErrorGuaranteed>, rustc_driver[e1b2f228a85077b4]::run_compiler::{closure#1}>::{closure#0}, core[b1d560ef9a788be6]::result::Result<(), rustc_errors[63528033c844095f]::ErrorGuaranteed>>::{closure#0}, core[b1d560ef9a788be6]::result::Result<(), rustc_errors[63528033c844095f]::ErrorGuaranteed>>
  38:     0x7f625af4dab9 - <<std[a4e4726f53690811]::thread::Builder>::spawn_unchecked_<rustc_interface[ab3bf0f9bc89f837]::util::run_in_thread_pool_with_globals<rustc_interface[ab3bf0f9bc89f837]::interface::run_compiler<core[b1d560ef9a788be6]::result::Result<(), rustc_errors[63528033c844095f]::ErrorGuaranteed>, rustc_driver[e1b2f228a85077b4]::run_compiler::{closure#1}>::{closure#0}, core[b1d560ef9a788be6]::result::Result<(), rustc_errors[63528033c844095f]::ErrorGuaranteed>>::{closure#0}, core[b1d560ef9a788be6]::result::Result<(), rustc_errors[63528033c844095f]::ErrorGuaranteed>>::{closure#1} as core[b1d560ef9a788be6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f625a3cc3a3 - std::sys::unix::thread::Thread::new::thread_start::h52ea01a285ccf4aa
  40:     0x7f625491f609 - start_thread
  41:     0x7f625a232133 - clone
  42:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (e9e700aa6 2022-06-05) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack

[[[ end stdout ]]]
Error: cannot match: found invalid metadata
--- stderr -------------------------------
ar: `u' modifier ignored since `D' is the default (see `U')
ar: `u' modifier ignored since `D' is the default (see `U')
make: *** [Makefile:6: all] Error 1



failures:
