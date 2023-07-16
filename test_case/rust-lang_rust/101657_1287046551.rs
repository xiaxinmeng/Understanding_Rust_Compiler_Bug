plain
........................................................................................ 10560/13692
........................................................................................ 10648/13692
........................................................................................ 10736/13692
........................................................................................ 10824/13692
......iiiii...i....i.i...............................................................F.. 10912/13692
..................F.F............................................................i...... 11000/13692
...iiiiii.i..iiiiiiiii.i................................................................ 11176/13692
........................................................................................ 11264/13692
........................................................................................ 11352/13692
........................................................................................ 11440/13692
---
failures:

---- [ui] src/test/ui/rfc-2632-const-trait-impl/issue-90052.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/issue-90052.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-90052" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-90052/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:78:9
stack backtrace:
   0:     0x7f9ce729359e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h73de626ff4912b69
   1:     0x7f9ce72fcb68 - core::fmt::write::ha21c734e4136060d
   2:     0x7f9ce7284c11 - std::io::Write::write_fmt::h4ce5687a885624ba
   3:     0x7f9ce72933a1 - std::sys_common::backtrace::print::hf145db50eba6a22c
   4:     0x7f9ce7296524 - std::panicking::default_hook::{{closure}}::hbae0a6e8111776a7
   5:     0x7f9ce72961e9 - std::panicking::default_hook::h991118e529730426
   6:     0x7f9ce7c82744 - rustc_driver[a10d394cf4440462]::DEFAULT_HOOK::{closure#0}::{closure#0}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   7:     0x7f9ce7296c74 - std::panicking::rust_panic_with_hook::hb0a1891fbc8247b2
   8:     0x7f9ce72969d7 - std::panicking::begin_panic_handler::{{closure}}::h9751080288fc8f30
   9:     0x7f9ce7293ad4 - std::sys_common::backtrace::__rust_end_short_backtrace::h7be62e5b64c20712
  10:     0x7f9ce72966a2 - rust_begin_unwind
  11:     0x7f9ce7247a93 - core::panicking::panic_fmt::hcb732a0cefa92783
  12:     0x7f9ce72f9211 - core::panicking::panic_display::h2d8b690866968e1f
  13:     0x7f9ce72f91bb - core::panicking::panic_str::h0002e93bd22414a0
  14:     0x7f9ce7247a56 - core::option::expect_failed::hf464f87627d63b96
  15:     0x7f9ceab14bb7 - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter as rustc_errors[fef23250c3f33dae]::translation::Translate>::translate_message
  16:     0x7f9ceab096f4 - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter>::msg_to_buffer
  17:     0x7f9ceab0a10a - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter>::emit_message_default
  18:     0x7f9ceab07a0c - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter as rustc_errors[fef23250c3f33dae]::emitter::Emitter>::emit_diagnostic
  19:     0x7f9ceab1fba7 - <rustc_errors[fef23250c3f33dae]::json::Diagnostic>::from_errors_diagnostic
  20:     0x7f9ceab1e6ac - <rustc_errors[fef23250c3f33dae]::json::JsonEmitter as rustc_errors[fef23250c3f33dae]::emitter::Emitter>::emit_diagnostic
  21:     0x7f9ceab7cdcb - <rustc_errors[fef23250c3f33dae]::HandlerInner>::emit_diagnostic
  22:     0x7f9ceab76856 - <rustc_errors[fef23250c3f33dae]::ErrorGuaranteed as rustc_errors[fef23250c3f33dae]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  23:     0x7f9cea11d905 - <rustc_session[52e2fe0e7ab8e2be]::parse::ParseSess>::emit_err::<rustc_ast_passes[dd23d21f76b8196c]::errors::ForbiddenMaybeConst>
  24:     0x7f9cea136428 - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_param_bound
  25:     0x7f9cea16585c - rustc_ast[93b66770d8d448ff]::visit::walk_where_predicate::<rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator>
  26:     0x7f9cea1359a4 - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_generics
  27:     0x7f9cea166975 - rustc_ast[93b66770d8d448ff]::visit::walk_fn::<rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator>
  28:     0x7f9cea1368d4 - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_fn
  29:     0x7f9cea132a3d - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_item
  30:     0x7f9cea15dd2d - rustc_ast[93b66770d8d448ff]::visit::walk_crate::<rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator>
  31:     0x7f9cea137686 - rustc_ast_passes[dd23d21f76b8196c]::ast_validation::check_crate
  32:     0x7f9ce7dd4ac4 - rustc_interface[7feb8f941ff2de5d]::passes::configure_and_expand
  33:     0x7f9ce7de4476 - <rustc_interface[7feb8f941ff2de5d]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[7feb8f941ff2de5d]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<rustc_ast[93b66770d8d448ff]::ast::Crate, rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  34:     0x7f9ce7dba1a8 - <rustc_interface[7feb8f941ff2de5d]::queries::Queries>::expansion
  35:     0x7f9ce7cf1913 - <rustc_interface[7feb8f941ff2de5d]::interface::Compiler>::enter::<rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}::{closure#2}, core[182186c5516ead32]::result::Result<core[182186c5516ead32]::option::Option<rustc_interface[7feb8f941ff2de5d]::queries::Linker>, rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  36:     0x7f9ce7c83e8e - rustc_span[6de14575150da91]::with_source_map::<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  37:     0x7f9ce7ce50dc - <scoped_tls[1da844f2237891e4]::ScopedKey<rustc_span[6de14575150da91]::SessionGlobals>>::set::<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  38:     0x7f9ce7cdfc29 - std[c4d555e2821146d7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7feb8f941ff2de5d]::util::run_in_thread_pool_with_globals<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  39:     0x7f9ce7cea378 - std[c4d555e2821146d7]::panic::catch_unwind::<core[182186c5516ead32]::panic::unwind_safe::AssertUnwindSafe<<std[c4d555e2821146d7]::thread::Builder>::spawn_unchecked_<rustc_interface[7feb8f941ff2de5d]::util::run_in_thread_pool_with_globals<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  40:     0x7f9ce7c95f6a - <<std[c4d555e2821146d7]::thread::Builder>::spawn_unchecked_<rustc_interface[7feb8f941ff2de5d]::util::run_in_thread_pool_with_globals<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#1} as core[182186c5516ead32]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f9ce72a34e5 - std::sys::unix::thread::Thread::new::thread_start::h72281c86b27b6d43
  42:     0x7f9ce703db43 - <unknown>
  43:     0x7f9ce70cfa00 - <unknown>
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (38f412cc5 2022-10-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/tilde-const-and-const-params.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/tilde-const-and-const-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/tilde-const-and-const-params" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/tilde-const-and-const-params/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:78:9
stack backtrace:
   0:     0x7f5906b8659e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h73de626ff4912b69
   1:     0x7f5906befb68 - core::fmt::write::ha21c734e4136060d
   2:     0x7f5906b77c11 - std::io::Write::write_fmt::h4ce5687a885624ba
   3:     0x7f5906b863a1 - std::sys_common::backtrace::print::hf145db50eba6a22c
   4:     0x7f5906b89524 - std::panicking::default_hook::{{closure}}::hbae0a6e8111776a7
   5:     0x7f5906b891e9 - std::panicking::default_hook::h991118e529730426
   6:     0x7f5907575744 - rustc_driver[a10d394cf4440462]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5906b89c74 - std::panicking::rust_panic_with_hook::hb0a1891fbc8247b2
   8:     0x7f5906b899d7 - std::panicking::begin_panic_handler::{{closure}}::h9751080288fc8f30
   9:     0x7f5906b86ad4 - std::sys_common::backtrace::__rust_end_short_backtrace::h7be62e5b64c20712
  10:     0x7f5906b896a2 - rust_begin_unwind
  11:     0x7f5906b3aa93 - core::panicking::panic_fmt::hcb732a0cefa92783
  12:     0x7f5906bec211 - core::panicking::panic_display::h2d8b690866968e1f
  13:     0x7f5906bec1bb - core::panicking::panic_str::h0002e93bd22414a0
  14:     0x7f5906b3aa56 - core::option::expect_failed::hf464f87627d63b96
  15:     0x7f590a407bb7 - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter as rustc_errors[fef23250c3f33dae]::translation::Translate>::translate_message
  16:     0x7f590a3fc6f4 - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter>::msg_to_buffer
  17:     0x7f590a3fd10a - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter>::emit_message_default
  18:     0x7f590a3faa0c - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter as rustc_errors[fef23250c3f33dae]::emitter::Emitter>::emit_diagnostic
  19:     0x7f590a412ba7 - <rustc_errors[fef23250c3f33dae]::json::Diagnostic>::from_errors_diagnostic
  20:     0x7f590a4116ac - <rustc_errors[fef23250c3f33dae]::json::JsonEmitter as rustc_errors[fef23250c3f33dae]::emitter::Emitter>::emit_diagnostic
  21:     0x7f590a46fdcb - <rustc_errors[fef23250c3f33dae]::HandlerInner>::emit_diagnostic
  22:     0x7f590a469856 - <rustc_errors[fef23250c3f33dae]::ErrorGuaranteed as rustc_errors[fef23250c3f33dae]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  23:     0x7f5909a10905 - <rustc_session[52e2fe0e7ab8e2be]::parse::ParseSess>::emit_err::<rustc_ast_passes[dd23d21f76b8196c]::errors::ForbiddenMaybeConst>
  24:     0x7f5909a29428 - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_param_bound
  25:     0x7f5909a5755c - rustc_ast[93b66770d8d448ff]::visit::walk_generic_param::<rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator>
  26:     0x7f5909a288a0 - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_generics
  27:     0x7f5909a59975 - rustc_ast[93b66770d8d448ff]::visit::walk_fn::<rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator>
  28:     0x7f5909a298d4 - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_fn
  29:     0x7f5909a25a3d - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_item
  30:     0x7f5909a50d2d - rustc_ast[93b66770d8d448ff]::visit::walk_crate::<rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator>
  31:     0x7f5909a2a686 - rustc_ast_passes[dd23d21f76b8196c]::ast_validation::check_crate
  32:     0x7f59076c7ac4 - rustc_interface[7feb8f941ff2de5d]::passes::configure_and_expand
  33:     0x7f59076d7476 - <rustc_interface[7feb8f941ff2de5d]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[7feb8f941ff2de5d]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<rustc_ast[93b66770d8d448ff]::ast::Crate, rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  34:     0x7f59076ad1a8 - <rustc_interface[7feb8f941ff2de5d]::queries::Queries>::expansion
  35:     0x7f59075e4913 - <rustc_interface[7feb8f941ff2de5d]::interface::Compiler>::enter::<rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}::{closure#2}, core[182186c5516ead32]::result::Result<core[182186c5516ead32]::option::Option<rustc_interface[7feb8f941ff2de5d]::queries::Linker>, rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  36:     0x7f5907576e8e - rustc_span[6de14575150da91]::with_source_map::<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  37:     0x7f59075d80dc - <scoped_tls[1da844f2237891e4]::ScopedKey<rustc_span[6de14575150da91]::SessionGlobals>>::set::<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  38:     0x7f59075d2c29 - std[c4d555e2821146d7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7feb8f941ff2de5d]::util::run_in_thread_pool_with_globals<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  39:     0x7f59075dd378 - std[c4d555e2821146d7]::panic::catch_unwind::<core[182186c5516ead32]::panic::unwind_safe::AssertUnwindSafe<<std[c4d555e2821146d7]::thread::Builder>::spawn_unchecked_<rustc_interface[7feb8f941ff2de5d]::util::run_in_thread_pool_with_globals<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  40:     0x7f5907588f6a - <<std[c4d555e2821146d7]::thread::Builder>::spawn_unchecked_<rustc_interface[7feb8f941ff2de5d]::util::run_in_thread_pool_with_globals<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#1} as core[182186c5516ead32]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f5906b964e5 - std::sys::unix::thread::Thread::new::thread_start::h72281c86b27b6d43
  42:     0x7f5906930b43 - <unknown>
  43:     0x7f59069c2a00 - <unknown>
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (38f412cc5 2022-10-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/rfc-2632-const-trait-impl/tilde-const-invalid-places.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/tilde-const-invalid-places.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/tilde-const-invalid-places" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/tilde-const-invalid-places/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:78:9
stack backtrace:
   0:     0x7f77cc88b59e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h73de626ff4912b69
   1:     0x7f77cc8f4b68 - core::fmt::write::ha21c734e4136060d
   2:     0x7f77cc87cc11 - std::io::Write::write_fmt::h4ce5687a885624ba
   3:     0x7f77cc88b3a1 - std::sys_common::backtrace::print::hf145db50eba6a22c
   4:     0x7f77cc88e524 - std::panicking::default_hook::{{closure}}::hbae0a6e8111776a7
   5:     0x7f77cc88e1e9 - std::panicking::default_hook::h991118e529730426
   6:     0x7f77cd27a744 - rustc_driver[a10d394cf4440462]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f77cc88ec74 - std::panicking::rust_panic_with_hook::hb0a1891fbc8247b2
   8:     0x7f77cc88e9d7 - std::panicking::begin_panic_handler::{{closure}}::h9751080288fc8f30
   9:     0x7f77cc88bad4 - std::sys_common::backtrace::__rust_end_short_backtrace::h7be62e5b64c20712
  10:     0x7f77cc88e6a2 - rust_begin_unwind
  11:     0x7f77cc83fa93 - core::panicking::panic_fmt::hcb732a0cefa92783
  12:     0x7f77cc8f1211 - core::panicking::panic_display::h2d8b690866968e1f
  13:     0x7f77cc8f11bb - core::panicking::panic_str::h0002e93bd22414a0
  14:     0x7f77cc83fa56 - core::option::expect_failed::hf464f87627d63b96
  15:     0x7f77d010cbb7 - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter as rustc_errors[fef23250c3f33dae]::translation::Translate>::translate_message
  16:     0x7f77d01016f4 - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter>::msg_to_buffer
  17:     0x7f77d010210a - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter>::emit_message_default
  18:     0x7f77d00ffa0c - <rustc_errors[fef23250c3f33dae]::emitter::EmitterWriter as rustc_errors[fef23250c3f33dae]::emitter::Emitter>::emit_diagnostic
  19:     0x7f77d0117ba7 - <rustc_errors[fef23250c3f33dae]::json::Diagnostic>::from_errors_diagnostic
  20:     0x7f77d01166ac - <rustc_errors[fef23250c3f33dae]::json::JsonEmitter as rustc_errors[fef23250c3f33dae]::emitter::Emitter>::emit_diagnostic
  21:     0x7f77d0174dcb - <rustc_errors[fef23250c3f33dae]::HandlerInner>::emit_diagnostic
  22:     0x7f77d016e856 - <rustc_errors[fef23250c3f33dae]::ErrorGuaranteed as rustc_errors[fef23250c3f33dae]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  23:     0x7f77cf715905 - <rustc_session[52e2fe0e7ab8e2be]::parse::ParseSess>::emit_err::<rustc_ast_passes[dd23d21f76b8196c]::errors::ForbiddenMaybeConst>
  24:     0x7f77cf72e428 - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_param_bound
  25:     0x7f77cf75faef - rustc_ast[93b66770d8d448ff]::visit::walk_ty::<rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator>
  26:     0x7f77cf7284e1 - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator>::walk_ty
  27:     0x7f77cf75ea93 - rustc_ast[93b66770d8d448ff]::visit::walk_fn::<rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator>
  28:     0x7f77cf72e8d4 - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_fn
  29:     0x7f77cf72aa3d - <rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator as rustc_ast[93b66770d8d448ff]::visit::Visitor>::visit_item
  30:     0x7f77cf755d2d - rustc_ast[93b66770d8d448ff]::visit::walk_crate::<rustc_ast_passes[dd23d21f76b8196c]::ast_validation::AstValidator>
  31:     0x7f77cf72f686 - rustc_ast_passes[dd23d21f76b8196c]::ast_validation::check_crate
  32:     0x7f77cd3ccac4 - rustc_interface[7feb8f941ff2de5d]::passes::configure_and_expand
  33:     0x7f77cd3dc476 - <rustc_interface[7feb8f941ff2de5d]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[7feb8f941ff2de5d]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<rustc_ast[93b66770d8d448ff]::ast::Crate, rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  34:     0x7f77cd3b21a8 - <rustc_interface[7feb8f941ff2de5d]::queries::Queries>::expansion
  35:     0x7f77cd2e9913 - <rustc_interface[7feb8f941ff2de5d]::interface::Compiler>::enter::<rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}::{closure#2}, core[182186c5516ead32]::result::Result<core[182186c5516ead32]::option::Option<rustc_interface[7feb8f941ff2de5d]::queries::Linker>, rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  36:     0x7f77cd27be8e - rustc_span[6de14575150da91]::with_source_map::<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  37:     0x7f77cd2dd0dc - <scoped_tls[1da844f2237891e4]::ScopedKey<rustc_span[6de14575150da91]::SessionGlobals>>::set::<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  38:     0x7f77cd2d7c29 - std[c4d555e2821146d7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7feb8f941ff2de5d]::util::run_in_thread_pool_with_globals<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  39:     0x7f77cd2e2378 - std[c4d555e2821146d7]::panic::catch_unwind::<core[182186c5516ead32]::panic::unwind_safe::AssertUnwindSafe<<std[c4d555e2821146d7]::thread::Builder>::spawn_unchecked_<rustc_interface[7feb8f941ff2de5d]::util::run_in_thread_pool_with_globals<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>
  40:     0x7f77cd28df6a - <<std[c4d555e2821146d7]::thread::Builder>::spawn_unchecked_<rustc_interface[7feb8f941ff2de5d]::util::run_in_thread_pool_with_globals<rustc_interface[7feb8f941ff2de5d]::interface::run_compiler<core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>, rustc_driver[a10d394cf4440462]::run_compiler::{closure#1}>::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[182186c5516ead32]::result::Result<(), rustc_errors[fef23250c3f33dae]::ErrorGuaranteed>>::{closure#1} as core[182186c5516ead32]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f77cc89b4e5 - std::sys::unix::thread::Thread::new::thread_start::h72281c86b27b6d43
  42:     0x7f77cc635b43 - <unknown>
  43:     0x7f77cc6c7a00 - <unknown>
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (38f412cc5 2022-10-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

