plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 13471 tests
........................................................................................ 88/13471
....................................................F.....................iiiiiiiiiiiiii 176/13471
.....................i.................i......F....F.................................... 264/13471
........................................................................................ 440/13471
........................................................................................ 528/13471
........................................................................................ 616/13471
........................................................................................ 704/13471
---
........................................................................................ 2552/13471
........................................................................................ 2640/13471
........................................................................................ 2728/13471
........................................................................................ 2816/13471
....................................................F................................... 2904/13471
...................................FFF.............................................F.... 2992/13471
.......i................................................................................ 3168/13471
........................................................................................ 3256/13471
.......................................................................iiiii............ 3344/13471
........................................................................................ 3432/13471
........................................................................................ 3432/13471
.F..F................................................................................... 3520/13471
........................................................................................ 3608/13471
.......................F.........................F............................F......F.F 3696/13471
FFF.....F...F..FF....................................................................... 3784/13471
.i..........i..........i................................................................ 3960/13471
......................................................................................ii 4048/13471
........................................................................................ 4136/13471
........................................................................................ 4136/13471
................................................i...F................................... 4224/13471
........................................................................................ 4400/13471
.......................F................................................................ 4488/13471
............................................F........................................... 4576/13471
........................................................................................ 4664/13471
........................................................................................ 4664/13471
................................................F..F..................F.FF.F............ 4752/13471
........................................................................................ 4840/13471
.....F...........................................................F...................... 4928/13471
........................................................................................ 5016/13471
.................................................................F.......F.............. 5104/13471
.......i..................................................................i............. 5280/13471
........................................................................................ 5368/13471
........................................................................................ 5456/13471
........................................................................................ 5544/13471
........................................................................................ 5544/13471
........................................................................................ 5632/13471
........................................................................................ 5720/13471
........................................................................................ 5808/13471
........................................................................................ 5896/13471
....................F.................F................................................. 5984/13471
.......................F.F......F....................................................... 6072/13471
...............................................F.............................F.F........ 6160/13471
........................................................................................ 6248/13471
......................................F................................................F 6336/13471
.............F.......................................................................... 6512/13471
...........................................................i............................ 6600/13471
...........................................................i............................ 6600/13471
..........................F.............F...............................F............... 6688/13471
....................................i.....F..........................F.................. 6776/13471
........................................................................................ 6952/13471
.................F.........................................i...i........................ 7040/13471
.................i..................i.............i..................................... 7128/13471
....................i................................................................... 7216/13471
---
........................................................................................ 7744/13471
.................................................................ii..................... 7832/13471
........................................................................................ 7920/13471
........................................................................................ 8008/13471
...............................F...............................F.................ii..... 8096/13471
........................................................................................ 8272/13471
........................................................................................ 8360/13471
........................................................................................ 8448/13471
........................................................................................ 8536/13471
........................................................................................ 8536/13471
.......................................i..ii......F..................................... 8624/13471
..................ii.................................................................... 8712/13471
........................iiii.....F..................F................................... 8800/13471
...................i..................................F................................i 8976/13471
........................................................................................ 9064/13471
....................................................................................i... 9152/13471
............................F........................................................... 9240/13471
---
..........................................F............................................. 9944/13471
..............ii........F......i........................................................ 10032/13471
........................................................................................ 10120/13471
........................................................................................ 10208/13471
.................................................................................F...... 10296/13471
................................................................F.................FF...F 10384/13471
..........................................F............................................. 10472/13471
....F..F................................................................................ 10560/13471
....F................................................................................... 10648/13471
........................................................................................ 10824/13471
..........i............................................................................. 10912/13471
....................iiiiii.i..iiiiii.i.................................................. 11000/13471
............................................................................FF.......... 11088/13471
---
........................................................................................ 11528/13471
........................................................................................ 11616/13471
........................................................................................ 11704/13471
........................................................................................ 11792/13471
................F..........F..........................................................i. 11880/13471
........................................................................................ 12056/13471
........................................................................................ 12144/13471
........................................................................................ 12144/13471
....F.....................................................................F............. 12232/13471
..................F..................................................................... 12320/13471
........................................................................................ 12496/13471
........................................................................................ 12584/13471
.........................i.............................................................. 12672/13471
......................................................................F................. 12760/13471
......................................................................F................. 12760/13471
........................................................................................ 12848/13471
............F........................................................................... 12936/13471
........................................................................................ 13024/13471
........................................................................................ 13112/13471
........................................................................................ 13200/13471
........................................................................................ 13288/13471
...............................F.FF.F.......................................iii......... 13376/13471
.......
failures:

