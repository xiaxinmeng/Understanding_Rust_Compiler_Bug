plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 13442 tests
........................................................................................ 88/13442
....................................................F.....................iiiiiiii.iiiii 176/13442
i....................i.................i....F.....F..................................... 264/13442
........................................................................................ 440/13442
........................................................................................ 528/13442
........................................................................................ 616/13442
........................................................................................ 704/13442
---
........................................................................................ 2552/13442
........................................................................................ 2640/13442
........................................................................................ 2728/13442
........................................................................................ 2816/13442
........................................F............................................... 2904/13442
.....................FF....F.........................................F.................. 2992/13442
........................................................................................ 3168/13442
........................................................................................ 3256/13442
.........................................................iiiii.......................... 3344/13442
.........................................................iiiii.......................... 3344/13442
..........................................................................F.F........... 3432/13442
........................................................................................ 3608/13442
........................................................................................ 3608/13442
.........F......................F............................F..F...F..F...F......F..FF. 3696/13442
F....................................................................................... 3784/13442
.........i.............................................................................. 3960/13442
........................................................................ii.............. 4048/13442
........................................................................................ 4136/13442
........................................................................................ 4136/13442
..................................i...F................................................. 4224/13442
........................................................................................ 4400/13442
........F............................................................................... 4488/13442
............................F........................................................... 4576/13442
........................................................................................ 4664/13442
........................................................................................ 4664/13442
.................................F...F.................FF..FF........................... 4752/13442
..............................................................................F......... 4840/13442
...................................................F.................................... 4928/13442
........................................................................................ 5104/13442
.............................................................................i.......... 5192/13442
........................................................i............................... 5280/13442
........................................................................................ 5368/13442
........................................................................................ 5368/13442
........................................................................................ 5456/13442
........................................................................................ 5544/13442
........................................................................................ 5632/13442
........................................................................................ 5720/13442
........................................................................................ 5808/13442
.......................................................................................F 5896/13442
....................F................................................................... 5984/13442
....FF.........F........................................................................ 6072/13442
...............................F.........................FF............................. 6160/13442
........................................................................................ 6248/13442
....................F..............................................F.................... 6336/13442
.............................................i.....................................F.... 6424/13442
.........................................i.............................................. 6600/13442
.........................................i.............................................. 6600/13442
........F..............F..............................F................................. 6688/13442
..................i.....F...........................F.....................ii.ii........i 6776/13442
...................................................................................F.... 6952/13442
........................................i....i.........................................i 7040/13442
..................i.............i....................................................... 7128/13442
..i..................................................................................... 7216/13442
---
........................................................................................ 7744/13442
.............................................ii......................................... 7832/13442
........................................................................................ 7920/13442
........................................................................................ 8008/13442
.........F..............................F..................ii................i....i..ii. 8096/13442
........................................................................................ 8272/13442
........................................................................................ 8360/13442
........................................................................................ 8448/13442
........................................................................................ 8536/13442
........................................................................................ 8536/13442
...............i...i.i..F.........................................................ii.... 8624/13442
........................................................................................ 8712/13442
iiii....F...................F........................................................... 8800/13442
....................F..........................................i........................ 8976/13442
........................................................................................ 9064/13442
...........................................................i............................ 9152/13442
.......F................................................................................ 9240/13442
---
................F..............................................................ii....... 9944/13442
.F......i............................................................................... 10032/13442
........................................................................................ 10120/13442
........................................................................................ 10208/13442
.......................................................F................................ 10296/13442
....................................F........................FFF........................ 10384/13442
............F......................................................F.F.................. 10472/13442
....................................................................F................... 10560/13442
..................iiiii...i....i.i...................................................... 10736/13442
...........................................................................i............ 10824/13442
.....................................................................................iii 10912/13442
iii.i..iiiiii.i......................................................................... 11000/13442
iii.i..iiiiii.i......................................................................... 11000/13442
....................................................F..F................................ 11088/13442
........................................................................................ 11264/13442
........................................................................................ 11352/13442
........................................................................................ 11440/13442
........................................................................................ 11528/13442
........................................................................................ 11528/13442
........................................................................................ 11616/13442
........................................................................................ 11704/13442
...........................................................................F............ 11792/13442
.F.........................................................i.......i.........i....i..... 11880/13442
................i....................................................................... 11968/13442
........................................................................................ 12056/13442
.............................................................F.......................... 12144/13442
...............................................F..............................F......... 12232/13442
........................................................................................ 12408/13442
........................................................................................ 12496/13442
......................................................................................i. 12584/13442
........................................................................................ 12672/13442
---
failures:

