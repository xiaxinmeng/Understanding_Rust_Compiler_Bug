plain
failures:

---- [ui] tests/ui/imports/extern-crate-self/extern-crate-self-fail.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/imports/extern-crate-self/extern-crate-self-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-crate-self/extern-crate-self-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-crate-self/extern-crate-self-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error: `extern crate self;` requires renaming
  --> fake-test-src-base/imports/extern-crate-self/extern-crate-self-fail.rs:1:1
   |
LL | extern crate self; //~ ERROR `extern crate self;` requires renaming
   |
help: rename the `self` crate to be able to import it
   |
   |
LL | extern crate self as name; //~ ERROR `extern crate self;` requires renaming
   | ~~~~~~~~~~~~~~~~~~~~~~~~~~

thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: failed while formatting fluent string `resolve_macro_use_extern_crate_self`: 
the attribute `note` was missing
help: add `.note = <message>`
stack backtrace:
   0:     0x7f2652913bc5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd6dc44a6b2b60215
   1:     0x7f2652980f38 - core::fmt::write::hab01729ecf32916d
   2:     0x7f2652907eb1 - std::io::Write::write_fmt::h4407b32b392a9c48
   2:     0x7f2652907eb1 - std::io::Write::write_fmt::h4407b32b392a9c48
   3:     0x7f26529139d1 - std::sys_common::backtrace::print::hb491e3bd7405bbfd
   4:     0x7f2652916bd4 - std::panicking::default_hook::{{closure}}::hd5c947cdd5142e86
   5:     0x7f26529168b9 - std::panicking::default_hook::h2febcd82eb770f9e
   6:     0x7f26533e4205 - rustc_driver_impl[1f89cd1f625da4c8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f26529172f1 - std::panicking::rust_panic_with_hook::he073af466d3ddda9
   8:     0x7f2652917069 - std::panicking::begin_panic_handler::{{closure}}::h2d4d44c47e297203
   9:     0x7f2652914096 - std::sys_common::backtrace::__rust_end_short_backtrace::h7efa68a916313c69
  10:     0x7f2652916d47 - rust_begin_unwind
  11:     0x7f26528cc473 - core::panicking::panic_fmt::h9d513263bd4c9e31
  12:     0x7f26528cca63 - core::result::unwrap_failed::hafa5c6c93d97aead
  13:     0x7f26563bf610 - <rustc_errors[ebd30b27a61daf46]::emitter::EmitterWriter>::msg_to_buffer
  14:     0x7f26563c9807 - <rustc_errors[ebd30b27a61daf46]::emitter::EmitterWriter>::emit_message_default::{closure#0}
  15:     0x7f26563c8311 - <rustc_errors[ebd30b27a61daf46]::emitter::EmitterWriter>::emit_message_default
  16:     0x7f26563bd25a - <rustc_errors[ebd30b27a61daf46]::emitter::EmitterWriter as rustc_errors[ebd30b27a61daf46]::emitter::Emitter>::emit_diagnostic
  17:     0x7f26563f11bc - <rustc_errors[ebd30b27a61daf46]::json::Diagnostic>::from_errors_diagnostic
  18:     0x7f26563efc4c - <rustc_errors[ebd30b27a61daf46]::json::JsonEmitter as rustc_errors[ebd30b27a61daf46]::emitter::Emitter>::emit_diagnostic
  19:     0x7f26563e75bb - <rustc_errors[ebd30b27a61daf46]::HandlerInner>::emit_diagnostic::{closure#2}
  20:     0x7f265349498b - <std[b4eea0cff40a6ced]::thread::local::LocalKey<core[bb1ec2c4cc794094]::cell::Cell<*const ()>>>::with::<rustc_middle[cf087189fe22d5de]::ty::context::tls::enter_context<rustc_interface[9effd9ff717fdcfb]::callbacks::track_diagnostic::{closure#0}::{closure#0}, ()>::{closure#0}, ()>
  21:     0x7f26534d80ca - rustc_interface[9effd9ff717fdcfb]::callbacks::track_diagnostic
  22:     0x7f26563e6eb2 - <rustc_errors[ebd30b27a61daf46]::HandlerInner>::emit_diagnostic
  23:     0x7f26563e5d81 - <rustc_errors[ebd30b27a61daf46]::Handler>::emit_diagnostic
  24:     0x7f26563ef579 - <rustc_span[fa480c19b527bcd5]::ErrorGuaranteed as rustc_errors[ebd30b27a61daf46]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  25:     0x7f2654545474 - <rustc_session[d55ed62fd234bbf1]::parse::ParseSess>::emit_err::<rustc_resolve[e0c4bbf9f7dac25c]::errors::MacroUseExternCrateSelf>
  26:     0x7f26544787a7 - <rustc_resolve[e0c4bbf9f7dac25c]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[26b8cab50b252c4d]::visit::Visitor>::visit_item
  27:     0x7f2654521b8d - rustc_ast[26b8cab50b252c4d]::visit::walk_crate::<rustc_resolve[e0c4bbf9f7dac25c]::build_reduced_graph::BuildReducedGraphVisitor>
  28:     0x7f265447a95d - <rustc_resolve[e0c4bbf9f7dac25c]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[26b8cab50b252c4d]::visit::Visitor>::visit_crate
  29:     0x7f26544d4e79 - <rustc_resolve[e0c4bbf9f7dac25c]::Resolver as rustc_expand[71966530f2bf098c]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  30:     0x7f265579b9a3 - <rustc_expand[71966530f2bf098c]::expand::MacroExpander>::collect_invocations
  31:     0x7f26557971fd - <rustc_expand[71966530f2bf098c]::expand::MacroExpander>::fully_expand_fragment
  32:     0x7f2655796dc9 - <rustc_expand[71966530f2bf098c]::expand::MacroExpander>::expand_crate
  33:     0x7f265355a2e7 - <rustc_session[d55ed62fd234bbf1]::session::Session>::time::<rustc_ast[26b8cab50b252c4d]::ast::Crate, rustc_interface[9effd9ff717fdcfb]::passes::configure_and_expand::{closure#1}>
  34:     0x7f26534c8c7a - rustc_interface[9effd9ff717fdcfb]::passes::resolver_for_lowering
  35:     0x7f2654f682f1 - <std[b4eea0cff40a6ced]::thread::local::LocalKey<core[bb1ec2c4cc794094]::cell::Cell<*const ()>>>::with::<rustc_middle[cf087189fe22d5de]::ty::context::tls::enter_context<rustc_query_system[bf85d7efdebd998]::query::plumbing::execute_job_non_incr<rustc_query_impl[ecfc092f8e49562f]::queries::resolver_for_lowering, rustc_query_impl[ecfc092f8e49562f]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[cf087189fe22d5de]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[cf087189fe22d5de]::query::erase::Erased<[u8; 8usize]>>
  36:     0x7f2655264427 - rustc_query_system[bf85d7efdebd998]::query::plumbing::try_execute_query::<rustc_query_impl[ecfc092f8e49562f]::queries::resolver_for_lowering, rustc_query_impl[ecfc092f8e49562f]::plumbing::QueryCtxt>
  37:     0x7f265510353b - <rustc_query_impl[ecfc092f8e49562f]::Queries as rustc_middle[cf087189fe22d5de]::ty::query::QueryEngine>::resolver_for_lowering
  38:     0x7f2653422043 - <rustc_interface[9effd9ff717fdcfb]::queries::QueryResult<&rustc_middle[cf087189fe22d5de]::ty::context::GlobalCtxt>>::enter::<&rustc_data_structures[a88c06ca05ca7b1a]::steal::Steal<(rustc_middle[cf087189fe22d5de]::ty::ResolverAstLowering, alloc[a9aec2dcc35bae73]::rc::Rc<rustc_ast[26b8cab50b252c4d]::ast::Crate>)>, rustc_driver_impl[1f89cd1f625da4c8]::run_compiler::{closure#1}::{closure#2}::{closure#2}>
  39:     0x7f265342fac2 - <rustc_interface[9effd9ff717fdcfb]::interface::Compiler>::enter::<rustc_driver_impl[1f89cd1f625da4c8]::run_compiler::{closure#1}::{closure#2}, core[bb1ec2c4cc794094]::result::Result<core[bb1ec2c4cc794094]::option::Option<rustc_interface[9effd9ff717fdcfb]::queries::Linker>, rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>>
  40:     0x7f26533f4797 - std[b4eea0cff40a6ced]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9effd9ff717fdcfb]::util::run_in_thread_pool_with_globals<rustc_interface[9effd9ff717fdcfb]::interface::run_compiler<core[bb1ec2c4cc794094]::result::Result<(), rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>, rustc_driver_impl[1f89cd1f625da4c8]::run_compiler::{closure#1}>::{closure#0}, core[bb1ec2c4cc794094]::result::Result<(), rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bb1ec2c4cc794094]::result::Result<(), rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>>
  41:     0x7f2653431998 - std[b4eea0cff40a6ced]::panicking::try::<core[bb1ec2c4cc794094]::result::Result<(), rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>, core[bb1ec2c4cc794094]::panic::unwind_safe::AssertUnwindSafe<<std[b4eea0cff40a6ced]::thread::Builder>::spawn_unchecked_<rustc_interface[9effd9ff717fdcfb]::util::run_in_thread_pool_with_globals<rustc_interface[9effd9ff717fdcfb]::interface::run_compiler<core[bb1ec2c4cc794094]::result::Result<(), rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>, rustc_driver_impl[1f89cd1f625da4c8]::run_compiler::{closure#1}>::{closure#0}, core[bb1ec2c4cc794094]::result::Result<(), rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bb1ec2c4cc794094]::result::Result<(), rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  42:     0x7f26533f61b7 - <<std[b4eea0cff40a6ced]::thread::Builder>::spawn_unchecked_<rustc_interface[9effd9ff717fdcfb]::util::run_in_thread_pool_with_globals<rustc_interface[9effd9ff717fdcfb]::interface::run_compiler<core[bb1ec2c4cc794094]::result::Result<(), rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>, rustc_driver_impl[1f89cd1f625da4c8]::run_compiler::{closure#1}>::{closure#0}, core[bb1ec2c4cc794094]::result::Result<(), rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bb1ec2c4cc794094]::result::Result<(), rustc_span[fa480c19b527bcd5]::ErrorGuaranteed>>::{closure#1} as core[bb1ec2c4cc794094]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:     0x7f2652923b8e - std::sys::unix::thread::Thread::new::thread_start::hc43354453d9e48e4
  44:     0x7f26526bbb43 - <unknown>
  45:     0x7f265274da00 - <unknown>
  46:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (517e9b95c 2023-04-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
error: aborting due to previous error
