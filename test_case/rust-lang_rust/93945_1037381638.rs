plain
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
thread 'rustc' panicked at 'assertion failed: `(left != right)`
  left: `0`,
 right: `0`: Tried to get crate root for reference-only metadata', compiler/rustc_metadata/src/rmeta/decoder.rs:671:9
stack backtrace:
   0:     0x7f77e5d0472c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc18baf0e5ad62c0e
   1:     0x7f77e5d72e9e - core::fmt::write::h04f3cb9c5bd3369c
   2:     0x7f77e5cf36e1 - std::io::Write::write_fmt::h73ff4fde244178ae
   3:     0x7f77e5d0455b - std::sys_common::backtrace::print::hc84114397f416d7d
   4:     0x7f77e5d08d24 - std::panicking::default_hook::{{closure}}::hf05675e5828388ac
   5:     0x7f77e5d08906 - std::panicking::default_hook::h05e4c1dbbc633a4e
   6:     0x7f77e67dc801 - rustc_driver[b4ef4919aa58b87e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f77e5d09443 - std::panicking::rust_panic_with_hook::h96c2153e7808e938
   8:     0x7f77e5d09257 - std::panicking::begin_panic_handler::{{closure}}::hc36987e05b06ca89
   9:     0x7f77e5d04c44 - std::sys_common::backtrace::__rust_end_short_backtrace::h30c4ddf35a5c2752
  10:     0x7f77e5d08f19 - rust_begin_unwind
  11:     0x7f77e5cbf9d3 - core::panicking::panic_fmt::h7802ba5043cb2ca5
  12:     0x7f77e5d6f90e - core::panicking::assert_failed_inner::h6dca9f9f6afd9af4
  13:     0x7f77e6671bfb - core[ba5cb6be30a93795]::panicking::assert_failed::<usize, usize>
  14:     0x7f77e85940a4 - <rustc_metadata[2dbd8235e3c3c141]::rmeta::decoder::MetadataBlob>::get_root
  15:     0x7f77e8578545 - <rustc_metadata[2dbd8235e3c3c141]::locator::CrateLocator>::extract_one
  16:     0x7f77e8577a92 - <rustc_metadata[2dbd8235e3c3c141]::locator::CrateLocator>::extract_lib
  17:     0x7f77e8575711 - <rustc_metadata[2dbd8235e3c3c141]::locator::CrateLocator>::maybe_load_library_crate
  18:     0x7f77e851d64a - <rustc_metadata[2dbd8235e3c3c141]::creader::CrateLoader>::load
  19:     0x7f77e8519b6a - <rustc_metadata[2dbd8235e3c3c141]::creader::CrateLoader>::maybe_resolve_crate
  20:     0x7f77e8518ce2 - <rustc_metadata[2dbd8235e3c3c141]::creader::CrateLoader>::resolve_crate
  21:     0x7f77e852012c - <rustc_metadata[2dbd8235e3c3c141]::creader::CrateLoader>::process_extern_crate
  22:     0x7f77e72d32f3 - <rustc_resolve[ba041298a12665e0]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[ee10e5f9e26a6d7]::visit::Visitor>::visit_item
  23:     0x7f77e7314b3d - rustc_ast[ee10e5f9e26a6d7]::visit::walk_crate::<rustc_resolve[ba041298a12665e0]::build_reduced_graph::BuildReducedGraphVisitor>
  24:     0x7f77e72d5bac - <rustc_resolve[ba041298a12665e0]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[ee10e5f9e26a6d7]::visit::Visitor>::visit_crate
  25:     0x7f77e72f596d - <rustc_resolve[ba041298a12665e0]::Resolver as rustc_expand[8253b1dbfb71c6af]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  26:     0x7f77e87095c7 - <rustc_expand[8253b1dbfb71c6af]::expand::MacroExpander>::collect_invocations
  27:     0x7f77e870494f - <rustc_expand[8253b1dbfb71c6af]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7f77e87045fb - <rustc_expand[8253b1dbfb71c6af]::expand::MacroExpander>::expand_crate
  29:     0x7f77e68ef063 - <rustc_session[d1341c97f23ce4da]::session::Session>::time::<core[ba5cb6be30a93795]::result::Result<rustc_ast[ee10e5f9e26a6d7]::ast::Crate, rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_interface[c015e0d6d1a2e32c]::passes::configure_and_expand::{closure#1}>
  30:     0x7f77e6985c7d - rustc_interface[c015e0d6d1a2e32c]::passes::configure_and_expand
  31:     0x7f77e68d27a0 - <rustc_interface[c015e0d6d1a2e32c]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[c015e0d6d1a2e32c]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<rustc_ast[ee10e5f9e26a6d7]::ast::Crate, rustc_errors[5cad097c964025d7]::ErrorReported>>
  32:     0x7f77e696662d - <rustc_interface[c015e0d6d1a2e32c]::queries::Queries>::expansion
  33:     0x7f77e678dc8f - <rustc_interface[c015e0d6d1a2e32c]::interface::Compiler>::enter::<rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}::{closure#2}, core[ba5cb6be30a93795]::result::Result<core[ba5cb6be30a93795]::option::Option<rustc_interface[c015e0d6d1a2e32c]::queries::Linker>, rustc_errors[5cad097c964025d7]::ErrorReported>>
  34:     0x7f77e67cbb69 - rustc_span[519afcfa7470336f]::with_source_map::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_interface[c015e0d6d1a2e32c]::interface::create_compiler_and_run<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7f77e678bfc9 - rustc_interface[c015e0d6d1a2e32c]::interface::create_compiler_and_run::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>
  36:     0x7f77e676911e - <scoped_tls[9ef8146b5f47b671]::ScopedKey<rustc_span[519afcfa7470336f]::SessionGlobals>>::set::<rustc_interface[c015e0d6d1a2e32c]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c015e0d6d1a2e32c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>
  37:     0x7f77e67668a5 - std[eec5ef45012f6570]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c015e0d6d1a2e32c]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c015e0d6d1a2e32c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>
  38:     0x7f77e67cee41 - std[eec5ef45012f6570]::panicking::try::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, core[ba5cb6be30a93795]::panic::unwind_safe::AssertUnwindSafe<<std[eec5ef45012f6570]::thread::Builder>::spawn_unchecked_<rustc_interface[c015e0d6d1a2e32c]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c015e0d6d1a2e32c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#1}::{closure#0}>>
  39:     0x7f77e6768d92 - <<std[eec5ef45012f6570]::thread::Builder>::spawn_unchecked_<rustc_interface[c015e0d6d1a2e32c]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c015e0d6d1a2e32c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#1} as core[ba5cb6be30a93795]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f77e5d18213 - std::sys::unix::thread::Thread::new::thread_start::he83677990b8dc5fd
  41:     0x7f77e008a609 - start_thread
  42:     0x7f77e5b81293 - clone
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (5f8140a5d 2022-02-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z split-metadata -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
