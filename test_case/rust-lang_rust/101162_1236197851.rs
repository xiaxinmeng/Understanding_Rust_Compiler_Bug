plain
   |
81 |     alloc_range, AllocRange, InterpResult, MPlaceTy, ScalarMaybeUninit,
   |                                                      ^^^^^^^^^^^^^^^^^ no `ScalarMaybeUninit` in `interpret`

thread 'rustc' panicked at 'identifier: "resolve_method_not_member_of_trait", attr: None, args: FluentArgs([("candidate", String("use_addr_for_alignment_check")), ("method", String("force_int_for_alignment_check")), ("trait_", String("Machine"))]), errors: [ResolverError(Reference(Message { id: "method", attribute: None })), ResolverError(Reference(Message { id: "trait_", attribute: None }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f90687f0e1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45e40bb98de4c3d3
   1:     0x7f9068855b69 - core::fmt::write::ha1e5e5dbace7054c
   2:     0x7f90687e20b1 - std::io::Write::write_fmt::h0eb9acc595520ad1
   2:     0x7f90687e20b1 - std::io::Write::write_fmt::h0eb9acc595520ad1
   3:     0x7f90687f3cc8 - std::panicking::default_hook::{{closure}}::hb6ba27abd886302e
   4:     0x7f90687f3a2a - std::panicking::default_hook::hcd0c9c568b145db7
   5:     0x7f9069148374 - rustc_driver[4f490e714d838722]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f90687f446a - std::panicking::rust_panic_with_hook::hc536d219ea67c000
   7:     0x7f90687f4297 - std::panicking::begin_panic_handler::{{closure}}::h0c69c3728168ba0e
   8:     0x7f90687f13bc - std::sys_common::backtrace::__rust_end_short_backtrace::h29215c2c0ff08de0
   9:     0x7f90687f3f62 - rust_begin_unwind
  10:     0x7f90687aacc3 - core::panicking::panic_fmt::hea72d0dde7d5a73e
  11:     0x7f906c60a62f - <rustc_errors[36b66c2fc792656]::emitter::EmitterWriter as rustc_errors[36b66c2fc792656]::translation::Translate>::translate_message
  12:     0x7f906c5fddee - <rustc_errors[36b66c2fc792656]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f906c602ae8 - <rustc_errors[36b66c2fc792656]::emitter::EmitterWriter>::emit_messages_default
  14:     0x7f906c5fbc7a - <rustc_errors[36b66c2fc792656]::emitter::EmitterWriter as rustc_errors[36b66c2fc792656]::emitter::Emitter>::emit_diagnostic
  15:     0x7f906c6157dc - <rustc_errors[36b66c2fc792656]::json::Diagnostic>::from_errors_diagnostic
  16:     0x7f906c613a1c - <rustc_errors[36b66c2fc792656]::json::JsonEmitter as rustc_errors[36b66c2fc792656]::emitter::Emitter>::emit_diagnostic
  17:     0x7f906c66bc20 - <rustc_errors[36b66c2fc792656]::HandlerInner>::emit_diagnostic
  18:     0x7f906c666916 - <rustc_errors[36b66c2fc792656]::ErrorGuaranteed as rustc_errors[36b66c2fc792656]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  19:     0x7f906a17e9d8 - <rustc_resolve[bf4a07c12856838a]::Resolver>::report_error
  20:     0x7f906a0e7bdb - <rustc_resolve[bf4a07c12856838a]::late::LateResolutionVisitor>::resolve_impl_item
  21:     0x7f906a0d492e - <rustc_resolve[bf4a07c12856838a]::late::LateResolutionVisitor>::resolve_item
  22:     0x7f906a0a6563 - <rustc_resolve[bf4a07c12856838a]::late::LateResolutionVisitor as rustc_ast[122c12a20a8fbf89]::visit::Visitor>::visit_item
  23:     0x7f906a1e0a8a - rustc_ast[122c12a20a8fbf89]::visit::walk_item::<rustc_resolve[bf4a07c12856838a]::late::LateResolutionVisitor>
  24:     0x7f906a0cb0d4 - <rustc_resolve[bf4a07c12856838a]::late::LateResolutionVisitor>::resolve_item
  25:     0x7f906a0a6563 - <rustc_resolve[bf4a07c12856838a]::late::LateResolutionVisitor as rustc_ast[122c12a20a8fbf89]::visit::Visitor>::visit_item
  26:     0x7f906a194469 - <rustc_resolve[bf4a07c12856838a]::Resolver>::late_resolve_crate
  27:     0x7f906a14a4a1 - <rustc_session[2b101afcea94c708]::session::Session>::time::<(), <rustc_resolve[bf4a07c12856838a]::Resolver>::resolve_crate::{closure#0}>
  28:     0x7f90692f0fbc - <rustc_interface[192311701cb9680e]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[192311701cb9680e]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[c05275981590265e]::result::Result<rustc_ast[122c12a20a8fbf89]::ast::Crate, rustc_errors[36b66c2fc792656]::ErrorGuaranteed>>
  29:     0x7f90692b9a6d - <rustc_interface[192311701cb9680e]::queries::Queries>::expansion
  30:     0x7f9069152fbf - <rustc_interface[192311701cb9680e]::interface::Compiler>::enter::<rustc_driver[4f490e714d838722]::run_compiler::{closure#1}::{closure#2}, core[c05275981590265e]::result::Result<core[c05275981590265e]::option::Option<rustc_interface[192311701cb9680e]::queries::Linker>, rustc_errors[36b66c2fc792656]::ErrorGuaranteed>>
  31:     0x7f9069130e52 - rustc_span[d84aeba3e80e3890]::with_source_map::<core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>, rustc_interface[192311701cb9680e]::interface::create_compiler_and_run<core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>, rustc_driver[4f490e714d838722]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7f9069154d44 - rustc_interface[192311701cb9680e]::interface::create_compiler_and_run::<core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>, rustc_driver[4f490e714d838722]::run_compiler::{closure#1}>
  33:     0x7f906912a0b1 - <scoped_tls[fccb67263abf41ed]::ScopedKey<rustc_span[d84aeba3e80e3890]::SessionGlobals>>::set::<rustc_interface[192311701cb9680e]::interface::run_compiler<core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>, rustc_driver[4f490e714d838722]::run_compiler::{closure#1}>::{closure#0}, core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>>
  34:     0x7f90691ea810 - std[bc53bdb3d4e0c2d1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[192311701cb9680e]::util::run_in_thread_pool_with_globals<rustc_interface[192311701cb9680e]::interface::run_compiler<core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>, rustc_driver[4f490e714d838722]::run_compiler::{closure#1}>::{closure#0}, core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>>::{closure#0}, core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>>
  35:     0x7f90691ec0a8 - <<std[bc53bdb3d4e0c2d1]::thread::Builder>::spawn_unchecked_<rustc_interface[192311701cb9680e]::util::run_in_thread_pool_with_globals<rustc_interface[192311701cb9680e]::interface::run_compiler<core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>, rustc_driver[4f490e714d838722]::run_compiler::{closure#1}>::{closure#0}, core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>>::{closure#0}, core[c05275981590265e]::result::Result<(), rustc_errors[36b66c2fc792656]::ErrorGuaranteed>>::{closure#1} as core[c05275981590265e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f9068800255 - std::sys::unix::thread::Thread::new::thread_start::h166f452f4cbe9a52
  37:     0x7f90685a3b43 - <unknown>
  38:     0x7f9068635a00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (06dc5cf1c 2022-09-03) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z binary-dep-depinfo -Z tls-model=initial-exec
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
---
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...

We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
