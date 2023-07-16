plain
........................................................................................ 2728/13425
........................................................................................ 2816/13425
........................................................................................ 2904/13425
........................................................................................ 2992/13425
........................i.....................................FF..F.F.........i......... 3080/13425
........................................................................................ 3256/13425
......................................................iiiii............................. 3344/13425
........................................................................................ 3432/13425
........................................F............................................... 3520/13425
........................................F............................................... 3520/13425
........................................................................................ 3608/13425
...................................................................................F.FF. 3696/13425
......F..............................................................F.................. 3784/13425
..................................................................F.....i..........i.... 3872/13425
......................................................................i................. 4048/13425
........................................................................................ 4136/13425
...............................i........................................................ 4224/13425
........................................................................................ 4312/13425
---
failures:

---- [ui] src/test/ui/crate-loading/invalid-rlib.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/invalid-rlib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/invalid-rlib" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "lib" "--extern" "foo=/checkout/src/test/ui/crate-loading/auxiliary/libfoo.rlib" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/invalid-rlib/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "metadata_invalid_meta_files", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "crate_name" })), ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f3db340ab5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   1:     0x7f3db3473508 - core::fmt::write::h622ccda3d7f48263
   1:     0x7f3db3473508 - core::fmt::write::h622ccda3d7f48263
   2:     0x7f3db33fb7d1 - std::io::Write::write_fmt::hdd0bcc3d51a95b66
   3:     0x7f3db340db5e - std::panicking::default_hook::{{closure}}::hb63bf482bb996068
   4:     0x7f3db340d827 - std::panicking::default_hook::h1c8d1247b1fca3f3
   5:     0x7f3db3d93c44 - rustc_driver[2b033e751948032]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f3db340e311 - std::panicking::rust_panic_with_hook::h2015d0f8cb0a7e52
   7:     0x7f3db340e137 - std::panicking::begin_panic_handler::{{closure}}::h92fcf298ed83eb87
   8:     0x7f3db340b0d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf233cf8fe3635627
   9:     0x7f3db340de02 - rust_begin_unwind
  10:     0x7f3db33bee43 - core::panicking::panic_fmt::hddf3504dfa1bb37a
  11:     0x7f3db6a92377 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::translation::Translate>::translate_message
  12:     0x7f3db6a877c2 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f3db6a85266 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  14:     0x7f3db6a9d592 - <rustc_errors[36d18b9907453f35]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f3db6a9bfac - <rustc_errors[36d18b9907453f35]::json::JsonEmitter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  16:     0x7f3db6ae1cd8 - <rustc_errors[36d18b9907453f35]::HandlerInner>::emit_diagnostic
  17:     0x7f3db6add9c6 - <rustc_errors[36d18b9907453f35]::ErrorGuaranteed as rustc_errors[36d18b9907453f35]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f3db5e4bf6a - <rustc_session[8c18bc53086a9b34]::parse::ParseSess>::emit_err::<rustc_metadata[3d1c815352b1b4dd]::errors::InvalidMetadataFiles>
  19:     0x7f3db5e8a1fd - <rustc_metadata[3d1c815352b1b4dd]::locator::CrateError>::report
  20:     0x7f3db5ecbd97 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::resolve_crate
  21:     0x7f3db5ed3728 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::process_path_extern
  22:     0x7f3db4bc8e9a - <rustc_resolve[252dc9306f4267df]::Resolver>::extern_prelude_get
  23:     0x7f3db4bd2866 - <rustc_resolve[252dc9306f4267df]::Resolver>::resolve_ident_in_module_unadjusted_ext
  24:     0x7f3db4bd1434 - <rustc_resolve[252dc9306f4267df]::Resolver>::resolve_ident_in_module_ext
  25:     0x7f3db4bd0b61 - <rustc_resolve[252dc9306f4267df]::Resolver>::resolve_ident_in_module
  26:     0x7f3db4bb5b83 - <rustc_resolve[252dc9306f4267df]::imports::ImportResolver>::finalize_import::{closure#0}
  27:     0x7f3db4b540f3 - <rustc_resolve[252dc9306f4267df]::imports::ImportResolver>::finalize_import
  28:     0x7f3db4b52496 - <rustc_resolve[252dc9306f4267df]::imports::ImportResolver>::finalize_imports
  29:     0x7f3db4c32a19 - <rustc_session[8c18bc53086a9b34]::session::Session>::time::<(), <rustc_resolve[252dc9306f4267df]::Resolver>::resolve_crate::{closure#0}>
  30:     0x7f3db3ed1414 - rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand
  31:     0x7f3db3edefd6 - <rustc_interface[75bd1abe22f1fd46]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  32:     0x7f3db3eb7e76 - <rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion
  33:     0x7f3db3d9ca2a - <rustc_interface[75bd1abe22f1fd46]::interface::Compiler>::enter::<rustc_driver[2b033e751948032]::run_compiler::{closure#1}::{closure#2}, core[2506165bb0faadb9]::result::Result<core[2506165bb0faadb9]::option::Option<rustc_interface[75bd1abe22f1fd46]::queries::Linker>, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  34:     0x7f3db3d807c1 - rustc_span[4b37be1e64a2ccd5]::with_source_map::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7f3db3db8571 - rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>
  36:     0x7f3db3d78392 - <scoped_tls[e23be4e8d7e01d76]::ScopedKey<rustc_span[4b37be1e64a2ccd5]::SessionGlobals>>::set::<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  37:     0x7f3db3d840e9 - std[dbf3afd8bb1e5979]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  38:     0x7f3db3dfecfe - std[dbf3afd8bb1e5979]::panicking::try::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, core[2506165bb0faadb9]::panic::unwind_safe::AssertUnwindSafe<<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  39:     0x7f3db3df9e60 - <<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1} as core[2506165bb0faadb9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f3db341ab75 - std::sys::unix::thread::Thread::new::thread_start::ha85b465a65a1c6c4
  41:     0x7f3db31b8b43 - <unknown>
  42:     0x7f3db324aa00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: rustc 1.65.0-nightly (4a60c720d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/crate-loading/missing-std.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/missing-std.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/missing-std" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-uefi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/missing-std/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "metadata_cannot_find_crate", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "crate_name" })), ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7fb53ac42b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   0:     0x7fb53ac42b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   1:     0x7fb53acab508 - core::fmt::write::h622ccda3d7f48263
   2:     0x7fb53ac337d1 - std::io::Write::write_fmt::hdd0bcc3d51a95b66
   3:     0x7fb53ac45b5e - std::panicking::default_hook::{{closure}}::hb63bf482bb996068
   4:     0x7fb53ac45827 - std::panicking::default_hook::h1c8d1247b1fca3f3
   5:     0x7fb53b5cbc44 - rustc_driver[2b033e751948032]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb53ac46311 - std::panicking::rust_panic_with_hook::h2015d0f8cb0a7e52
   7:     0x7fb53ac46137 - std::panicking::begin_panic_handler::{{closure}}::h92fcf298ed83eb87
   8:     0x7fb53ac430d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf233cf8fe3635627
   9:     0x7fb53ac45e02 - rust_begin_unwind
  10:     0x7fb53abf6e43 - core::panicking::panic_fmt::hddf3504dfa1bb37a
  11:     0x7fb53e2ca377 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::translation::Translate>::translate_message
  12:     0x7fb53e2bf7c2 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter>::emit_message_default
  13:     0x7fb53e2bd266 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  14:     0x7fb53e2d5592 - <rustc_errors[36d18b9907453f35]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7fb53e2d3fac - <rustc_errors[36d18b9907453f35]::json::JsonEmitter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  16:     0x7fb53e319cd8 - <rustc_errors[36d18b9907453f35]::HandlerInner>::emit_diagnostic
  17:     0x7fb53e3159c6 - <rustc_errors[36d18b9907453f35]::ErrorGuaranteed as rustc_errors[36d18b9907453f35]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7fb53d681533 - <rustc_session[8c18bc53086a9b34]::parse::ParseSess>::emit_err::<rustc_metadata[3d1c815352b1b4dd]::errors::CannotFindCrate>
  19:     0x7fb53d6c2507 - <rustc_metadata[3d1c815352b1b4dd]::locator::CrateError>::report
  20:     0x7fb53d703d97 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::resolve_crate
  21:     0x7fb53d70b434 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::process_extern_crate
  22:     0x7fb53c3c6188 - <rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[3242210473e1da7f]::visit::Visitor>::visit_item
  23:     0x7fb53c412afd - rustc_ast[3242210473e1da7f]::visit::walk_crate::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  24:     0x7fb53c3ab543 - <rustc_expand[b8ce2325e59f79ab]::expand::AstFragment>::visit_with::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  25:     0x7fb53c3ee5a0 - <rustc_resolve[252dc9306f4267df]::Resolver as rustc_expand[b8ce2325e59f79ab]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  26:     0x7fb53d8a1e92 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::collect_invocations
  27:     0x7fb53d89d5c2 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7fb53d89d227 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::expand_crate
  29:     0x7fb53b74df2e - <rustc_session[8c18bc53086a9b34]::session::Session>::time::<core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand::{closure#1}>
  30:     0x7fb53b7090d9 - rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand
  31:     0x7fb53b716fd6 - <rustc_interface[75bd1abe22f1fd46]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  32:     0x7fb53b6efe76 - <rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion
  33:     0x7fb53b5d4a2a - <rustc_interface[75bd1abe22f1fd46]::interface::Compiler>::enter::<rustc_driver[2b033e751948032]::run_compiler::{closure#1}::{closure#2}, core[2506165bb0faadb9]::result::Result<core[2506165bb0faadb9]::option::Option<rustc_interface[75bd1abe22f1fd46]::queries::Linker>, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  34:     0x7fb53b5b87c1 - rustc_span[4b37be1e64a2ccd5]::with_source_map::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7fb53b5f0571 - rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>
  36:     0x7fb53b5b0392 - <scoped_tls[e23be4e8d7e01d76]::ScopedKey<rustc_span[4b37be1e64a2ccd5]::SessionGlobals>>::set::<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  37:     0x7fb53b5bc0e9 - std[dbf3afd8bb1e5979]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  38:     0x7fb53b636cfe - std[dbf3afd8bb1e5979]::panicking::try::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, core[2506165bb0faadb9]::panic::unwind_safe::AssertUnwindSafe<<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  39:     0x7fb53b631e60 - <<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1} as core[2506165bb0faadb9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7fb53ac52b75 - std::sys::unix::thread::Thread::new::thread_start::ha85b465a65a1c6c4
  41:     0x7fb53a9f0b43 - <unknown>
  42:     0x7fb53aa82a00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4a60c720d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

---
4 LL | extern crate crateresolve2;
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |
-    = note: candidates:
-            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2/auxiliary/libcrateresolve2-1.rmeta
-            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2/auxiliary/libcrateresolve2-2.rmeta
-            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2/auxiliary/libcrateresolve2-3.rmeta
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2/crateresolve2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crate-loading/crateresolve2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/crateresolve2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0464]: multiple matching crates for `crateresolve2`
   |
LL | extern crate crateresolve2;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

---
4 LL | extern crate crateresolve1;
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |
-    = note: candidates:
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.somelib
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.somelib
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.somelib
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/crateresolve1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crate-loading/crateresolve1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/crateresolve1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0464]: multiple matching crates for `crateresolve1`
   |
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

---
diff of stderr:

2   --> $DIR/empty-linkname.rs:1:15
3    |
4 LL | #[link(name = "")]
-    |               ^^ empty link name
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args empty/empty-linkname.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-linkname.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-linkname" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-linkname/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0454]: link name must not be empty
   |
   |
LL | #[link(name = "")] //~ ERROR: link name must not be empty

error: aborting due to previous error

For more information about this error, try `rustc --explain E0454`.
---
diff of stderr:

2   --> $DIR/E0454.rs:1:15
3    |
4 LL | #[link(name = "")] extern "C" {}
-    |               ^^ empty link name
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args error-codes/E0454.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0454.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0454" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0454/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0454]: link name must not be empty
   |
   |
LL | #[link(name = "")] extern "C" {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0454`.
---
diff of stderr:

2   --> $DIR/E0458.rs:1:15
3    |
4 LL | #[link(kind = "wonderful_unicorn")] extern "C" {}
-    |               ^^^^^^^^^^^^^^^^^^^ unknown link kind
6 
6 
7 error[E0459]: `#[link]` attribute requires a `name = "string"` argument
8   --> $DIR/E0458.rs:1:1
9    |
9    |
10 LL | #[link(kind = "wonderful_unicorn")] extern "C" {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `name` argument
12 
13 error: aborting due to 2 previous errors
14 

---
To only update this specific test, also pass `--test-args error-codes/E0458.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0458.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0458" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0458/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0458]: unknown link kind `wonderful_unicorn`, expected one of: static, dylib, framework, raw-dylib
   |
   |
LL | #[link(kind = "wonderful_unicorn")] extern "C" {} //~ ERROR E0458


error[E0459]: `#[link]` attribute requires a `name = "string"` argument
   |
   |
LL | #[link(kind = "wonderful_unicorn")] extern "C" {} //~ ERROR E0458

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0458, E0459.
---
diff of stderr:

2   --> $DIR/E0459.rs:1:1
3    |
4 LL | #[link(kind = "dylib")] extern "C" {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^ missing `name` argument
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args error-codes/E0459.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0459.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0459" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0459/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0459]: `#[link]` attribute requires a `name = "string"` argument
   |
   |
LL | #[link(kind = "dylib")] extern "C" {} //~ ERROR E0459

error: aborting due to previous error

For more information about this error, try `rustc --explain E0459`.
For more information about this error, try `rustc --explain E0459`.
------------------------------------------


---- [ui] src/test/ui/error-codes/E0463.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0463.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0463" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0463/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "metadata_cannot_find_crate", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "crate_name" })), ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f93bd157b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   1:     0x7f93bd1c0508 - core::fmt::write::h622ccda3d7f48263
   1:     0x7f93bd1c0508 - core::fmt::write::h622ccda3d7f48263
   2:     0x7f93bd1487d1 - std::io::Write::write_fmt::hdd0bcc3d51a95b66
   3:     0x7f93bd15ab5e - std::panicking::default_hook::{{closure}}::hb63bf482bb996068
   4:     0x7f93bd15a827 - std::panicking::default_hook::h1c8d1247b1fca3f3
   5:     0x7f93bdae0c44 - rustc_driver[2b033e751948032]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f93bd15b311 - std::panicking::rust_panic_with_hook::h2015d0f8cb0a7e52
   7:     0x7f93bd15b137 - std::panicking::begin_panic_handler::{{closure}}::h92fcf298ed83eb87
   8:     0x7f93bd1580d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf233cf8fe3635627
   9:     0x7f93bd15ae02 - rust_begin_unwind
  10:     0x7f93bd10be43 - core::panicking::panic_fmt::hddf3504dfa1bb37a
  11:     0x7f93c07df377 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::translation::Translate>::translate_message
  12:     0x7f93c07d47c2 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f93c07d2266 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  14:     0x7f93c07ea592 - <rustc_errors[36d18b9907453f35]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f93c07e8fac - <rustc_errors[36d18b9907453f35]::json::JsonEmitter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  16:     0x7f93c082ecd8 - <rustc_errors[36d18b9907453f35]::HandlerInner>::emit_diagnostic
  17:     0x7f93c082a9c6 - <rustc_errors[36d18b9907453f35]::ErrorGuaranteed as rustc_errors[36d18b9907453f35]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f93bfb96533 - <rustc_session[8c18bc53086a9b34]::parse::ParseSess>::emit_err::<rustc_metadata[3d1c815352b1b4dd]::errors::CannotFindCrate>
  19:     0x7f93bfbd7507 - <rustc_metadata[3d1c815352b1b4dd]::locator::CrateError>::report
  20:     0x7f93bfbd4cf2 - rustc_metadata[3d1c815352b1b4dd]::locator::find_plugin_registrar
  21:     0x7f93bfb54eb1 - rustc_plugin_impl[9ff5ef315afc8d3f]::load::load_plugins
  22:     0x7f93bdc622f1 - <rustc_session[8c18bc53086a9b34]::session::Session>::time::<alloc[19247fcd3ce2598c]::vec::Vec<for<'a, 'b> fn(&'a mut rustc_plugin_impl[9ff5ef315afc8d3f]::Registry<'b>)>, rustc_interface[75bd1abe22f1fd46]::passes::register_plugins<&dyn for<'a, 'b> core[2506165bb0faadb9]::ops::function::Fn<(&'a rustc_session[8c18bc53086a9b34]::session::Session, &'b mut rustc_lint[9c867c229540cb09]::context::LintStore), Output = ()> + core[2506165bb0faadb9]::marker::Send + core[2506165bb0faadb9]::marker::Sync>::{closure#2}>
  23:     0x7f93bdc1dbe1 - rustc_interface[75bd1abe22f1fd46]::passes::register_plugins::<&dyn for<'a, 'b> core[2506165bb0faadb9]::ops::function::Fn<(&'a rustc_session[8c18bc53086a9b34]::session::Session, &'b mut rustc_lint[9c867c229540cb09]::context::LintStore), Output = ()> + core[2506165bb0faadb9]::marker::Send + core[2506165bb0faadb9]::marker::Sync>
  24:     0x7f93bdc04457 - <rustc_interface[75bd1abe22f1fd46]::queries::Queries>::register_plugins
  25:     0x7f93bdae9809 - <rustc_interface[75bd1abe22f1fd46]::interface::Compiler>::enter::<rustc_driver[2b033e751948032]::run_compiler::{closure#1}::{closure#2}, core[2506165bb0faadb9]::result::Result<core[2506165bb0faadb9]::option::Option<rustc_interface[75bd1abe22f1fd46]::queries::Linker>, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  26:     0x7f93bdacd7c1 - rustc_span[4b37be1e64a2ccd5]::with_source_map::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#1}>
  27:     0x7f93bdb05571 - rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>
  28:     0x7f93bdac5392 - <scoped_tls[e23be4e8d7e01d76]::ScopedKey<rustc_span[4b37be1e64a2ccd5]::SessionGlobals>>::set::<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  29:     0x7f93bdad10e9 - std[dbf3afd8bb1e5979]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  30:     0x7f93bdb4bcfe - std[dbf3afd8bb1e5979]::panicking::try::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, core[2506165bb0faadb9]::panic::unwind_safe::AssertUnwindSafe<<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  31:     0x7f93bdb46e60 - <<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1} as core[2506165bb0faadb9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f93bd167b75 - std::sys::unix::thread::Thread::new::thread_start::ha85b465a65a1c6c4
  33:     0x7f93bcf05b43 - <unknown>
  34:     0x7f93bcf97a00 - <unknown>
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4a60c720d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

---
4 LL | extern crate crateresolve1;
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |
-    = note: candidates:
-            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-1.somelib
-            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-2.somelib
-            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-3.somelib
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/E0464.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0464.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0464.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0464]: multiple matching crates for `crateresolve1`
   |
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

---


---- [ui] src/test/ui/extern/extern-crate-multiple-missing.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/extern-crate-multiple-missing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-crate-multiple-missing" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-crate-multiple-missing/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "metadata_cannot_find_crate", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "crate_name" })), ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f37c8ec2b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   0:     0x7f37c8ec2b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   1:     0x7f37c8f2b508 - core::fmt::write::h622ccda3d7f48263
   2:     0x7f37c8eb37d1 - std::io::Write::write_fmt::hdd0bcc3d51a95b66
   3:     0x7f37c8ec5b5e - std::panicking::default_hook::{{closure}}::hb63bf482bb996068
   4:     0x7f37c8ec5827 - std::panicking::default_hook::h1c8d1247b1fca3f3
   5:     0x7f37c984bc44 - rustc_driver[2b033e751948032]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f37c8ec6311 - std::panicking::rust_panic_with_hook::h2015d0f8cb0a7e52
   7:     0x7f37c8ec6137 - std::panicking::begin_panic_handler::{{closure}}::h92fcf298ed83eb87
   8:     0x7f37c8ec30d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf233cf8fe3635627
   9:     0x7f37c8ec5e02 - rust_begin_unwind
  10:     0x7f37c8e76e43 - core::panicking::panic_fmt::hddf3504dfa1bb37a
  11:     0x7f37cc54a377 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::translation::Translate>::translate_message
  12:     0x7f37cc53f7c2 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f37cc53d266 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  14:     0x7f37cc555592 - <rustc_errors[36d18b9907453f35]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f37cc553fac - <rustc_errors[36d18b9907453f35]::json::JsonEmitter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  16:     0x7f37cc599cd8 - <rustc_errors[36d18b9907453f35]::HandlerInner>::emit_diagnostic
  17:     0x7f37cc5959c6 - <rustc_errors[36d18b9907453f35]::ErrorGuaranteed as rustc_errors[36d18b9907453f35]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f37cb901533 - <rustc_session[8c18bc53086a9b34]::parse::ParseSess>::emit_err::<rustc_metadata[3d1c815352b1b4dd]::errors::CannotFindCrate>
  19:     0x7f37cb942507 - <rustc_metadata[3d1c815352b1b4dd]::locator::CrateError>::report
  20:     0x7f37cb983d97 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::resolve_crate
  21:     0x7f37cb98b434 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::process_extern_crate
  22:     0x7f37ca646188 - <rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[3242210473e1da7f]::visit::Visitor>::visit_item
  23:     0x7f37ca692afd - rustc_ast[3242210473e1da7f]::visit::walk_crate::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  24:     0x7f37ca62b543 - <rustc_expand[b8ce2325e59f79ab]::expand::AstFragment>::visit_with::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  25:     0x7f37ca66e5a0 - <rustc_resolve[252dc9306f4267df]::Resolver as rustc_expand[b8ce2325e59f79ab]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  26:     0x7f37cbb21e92 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::collect_invocations
  27:     0x7f37cbb1d5c2 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7f37cbb1d227 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::expand_crate
  29:     0x7f37c99cdf2e - <rustc_session[8c18bc53086a9b34]::session::Session>::time::<core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand::{closure#1}>
  30:     0x7f37c99890d9 - rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand
  31:     0x7f37c9996fd6 - <rustc_interface[75bd1abe22f1fd46]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  32:     0x7f37c996fe76 - <rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion
  33:     0x7f37c9854a2a - <rustc_interface[75bd1abe22f1fd46]::interface::Compiler>::enter::<rustc_driver[2b033e751948032]::run_compiler::{closure#1}::{closure#2}, core[2506165bb0faadb9]::result::Result<core[2506165bb0faadb9]::option::Option<rustc_interface[75bd1abe22f1fd46]::queries::Linker>, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  34:     0x7f37c98387c1 - rustc_span[4b37be1e64a2ccd5]::with_source_map::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7f37c9870571 - rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>
  36:     0x7f37c9830392 - <scoped_tls[e23be4e8d7e01d76]::ScopedKey<rustc_span[4b37be1e64a2ccd5]::SessionGlobals>>::set::<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  37:     0x7f37c983c0e9 - std[dbf3afd8bb1e5979]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  38:     0x7f37c98b6cfe - std[dbf3afd8bb1e5979]::panicking::try::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, core[2506165bb0faadb9]::panic::unwind_safe::AssertUnwindSafe<<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  39:     0x7f37c98b1e60 - <<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1} as core[2506165bb0faadb9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f37c8ed2b75 - std::sys::unix::thread::Thread::new::thread_start::ha85b465a65a1c6c4
  41:     0x7f37c8c70b43 - <unknown>
  42:     0x7f37c8d02a00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4a60c720d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/issues/issue-37131.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37131.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37131" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=thumbv6m-none-eabi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37131/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "metadata_cannot_find_crate", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "crate_name" })), ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f6ed9063b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   1:     0x7f6ed90cc508 - core::fmt::write::h622ccda3d7f48263
   1:     0x7f6ed90cc508 - core::fmt::write::h622ccda3d7f48263
   2:     0x7f6ed90547d1 - std::io::Write::write_fmt::hdd0bcc3d51a95b66
   3:     0x7f6ed9066b5e - std::panicking::default_hook::{{closure}}::hb63bf482bb996068
   4:     0x7f6ed9066827 - std::panicking::default_hook::h1c8d1247b1fca3f3
   5:     0x7f6ed99ecc44 - rustc_driver[2b033e751948032]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6ed9067311 - std::panicking::rust_panic_with_hook::h2015d0f8cb0a7e52
   7:     0x7f6ed9067137 - std::panicking::begin_panic_handler::{{closure}}::h92fcf298ed83eb87
   8:     0x7f6ed90640d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf233cf8fe3635627
   9:     0x7f6ed9066e02 - rust_begin_unwind
  10:     0x7f6ed9017e43 - core::panicking::panic_fmt::hddf3504dfa1bb37a
  11:     0x7f6edc6eb377 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::translation::Translate>::translate_message
  12:     0x7f6edc6e07c2 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f6edc6de266 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  14:     0x7f6edc6f6592 - <rustc_errors[36d18b9907453f35]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f6edc6f4fac - <rustc_errors[36d18b9907453f35]::json::JsonEmitter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  16:     0x7f6edc73acd8 - <rustc_errors[36d18b9907453f35]::HandlerInner>::emit_diagnostic
  17:     0x7f6edc7369c6 - <rustc_errors[36d18b9907453f35]::ErrorGuaranteed as rustc_errors[36d18b9907453f35]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f6edbaa2533 - <rustc_session[8c18bc53086a9b34]::parse::ParseSess>::emit_err::<rustc_metadata[3d1c815352b1b4dd]::errors::CannotFindCrate>
  19:     0x7f6edbae3507 - <rustc_metadata[3d1c815352b1b4dd]::locator::CrateError>::report
  20:     0x7f6edbb24d97 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::resolve_crate
  21:     0x7f6edbb2c434 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::process_extern_crate
  22:     0x7f6eda7e7188 - <rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[3242210473e1da7f]::visit::Visitor>::visit_item
  23:     0x7f6eda833afd - rustc_ast[3242210473e1da7f]::visit::walk_crate::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  24:     0x7f6eda7cc543 - <rustc_expand[b8ce2325e59f79ab]::expand::AstFragment>::visit_with::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  25:     0x7f6eda80f5a0 - <rustc_resolve[252dc9306f4267df]::Resolver as rustc_expand[b8ce2325e59f79ab]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  26:     0x7f6edbcc2e92 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::collect_invocations
  27:     0x7f6edbcbe5c2 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7f6edbcbe227 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::expand_crate
  29:     0x7f6ed9b6ef2e - <rustc_session[8c18bc53086a9b34]::session::Session>::time::<core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand::{closure#1}>
  30:     0x7f6ed9b2a0d9 - rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand
  31:     0x7f6ed9b37fd6 - <rustc_interface[75bd1abe22f1fd46]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  32:     0x7f6ed9b10e76 - <rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion
  33:     0x7f6ed99f5a2a - <rustc_interface[75bd1abe22f1fd46]::interface::Compiler>::enter::<rustc_driver[2b033e751948032]::run_compiler::{closure#1}::{closure#2}, core[2506165bb0faadb9]::result::Result<core[2506165bb0faadb9]::option::Option<rustc_interface[75bd1abe22f1fd46]::queries::Linker>, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  34:     0x7f6ed99d97c1 - rustc_span[4b37be1e64a2ccd5]::with_source_map::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7f6ed9a11571 - rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>
  36:     0x7f6ed99d1392 - <scoped_tls[e23be4e8d7e01d76]::ScopedKey<rustc_span[4b37be1e64a2ccd5]::SessionGlobals>>::set::<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  37:     0x7f6ed99dd0e9 - std[dbf3afd8bb1e5979]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  38:     0x7f6ed9a57cfe - std[dbf3afd8bb1e5979]::panicking::try::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, core[2506165bb0faadb9]::panic::unwind_safe::AssertUnwindSafe<<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  39:     0x7f6ed9a52e60 - <<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1} as core[2506165bb0faadb9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f6ed9073b75 - std::sys::unix::thread::Thread::new::thread_start::ha85b465a65a1c6c4
  41:     0x7f6ed8e11b43 - <unknown>
  42:     0x7f6ed8ea3a00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4a60c720d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/issues/issue-49851/compiler-builtins-error.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-49851/compiler-builtins-error.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49851/compiler-builtins-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv7em-none-eabihf" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49851/compiler-builtins-error/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "metadata_cannot_find_crate", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "crate_name" })), ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f4c7a58bb5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   1:     0x7f4c7a5f4508 - core::fmt::write::h622ccda3d7f48263
   1:     0x7f4c7a5f4508 - core::fmt::write::h622ccda3d7f48263
   2:     0x7f4c7a57c7d1 - std::io::Write::write_fmt::hdd0bcc3d51a95b66
   3:     0x7f4c7a58eb5e - std::panicking::default_hook::{{closure}}::hb63bf482bb996068
   4:     0x7f4c7a58e827 - std::panicking::default_hook::h1c8d1247b1fca3f3
   5:     0x7f4c7af14c44 - rustc_driver[2b033e751948032]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4c7a58f311 - std::panicking::rust_panic_with_hook::h2015d0f8cb0a7e52
   7:     0x7f4c7a58f137 - std::panicking::begin_panic_handler::{{closure}}::h92fcf298ed83eb87
   8:     0x7f4c7a58c0d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf233cf8fe3635627
   9:     0x7f4c7a58ee02 - rust_begin_unwind
  10:     0x7f4c7a53fe43 - core::panicking::panic_fmt::hddf3504dfa1bb37a
  11:     0x7f4c7dc13377 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::translation::Translate>::translate_message
  12:     0x7f4c7dc087c2 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f4c7dc06266 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  14:     0x7f4c7dc1e592 - <rustc_errors[36d18b9907453f35]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f4c7dc1cfac - <rustc_errors[36d18b9907453f35]::json::JsonEmitter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  16:     0x7f4c7dc62cd8 - <rustc_errors[36d18b9907453f35]::HandlerInner>::emit_diagnostic
  17:     0x7f4c7dc5e9c6 - <rustc_errors[36d18b9907453f35]::ErrorGuaranteed as rustc_errors[36d18b9907453f35]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f4c7cfca533 - <rustc_session[8c18bc53086a9b34]::parse::ParseSess>::emit_err::<rustc_metadata[3d1c815352b1b4dd]::errors::CannotFindCrate>
  19:     0x7f4c7d00b507 - <rustc_metadata[3d1c815352b1b4dd]::locator::CrateError>::report
  20:     0x7f4c7d04cd97 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::resolve_crate
  21:     0x7f4c7d054434 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::process_extern_crate
  22:     0x7f4c7bd0f188 - <rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[3242210473e1da7f]::visit::Visitor>::visit_item
  23:     0x7f4c7bd5bafd - rustc_ast[3242210473e1da7f]::visit::walk_crate::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  24:     0x7f4c7bcf4543 - <rustc_expand[b8ce2325e59f79ab]::expand::AstFragment>::visit_with::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  25:     0x7f4c7bd375a0 - <rustc_resolve[252dc9306f4267df]::Resolver as rustc_expand[b8ce2325e59f79ab]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  26:     0x7f4c7d1eae92 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::collect_invocations
  27:     0x7f4c7d1e65c2 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7f4c7d1e6227 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::expand_crate
  29:     0x7f4c7b096f2e - <rustc_session[8c18bc53086a9b34]::session::Session>::time::<core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand::{closure#1}>
  30:     0x7f4c7b0520d9 - rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand
  31:     0x7f4c7b05ffd6 - <rustc_interface[75bd1abe22f1fd46]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  32:     0x7f4c7b038e76 - <rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion
  33:     0x7f4c7af1da2a - <rustc_interface[75bd1abe22f1fd46]::interface::Compiler>::enter::<rustc_driver[2b033e751948032]::run_compiler::{closure#1}::{closure#2}, core[2506165bb0faadb9]::result::Result<core[2506165bb0faadb9]::option::Option<rustc_interface[75bd1abe22f1fd46]::queries::Linker>, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  34:     0x7f4c7af017c1 - rustc_span[4b37be1e64a2ccd5]::with_source_map::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7f4c7af39571 - rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>
  36:     0x7f4c7aef9392 - <scoped_tls[e23be4e8d7e01d76]::ScopedKey<rustc_span[4b37be1e64a2ccd5]::SessionGlobals>>::set::<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  37:     0x7f4c7af050e9 - std[dbf3afd8bb1e5979]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  38:     0x7f4c7af7fcfe - std[dbf3afd8bb1e5979]::panicking::try::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, core[2506165bb0faadb9]::panic::unwind_safe::AssertUnwindSafe<<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  39:     0x7f4c7af7ae60 - <<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1} as core[2506165bb0faadb9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f4c7a59bb75 - std::sys::unix::thread::Thread::new::thread_start::ha85b465a65a1c6c4
  41:     0x7f4c7a339b43 - <unknown>
  42:     0x7f4c7a3cba00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4a60c720d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

---
52 LL | #[link(name)]
-    | ^^^^^^^^^^^^^ missing `name` argument
+    | ^^^^^^^^^^^^^
54 
55 error: link name must be of the form `name = "string"`

62   --> $DIR/link-attr-validation-late.rs:20:1
63    |
64 LL | #[link(name())]
64 LL | #[link(name())]
-    | ^^^^^^^^^^^^^^^ missing `name` argument
+    | ^^^^^^^^^^^^^^^
66 
67 error: link kind must be of the form `kind = "string"`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/link-attr-validation-late/link-attr-validation-late.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/link-attr-validation-late/link-attr-validation-late.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args linkage-attr/link-attr-validation-late.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/linkage-attr/link-attr-validation-late.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/link-attr-validation-late" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/link-attr-validation-late/auxiliary"
stdout: none
--- stderr -------------------------------
error: unexpected `#[link]` argument, expected one of: name, kind, modifiers, cfg, wasm_import_module
   |
   |
LL | #[link(name = "...", "literal")] //~ ERROR unexpected `#[link]` argument


error: unexpected `#[link]` argument, expected one of: name, kind, modifiers, cfg, wasm_import_module
   |
   |
LL | #[link(name = "...", unknown)] //~ ERROR unexpected `#[link]` argument


error: multiple `name` arguments in a single `#[link]` attribute
   |
   |
LL | #[link(name = "foo", name = "bar")] //~ ERROR multiple `name` arguments


error: multiple `kind` arguments in a single `#[link]` attribute
   |
   |
LL | #[link(name = "...", kind = "dylib", kind = "bar")] //~ ERROR multiple `kind` arguments


error: multiple `modifiers` arguments in a single `#[link]` attribute
   |
   |
LL | #[link(name = "...", modifiers = "+verbatim", modifiers = "bar")] //~ ERROR multiple `modifiers` arguments


error: multiple `cfg` arguments in a single `#[link]` attribute
   |
   |
LL | #[link(name = "...", cfg(FALSE), cfg(FALSE))] //~ ERROR multiple `cfg` arguments


error: multiple `wasm_import_module` arguments in a single `#[link]` attribute
   |
   |
LL | #[link(wasm_import_module = "foo", wasm_import_module = "bar")] //~ ERROR multiple `wasm_import_module` arguments


error: link name must be of the form `name = "string"`
   |
   |
LL | #[link(name)] //~ ERROR link name must be of the form `name = "string"`


error[E0459]: `#[link]` attribute requires a `name = "string"` argument
   |
   |
LL | #[link(name)] //~ ERROR link name must be of the form `name = "string"`


error: link name must be of the form `name = "string"`
   |
   |
LL | #[link(name())] //~ ERROR link name must be of the form `name = "string"`


error[E0459]: `#[link]` attribute requires a `name = "string"` argument
   |
   |
LL | #[link(name())] //~ ERROR link name must be of the form `name = "string"`


error: link kind must be of the form `kind = "string"`
   |
   |
LL | #[link(name = "...", kind)] //~ ERROR link kind must be of the form `kind = "string"`


error: link kind must be of the form `kind = "string"`
   |
   |
LL | #[link(name = "...", kind())] //~ ERROR link kind must be of the form `kind = "string"`


error: link modifiers must be of the form `modifiers = "string"`
   |
   |
LL | #[link(name = "...", modifiers)] //~ ERROR link modifiers must be of the form `modifiers = "string"`


error: link modifiers must be of the form `modifiers = "string"`
   |
   |
LL | #[link(name = "...", modifiers())] //~ ERROR link modifiers must be of the form `modifiers = "string"`


error: link cfg must be of the form `cfg(/* predicate */)`
   |
   |
LL | #[link(name = "...", cfg)] //~ ERROR link cfg must be of the form `cfg(/* predicate */)`


error: link cfg must be of the form `cfg(/* predicate */)`
   |
   |
LL | #[link(name = "...", cfg = "literal")] //~ ERROR link cfg must be of the form `cfg(/* predicate */)`


error: link cfg must have a single predicate argument
   |
   |
LL | #[link(name = "...", cfg("literal"))] //~ ERROR link cfg must have a single predicate argument


error: wasm import module must be of the form `wasm_import_module = "string"`
   |
   |
LL | #[link(name = "...", wasm_import_module)] //~ ERROR wasm import module must be of the form `wasm_import_module = "string"`


error: wasm import module must be of the form `wasm_import_module = "string"`
   |
   |
LL | #[link(name = "...", wasm_import_module())] //~ ERROR wasm import module must be of the form `wasm_import_module = "string"`


error: invalid linking modifier syntax, expected '+' or '-' prefix before one of: bundle, verbatim, whole-archive, as-needed
   |
   |
LL | #[link(name = "...", modifiers = "")] //~ ERROR invalid linking modifier syntax, expected '+' or '-' prefix


error: invalid linking modifier syntax, expected '+' or '-' prefix before one of: bundle, verbatim, whole-archive, as-needed
   |
   |
LL | #[link(name = "...", modifiers = "no-plus-minus")] //~ ERROR invalid linking modifier syntax, expected '+' or '-' prefix


error: unknown linking modifier `unknown`, expected one of: bundle, verbatim, whole-archive, as-needed
   |
   |
LL | #[link(name = "...", modifiers = "+unknown")] //~ ERROR unknown linking modifier `unknown`


error: multiple `verbatim` modifiers in a single `modifiers` argument
   |
   |
LL | #[link(name = "...", modifiers = "+verbatim,+verbatim")] //~ ERROR multiple `verbatim` modifiers

error: aborting due to 24 previous errors

For more information about this error, try `rustc --explain E0459`.
---
diff of stderr:

2   --> $DIR/link-arg-from-rs.rs:6:15
3    |
4 LL | #[link(kind = "link-arg")]
-    |               ^^^^^^^^^^ unknown link kind
6 
6 
7 error[E0459]: `#[link]` attribute requires a `name = "string"` argument

9    |
9    |
10 LL | #[link(kind = "link-arg")]
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `name` argument
12 
13 error: aborting due to 2 previous errors
14 

---
To only update this specific test, also pass `--test-args native-library-link-flags/link-arg-from-rs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/native-library-link-flags/link-arg-from-rs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/link-arg-from-rs" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/link-arg-from-rs/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0458]: unknown link kind `link-arg`, expected one of: static, dylib, framework, raw-dylib
   |
   |
LL | #[link(kind = "link-arg")]


error[E0459]: `#[link]` attribute requires a `name = "string"` argument
   |
   |
LL | #[link(kind = "link-arg")]

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0458, E0459.
Some errors have detailed explanations: E0458, E0459.
For more information about an error, try `rustc --explain E0458`.
------------------------------------------


---- [ui] src/test/ui/no-link-unknown-crate.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-link-unknown-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-link-unknown-crate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-link-unknown-crate/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "metadata_cannot_find_crate", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "crate_name" })), ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f80cc2b6b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   0:     0x7f80cc2b6b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   1:     0x7f80cc31f508 - core::fmt::write::h622ccda3d7f48263
   2:     0x7f80cc2a77d1 - std::io::Write::write_fmt::hdd0bcc3d51a95b66
   3:     0x7f80cc2b9b5e - std::panicking::default_hook::{{closure}}::hb63bf482bb996068
   4:     0x7f80cc2b9827 - std::panicking::default_hook::h1c8d1247b1fca3f3
   5:     0x7f80ccc3fc44 - rustc_driver[2b033e751948032]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f80cc2ba311 - std::panicking::rust_panic_with_hook::h2015d0f8cb0a7e52
   7:     0x7f80cc2ba137 - std::panicking::begin_panic_handler::{{closure}}::h92fcf298ed83eb87
   8:     0x7f80cc2b70d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf233cf8fe3635627
   9:     0x7f80cc2b9e02 - rust_begin_unwind
  10:     0x7f80cc26ae43 - core::panicking::panic_fmt::hddf3504dfa1bb37a
  11:     0x7f80cf93e377 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::translation::Translate>::translate_message
  12:     0x7f80cf9337c2 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f80cf931266 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  14:     0x7f80cf949592 - <rustc_errors[36d18b9907453f35]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f80cf947fac - <rustc_errors[36d18b9907453f35]::json::JsonEmitter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  16:     0x7f80cf98dcd8 - <rustc_errors[36d18b9907453f35]::HandlerInner>::emit_diagnostic
  17:     0x7f80cf9899c6 - <rustc_errors[36d18b9907453f35]::ErrorGuaranteed as rustc_errors[36d18b9907453f35]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f80cecf5533 - <rustc_session[8c18bc53086a9b34]::parse::ParseSess>::emit_err::<rustc_metadata[3d1c815352b1b4dd]::errors::CannotFindCrate>
  19:     0x7f80ced36507 - <rustc_metadata[3d1c815352b1b4dd]::locator::CrateError>::report
  20:     0x7f80ced77d97 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::resolve_crate
  21:     0x7f80ced7f434 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::process_extern_crate
  22:     0x7f80cda3a188 - <rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[3242210473e1da7f]::visit::Visitor>::visit_item
  23:     0x7f80cda86afd - rustc_ast[3242210473e1da7f]::visit::walk_crate::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  24:     0x7f80cda1f543 - <rustc_expand[b8ce2325e59f79ab]::expand::AstFragment>::visit_with::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  25:     0x7f80cda625a0 - <rustc_resolve[252dc9306f4267df]::Resolver as rustc_expand[b8ce2325e59f79ab]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  26:     0x7f80cef15e92 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::collect_invocations
  27:     0x7f80cef115c2 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7f80cef11227 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::expand_crate
  29:     0x7f80ccdc1f2e - <rustc_session[8c18bc53086a9b34]::session::Session>::time::<core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand::{closure#1}>
  30:     0x7f80ccd7d0d9 - rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand
  31:     0x7f80ccd8afd6 - <rustc_interface[75bd1abe22f1fd46]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  32:     0x7f80ccd63e76 - <rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion
  33:     0x7f80ccc48a2a - <rustc_interface[75bd1abe22f1fd46]::interface::Compiler>::enter::<rustc_driver[2b033e751948032]::run_compiler::{closure#1}::{closure#2}, core[2506165bb0faadb9]::result::Result<core[2506165bb0faadb9]::option::Option<rustc_interface[75bd1abe22f1fd46]::queries::Linker>, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  34:     0x7f80ccc2c7c1 - rustc_span[4b37be1e64a2ccd5]::with_source_map::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7f80ccc64571 - rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>
  36:     0x7f80ccc24392 - <scoped_tls[e23be4e8d7e01d76]::ScopedKey<rustc_span[4b37be1e64a2ccd5]::SessionGlobals>>::set::<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  37:     0x7f80ccc300e9 - std[dbf3afd8bb1e5979]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  38:     0x7f80cccaacfe - std[dbf3afd8bb1e5979]::panicking::try::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, core[2506165bb0faadb9]::panic::unwind_safe::AssertUnwindSafe<<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  39:     0x7f80ccca5e60 - <<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1} as core[2506165bb0faadb9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f80cc2c6b75 - std::sys::unix::thread::Thread::new::thread_start::ha85b465a65a1c6c4
  41:     0x7f80cc064b43 - <unknown>
  42:     0x7f80cc0f6a00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4a60c720d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/parser/bad-crate-name.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/bad-crate-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/bad-crate-name" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/bad-crate-name/auxiliary"
stdout: none
--- stderr -------------------------------
error: crate name using dashes are not valid in `extern crate` statements
   |
   |
LL | extern crate krate-name-here;
   |              ^^^^^^^^^^^^^^^ dash-separated idents are not valid
   |
help: if the original crate name uses dashes you need to use underscores in the code
   |
LL | extern crate krate_name_here;


thread 'rustc' panicked at 'identifier: "metadata_cannot_find_crate", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "crate_name" })), ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f3194b1db5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   1:     0x7f3194b86508 - core::fmt::write::h622ccda3d7f48263
   1:     0x7f3194b86508 - core::fmt::write::h622ccda3d7f48263
   2:     0x7f3194b0e7d1 - std::io::Write::write_fmt::hdd0bcc3d51a95b66
   3:     0x7f3194b20b5e - std::panicking::default_hook::{{closure}}::hb63bf482bb996068
   4:     0x7f3194b20827 - std::panicking::default_hook::h1c8d1247b1fca3f3
   5:     0x7f31954a6c44 - rustc_driver[2b033e751948032]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f3194b21311 - std::panicking::rust_panic_with_hook::h2015d0f8cb0a7e52
   7:     0x7f3194b21137 - std::panicking::begin_panic_handler::{{closure}}::h92fcf298ed83eb87
   8:     0x7f3194b1e0d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf233cf8fe3635627
   9:     0x7f3194b20e02 - rust_begin_unwind
  10:     0x7f3194ad1e43 - core::panicking::panic_fmt::hddf3504dfa1bb37a
  11:     0x7f31981a5377 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::translation::Translate>::translate_message
  12:     0x7f319819a7c2 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f3198198266 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  14:     0x7f31981b0592 - <rustc_errors[36d18b9907453f35]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f31981aefac - <rustc_errors[36d18b9907453f35]::json::JsonEmitter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  16:     0x7f31981f4cd8 - <rustc_errors[36d18b9907453f35]::HandlerInner>::emit_diagnostic
  17:     0x7f31981f09c6 - <rustc_errors[36d18b9907453f35]::ErrorGuaranteed as rustc_errors[36d18b9907453f35]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f319755c533 - <rustc_session[8c18bc53086a9b34]::parse::ParseSess>::emit_err::<rustc_metadata[3d1c815352b1b4dd]::errors::CannotFindCrate>
  19:     0x7f319759d507 - <rustc_metadata[3d1c815352b1b4dd]::locator::CrateError>::report
  20:     0x7f31975ded97 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::resolve_crate
  21:     0x7f31975e6434 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::process_extern_crate
  22:     0x7f31962a1188 - <rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[3242210473e1da7f]::visit::Visitor>::visit_item
  23:     0x7f31962edafd - rustc_ast[3242210473e1da7f]::visit::walk_crate::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  24:     0x7f3196286543 - <rustc_expand[b8ce2325e59f79ab]::expand::AstFragment>::visit_with::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  25:     0x7f31962c95a0 - <rustc_resolve[252dc9306f4267df]::Resolver as rustc_expand[b8ce2325e59f79ab]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  26:     0x7f319777ce92 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::collect_invocations
  27:     0x7f31977785c2 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7f3197778227 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::expand_crate
  29:     0x7f3195628f2e - <rustc_session[8c18bc53086a9b34]::session::Session>::time::<core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand::{closure#1}>
  30:     0x7f31955e40d9 - rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand
  31:     0x7f31955f1fd6 - <rustc_interface[75bd1abe22f1fd46]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  32:     0x7f31955cae76 - <rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion
  33:     0x7f31954afa2a - <rustc_interface[75bd1abe22f1fd46]::interface::Compiler>::enter::<rustc_driver[2b033e751948032]::run_compiler::{closure#1}::{closure#2}, core[2506165bb0faadb9]::result::Result<core[2506165bb0faadb9]::option::Option<rustc_interface[75bd1abe22f1fd46]::queries::Linker>, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  34:     0x7f31954937c1 - rustc_span[4b37be1e64a2ccd5]::with_source_map::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7f31954cb571 - rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>
  36:     0x7f319548b392 - <scoped_tls[e23be4e8d7e01d76]::ScopedKey<rustc_span[4b37be1e64a2ccd5]::SessionGlobals>>::set::<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  37:     0x7f31954970e9 - std[dbf3afd8bb1e5979]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  38:     0x7f3195511cfe - std[dbf3afd8bb1e5979]::panicking::try::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, core[2506165bb0faadb9]::panic::unwind_safe::AssertUnwindSafe<<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  39:     0x7f319550ce60 - <<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1} as core[2506165bb0faadb9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f3194b2db75 - std::sys::unix::thread::Thread::new::thread_start::ha85b465a65a1c6c4
  41:     0x7f31948cbb43 - <unknown>
  42:     0x7f319495da00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4a60c720d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error


For more information about this error, try `rustc --explain E0463`.
------------------------------------------


---- [ui] src/test/ui/rust-2018/uniform-paths/deadlock.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/deadlock.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/deadlock" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "foo" "--extern" "bar" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/deadlock/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "metadata_cannot_find_crate", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "crate_name" })), ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f1b26f5cb5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   1:     0x7f1b26fc5508 - core::fmt::write::h622ccda3d7f48263
   1:     0x7f1b26fc5508 - core::fmt::write::h622ccda3d7f48263
   2:     0x7f1b26f4d7d1 - std::io::Write::write_fmt::hdd0bcc3d51a95b66
   3:     0x7f1b26f5fb5e - std::panicking::default_hook::{{closure}}::hb63bf482bb996068
   4:     0x7f1b26f5f827 - std::panicking::default_hook::h1c8d1247b1fca3f3
   5:     0x7f1b278e5c44 - rustc_driver[2b033e751948032]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1b26f60311 - std::panicking::rust_panic_with_hook::h2015d0f8cb0a7e52
   7:     0x7f1b26f60137 - std::panicking::begin_panic_handler::{{closure}}::h92fcf298ed83eb87
   8:     0x7f1b26f5d0d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf233cf8fe3635627
   9:     0x7f1b26f5fe02 - rust_begin_unwind
  10:     0x7f1b26f10e43 - core::panicking::panic_fmt::hddf3504dfa1bb37a
  11:     0x7f1b2a5e4377 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::translation::Translate>::translate_message
  12:     0x7f1b2a5d97c2 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f1b2a5d7266 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  14:     0x7f1b2a5ef592 - <rustc_errors[36d18b9907453f35]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f1b2a5edfac - <rustc_errors[36d18b9907453f35]::json::JsonEmitter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  16:     0x7f1b2a633cd8 - <rustc_errors[36d18b9907453f35]::HandlerInner>::emit_diagnostic
  17:     0x7f1b2a62f9c6 - <rustc_errors[36d18b9907453f35]::ErrorGuaranteed as rustc_errors[36d18b9907453f35]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f1b2999b533 - <rustc_session[8c18bc53086a9b34]::parse::ParseSess>::emit_err::<rustc_metadata[3d1c815352b1b4dd]::errors::CannotFindCrate>
  19:     0x7f1b299dc507 - <rustc_metadata[3d1c815352b1b4dd]::locator::CrateError>::report
  20:     0x7f1b29a1dd97 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::resolve_crate
  21:     0x7f1b29a25728 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::process_path_extern
  22:     0x7f1b2871ae9a - <rustc_resolve[252dc9306f4267df]::Resolver>::extern_prelude_get
  23:     0x7f1b2872082f - <rustc_resolve[252dc9306f4267df]::Resolver>::early_resolve_ident_in_lexical_scope
  24:     0x7f1b28704451 - <rustc_resolve[252dc9306f4267df]::Resolver>::resolve_path_with_ribs
  25:     0x7f1b28726350 - <rustc_resolve[252dc9306f4267df]::Resolver>::resolve_path
  26:     0x7f1b286a59c9 - <rustc_resolve[252dc9306f4267df]::imports::ImportResolver>::finalize_import
  27:     0x7f1b286a4496 - <rustc_resolve[252dc9306f4267df]::imports::ImportResolver>::finalize_imports
  28:     0x7f1b28784a19 - <rustc_session[8c18bc53086a9b34]::session::Session>::time::<(), <rustc_resolve[252dc9306f4267df]::Resolver>::resolve_crate::{closure#0}>
  29:     0x7f1b27a23414 - rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand
  30:     0x7f1b27a30fd6 - <rustc_interface[75bd1abe22f1fd46]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  31:     0x7f1b27a09e76 - <rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion
  32:     0x7f1b278eea2a - <rustc_interface[75bd1abe22f1fd46]::interface::Compiler>::enter::<rustc_driver[2b033e751948032]::run_compiler::{closure#1}::{closure#2}, core[2506165bb0faadb9]::result::Result<core[2506165bb0faadb9]::option::Option<rustc_interface[75bd1abe22f1fd46]::queries::Linker>, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  33:     0x7f1b278d27c1 - rustc_span[4b37be1e64a2ccd5]::with_source_map::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#1}>
  34:     0x7f1b2790a571 - rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>
  35:     0x7f1b278ca392 - <scoped_tls[e23be4e8d7e01d76]::ScopedKey<rustc_span[4b37be1e64a2ccd5]::SessionGlobals>>::set::<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  36:     0x7f1b278d60e9 - std[dbf3afd8bb1e5979]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  37:     0x7f1b27950cfe - std[dbf3afd8bb1e5979]::panicking::try::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, core[2506165bb0faadb9]::panic::unwind_safe::AssertUnwindSafe<<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  38:     0x7f1b2794be60 - <<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1} as core[2506165bb0faadb9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f1b26f6cb75 - std::sys::unix::thread::Thread::new::thread_start::ha85b465a65a1c6c4
  40:     0x7f1b26d0ab43 - <unknown>
  41:     0x7f1b26d9ca00 - <unknown>
  42:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4a60c720d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/use/use-meta-mismatch.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/use/use-meta-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-meta-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-meta-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "metadata_cannot_find_crate", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "crate_name" })), ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f9bab3ccb5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2542abea4ee910f2
   1:     0x7f9bab435508 - core::fmt::write::h622ccda3d7f48263
   1:     0x7f9bab435508 - core::fmt::write::h622ccda3d7f48263
   2:     0x7f9bab3bd7d1 - std::io::Write::write_fmt::hdd0bcc3d51a95b66
   3:     0x7f9bab3cfb5e - std::panicking::default_hook::{{closure}}::hb63bf482bb996068
   4:     0x7f9bab3cf827 - std::panicking::default_hook::h1c8d1247b1fca3f3
   5:     0x7f9babd55c44 - rustc_driver[2b033e751948032]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f9bab3d0311 - std::panicking::rust_panic_with_hook::h2015d0f8cb0a7e52
   7:     0x7f9bab3d0137 - std::panicking::begin_panic_handler::{{closure}}::h92fcf298ed83eb87
   8:     0x7f9bab3cd0d4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf233cf8fe3635627
   9:     0x7f9bab3cfe02 - rust_begin_unwind
  10:     0x7f9bab380e43 - core::panicking::panic_fmt::hddf3504dfa1bb37a
  11:     0x7f9baea54377 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::translation::Translate>::translate_message
  12:     0x7f9baea497c2 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f9baea47266 - <rustc_errors[36d18b9907453f35]::emitter::EmitterWriter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  14:     0x7f9baea5f592 - <rustc_errors[36d18b9907453f35]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f9baea5dfac - <rustc_errors[36d18b9907453f35]::json::JsonEmitter as rustc_errors[36d18b9907453f35]::emitter::Emitter>::emit_diagnostic
  16:     0x7f9baeaa3cd8 - <rustc_errors[36d18b9907453f35]::HandlerInner>::emit_diagnostic
  17:     0x7f9baea9f9c6 - <rustc_errors[36d18b9907453f35]::ErrorGuaranteed as rustc_errors[36d18b9907453f35]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f9bade0b533 - <rustc_session[8c18bc53086a9b34]::parse::ParseSess>::emit_err::<rustc_metadata[3d1c815352b1b4dd]::errors::CannotFindCrate>
  19:     0x7f9bade4c507 - <rustc_metadata[3d1c815352b1b4dd]::locator::CrateError>::report
  20:     0x7f9bade8dd97 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::resolve_crate
  21:     0x7f9bade95434 - <rustc_metadata[3d1c815352b1b4dd]::creader::CrateLoader>::process_extern_crate
  22:     0x7f9bacb50188 - <rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[3242210473e1da7f]::visit::Visitor>::visit_item
  23:     0x7f9bacb9cafd - rustc_ast[3242210473e1da7f]::visit::walk_crate::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  24:     0x7f9bacb35543 - <rustc_expand[b8ce2325e59f79ab]::expand::AstFragment>::visit_with::<rustc_resolve[252dc9306f4267df]::build_reduced_graph::BuildReducedGraphVisitor>
  25:     0x7f9bacb785a0 - <rustc_resolve[252dc9306f4267df]::Resolver as rustc_expand[b8ce2325e59f79ab]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  26:     0x7f9bae02be92 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::collect_invocations
  27:     0x7f9bae0275c2 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7f9bae027227 - <rustc_expand[b8ce2325e59f79ab]::expand::MacroExpander>::expand_crate
  29:     0x7f9babed7f2e - <rustc_session[8c18bc53086a9b34]::session::Session>::time::<core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand::{closure#1}>
  30:     0x7f9babe930d9 - rustc_interface[75bd1abe22f1fd46]::passes::configure_and_expand
  31:     0x7f9babea0fd6 - <rustc_interface[75bd1abe22f1fd46]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[2506165bb0faadb9]::result::Result<rustc_ast[3242210473e1da7f]::ast::Crate, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  32:     0x7f9babe79e76 - <rustc_interface[75bd1abe22f1fd46]::queries::Queries>::expansion
  33:     0x7f9babd5ea2a - <rustc_interface[75bd1abe22f1fd46]::interface::Compiler>::enter::<rustc_driver[2b033e751948032]::run_compiler::{closure#1}::{closure#2}, core[2506165bb0faadb9]::result::Result<core[2506165bb0faadb9]::option::Option<rustc_interface[75bd1abe22f1fd46]::queries::Linker>, rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  34:     0x7f9babd427c1 - rustc_span[4b37be1e64a2ccd5]::with_source_map::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7f9babd7a571 - rustc_interface[75bd1abe22f1fd46]::interface::create_compiler_and_run::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>
  36:     0x7f9babd3a392 - <scoped_tls[e23be4e8d7e01d76]::ScopedKey<rustc_span[4b37be1e64a2ccd5]::SessionGlobals>>::set::<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  37:     0x7f9babd460e9 - std[dbf3afd8bb1e5979]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>
  38:     0x7f9babdc0cfe - std[dbf3afd8bb1e5979]::panicking::try::<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, core[2506165bb0faadb9]::panic::unwind_safe::AssertUnwindSafe<<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  39:     0x7f9babdbbe60 - <<std[dbf3afd8bb1e5979]::thread::Builder>::spawn_unchecked_<rustc_interface[75bd1abe22f1fd46]::util::run_in_thread_pool_with_globals<rustc_interface[75bd1abe22f1fd46]::interface::run_compiler<core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>, rustc_driver[2b033e751948032]::run_compiler::{closure#1}>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#0}, core[2506165bb0faadb9]::result::Result<(), rustc_errors[36d18b9907453f35]::ErrorGuaranteed>>::{closure#1} as core[2506165bb0faadb9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f9bab3dcb75 - std::sys::unix::thread::Thread::new::thread_start::ha85b465a65a1c6c4
  41:     0x7f9bab17ab43 - <unknown>
  42:     0x7f9bab20ca00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (4a60c720d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

