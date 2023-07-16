plain
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling hashbrown v0.12.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
thread 'rustc' panicked at 'not yet implemented: return error', compiler/rustc_metadata/src/locator.rs:544:37
stack backtrace:
   0:     0x7f7549eb86dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4c1ed0e5024ec2d0
   1:     0x7f7549f26e4e - core::fmt::write::h04f3cb9c5bd3369c
   2:     0x7f7549ea76e1 - std::io::Write::write_fmt::hec437177037c4304
   3:     0x7f7549eb850b - std::sys_common::backtrace::print::h9084352f72bce3d6
   4:     0x7f7549ebccd4 - std::panicking::default_hook::{{closure}}::h4e03c6131e9375ec
   5:     0x7f7549ebc8b6 - std::panicking::default_hook::h7c7d01494eae6d70
   6:     0x7f754a992e11 - rustc_driver[a93e051fcd12f9f8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f7549ebd3f3 - std::panicking::rust_panic_with_hook::hea69582e6c4a33d9
   8:     0x7f7549ebd207 - std::panicking::begin_panic_handler::{{closure}}::he6a21a353581f6cb
   9:     0x7f7549eb8bf4 - std::sys_common::backtrace::__rust_end_short_backtrace::h2d0a50649be03b70
  10:     0x7f7549ebcec9 - rust_begin_unwind
  11:     0x7f7549e739d3 - core::panicking::panic_fmt::h7802ba5043cb2ca5
  12:     0x7f754c72edd8 - <rustc_metadata[2c610e6225a46cf]::locator::CrateLocator>::extract_one
  13:     0x7f754c72c692 - <rustc_metadata[2c610e6225a46cf]::locator::CrateLocator>::extract_lib
  14:     0x7f754c72a311 - <rustc_metadata[2c610e6225a46cf]::locator::CrateLocator>::maybe_load_library_crate
  15:     0x7f754c6d203a - <rustc_metadata[2c610e6225a46cf]::creader::CrateLoader>::load
  16:     0x7f754c6ce55a - <rustc_metadata[2c610e6225a46cf]::creader::CrateLoader>::maybe_resolve_crate
  17:     0x7f754c6cd6d2 - <rustc_metadata[2c610e6225a46cf]::creader::CrateLoader>::resolve_crate
  18:     0x7f754c6d4b1c - <rustc_metadata[2c610e6225a46cf]::creader::CrateLoader>::process_extern_crate
  19:     0x7f754b488a43 - <rustc_resolve[e93c816d39f0de31]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[fb718f914770ae88]::visit::Visitor>::visit_item
  20:     0x7f754b4ca29d - rustc_ast[fb718f914770ae88]::visit::walk_crate::<rustc_resolve[e93c816d39f0de31]::build_reduced_graph::BuildReducedGraphVisitor>
  21:     0x7f754b48b2fc - <rustc_resolve[e93c816d39f0de31]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[fb718f914770ae88]::visit::Visitor>::visit_crate
  22:     0x7f754b4ab0cd - <rustc_resolve[e93c816d39f0de31]::Resolver as rustc_expand[3f6f1937b52b127f]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  23:     0x7f754c8bf357 - <rustc_expand[3f6f1937b52b127f]::expand::MacroExpander>::collect_invocations
  24:     0x7f754c8ba6df - <rustc_expand[3f6f1937b52b127f]::expand::MacroExpander>::fully_expand_fragment
  25:     0x7f754c8ba38b - <rustc_expand[3f6f1937b52b127f]::expand::MacroExpander>::expand_crate
  26:     0x7f754aaa5a63 - <rustc_session[22750a20d77a6d56]::session::Session>::time::<core[ba5cb6be30a93795]::result::Result<rustc_ast[fb718f914770ae88]::ast::Crate, rustc_errors[938271d98dcf7a]::ErrorReported>, rustc_interface[768531affdc57d1c]::passes::configure_and_expand::{closure#1}>
  27:     0x7f754ab3c28d - rustc_interface[768531affdc57d1c]::passes::configure_and_expand
  28:     0x7f754aa89250 - <rustc_interface[768531affdc57d1c]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[768531affdc57d1c]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<rustc_ast[fb718f914770ae88]::ast::Crate, rustc_errors[938271d98dcf7a]::ErrorReported>>
  29:     0x7f754ab1cc3d - <rustc_interface[768531affdc57d1c]::queries::Queries>::expansion
  30:     0x7f754a94412f - <rustc_interface[768531affdc57d1c]::interface::Compiler>::enter::<rustc_driver[a93e051fcd12f9f8]::run_compiler::{closure#1}::{closure#2}, core[ba5cb6be30a93795]::result::Result<core[ba5cb6be30a93795]::option::Option<rustc_interface[768531affdc57d1c]::queries::Linker>, rustc_errors[938271d98dcf7a]::ErrorReported>>
  31:     0x7f754a981ac9 - rustc_span[db79cfb26f8098d6]::with_source_map::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>, rustc_interface[768531affdc57d1c]::interface::create_compiler_and_run<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>, rustc_driver[a93e051fcd12f9f8]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7f754a94242b - rustc_interface[768531affdc57d1c]::interface::create_compiler_and_run::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>, rustc_driver[a93e051fcd12f9f8]::run_compiler::{closure#1}>
  33:     0x7f754a91f57e - <scoped_tls[9ef8146b5f47b671]::ScopedKey<rustc_span[db79cfb26f8098d6]::SessionGlobals>>::set::<rustc_interface[768531affdc57d1c]::util::run_in_thread_pool_with_globals<rustc_interface[768531affdc57d1c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>, rustc_driver[a93e051fcd12f9f8]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>>::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>>
  34:     0x7f754a91cd05 - std[39bfa0482ff6d9eb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[768531affdc57d1c]::util::run_in_thread_pool_with_globals<rustc_interface[768531affdc57d1c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>, rustc_driver[a93e051fcd12f9f8]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>>
  35:     0x7f754a984e21 - std[39bfa0482ff6d9eb]::panicking::try::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>, core[ba5cb6be30a93795]::panic::unwind_safe::AssertUnwindSafe<<std[39bfa0482ff6d9eb]::thread::Builder>::spawn_unchecked_<rustc_interface[768531affdc57d1c]::util::run_in_thread_pool_with_globals<rustc_interface[768531affdc57d1c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>, rustc_driver[a93e051fcd12f9f8]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>>::{closure#1}::{closure#0}>>
  36:     0x7f754a91f1f2 - <<std[39bfa0482ff6d9eb]::thread::Builder>::spawn_unchecked_<rustc_interface[768531affdc57d1c]::util::run_in_thread_pool_with_globals<rustc_interface[768531affdc57d1c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>, rustc_driver[a93e051fcd12f9f8]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[938271d98dcf7a]::ErrorReported>>::{closure#1} as core[ba5cb6be30a93795]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7f7549ecc1c3 - std::sys::unix::thread::Thread::new::thread_start::h06eaa01755c40ab7
  38:     0x7f754423e609 - start_thread
  39:     0x7f7549d35293 - clone
  40:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (e6e90a610 2022-02-13) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z split-metadata -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