---- [ui] src/test/ui/array-slice-vec/subslice-only-once-semantic-restriction.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/subslice-only-once-semantic-restriction.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/subslice-only-once-semantic-restriction" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/subslice-only-once-semantic-restriction/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "resolve_ident_bound_more_than_once_in_same_pattern", attr: None, args: FluentArgs([("identifier", String("tail"))]), errors: [ResolverError(Reference(Message { id: "identifier", attribute: None }))]', compiler/rustc_errors/src/translation.rs:91:17
stack backtrace:
   0:     0x7f386ecbb12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::haef0e57182cff27e
   1:     0x7f386ed23da8 - core::fmt::write::h8f3832efb8012fdd
   2:     0x7f386ecab901 - std::io::Write::write_fmt::hac773a8b3e6facb1
   3:     0x7f386ecbe11e - std::panicking::default_hook::{{closure}}::h8fcc2d90b9b7cafd
   4:     0x7f386ecbdde7 - std::panicking::default_hook::h04c83f4486ad61ea
   5:     0x7f386f64db34 - rustc_driver[38c78fe59485a0c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f386ecbe8d1 - std::panicking::rust_panic_with_hook::h0ebd4e41cd9767ec
   7:     0x7f386ecbe6f7 - std::panicking::begin_panic_handler::{{closure}}::h661558e3dbb67af8
   8:     0x7f386ecbb6a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h37a58d6b6c3875b8
   9:     0x7f386ecbe3c2 - rust_begin_unwind
  10:     0x7f386ec6ee43 - core::panicking::panic_fmt::hcad844cb234ec812
  11:     0x7f387232eb9a - <rustc_errors[159519190c68e216]::emitter::EmitterWriter as rustc_errors[159519190c68e216]::translation::Translate>::translate_message
  12:     0x7f3872323f42 - <rustc_errors[159519190c68e216]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f3872321a5d - <rustc_errors[159519190c68e216]::emitter::EmitterWriter as rustc_errors[159519190c68e216]::emitter::Emitter>::emit_diagnostic
  14:     0x7f38723398a2 - <rustc_errors[159519190c68e216]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f38723382bc - <rustc_errors[159519190c68e216]::json::JsonEmitter as rustc_errors[159519190c68e216]::emitter::Emitter>::emit_diagnostic
  16:     0x7f387237eb68 - <rustc_errors[159519190c68e216]::HandlerInner>::emit_diagnostic
  17:     0x7f387237a846 - <rustc_errors[159519190c68e216]::ErrorGuaranteed as rustc_errors[159519190c68e216]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f3870418568 - <rustc_resolve[53eaf68e0605af1d]::Resolver>::report_error
  19:     0x7f3870394c47 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::fresh_binding
  20:     0x7f38704d8034 - <rustc_ast[851e558ea592c76c]::ast::Pat>::walk::<<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_pattern_inner::{closure#0}>
  21:     0x7f38704d848c - <rustc_ast[851e558ea592c76c]::ast::Pat>::walk::<<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_pattern_inner::{closure#0}>
  22:     0x7f38703948a6 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_pattern_top
  23:     0x7f387035a602 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_arm
  24:     0x7f387045d39a - rustc_ast[851e558ea592c76c]::visit::walk_expr::<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>
  25:     0x7f387039be71 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_expr
  26:     0x7f387035aa1f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_block
  27:     0x7f387036d74f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_fn::{closure#1}::{closure#0}
  28:     0x7f38703679fc - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_fn
  29:     0x7f38704613be - rustc_ast[851e558ea592c76c]::visit::walk_item::<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>
  30:     0x7f38703801f1 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_item
  31:     0x7f3870359f07 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_item
  32:     0x7f3870447c5d - rustc_ast[851e558ea592c76c]::visit::walk_crate::<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>
  33:     0x7f387042874e - <rustc_resolve[53eaf68e0605af1d]::Resolver>::late_resolve_crate
  34:     0x7f38703eac85 - <rustc_session[593e660ae94605db]::session::Session>::time::<(), <rustc_resolve[53eaf68e0605af1d]::Resolver>::resolve_crate::{closure#0}>
  35:     0x7f386f7a0a78 - rustc_interface[678d2d04e4bf5942]::passes::configure_and_expand
  36:     0x7f386f7ec936 - <rustc_interface[678d2d04e4bf5942]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[678d2d04e4bf5942]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[65c727f087702ea3]::result::Result<rustc_ast[851e558ea592c76c]::ast::Crate, rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  37:     0x7f386f7885e6 - <rustc_interface[678d2d04e4bf5942]::queries::Queries>::expansion
  38:     0x7f386f655b77 - <rustc_interface[678d2d04e4bf5942]::interface::Compiler>::enter::<rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}::{closure#2}, core[65c727f087702ea3]::result::Result<core[65c727f087702ea3]::option::Option<rustc_interface[678d2d04e4bf5942]::queries::Linker>, rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  39:     0x7f386f638475 - rustc_span[3969291980c0babf]::with_source_map::<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_interface[678d2d04e4bf5942]::interface::create_compiler_and_run<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f386f670e51 - rustc_interface[678d2d04e4bf5942]::interface::create_compiler_and_run::<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>
  41:     0x7f386f63a4b2 - <scoped_tls[c3b2fd947603b8d7]::ScopedKey<rustc_span[3969291980c0babf]::SessionGlobals>>::set::<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  42:     0x7f386f6b0459 - std[f086a7a38219cd93]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[678d2d04e4bf5942]::util::run_in_thread_pool_with_globals<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  43:     0x7f386f63cb1e - std[f086a7a38219cd93]::panicking::try::<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, core[65c727f087702ea3]::panic::unwind_safe::AssertUnwindSafe<<std[f086a7a38219cd93]::thread::Builder>::spawn_unchecked_<rustc_interface[678d2d04e4bf5942]::util::run_in_thread_pool_with_globals<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7f386f6b4a90 - <<std[f086a7a38219cd93]::thread::Builder>::spawn_unchecked_<rustc_interface[678d2d04e4bf5942]::util::run_in_thread_pool_with_globals<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#1} as core[65c727f087702ea3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f386eccb645 - std::sys::unix::thread::Thread::new::thread_start::ha0dece557faac109
  46:     0x7f386ea67b43 - <unknown>
  47:     0x7f386eaf9a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (39670d110 2022-08-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/asm/type-check-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/type-check-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "resolve_attempt_to_use_non_constant_value_in_constant_with_suggestion", attr: None, args: FluentArgs([("current", String("let")), ("ident", String("x")), ("suggestion", String("const"))]), errors: [ResolverError(Reference(Message { id: "suggestion", attribute: None })), ResolverError(Reference(Message { id: "current", attribute: None }))]', compiler/rustc_errors/src/translation.rs:91:17
stack backtrace:
   0:     0x7f087cb6f12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::haef0e57182cff27e
   1:     0x7f087cbd7da8 - core::fmt::write::h8f3832efb8012fdd
   2:     0x7f087cb5f901 - std::io::Write::write_fmt::hac773a8b3e6facb1
   3:     0x7f087cb7211e - std::panicking::default_hook::{{closure}}::h8fcc2d90b9b7cafd
   4:     0x7f087cb71de7 - std::panicking::default_hook::h04c83f4486ad61ea
   5:     0x7f087d501b34 - rustc_driver[38c78fe59485a0c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f087cb728d1 - std::panicking::rust_panic_with_hook::h0ebd4e41cd9767ec
   7:     0x7f087cb726f7 - std::panicking::begin_panic_handler::{{closure}}::h661558e3dbb67af8
   8:     0x7f087cb6f6a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h37a58d6b6c3875b8
   9:     0x7f087cb723c2 - rust_begin_unwind
  10:     0x7f087cb22e43 - core::panicking::panic_fmt::hcad844cb234ec812
  11:     0x7f08801e2b9a - <rustc_errors[159519190c68e216]::emitter::EmitterWriter as rustc_errors[159519190c68e216]::translation::Translate>::translate_message
  12:     0x7f08801d372c - <rustc_errors[159519190c68e216]::emitter::EmitterWriter as rustc_errors[159519190c68e216]::emitter::Emitter>::primary_span_formatted
  13:     0x7f08801d56fb - <rustc_errors[159519190c68e216]::emitter::EmitterWriter as rustc_errors[159519190c68e216]::emitter::Emitter>::emit_diagnostic
  14:     0x7f08801ed8a2 - <rustc_errors[159519190c68e216]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f08801ec2bc - <rustc_errors[159519190c68e216]::json::JsonEmitter as rustc_errors[159519190c68e216]::emitter::Emitter>::emit_diagnostic
  16:     0x7f0880232b68 - <rustc_errors[159519190c68e216]::HandlerInner>::emit_diagnostic
  17:     0x7f088022e846 - <rustc_errors[159519190c68e216]::ErrorGuaranteed as rustc_errors[159519190c68e216]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f087e2cc568 - <rustc_resolve[53eaf68e0605af1d]::Resolver>::report_error
  19:     0x7f087e2f2fdb - <rustc_resolve[53eaf68e0605af1d]::Resolver>::resolve_ident_in_lexical_scope
  20:     0x7f087e2d87c3 - <rustc_resolve[53eaf68e0605af1d]::Resolver>::resolve_path_with_ribs
  21:     0x7f087e2499fb - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  22:     0x7f087e249469 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::smart_resolve_path
  23:     0x7f087e24fe66 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_expr
  24:     0x7f087e24ecd6 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_inline_const
  25:     0x7f087e229926 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_inline_asm
  26:     0x7f087e24fe71 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_expr
  27:     0x7f087e20ea1f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_block
  28:     0x7f087e24fe3c - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_expr
  29:     0x7f087e20ea1f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_block
  30:     0x7f087e22174f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_fn::{closure#1}::{closure#0}
  31:     0x7f087e21b9fc - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_fn
  32:     0x7f087e3153be - rustc_ast[851e558ea592c76c]::visit::walk_item::<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>
  33:     0x7f087e2341f1 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_item
  34:     0x7f087e20df07 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_item
  35:     0x7f087e2fbc5d - rustc_ast[851e558ea592c76c]::visit::walk_crate::<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>
  36:     0x7f087e2dc74e - <rustc_resolve[53eaf68e0605af1d]::Resolver>::late_resolve_crate
  37:     0x7f087e29ec85 - <rustc_session[593e660ae94605db]::session::Session>::time::<(), <rustc_resolve[53eaf68e0605af1d]::Resolver>::resolve_crate::{closure#0}>
  38:     0x7f087d654a78 - rustc_interface[678d2d04e4bf5942]::passes::configure_and_expand
  39:     0x7f087d6a0936 - <rustc_interface[678d2d04e4bf5942]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[678d2d04e4bf5942]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[65c727f087702ea3]::result::Result<rustc_ast[851e558ea592c76c]::ast::Crate, rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  40:     0x7f087d63c5e6 - <rustc_interface[678d2d04e4bf5942]::queries::Queries>::expansion
  41:     0x7f087d509b77 - <rustc_interface[678d2d04e4bf5942]::interface::Compiler>::enter::<rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}::{closure#2}, core[65c727f087702ea3]::result::Result<core[65c727f087702ea3]::option::Option<rustc_interface[678d2d04e4bf5942]::queries::Linker>, rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  42:     0x7f087d4ec475 - rustc_span[3969291980c0babf]::with_source_map::<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_interface[678d2d04e4bf5942]::interface::create_compiler_and_run<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#1}>
  43:     0x7f087d524e51 - rustc_interface[678d2d04e4bf5942]::interface::create_compiler_and_run::<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>
  44:     0x7f087d4ee4b2 - <scoped_tls[c3b2fd947603b8d7]::ScopedKey<rustc_span[3969291980c0babf]::SessionGlobals>>::set::<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  45:     0x7f087d564459 - std[f086a7a38219cd93]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[678d2d04e4bf5942]::util::run_in_thread_pool_with_globals<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  46:     0x7f087d4f0b1e - std[f086a7a38219cd93]::panicking::try::<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, core[65c727f087702ea3]::panic::unwind_safe::AssertUnwindSafe<<std[f086a7a38219cd93]::thread::Builder>::spawn_unchecked_<rustc_interface[678d2d04e4bf5942]::util::run_in_thread_pool_with_globals<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  47:     0x7f087d568a90 - <<std[f086a7a38219cd93]::thread::Builder>::spawn_unchecked_<rustc_interface[678d2d04e4bf5942]::util::run_in_thread_pool_with_globals<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#1} as core[65c727f087702ea3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:     0x7f087cb7f645 - std::sys::unix::thread::Thread::new::thread_start::ha0dece557faac109
  49:     0x7f087c91bb43 - <unknown>
  50:     0x7f087c9ada00 - <unknown>
  51:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (39670d110 2022-08-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/asm/x86_64/parse-error.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/parse-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/parse-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/parse-error/auxiliary"
stdout: none
--- stderr -------------------------------
error: requires at least a template string argument
   |
LL |         asm!();
   |         ^^^^^^

---
   |
LL |         asm!("{}" foo);
   |                   ^^^ expected `,`

error: expected operand, clobber_abi, options, or additional template string
   |
LL |         asm!("{}", foo);
LL |         asm!("{}", foo);
   |                    ^^^ expected operand, clobber_abi, options, or additional template string
error: expected `(`, found `foo`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:19:23
   |
LL |         asm!("{}", in foo);
LL |         asm!("{}", in foo);
   |                       ^^^ expected `(`

error: expected `)`, found `foo`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:21:27
   |
LL |         asm!("{}", in(reg foo));
   |                           ^^^ expected `)`
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:23:27
   |
   |
LL |         asm!("{}", in(reg));

error: expected register class or explicit register
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:25:26
   |
   |
LL |         asm!("{}", inout(=) foo => bar);

error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:27:37
   |
   |
LL |         asm!("{}", inout(reg) foo =>);


error: expected one of `!`, `,`, `.`, `::`, `?`, `{`, or an operator, found `=>`
   |
   |
LL |         asm!("{}", in(reg) foo => bar);
   |                                ^^ expected one of 7 possible tokens
error: expected a path for argument to `sym`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:31:24
   |
   |
LL |         asm!("{}", sym foo + bar);


error: expected one of `)`, `att_syntax`, `may_unwind`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, `raw`, or `readonly`, found `foo`
   |
   |
LL |         asm!("", options(foo));
   |                          ^^^ expected one of 10 possible tokens

error: expected one of `)` or `,`, found `foo`
   |
   |
LL |         asm!("", options(nomem foo));
   |                                ^^^ expected one of `)` or `,`

error: expected one of `)`, `att_syntax`, `may_unwind`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, `raw`, or `readonly`, found `foo`
   |
   |
LL |         asm!("", options(nomem, foo));
   |                                 ^^^ expected one of 10 possible tokens
error: arguments are not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:39:31
   |
LL |         asm!("{}", options(), const foo);
LL |         asm!("{}", options(), const foo);
   |                    ---------  ^^^^^^^^^ argument
   |                    |
   |                    previous options

error: at least one abi must be provided as an argument to `clobber_abi`
   |
   |
LL |         asm!("", clobber_abi());

error: expected string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:44:30
   |
   |
LL |         asm!("", clobber_abi(foo));


error: expected one of `)` or `,`, found `foo`
   |
   |
LL |         asm!("", clobber_abi("C" foo));
   |                                  ^^^ expected one of `)` or `,`
error: expected string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:48:35
   |
   |
LL |         asm!("", clobber_abi("C", foo));

error: arguments are not allowed after clobber_abi
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:50:38
   |
   |
LL |         asm!("{}", clobber_abi("C"), const foo);
   |                    ----------------  ^^^^^^^^^ argument
   |                    clobber_abi

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:53:29
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:53:29
   |
LL |         asm!("", options(), clobber_abi("C"));
   |                  ---------  ^^^^^^^^^^^^^^^^
   |                  options

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:31
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:31
   |
LL |         asm!("{}", options(), clobber_abi("C"), const foo);
   |                    ---------  ^^^^^^^^^^^^^^^^
   |                    options

error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:57:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:57:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ duplicate argument
   |                     previously here

error: argument never used
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:57:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:57:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                    ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`
error: explicit register arguments cannot have names
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:62:18
   |
   |
LL |         asm!("", a = in("eax") foo);

error: named arguments cannot follow explicit register arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:64:36
   |
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument

error: named arguments cannot follow explicit register arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:67:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:67:36
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument

error: positional arguments cannot follow named arguments or explicit register arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:70:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:70:36
   |
LL |         asm!("{1}", in("eax") foo, const bar);
   |                     -------------  ^^^^^^^^^ positional argument
   |                     explicit register argument


error: expected one of `clobber_abi`, `const`, `in`, `inlateout`, `inout`, `lateout`, `options`, `out`, or `sym`, found `""`
   |
   |
LL |         asm!("", options(), "");
   |                             ^^ expected one of 9 possible tokens

error: expected one of `clobber_abi`, `const`, `in`, `inlateout`, `inout`, `lateout`, `options`, `out`, or `sym`, found `"{}"`
   |
   |
LL |         asm!("{}", in(reg) foo, "{}", out(reg) foo);
   |                                 ^^^^ expected one of 9 possible tokens
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:77:14
   |
   |
LL |         asm!(format!("{{{}}}", 0), in(reg) foo);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:79:21
   |
LL |         asm!("{1}", format!("{{{}}}", 0), in(reg) foo, out(reg) bar);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)


error: _ cannot be used for input operands
   |
   |
LL |         asm!("{}", in(reg) _);


error: _ cannot be used for input operands
   |
   |
LL |         asm!("{}", inout(reg) _);


error: _ cannot be used for input operands
   |
   |
LL |         asm!("{}", inlateout(reg) _);

error: requires at least a template string argument
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:92:1
   |
   |
LL | global_asm!();

error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:94:13
   |
   |
LL | global_asm!(FOO);

error: expected token: `,`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:96:18
   |
   |
LL | global_asm!("{}" FOO);
   |                  ^^^ expected `,`
error: expected operand, options, or additional template string
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:98:19
   |
   |
LL | global_asm!("{}", FOO);
   |                   ^^^ expected operand, options, or additional template string
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:100:24
   |
   |
LL | global_asm!("{}", const);


error: expected one of `,`, `.`, `?`, or an operator, found `FOO`
   |
   |
LL | global_asm!("{}", const(reg) FOO);
   |                              ^^^ expected one of `,`, `.`, `?`, or an operator

error: expected one of `)`, `att_syntax`, or `raw`, found `FOO`
   |
   |
LL | global_asm!("", options(FOO));
   |                         ^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
   |
   |
LL | global_asm!("", options(nomem FOO));
   |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
   |
   |
LL | global_asm!("", options(nomem, FOO));
   |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`
error: arguments are not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:110:30
   |
   |
LL | global_asm!("{}", options(), const FOO);
   |                   ---------  ^^^^^^^^^ argument
   |                   previous options

error: expected string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:112:29
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:112:29
   |
LL | global_asm!("", clobber_abi(FOO));


error: expected one of `)` or `,`, found `FOO`
   |
   |
LL | global_asm!("", clobber_abi("C" FOO));
   |                                 ^^^ expected one of `)` or `,`
error: expected string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:116:34
   |
   |
LL | global_asm!("", clobber_abi("C", FOO));

error: arguments are not allowed after clobber_abi
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:118:37
   |
   |
LL | global_asm!("{}", clobber_abi("C"), const FOO);
   |                   ----------------  ^^^^^^^^^ argument
   |                   clobber_abi


error: `clobber_abi` cannot be used with `global_asm!`
   |
   |
LL | global_asm!("{}", clobber_abi("C"), const FOO);

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:121:28
   |
   |
LL | global_asm!("", options(), clobber_abi("C"));
   |                 ---------  ^^^^^^^^^^^^^^^^
   |                 options

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:123:30
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:123:30
   |
LL | global_asm!("{}", options(), clobber_abi("C"), const FOO);
   |                   ---------  ^^^^^^^^^^^^^^^^
   |                   options


error: `clobber_abi` cannot be used with `global_asm!`
   |
   |
LL | global_asm!("", clobber_abi("C"), clobber_abi("C"));

error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:127:35
   |
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                    -------------  ^^^^^^^^^^^^^ duplicate argument
   |                    previously here

error: argument never used
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:127:35
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:127:35
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                                   ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`

error: expected one of `clobber_abi`, `const`, `options`, or `sym`, found `""`
   |
   |
LL | global_asm!("", options(), "");
   |                            ^^ expected one of `clobber_abi`, `const`, `options`, or `sym`

error: expected one of `clobber_abi`, `const`, `options`, or `sym`, found `"{}"`
   |
   |
LL | global_asm!("{}", const FOO, "{}", const FOO);
   |                              ^^^^ expected one of `clobber_abi`, `const`, `options`, or `sym`
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:134:13
   |
   |
LL | global_asm!(format!("{{{}}}", 0), const FOO);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:136:20
   |
LL | global_asm!("{1}", format!("{{{}}}", 0), const FOO, const BAR);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)


thread 'rustc' panicked at 'identifier: "resolve_attempt_to_use_non_constant_value_in_constant_with_suggestion", attr: None, args: FluentArgs([("current", String("let")), ("ident", String("foo")), ("suggestion", String("const"))]), errors: [ResolverError(Reference(Message { id: "suggestion", attribute: None })), ResolverError(Reference(Message { id: "current", attribute: None }))]', compiler/rustc_errors/src/translation.rs:91:17
stack backtrace:
   0:     0x7f70bc16212c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::haef0e57182cff27e
   1:     0x7f70bc1cada8 - core::fmt::write::h8f3832efb8012fdd
   2:     0x7f70bc152901 - std::io::Write::write_fmt::hac773a8b3e6facb1
   3:     0x7f70bc16511e - std::panicking::default_hook::{{closure}}::h8fcc2d90b9b7cafd
   4:     0x7f70bc164de7 - std::panicking::default_hook::h04c83f4486ad61ea
   5:     0x7f70bcaf4b34 - rustc_driver[38c78fe59485a0c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f70bc1658d1 - std::panicking::rust_panic_with_hook::h0ebd4e41cd9767ec
   7:     0x7f70bc1656f7 - std::panicking::begin_panic_handler::{{closure}}::h661558e3dbb67af8
   8:     0x7f70bc1626a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h37a58d6b6c3875b8
   9:     0x7f70bc1653c2 - rust_begin_unwind
  10:     0x7f70bc115e43 - core::panicking::panic_fmt::hcad844cb234ec812
  11:     0x7f70bf7d5b9a - <rustc_errors[159519190c68e216]::emitter::EmitterWriter as rustc_errors[159519190c68e216]::translation::Translate>::translate_message
  12:     0x7f70bf7c672c - <rustc_errors[159519190c68e216]::emitter::EmitterWriter as rustc_errors[159519190c68e216]::emitter::Emitter>::primary_span_formatted
  13:     0x7f70bf7c86fb - <rustc_errors[159519190c68e216]::emitter::EmitterWriter as rustc_errors[159519190c68e216]::emitter::Emitter>::emit_diagnostic
  14:     0x7f70bf7e08a2 - <rustc_errors[159519190c68e216]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f70bf7df2bc - <rustc_errors[159519190c68e216]::json::JsonEmitter as rustc_errors[159519190c68e216]::emitter::Emitter>::emit_diagnostic
  16:     0x7f70bf825b68 - <rustc_errors[159519190c68e216]::HandlerInner>::emit_diagnostic
  17:     0x7f70bf821846 - <rustc_errors[159519190c68e216]::ErrorGuaranteed as rustc_errors[159519190c68e216]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f70bd8bf568 - <rustc_resolve[53eaf68e0605af1d]::Resolver>::report_error
  19:     0x7f70bd8e5fdb - <rustc_resolve[53eaf68e0605af1d]::Resolver>::resolve_ident_in_lexical_scope
  20:     0x7f70bd8cb7c3 - <rustc_resolve[53eaf68e0605af1d]::Resolver>::resolve_path_with_ribs
  21:     0x7f70bd83c9fb - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  22:     0x7f70bd83c469 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::smart_resolve_path
  23:     0x7f70bd842e66 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_expr
  24:     0x7f70bd841cd6 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_inline_const
  25:     0x7f70bd81c926 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_inline_asm
  26:     0x7f70bd842e71 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_expr
  27:     0x7f70bd801a1f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_block
  28:     0x7f70bd842e3c - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_expr
  29:     0x7f70bd801a1f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_block
  30:     0x7f70bd81474f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_fn::{closure#1}::{closure#0}
  31:     0x7f70bd80e9fc - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_fn
  32:     0x7f70bd9083be - rustc_ast[851e558ea592c76c]::visit::walk_item::<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>
  33:     0x7f70bd8271f1 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_item
  34:     0x7f70bd800f07 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_item
  35:     0x7f70bd8eec5d - rustc_ast[851e558ea592c76c]::visit::walk_crate::<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>
  36:     0x7f70bd8cf74e - <rustc_resolve[53eaf68e0605af1d]::Resolver>::late_resolve_crate
  37:     0x7f70bd891c85 - <rustc_session[593e660ae94605db]::session::Session>::time::<(), <rustc_resolve[53eaf68e0605af1d]::Resolver>::resolve_crate::{closure#0}>
  38:     0x7f70bcc47a78 - rustc_interface[678d2d04e4bf5942]::passes::configure_and_expand
  39:     0x7f70bcc93936 - <rustc_interface[678d2d04e4bf5942]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[678d2d04e4bf5942]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[65c727f087702ea3]::result::Result<rustc_ast[851e558ea592c76c]::ast::Crate, rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  40:     0x7f70bcc2f5e6 - <rustc_interface[678d2d04e4bf5942]::queries::Queries>::expansion
  41:     0x7f70bcafcb77 - <rustc_interface[678d2d04e4bf5942]::interface::Compiler>::enter::<rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}::{closure#2}, core[65c727f087702ea3]::result::Result<core[65c727f087702ea3]::option::Option<rustc_interface[678d2d04e4bf5942]::queries::Linker>, rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  42:     0x7f70bcadf475 - rustc_span[3969291980c0babf]::with_source_map::<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_interface[678d2d04e4bf5942]::interface::create_compiler_and_run<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#1}>
  43:     0x7f70bcb17e51 - rustc_interface[678d2d04e4bf5942]::interface::create_compiler_and_run::<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>
  44:     0x7f70bcae14b2 - <scoped_tls[c3b2fd947603b8d7]::ScopedKey<rustc_span[3969291980c0babf]::SessionGlobals>>::set::<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  45:     0x7f70bcb57459 - std[f086a7a38219cd93]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[678d2d04e4bf5942]::util::run_in_thread_pool_with_globals<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>
  46:     0x7f70bcae3b1e - std[f086a7a38219cd93]::panicking::try::<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, core[65c727f087702ea3]::panic::unwind_safe::AssertUnwindSafe<<std[f086a7a38219cd93]::thread::Builder>::spawn_unchecked_<rustc_interface[678d2d04e4bf5942]::util::run_in_thread_pool_with_globals<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  47:     0x7f70bcb5ba90 - <<std[f086a7a38219cd93]::thread::Builder>::spawn_unchecked_<rustc_interface[678d2d04e4bf5942]::util::run_in_thread_pool_with_globals<rustc_interface[678d2d04e4bf5942]::interface::run_compiler<core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>, rustc_driver[38c78fe59485a0c]::run_compiler::{closure#1}>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#0}, core[65c727f087702ea3]::result::Result<(), rustc_errors[159519190c68e216]::ErrorGuaranteed>>::{closure#1} as core[65c727f087702ea3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:     0x7f70bc172645 - std::sys::unix::thread::Thread::new::thread_start::ha0dece557faac109
  49:     0x7f70bbf0eb43 - <unknown>
  50:     0x7f70bbfa0a00 - <unknown>
  51:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (39670d110 2022-08-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 59 previous errors


For more information about this error, try `rustc --explain E0435`.
------------------------------------------


---- [ui] src/test/ui/capture1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/capture1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/capture1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/capture1/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "resolve_cannot_capture_dynamic_environment_in_fn_item", attr: Some("help"), args: FluentArgs([("closure_hint", String("|| { ... }"))]), errors: [ResolverError(Reference(Message { id: "closure_hint", attribute: None }))]', compiler/rustc_errors/src/translation.rs:91:17
stack backtrace:
   0:     0x7f7588dc012c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::haef0e57182cff27e
   1:     0x7f7588e28da8 - core::fmt::write::h8f3832efb8012fdd
   2:     0x7f7588db0901 - std::io::Write::write_fmt::hac773a8b3e6facb1
   3:     0x7f7588dc311e - std::panicking::default_hook::{{closure}}::h8fcc2d90b9b7cafd
   4:     0x7f7588dc2de7 - std::panicking::default_hook::h04c83f4486ad61ea
   5:     0x7f7589752b34 - rustc_driver[38c78fe59485a0c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f7588dc38d1 - std::panicking::rust_panic_with_hook::h0ebd4e41cd9767ec
   7:     0x7f7588dc36f7 - std::panicking::begin_panic_handler::{{closure}}::h661558e3dbb67af8
   8:     0x7f7588dc06a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h37a58d6b6c3875b8
   9:     0x7f7588dc33c2 - rust_begin_unwind
  10:     0x7f7588d73e43 - core::panicking::panic_fmt::hcad844cb234ec812
  11:     0x7f758c433b9a - <rustc_errors[159519190c68e216]::emitter::EmitterWriter as rustc_errors[159519190c68e216]::translation::Translate>::translate_message
  12:     0x7f758c428f42 - <rustc_errors[159519190c68e216]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f758c426c49 - <rustc_errors[159519190c68e216]::emitter::EmitterWriter as rustc_errors[159519190c68e216]::emitter::Emitter>::emit_diagnostic
  14:     0x7f758c43e8a2 - <rustc_errors[159519190c68e216]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f758c43d2bc - <rustc_errors[159519190c68e216]::json::JsonEmitter as rustc_errors[159519190c68e216]::emitter::Emitter>::emit_diagnostic
  16:     0x7f758c483b68 - <rustc_errors[159519190c68e216]::HandlerInner>::emit_diagnostic
  17:     0x7f758c47f846 - <rustc_errors[159519190c68e216]::ErrorGuaranteed as rustc_errors[159519190c68e216]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f758a51d568 - <rustc_resolve[53eaf68e0605af1d]::Resolver>::report_error
  19:     0x7f758a543e95 - <rustc_resolve[53eaf68e0605af1d]::Resolver>::resolve_ident_in_lexical_scope
  20:     0x7f758a5297c3 - <rustc_resolve[53eaf68e0605af1d]::Resolver>::resolve_path_with_ribs
  21:     0x7f758a49a9fb - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  22:     0x7f758a49a469 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::smart_resolve_path
  23:     0x7f758a4a0e66 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_expr
  24:     0x7f758a4a0e71 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_expr
  25:     0x7f758a45fa1f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_block
  26:     0x7f758a47274f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_fn::{closure#1}::{closure#0}
  27:     0x7f758a46c9fc - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_fn
  28:     0x7f758a5663be - rustc_ast[851e558ea592c76c]::visit::walk_item::<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>
  29:     0x7f758a4851f1 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_item
  30:     0x7f758a45ef07 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_item
  31:     0x7f758a45fa1f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_block
  32:     0x7f758a47274f - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_fn::{closure#1}::{closure#0}
  33:     0x7f758a46c9fc - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_fn
  34:     0x7f758a5663be - rustc_ast[851e558ea592c76c]::visit::walk_item::<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>
  35:     0x7f758a4851f1 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>::resolve_item
  36:     0x7f758a45ef07 - <rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor as rustc_ast[851e558ea592c76c]::visit::Visitor>::visit_item
  37:     0x7f758a54cc5d - rustc_ast[851e558ea592c76c]::visit::walk_crate::<rustc_resolve[53eaf68e0605af1d]::late::LateResolutionVisitor>
  38:     0x7f758a52d74e - <rustc_resolve[53eaf68e0605af1d]::Resolver>::late_resolve_crate
  39:     0x7f758a4efc85 - <rustc_session[593e660ae94605db]::session::Session>::time::<(), <rustc_resolve[53eaf68e0605af1d]::Resolver>::resolve_crate::{closure#0}>