---- [ui] src/test/ui/array-slice-vec/subslice-only-once-semantic-restriction.rs stdout ----
---- [ui] src/test/ui/array-slice-vec/subslice-only-once-semantic-restriction.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/subslice-only-once-semantic-restriction.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/subslice-only-once-semantic-restriction" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/subslice-only-once-semantic-restriction/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "resolve_ident_bound_more_than_once_in_same_pattern", attr: None, args: FluentArgs([("identifier", String("tail"))]), errors: [ResolverError(Reference(Message { id: "identifier", attribute: None }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f2bd97aa15c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aa99dd858e8a758
   1:     0x7f2bd9812c98 - core::fmt::write::hecd54e75f30f9663
   2:     0x7f2bd979a9d1 - std::io::Write::write_fmt::h8988434cba967fd2
   2:     0x7f2bd979a9d1 - std::io::Write::write_fmt::h8988434cba967fd2
   3:     0x7f2bd97ad14e - std::panicking::default_hook::{{closure}}::haa1da06bdd4cfdeb
   4:     0x7f2bd97ace17 - std::panicking::default_hook::hac6cf1535750f7c5
   5:     0x7f2bda16dbd4 - rustc_driver[ee9f1d9dabd2d9ea]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2bd97ad901 - std::panicking::rust_panic_with_hook::h9901cbb7ff23ab55
   7:     0x7f2bd97ad727 - std::panicking::begin_panic_handler::{{closure}}::h4211a6156a7c84d8
   8:     0x7f2bd97aa6d4 - std::sys_common::backtrace::__rust_end_short_backtrace::he74bf58554f22649
   9:     0x7f2bd97ad3f2 - rust_begin_unwind
  10:     0x7f2bd975de43 - core::panicking::panic_fmt::h187bac1c4ad133a7
  11:     0x7f2bdcedff6a - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter as rustc_errors[6c22f8d2d30a3209]::translation::Translate>::translate_message
  12:     0x7f2bdced5552 - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f2bdced3072 - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter as rustc_errors[6c22f8d2d30a3209]::emitter::Emitter>::emit_diagnostic
  14:     0x7f2bdcee8ec6 - <rustc_errors[6c22f8d2d30a3209]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f2bdcee7abc - <rustc_errors[6c22f8d2d30a3209]::json::JsonEmitter as rustc_errors[6c22f8d2d30a3209]::emitter::Emitter>::emit_diagnostic
  16:     0x7f2bdcf2cea8 - <rustc_errors[6c22f8d2d30a3209]::HandlerInner>::emit_diagnostic
  17:     0x7f2bdcf28b56 - <rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed as rustc_errors[6c22f8d2d30a3209]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f2bdaf51dd8 - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::report_error
  19:     0x7f2bdaecf497 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::fresh_binding
  20:     0x7f2bdb002cb0 - <rustc_ast[b19f54aad5f6fd9b]::ast::Pat>::walk::<<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_pattern_inner::{closure#0}>
  21:     0x7f2bdb00343c - <rustc_ast[b19f54aad5f6fd9b]::ast::Pat>::walk::<<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_pattern_inner::{closure#0}>
  22:     0x7f2bdaecf0f6 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_pattern_top
  23:     0x7f2bdae94dc2 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_arm
  24:     0x7f2bdaf96d4a - rustc_ast[b19f54aad5f6fd9b]::visit::walk_expr::<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>
  25:     0x7f2bdaed66c1 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_expr
  26:     0x7f2bdae951df - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_block
  27:     0x7f2bdaea7f04 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_fn::{closure#1}::{closure#0}
  28:     0x7f2bdaea21cc - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_fn
  29:     0x7f2bdaf9ad9e - rustc_ast[b19f54aad5f6fd9b]::visit::walk_item::<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>
  30:     0x7f2bdaebaa41 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_item
  31:     0x7f2bdae946c7 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_item
  32:     0x7f2bdaf810dd - rustc_ast[b19f54aad5f6fd9b]::visit::walk_crate::<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>
  33:     0x7f2bdaf61dce - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::late_resolve_crate
  34:     0x7f2bdaf26435 - <rustc_session[b0411ee4f4f3e222]::session::Session>::time::<(), <rustc_resolve[c6b27bf52c28fd68]::Resolver>::resolve_crate::{closure#0}>
  35:     0x7f2bda2ab7d0 - <rustc_interface[b6eacd79ecd74a10]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[b6eacd79ecd74a10]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[23cec6303e7deeaf]::result::Result<rustc_ast[b19f54aad5f6fd9b]::ast::Crate, rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  36:     0x7f2bda2909a6 - <rustc_interface[b6eacd79ecd74a10]::queries::Queries>::expansion
  37:     0x7f2bda1752b0 - <rustc_interface[b6eacd79ecd74a10]::interface::Compiler>::enter::<rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}::{closure#2}, core[23cec6303e7deeaf]::result::Result<core[23cec6303e7deeaf]::option::Option<rustc_interface[b6eacd79ecd74a10]::queries::Linker>, rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  38:     0x7f2bda15cc31 - rustc_span[4802920ea0196ebe]::with_source_map::<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_interface[b6eacd79ecd74a10]::interface::create_compiler_and_run<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#1}>
  39:     0x7f2bda18c901 - rustc_interface[b6eacd79ecd74a10]::interface::create_compiler_and_run::<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>
  40:     0x7f2bda158662 - <scoped_tls[c7387018addcf5be]::ScopedKey<rustc_span[4802920ea0196ebe]::SessionGlobals>>::set::<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  41:     0x7f2bda1d396f - std[d52d74294e7af5c7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b6eacd79ecd74a10]::util::run_in_thread_pool_with_globals<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  42:     0x7f2bda15ee5e - std[d52d74294e7af5c7]::panicking::try::<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, core[23cec6303e7deeaf]::panic::unwind_safe::AssertUnwindSafe<<std[d52d74294e7af5c7]::thread::Builder>::spawn_unchecked_<rustc_interface[b6eacd79ecd74a10]::util::run_in_thread_pool_with_globals<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  43:     0x7f2bda1d5190 - <<std[d52d74294e7af5c7]::thread::Builder>::spawn_unchecked_<rustc_interface[b6eacd79ecd74a10]::util::run_in_thread_pool_with_globals<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#1} as core[23cec6303e7deeaf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f2bd97ba675 - std::sys::unix::thread::Thread::new::thread_start::hd7a98ba404bbf168
  45:     0x7f2bd9556b43 - <unknown>
  46:     0x7f2bd95e8a00 - <unknown>
  47:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7fffa41b3 2022-09-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/asm/type-check-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/type-check-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "resolve_attempt_to_use_non_constant_value_in_constant_with_suggestion", attr: None, args: FluentArgs([("current", String("let")), ("ident", String("x")), ("suggestion", String("const"))]), errors: [ResolverError(Reference(Message { id: "suggestion", attribute: None })), ResolverError(Reference(Message { id: "current", attribute: None }))]', compiler/rustc_errors/src/translation.rs:91:17
stack backtrace:
   0:     0x7fb5c081215c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aa99dd858e8a758
   1:     0x7fb5c087ac98 - core::fmt::write::hecd54e75f30f9663
   2:     0x7fb5c08029d1 - std::io::Write::write_fmt::h8988434cba967fd2
   3:     0x7fb5c081514e - std::panicking::default_hook::{{closure}}::haa1da06bdd4cfdeb
   4:     0x7fb5c0814e17 - std::panicking::default_hook::hac6cf1535750f7c5
   5:     0x7fb5c11d5bd4 - rustc_driver[ee9f1d9dabd2d9ea]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb5c0815901 - std::panicking::rust_panic_with_hook::h9901cbb7ff23ab55
   7:     0x7fb5c0815727 - std::panicking::begin_panic_handler::{{closure}}::h4211a6156a7c84d8
   8:     0x7fb5c08126d4 - std::sys_common::backtrace::__rust_end_short_backtrace::he74bf58554f22649
   9:     0x7fb5c08153f2 - rust_begin_unwind
  10:     0x7fb5c07c5e43 - core::panicking::panic_fmt::h187bac1c4ad133a7
  11:     0x7fb5c3f47f6a - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter as rustc_errors[6c22f8d2d30a3209]::translation::Translate>::translate_message
  12:     0x7fb5c3f38d3c - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter as rustc_errors[6c22f8d2d30a3209]::emitter::Emitter>::primary_span_formatted
  13:     0x7fb5c3f3ad10 - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter as rustc_errors[6c22f8d2d30a3209]::emitter::Emitter>::emit_diagnostic
  14:     0x7fb5c3f50ec6 - <rustc_errors[6c22f8d2d30a3209]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7fb5c3f4fabc - <rustc_errors[6c22f8d2d30a3209]::json::JsonEmitter as rustc_errors[6c22f8d2d30a3209]::emitter::Emitter>::emit_diagnostic
  16:     0x7fb5c3f94ea8 - <rustc_errors[6c22f8d2d30a3209]::HandlerInner>::emit_diagnostic
  17:     0x7fb5c3f90b56 - <rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed as rustc_errors[6c22f8d2d30a3209]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7fb5c1fb9dd8 - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::report_error
  19:     0x7fb5c1fe03cb - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::resolve_ident_in_lexical_scope
  20:     0x7fb5c1fc5e53 - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::resolve_path_with_ribs
  21:     0x7fb5c1f3824b - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  22:     0x7fb5c1f37cb9 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::smart_resolve_path
  23:     0x7fb5c1f3e6b6 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_expr
  24:     0x7fb5c1f3d526 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_inline_const
  25:     0x7fb5c1f18026 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_inline_asm
  26:     0x7fb5c1f3e6c1 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_expr
  27:     0x7fb5c1efd1df - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_block
  28:     0x7fb5c1f3e68c - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_expr
  29:     0x7fb5c1efd1df - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_block
  30:     0x7fb5c1f0ff04 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_fn::{closure#1}::{closure#0}
  31:     0x7fb5c1f0a1cc - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_fn
  32:     0x7fb5c2002d9e - rustc_ast[b19f54aad5f6fd9b]::visit::walk_item::<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>
  33:     0x7fb5c1f22a41 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_item
  34:     0x7fb5c1efc6c7 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_item
  35:     0x7fb5c1fe90dd - rustc_ast[b19f54aad5f6fd9b]::visit::walk_crate::<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>
  36:     0x7fb5c1fc9dce - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::late_resolve_crate
  37:     0x7fb5c1f8e435 - <rustc_session[b0411ee4f4f3e222]::session::Session>::time::<(), <rustc_resolve[c6b27bf52c28fd68]::Resolver>::resolve_crate::{closure#0}>
  38:     0x7fb5c13137d0 - <rustc_interface[b6eacd79ecd74a10]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[b6eacd79ecd74a10]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[23cec6303e7deeaf]::result::Result<rustc_ast[b19f54aad5f6fd9b]::ast::Crate, rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  39:     0x7fb5c12f89a6 - <rustc_interface[b6eacd79ecd74a10]::queries::Queries>::expansion
  40:     0x7fb5c11dd2b0 - <rustc_interface[b6eacd79ecd74a10]::interface::Compiler>::enter::<rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}::{closure#2}, core[23cec6303e7deeaf]::result::Result<core[23cec6303e7deeaf]::option::Option<rustc_interface[b6eacd79ecd74a10]::queries::Linker>, rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  41:     0x7fb5c11c4c31 - rustc_span[4802920ea0196ebe]::with_source_map::<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_interface[b6eacd79ecd74a10]::interface::create_compiler_and_run<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#1}>
  42:     0x7fb5c11f4901 - rustc_interface[b6eacd79ecd74a10]::interface::create_compiler_and_run::<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>
  43:     0x7fb5c11c0662 - <scoped_tls[c7387018addcf5be]::ScopedKey<rustc_span[4802920ea0196ebe]::SessionGlobals>>::set::<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  44:     0x7fb5c123b96f - std[d52d74294e7af5c7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b6eacd79ecd74a10]::util::run_in_thread_pool_with_globals<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  45:     0x7fb5c11c6e5e - std[d52d74294e7af5c7]::panicking::try::<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, core[23cec6303e7deeaf]::panic::unwind_safe::AssertUnwindSafe<<std[d52d74294e7af5c7]::thread::Builder>::spawn_unchecked_<rustc_interface[b6eacd79ecd74a10]::util::run_in_thread_pool_with_globals<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7fb5c123d190 - <<std[d52d74294e7af5c7]::thread::Builder>::spawn_unchecked_<rustc_interface[b6eacd79ecd74a10]::util::run_in_thread_pool_with_globals<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#1} as core[23cec6303e7deeaf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7fb5c0822675 - std::sys::unix::thread::Thread::new::thread_start::hd7a98ba404bbf168
  48:     0x7fb5c05beb43 - <unknown>
  49:     0x7fb5c0650a00 - <unknown>
  50:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7fffa41b3 2022-09-03) running on x86_64-unknown-linux-gnu

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
   0:     0x7f9688a1015c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aa99dd858e8a758
   1:     0x7f9688a78c98 - core::fmt::write::hecd54e75f30f9663
   1:     0x7f9688a78c98 - core::fmt::write::hecd54e75f30f9663
   2:     0x7f9688a009d1 - std::io::Write::write_fmt::h8988434cba967fd2
   3:     0x7f9688a1314e - std::panicking::default_hook::{{closure}}::haa1da06bdd4cfdeb
   4:     0x7f9688a12e17 - std::panicking::default_hook::hac6cf1535750f7c5
   5:     0x7f96893d3bd4 - rustc_driver[ee9f1d9dabd2d9ea]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f9688a13901 - std::panicking::rust_panic_with_hook::h9901cbb7ff23ab55
   7:     0x7f9688a13727 - std::panicking::begin_panic_handler::{{closure}}::h4211a6156a7c84d8
   8:     0x7f9688a106d4 - std::sys_common::backtrace::__rust_end_short_backtrace::he74bf58554f22649
   9:     0x7f9688a133f2 - rust_begin_unwind
  10:     0x7f96889c3e43 - core::panicking::panic_fmt::h187bac1c4ad133a7
  11:     0x7f968c145f6a - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter as rustc_errors[6c22f8d2d30a3209]::translation::Translate>::translate_message
  12:     0x7f968c136d3c - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter as rustc_errors[6c22f8d2d30a3209]::emitter::Emitter>::primary_span_formatted
  13:     0x7f968c138d10 - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter as rustc_errors[6c22f8d2d30a3209]::emitter::Emitter>::emit_diagnostic
  14:     0x7f968c14eec6 - <rustc_errors[6c22f8d2d30a3209]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f968c14dabc - <rustc_errors[6c22f8d2d30a3209]::json::JsonEmitter as rustc_errors[6c22f8d2d30a3209]::emitter::Emitter>::emit_diagnostic
  16:     0x7f968c192ea8 - <rustc_errors[6c22f8d2d30a3209]::HandlerInner>::emit_diagnostic
  17:     0x7f968c18eb56 - <rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed as rustc_errors[6c22f8d2d30a3209]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f968a1b7dd8 - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::report_error
  19:     0x7f968a1de3cb - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::resolve_ident_in_lexical_scope
  20:     0x7f968a1c3e53 - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::resolve_path_with_ribs
  21:     0x7f968a13624b - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  22:     0x7f968a135cb9 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::smart_resolve_path
  23:     0x7f968a13c6b6 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_expr
  24:     0x7f968a13b526 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_inline_const
  25:     0x7f968a116026 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_inline_asm
  26:     0x7f968a13c6c1 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_expr
  27:     0x7f968a0fb1df - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_block
  28:     0x7f968a13c68c - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_expr
  29:     0x7f968a0fb1df - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_block
  30:     0x7f968a10df04 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_fn::{closure#1}::{closure#0}
  31:     0x7f968a1081cc - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_fn
  32:     0x7f968a200d9e - rustc_ast[b19f54aad5f6fd9b]::visit::walk_item::<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>
  33:     0x7f968a120a41 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_item
  34:     0x7f968a0fa6c7 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_item
  35:     0x7f968a1e70dd - rustc_ast[b19f54aad5f6fd9b]::visit::walk_crate::<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>
  36:     0x7f968a1c7dce - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::late_resolve_crate
  37:     0x7f968a18c435 - <rustc_session[b0411ee4f4f3e222]::session::Session>::time::<(), <rustc_resolve[c6b27bf52c28fd68]::Resolver>::resolve_crate::{closure#0}>
  38:     0x7f96895117d0 - <rustc_interface[b6eacd79ecd74a10]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[b6eacd79ecd74a10]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[23cec6303e7deeaf]::result::Result<rustc_ast[b19f54aad5f6fd9b]::ast::Crate, rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  39:     0x7f96894f69a6 - <rustc_interface[b6eacd79ecd74a10]::queries::Queries>::expansion
  40:     0x7f96893db2b0 - <rustc_interface[b6eacd79ecd74a10]::interface::Compiler>::enter::<rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}::{closure#2}, core[23cec6303e7deeaf]::result::Result<core[23cec6303e7deeaf]::option::Option<rustc_interface[b6eacd79ecd74a10]::queries::Linker>, rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  41:     0x7f96893c2c31 - rustc_span[4802920ea0196ebe]::with_source_map::<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_interface[b6eacd79ecd74a10]::interface::create_compiler_and_run<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#1}>
  42:     0x7f96893f2901 - rustc_interface[b6eacd79ecd74a10]::interface::create_compiler_and_run::<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>
  43:     0x7f96893be662 - <scoped_tls[c7387018addcf5be]::ScopedKey<rustc_span[4802920ea0196ebe]::SessionGlobals>>::set::<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  44:     0x7f968943996f - std[d52d74294e7af5c7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b6eacd79ecd74a10]::util::run_in_thread_pool_with_globals<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
  45:     0x7f96893c4e5e - std[d52d74294e7af5c7]::panicking::try::<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, core[23cec6303e7deeaf]::panic::unwind_safe::AssertUnwindSafe<<std[d52d74294e7af5c7]::thread::Builder>::spawn_unchecked_<rustc_interface[b6eacd79ecd74a10]::util::run_in_thread_pool_with_globals<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7f968943b190 - <<std[d52d74294e7af5c7]::thread::Builder>::spawn_unchecked_<rustc_interface[b6eacd79ecd74a10]::util::run_in_thread_pool_with_globals<rustc_interface[b6eacd79ecd74a10]::interface::run_compiler<core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>, rustc_driver[ee9f1d9dabd2d9ea]::run_compiler::{closure#1}>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#0}, core[23cec6303e7deeaf]::result::Result<(), rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>::{closure#1} as core[23cec6303e7deeaf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7f9688a20675 - std::sys::unix::thread::Thread::new::thread_start::hd7a98ba404bbf168
  48:     0x7f96887bcb43 - <unknown>
  49:     0x7f968884ea00 - <unknown>
  50:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7fffa41b3 2022-09-03) running on x86_64-unknown-linux-gnu

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
   0:     0x7f98f9f5615c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3aa99dd858e8a758
   1:     0x7f98f9fbec98 - core::fmt::write::hecd54e75f30f9663
   2:     0x7f98f9f469d1 - std::io::Write::write_fmt::h8988434cba967fd2
   2:     0x7f98f9f469d1 - std::io::Write::write_fmt::h8988434cba967fd2
   3:     0x7f98f9f5914e - std::panicking::default_hook::{{closure}}::haa1da06bdd4cfdeb
   4:     0x7f98f9f58e17 - std::panicking::default_hook::hac6cf1535750f7c5
   5:     0x7f98fa919bd4 - rustc_driver[ee9f1d9dabd2d9ea]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f98f9f59901 - std::panicking::rust_panic_with_hook::h9901cbb7ff23ab55
   7:     0x7f98f9f59727 - std::panicking::begin_panic_handler::{{closure}}::h4211a6156a7c84d8
   8:     0x7f98f9f566d4 - std::sys_common::backtrace::__rust_end_short_backtrace::he74bf58554f22649
   9:     0x7f98f9f593f2 - rust_begin_unwind
  10:     0x7f98f9f09e43 - core::panicking::panic_fmt::h187bac1c4ad133a7
  11:     0x7f98fd68bf6a - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter as rustc_errors[6c22f8d2d30a3209]::translation::Translate>::translate_message
  12:     0x7f98fd681552 - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f98fd67f259 - <rustc_errors[6c22f8d2d30a3209]::emitter::EmitterWriter as rustc_errors[6c22f8d2d30a3209]::emitter::Emitter>::emit_diagnostic
  14:     0x7f98fd694ec6 - <rustc_errors[6c22f8d2d30a3209]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f98fd693abc - <rustc_errors[6c22f8d2d30a3209]::json::JsonEmitter as rustc_errors[6c22f8d2d30a3209]::emitter::Emitter>::emit_diagnostic
  16:     0x7f98fd6d8ea8 - <rustc_errors[6c22f8d2d30a3209]::HandlerInner>::emit_diagnostic
  17:     0x7f98fd6d4b56 - <rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed as rustc_errors[6c22f8d2d30a3209]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f98fb6fddd8 - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::report_error
  19:     0x7f98fb724285 - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::resolve_ident_in_lexical_scope
  20:     0x7f98fb709e53 - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::resolve_path_with_ribs
  21:     0x7f98fb67c24b - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  22:     0x7f98fb67bcb9 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::smart_resolve_path
  23:     0x7f98fb6826b6 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_expr
  24:     0x7f98fb6826c1 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_expr
  25:     0x7f98fb6411df - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_block
  26:     0x7f98fb653f04 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_fn::{closure#1}::{closure#0}
  27:     0x7f98fb64e1cc - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_fn
  28:     0x7f98fb746d9e - rustc_ast[b19f54aad5f6fd9b]::visit::walk_item::<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>
  29:     0x7f98fb666a41 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_item
  30:     0x7f98fb6406c7 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_item
  31:     0x7f98fb6411df - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_block
  32:     0x7f98fb653f04 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_fn::{closure#1}::{closure#0}
  33:     0x7f98fb64e1cc - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_fn
  34:     0x7f98fb746d9e - rustc_ast[b19f54aad5f6fd9b]::visit::walk_item::<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>
  35:     0x7f98fb666a41 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>::resolve_item
  36:     0x7f98fb6406c7 - <rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor as rustc_ast[b19f54aad5f6fd9b]::visit::Visitor>::visit_item
  37:     0x7f98fb72d0dd - rustc_ast[b19f54aad5f6fd9b]::visit::walk_crate::<rustc_resolve[c6b27bf52c28fd68]::late::LateResolutionVisitor>
  38:     0x7f98fb70ddce - <rustc_resolve[c6b27bf52c28fd68]::Resolver>::late_resolve_crate
  39:     0x7f98fb6d2435 - <rustc_session[b0411ee4f4f3e222]::session::Session>::time::<(), <rustc_resolve[c6b27bf52c28fd68]::Resolver>::resolve_crate::{closure#0}>
  40:     0x7f98faa577d0 - <rustc_interface[b6eacd79ecd74a10]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[b6eacd79ecd74a10]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[23cec6303e7deeaf]::result::Result<rustc_ast[b19f54aad5f6fd9b]::ast::Crate, rustc_errors[6c22f8d2d30a3209]::ErrorGuaranteed>>
