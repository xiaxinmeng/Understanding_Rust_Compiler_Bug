plain
failures:

---- [ui] src/test/ui/macros/issue-61033-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-61033-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-61033-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-61033-2/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "expand_var_still_repeating", attr: None, args: FluentArgs([("msg", String("meta-variable `id1` repeats 2 times, but `id2` repeats 3 times"))]), errors: [ResolverError(Reference(Variable { id: "ident" }))]', compiler/rustc_errors/src/emitter.rs:317:17
stack backtrace:
   0:     0x7fc5acc2da0c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf6ab55514e5534df
   1:     0x7fc5acc961a8 - core::fmt::write::h8414babd75cb530e
   2:     0x7fc5acc1e701 - std::io::Write::write_fmt::hc68a000dd062d1ba
   3:     0x7fc5acc309fe - std::panicking::default_hook::{{closure}}::h5c1ffd500a376eb2
   4:     0x7fc5acc306c7 - std::panicking::default_hook::h91627a1248588a88
   5:     0x7fc5ad5bec14 - rustc_driver[a4b39c2fb038fd9a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc5acc311b1 - std::panicking::rust_panic_with_hook::h7c31de60ab8b3176
   7:     0x7fc5acc30fd7 - std::panicking::begin_panic_handler::{{closure}}::h86c244dbd084f8e3
   8:     0x7fc5acc2df84 - std::sys_common::backtrace::__rust_end_short_backtrace::hb47fbfd93a4c8ed5
   9:     0x7fc5acc30ca2 - rust_begin_unwind
  10:     0x7fc5acbe0e43 - core::panicking::panic_fmt::hd8e0b0b443cad13c
  11:     0x7fc5b028ebba - <rustc_errors[8435948d5fa7c78d]::emitter::EmitterWriter as rustc_errors[8435948d5fa7c78d]::emitter::Emitter>::translate_message
  12:     0x7fc5b0293cc2 - <rustc_errors[8435948d5fa7c78d]::emitter::EmitterWriter>::emit_message_default
  13:     0x7fc5b0291786 - <rustc_errors[8435948d5fa7c78d]::emitter::EmitterWriter as rustc_errors[8435948d5fa7c78d]::emitter::Emitter>::emit_diagnostic
  14:     0x7fc5b02a97e2 - <rustc_errors[8435948d5fa7c78d]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7fc5b02a81dc - <rustc_errors[8435948d5fa7c78d]::json::JsonEmitter as rustc_errors[8435948d5fa7c78d]::emitter::Emitter>::emit_diagnostic
  16:     0x7fc5b02ec738 - <rustc_errors[8435948d5fa7c78d]::HandlerInner>::emit_diagnostic
  17:     0x7fc5b02e87d6 - <rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed as rustc_errors[8435948d5fa7c78d]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7fc5af8a92dc - <rustc_expand[17d257120e2608b8]::mbe::macro_rules::MacroRulesMacroExpander as rustc_expand[17d257120e2608b8]::base::TTMacroExpander>::expand
  19:     0x7fc5af87a8cd - <rustc_expand[17d257120e2608b8]::expand::MacroExpander>::fully_expand_fragment
  20:     0x7fc5af879a93 - <rustc_expand[17d257120e2608b8]::expand::MacroExpander>::expand_crate
  21:     0x7fc5ad740914 - <rustc_session[b3eacb7e61e43be4]::session::Session>::time::<core[ee477958f25c17b]::result::Result<rustc_ast[d96364eae8899824]::ast::Crate, rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>, rustc_interface[d5d99b53e706d51e]::passes::configure_and_expand::{closure#1}>
  22:     0x7fc5ad6fc89d - rustc_interface[d5d99b53e706d51e]::passes::configure_and_expand
  23:     0x7fc5ad709503 - <rustc_interface[d5d99b53e706d51e]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[d5d99b53e706d51e]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[ee477958f25c17b]::result::Result<rustc_ast[d96364eae8899824]::ast::Crate, rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>>
  24:     0x7fc5ad6e3277 - <rustc_interface[d5d99b53e706d51e]::queries::Queries>::expansion
  25:     0x7fc5ad5c5fea - <rustc_interface[d5d99b53e706d51e]::interface::Compiler>::enter::<rustc_driver[a4b39c2fb038fd9a]::run_compiler::{closure#1}::{closure#2}, core[ee477958f25c17b]::result::Result<core[ee477958f25c17b]::option::Option<rustc_interface[d5d99b53e706d51e]::queries::Linker>, rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>>
  26:     0x7fc5ad5ab161 - rustc_span[def7e00c6feb89d3]::with_source_map::<core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>, rustc_interface[d5d99b53e706d51e]::interface::create_compiler_and_run<core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>, rustc_driver[a4b39c2fb038fd9a]::run_compiler::{closure#1}>::{closure#1}>
  27:     0x7fc5ad5e0861 - rustc_interface[d5d99b53e706d51e]::interface::create_compiler_and_run::<core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>, rustc_driver[a4b39c2fb038fd9a]::run_compiler::{closure#1}>
  28:     0x7fc5ad5a30c2 - <scoped_tls[62cf8e50b67bab64]::ScopedKey<rustc_span[def7e00c6feb89d3]::SessionGlobals>>::set::<rustc_interface[d5d99b53e706d51e]::interface::run_compiler<core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>, rustc_driver[a4b39c2fb038fd9a]::run_compiler::{closure#1}>::{closure#0}, core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>>
  29:     0x7fc5ad5aeed9 - std[1f6eac5e91270f2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d5d99b53e706d51e]::util::run_in_thread_pool_with_globals<rustc_interface[d5d99b53e706d51e]::interface::run_compiler<core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>, rustc_driver[a4b39c2fb038fd9a]::run_compiler::{closure#1}>::{closure#0}, core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>>::{closure#0}, core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>>
  30:     0x7fc5ad62960e - std[1f6eac5e91270f2]::panicking::try::<core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>, core[ee477958f25c17b]::panic::unwind_safe::AssertUnwindSafe<<std[1f6eac5e91270f2]::thread::Builder>::spawn_unchecked_<rustc_interface[d5d99b53e706d51e]::util::run_in_thread_pool_with_globals<rustc_interface[d5d99b53e706d51e]::interface::run_compiler<core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>, rustc_driver[a4b39c2fb038fd9a]::run_compiler::{closure#1}>::{closure#0}, core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>>::{closure#0}, core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  31:     0x7fc5ad624930 - <<std[1f6eac5e91270f2]::thread::Builder>::spawn_unchecked_<rustc_interface[d5d99b53e706d51e]::util::run_in_thread_pool_with_globals<rustc_interface[d5d99b53e706d51e]::interface::run_compiler<core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>, rustc_driver[a4b39c2fb038fd9a]::run_compiler::{closure#1}>::{closure#0}, core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>>::{closure#0}, core[ee477958f25c17b]::result::Result<(), rustc_errors[8435948d5fa7c78d]::ErrorGuaranteed>>::{closure#1} as core[ee477958f25c17b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7fc5acc3dd45 - std::sys::unix::thread::Thread::new::thread_start::hbfd8d24eca5d0afc
  33:     0x7fc5ac9dab43 - <unknown>
  34:     0x7fc5aca6ca00 - <unknown>
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (2e5abeb4a 2022-08-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

