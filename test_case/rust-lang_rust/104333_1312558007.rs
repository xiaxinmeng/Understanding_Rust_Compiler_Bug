plain
iiii.....................i..................i........................................... 264/13827
........................................................................................ 352/13827
........................................................................................ 440/13827
........................................................................................ 528/13827
......................................................................F...........F.F... 616/13827
......................F.F.............................F................................. 704/13827
............F.....................F.F................................................... 792/13827
....F.........F.i....F.................................................................. 880/13827
........................................................................................ 1056/13827
........................................................................................ 1144/13827
........................................................................................ 1232/13827
........................................................................................ 1320/13827
---
.............................................i....i..................................... 7216/13827
....i..................i.............i.................................................. 7304/13827
.............i.......................................................................... 7392/13827
..................................i..................................................... 7480/13827
......................................................F.F..............F................ 7568/13827
.................................F...................................................... 7656/13827
......................................................i................................. 7832/13827
........................................................................................ 7920/13827
...................................................................ii................... 8008/13827
........................................................................................ 8096/13827
---
........................................................................................ 10648/13827
........................................................................................ 10736/13827
........................................................................................ 10824/13827
........................................................................................ 10912/13827
...........iiiii...i....i.iF............................................................ 11000/13827
..........i............................................................................. 11176/13827
.....................iiiiiii..iiiiiiiii.i............................................... 11264/13827
........................................................................................ 11352/13827
........................................................................................ 11440/13827
---
failures:

---- [ui] src/test/ui/async-await/async-borrowck-escaping-closure-error.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-borrowck-escaping-closure-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-closure-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-closure-error/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("foo"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("foo"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7f455f744d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7f455f7b4278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7f455f7367d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7f455f744b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7f455f744b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7f455f747e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7f455f747b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7f456014cb94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f455f7485d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7f455f748337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7f455f745254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7f455f748002 - rust_begin_unwind
  11:     0x7f455f6f9963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7f45631569fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7f456315949a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7f4561094662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7f456111d0d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7f456111e228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7f456111e23e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  18:     0x7f45610e0dd0 - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7f456111e23e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  20:     0x7f45610e0dfc - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7f456111e23e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  22:     0x7f45610dacca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7f456111d9fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  24:     0x7f45610e45ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  25:     0x7f456111d864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  26:     0x7f45610cce1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  27:     0x7f456111ce9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  28:     0x7f456108a9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  29:     0x7f4562612e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  30:     0x7f456260e513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  31:     0x7f456260e12b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  32:     0x7f456026b31a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  33:     0x7f45602a1517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  34:     0x7f456030914b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7f45602893f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  36:     0x7f45601bcc93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7f456014e2de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  38:     0x7f45601afafc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  39:     0x7f45601ab749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  40:     0x7f45601b51c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  41:     0x7f456015d3fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7f455f7549be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  43:     0x7f455f4ecb43 - <unknown>
  44:     0x7f455f57ea00 - <unknown>
  45:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/async-closure.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure.default/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure.default/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("async_closure"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("async_closure"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7fd50c238d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7fd50c2a8278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7fd50c22a7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7fd50c238b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7fd50c238b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7fd50c23be84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7fd50c23bb49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7fd50cc40b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fd50c23c5d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7fd50c23c337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7fd50c239254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7fd50c23c002 - rust_begin_unwind
  11:     0x7fd50c1ed963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7fd50fc4a9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7fd50fc4d49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7fd50db88662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7fd50dc110d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7fd50dc12228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7fd50dc1223e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  18:     0x7fd50dbd4dd0 - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7fd50dc1223e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  20:     0x7fd50dbcecca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7fd50dc119fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  22:     0x7fd50dbd85ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7fd50dc11864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  24:     0x7fd50dbc0e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  25:     0x7fd50dc10e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  26:     0x7fd50db7e9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  27:     0x7fd50f106e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  28:     0x7fd50f102513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  29:     0x7fd50f10212b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  30:     0x7fd50cd5f31a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  31:     0x7fd50cd95517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  32:     0x7fd50cdfd14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7fd50cd7d3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  34:     0x7fd50ccb0c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7fd50cc422de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  36:     0x7fd50cca3afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7fd50cc9f749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  38:     0x7fd50cca91c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  39:     0x7fd50cc513fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7fd50bfe0b43 - <unknown>
  42:     0x7fd50c072a00 - <unknown>
  43:                0x0 - <unknown>

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/async-closure.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure.nomiropt/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure.nomiropt/auxiliary" "-Z" "mir-opt-level=0" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("async_closure"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("async_closure"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7febe37cad1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7febe383a278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7febe37bc7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7febe37cab21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7febe37cab21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7febe37cde84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7febe37cdb49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7febe41d2b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7febe37ce5d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7febe37ce337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7febe37cb254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7febe37ce002 - rust_begin_unwind
  11:     0x7febe377f963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7febe71dc9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7febe71df49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7febe511a662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7febe51a30d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7febe51a4228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7febe51a423e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  18:     0x7febe5166dd0 - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7febe51a423e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  20:     0x7febe5160cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7febe51a39fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  22:     0x7febe516a5ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7febe51a3864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  24:     0x7febe5152e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  25:     0x7febe51a2e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  26:     0x7febe51109b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  27:     0x7febe6698e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  28:     0x7febe6694513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  29:     0x7febe669412b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  30:     0x7febe42f131a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  31:     0x7febe4327517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  32:     0x7febe438f14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7febe430f3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  34:     0x7febe4242c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7febe41d42de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  36:     0x7febe4235afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7febe4231749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  38:     0x7febe423b1c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  39:     0x7febe41e33fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7febe3572b43 - <unknown>
  42:     0x7febe3604a00 - <unknown>
  43:                0x0 - <unknown>

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/feature-async-closure.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/feature-async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/feature-async-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/feature-async-closure/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("f"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("f"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7fcbf0afdd1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7fcbf0b6d278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7fcbf0aef7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7fcbf0afdb21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7fcbf0afdb21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7fcbf0b00e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7fcbf0b00b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7fcbf1505b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fcbf0b015d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7fcbf0b01337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7fcbf0afe254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7fcbf0b01002 - rust_begin_unwind
  11:     0x7fcbf0ab2963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7fcbf450f9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7fcbf451249a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7fcbf244d662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7fcbf24d60d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7fcbf24d7228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7fcbf2486706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7fcbf2493cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7fcbf24d69fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7fcbf249d5ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7fcbf24d6864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7fcbf2485e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7fcbf24d5e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7fcbf24439b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7fcbf39cbe46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7fcbf39c7513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7fcbf39c712b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7fcbf162431a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7fcbf165a517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7fcbf16c214b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7fcbf16423f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7fcbf1575c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7fcbf15072de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7fcbf1568afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7fcbf1564749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7fcbf156e1c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7fcbf15163fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fcbf0b0d9be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  39:     0x7fcbf08a5b43 - <unknown>
  40:     0x7fcbf0937a00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/generator-desc.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/generator-desc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Block))), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
stack backtrace:
   0:     0x7ff7cddcad1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7ff7cde3a278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7ff7cddbc7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7ff7cddcab21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7ff7cddcde84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7ff7cddcdb49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7ff7ce7d2b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff7cddce5d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7ff7cddce337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7ff7cddcb254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7ff7cddce002 - rust_begin_unwind
  11:     0x7ff7cdd7f963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7ff7d17dc9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7ff7d17df49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7ff7cf71a662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7ff7cf7a30d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7ff7cf7a41fa - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7ff7cf7a423e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  18:     0x7ff7cf766dd0 - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7ff7cf7a423e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  20:     0x7ff7cf766dfc - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7ff7cf7a423e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  22:     0x7ff7cf760cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7ff7cf7a39fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  24:     0x7ff7cf76a5ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  25:     0x7ff7cf7a3864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  26:     0x7ff7cf752e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  27:     0x7ff7cf7a2e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  28:     0x7ff7cf7109b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  29:     0x7ff7d0c98e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  30:     0x7ff7d0c94513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  31:     0x7ff7d0c9412b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  32:     0x7ff7ce8f131a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  33:     0x7ff7ce927517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  34:     0x7ff7ce98f14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7ff7ce90f3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  36:     0x7ff7ce842c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7ff7ce7d42de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  38:     0x7ff7ce835afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  39:     0x7ff7ce831749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  40:     0x7ff7ce83b1c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  41:     0x7ff7ce7e33fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7ff7cddda9be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  43:     0x7ff7cdb72b43 - <unknown>
  44:     0x7ff7cdc04a00 - <unknown>
  45:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/issue-61793.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61793.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61793" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61793/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Block))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7f1399939d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7f13999a9278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7f139992b7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7f1399939b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7f1399939b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7f139993ce84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7f139993cb49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7f139a341b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f139993d5d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7f139993d337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7f139993a254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7f139993d002 - rust_begin_unwind
  11:     0x7f13998ee963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7f139d34b9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7f139d34e49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7f139b289662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7f139b3120d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7f139b313228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7f139b2cfcca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7f139b3129fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  19:     0x7f139b2d95ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  20:     0x7f139b312864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  21:     0x7f139b2c1e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  22:     0x7f139b311e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  23:     0x7f139b27f9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  24:     0x7f139c807e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  25:     0x7f139c803513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  26:     0x7f139c80312b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  27:     0x7f139a46031a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  28:     0x7f139a496517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  29:     0x7f139a4fe14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  30:     0x7f139a47e3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  31:     0x7f139a3b1c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  32:     0x7f139a3432de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7f139a3a4afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  34:     0x7f139a3a0749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7f139a3aa1c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7f139a3523fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f13996e1b43 - <unknown>
  39:     0x7f1399773a00 - <unknown>
  40:                0x0 - <unknown>

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74072-lifetime-name-annotations" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74072-lifetime-name-annotations/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("async_closure"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("async_closure"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7f63f0351d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7f63f03c1278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7f63f03437d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7f63f0351b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7f63f0351b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7f63f0354e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7f63f0354b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7f63f0d59b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f63f03555d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7f63f0355337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7f63f0352254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7f63f0355002 - rust_begin_unwind
  11:     0x7f63f0306963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7f63f3d639fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7f63f3d6649a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7f63f1ca1662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7f63f1d2a0d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7f63f1d2b228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7f63f1d2b23e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  18:     0x7f63f1ceddd0 - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7f63f1d2b23e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  20:     0x7f63f1ce7cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7f63f1d2a9fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  22:     0x7f63f1cf15ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7f63f1d2a864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  24:     0x7f63f1cd9e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  25:     0x7f63f1d29e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  26:     0x7f63f1c979b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  27:     0x7f63f321fe46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  28:     0x7f63f321b513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  29:     0x7f63f321b12b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  30:     0x7f63f0e7831a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  31:     0x7f63f0eae517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  32:     0x7f63f0f1614b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7f63f0e963f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  34:     0x7f63f0dc9c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7f63f0d5b2de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  36:     0x7f63f0dbcafc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7f63f0db8749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  38:     0x7f63f0dc21c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  39:     0x7f63f0d6a3fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f63f03619be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  41:     0x7f63f00f9b43 - <unknown>
  42:     0x7f63f018ba00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/issues/issue-62009-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-2/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7f4696f39d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7f4696fa9278 - core::fmt::write::h8dc0e6ab2c0956b1
   1:     0x7f4696fa9278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7f4696f2b7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7f4696f39b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7f4696f3ce84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7f4696f3cb49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7f4697941b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f4696f3d5d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7f4696f3d337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7f4696f3a254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7f4696f3d002 - rust_begin_unwind
  11:     0x7f4696eee963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7f469a94b9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7f469a94e49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7f4698889662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7f46989120d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7f4698913228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7f469891323e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  18:     0x7f46988d5dd0 - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7f469891323e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  20:     0x7f469891323e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  21:     0x7f46988cfcca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  22:     0x7f46989129fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  23:     0x7f46988d95ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  24:     0x7f4698912864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  25:     0x7f46988c1e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  26:     0x7f4698911e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  27:     0x7f469887f9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  28:     0x7f4699e07e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  29:     0x7f4699e03513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  30:     0x7f4699e0312b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  31:     0x7f4697a6031a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  32:     0x7f4697a96517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  33:     0x7f4697afe14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  34:     0x7f4697a7e3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  35:     0x7f46979b1c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7f46979432de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  37:     0x7f46979a4afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  38:     0x7f46979a0749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  39:     0x7f46979aa1c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  40:     0x7f46979523fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f4696f499be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  42:     0x7f4696ce1b43 - <unknown>
  43:     0x7f4696d73a00 - <unknown>
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/issues/issue-62009-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Block))), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7f6e596a9d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7f6e59719278 - core::fmt::write::h8dc0e6ab2c0956b1
   1:     0x7f6e59719278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7f6e5969b7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7f6e596a9b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7f6e596ace84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7f6e596acb49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7f6e5a0b1b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f6e596ad5d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7f6e596ad337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7f6e596aa254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7f6e596ad002 - rust_begin_unwind
  11:     0x7f6e5965e963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7f6e5d0bb9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7f6e5d0be49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7f6e5aff9662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7f6e5b0820d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7f6e5b0831fa - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7f6e5b08323e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  18:     0x7f6e5b08323e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  19:     0x7f6e5b03fcca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  20:     0x7f6e5b0829fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  21:     0x7f6e5b0495ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  22:     0x7f6e5b082864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  23:     0x7f6e5b031e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  24:     0x7f6e5b081e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  25:     0x7f6e5afef9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  26:     0x7f6e5c577e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  27:     0x7f6e5c573513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7f6e5c57312b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  29:     0x7f6e5a1d031a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  30:     0x7f6e5a206517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  31:     0x7f6e5a26e14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  32:     0x7f6e5a1ee3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  33:     0x7f6e5a121c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  34:     0x7f6e5a0b32de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  35:     0x7f6e5a114afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7f6e5a110749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7f6e5a11a1c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  38:     0x7f6e5a0c23fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f6e59451b43 - <unknown>
  41:     0x7f6e594e3a00 - <unknown>
  42:                0x0 - <unknown>

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/no-params-non-move-async-closure.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-params-non-move-async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-params-non-move-async-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-params-non-move-async-closure/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7fc91fb9dd1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7fc91fc0d278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7fc91fb8f7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7fc91fb9db21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7fc91fb9db21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7fc91fba0e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7fc91fba0b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7fc9205a5b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc91fba15d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7fc91fba1337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7fc91fb9e254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7fc91fba1002 - rust_begin_unwind
  11:     0x7fc91fb52963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7fc9235af9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7fc9235b249a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7fc9214ed662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7fc9215760d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7fc921577228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7fc921526706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7fc921533cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7fc9215769fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7fc92153d5ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7fc921576864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7fc921525e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7fc921575e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7fc9214e39b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7fc922a6be46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7fc922a67513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7fc922a6712b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7fc9206c431a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7fc9206fa517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7fc92076214b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7fc9206e23f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7fc920615c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7fc9205a72de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7fc920608afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7fc920604749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7fc92060e1c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7fc9205b63fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fc91fbad9be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  39:     0x7fc91f945b43 - <unknown>
  40:     0x7fc91f9d7a00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/suggest-missing-await-closure.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-missing-await-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("suggest_await_in_async_closure"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Fn))), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("suggest_await_in_async_closure"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Fn))), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7fa282dccd1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7fa282e3c278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7fa282dbe7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7fa282dccb21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7fa282dccb21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7fa282dcfe84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7fa282dcfb49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7fa2837d4b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa282dd05d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7fa282dd0337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7fa282dcd254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7fa282dd0002 - rust_begin_unwind
  11:     0x7fa282d81963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7fa2867de9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7fa2867e149a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7fa28471c662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7fa2847a50d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7fa2847a6228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7fa2847549ea - rustc_ast[e60304300e2a2326]::visit::walk_block::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7fa2847a5bb0 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  19:     0x7fa28476c5ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  20:     0x7fa2847a5864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  21:     0x7fa284754e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  22:     0x7fa2847a4e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  23:     0x7fa2847129b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  24:     0x7fa285c9ae46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  25:     0x7fa285c96513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  26:     0x7fa285c9612b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  27:     0x7fa2838f331a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  28:     0x7fa283929517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  29:     0x7fa28399114b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  30:     0x7fa2839113f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  31:     0x7fa283844c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  32:     0x7fa2837d62de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7fa283837afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  34:     0x7fa283833749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7fa28383d1c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7fa2837e53fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7fa282ddc9be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  38:     0x7fa282b74b43 - <unknown>
  39:     0x7fa282c06a00 - <unknown>
  40:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/async-await/try-on-option-in-async.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/try-on-option-in-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/try-on-option-in-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/try-on-option-in-async/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("async_closure_containing_fn"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Fn))), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("async_closure_containing_fn"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Fn))), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7fd40f023d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7fd40f093278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7fd40f0157d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7fd40f023b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7fd40f023b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7fd40f026e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7fd40f026b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7fd40fa2bb94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fd40f0275d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7fd40f027337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7fd40f024254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7fd40f027002 - rust_begin_unwind
  11:     0x7fd40efd8963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7fd412a359fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7fd412a3849a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7fd410973662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7fd4109fc0d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7fd4109fd228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7fd4109ac706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7fd4109ab9ea - rustc_ast[e60304300e2a2326]::visit::walk_block::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7fd4109fcbb0 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7fd4109c35ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7fd4109fc864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7fd4109abe1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7fd4109fbe9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7fd4109699b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7fd411ef1e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7fd411eed513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7fd411eed12b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7fd40fb4a31a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7fd40fb80517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7fd40fbe814b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7fd40fb683f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7fd40fa9bc93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7fd40fa2d2de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7fd40fa8eafc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7fd40fa8a749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7fd40fa941c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7fd40fa3c3fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fd40f0339be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  39:     0x7fd40edcbb43 - <unknown>
  40:     0x7fd40ee5da00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/closures/2229_closure_analysis/migrations/issue-86753.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/migrations/issue-86753.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/issue-86753" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/issue-86753/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("http"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Block))), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("http"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
stack backtrace:
   0:     0x7fe37e8f0d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7fe37e960278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7fe37e8e27d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7fe37e8f0b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7fe37e8f3e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7fe37e8f3b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7fe37f2f8b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe37e8f45d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7fe37e8f4337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7fe37e8f1254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7fe37e8f4002 - rust_begin_unwind
  11:     0x7fe37e8a5963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7fe3823029fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7fe38230549a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7fe380240662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7fe3802c90d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7fe3802ca1fa - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7fe380279706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7fe380286cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7fe3802c99fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7fe3802905ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7fe3802c9864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7fe380278e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7fe3802c8e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7fe3802369b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7fe3817bee46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7fe3817ba513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7fe3817ba12b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7fe37f41731a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7fe37f44d517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7fe37f4b514b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7fe37f4353f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7fe37f368c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7fe37f2fa2de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7fe37f35bafc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7fe37f357749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7fe37f3611c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7fe37f3093fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7fe37e698b43 - <unknown>
  40:     0x7fe37e72aa00 - <unknown>
  41:                0x0 - <unknown>

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/closures/binder/async-closure-with-binder.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/binder/async-closure-with-binder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/binder/async-closure-with-binder" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/binder/async-closure-with-binder/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7f96a9243d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7f96a92b3278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7f96a92357d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7f96a9243b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7f96a9243b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7f96a9246e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7f96a9246b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7f96a9c4bb94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f96a92475d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7f96a9247337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7f96a9244254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7f96a9247002 - rust_begin_unwind
  11:     0x7f96a91f8963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7f96acc559fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7f96acc5849a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7f96aab93662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7f96aac1c0d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7f96aac1d228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7f96aabd9cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7f96aac1c9fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  19:     0x7f96aabe35ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  20:     0x7f96aac1c864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  21:     0x7f96aabcbe1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  22:     0x7f96aac1be9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  23:     0x7f96aab899b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  24:     0x7f96ac111e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  25:     0x7f96ac10d513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  26:     0x7f96ac10d12b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  27:     0x7f96a9d6a31a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  28:     0x7f96a9da0517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  29:     0x7f96a9e0814b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  30:     0x7f96a9d883f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  31:     0x7f96a9cbbc93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  32:     0x7f96a9c4d2de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7f96a9caeafc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  34:     0x7f96a9caa749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7f96a9cb41c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7f96a9c5c3fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7f96a92539be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  38:     0x7f96a8febb43 - <unknown>
  39:     0x7f96a907da00 - <unknown>
  40:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/closures/local-type-mix.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/local-type-mix.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/local-type-mix" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/local-type-mix/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7fe4c3e70d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7fe4c3ee0278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7fe4c3e627d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7fe4c3e70b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7fe4c3e70b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7fe4c3e73e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7fe4c3e73b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7fe4c4878b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe4c3e745d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7fe4c3e74337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7fe4c3e71254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7fe4c3e74002 - rust_begin_unwind
  11:     0x7fe4c3e25963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7fe4c78829fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7fe4c788549a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7fe4c57c0662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7fe4c58490d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7fe4c584a228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7fe4c57f9706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7fe4c5806cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7fe4c58499fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7fe4c58105ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7fe4c5849864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7fe4c57f8e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7fe4c5848e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7fe4c57b69b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7fe4c6d3ee46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7fe4c6d3a513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7fe4c6d3a12b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7fe4c499731a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7fe4c49cd517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7fe4c4a3514b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7fe4c49b53f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7fe4c48e8c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7fe4c487a2de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7fe4c48dbafc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7fe4c48d7749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7fe4c48e11c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7fe4c48893fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7fe4c3c18b43 - <unknown>
  40:     0x7fe4c3caaa00 - <unknown>
  41:                0x0 - <unknown>

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/lint/unused/lint-unused-mut-variables.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-variables" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-variables/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7f64dc4cfd1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7f64dc53f278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7f64dc4c17d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7f64dc4cfb21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7f64dc4cfb21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7f64dc4d2e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7f64dc4d2b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7f64dced7b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f64dc4d35d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7f64dc4d3337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7f64dc4d0254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7f64dc4d3002 - rust_begin_unwind
  11:     0x7f64dc484963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7f64dfee19fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7f64dfee449a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7f64dde1f662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7f64ddea80d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7f64ddea9228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7f64dde58706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7f64dde65cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7f64ddea89fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7f64dde6f5ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7f64ddea8864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7f64dde57e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7f64ddea7e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7f64dde159b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7f64df39de46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7f64df399513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7f64df39912b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7f64dcff631a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7f64dd02c517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7f64dd09414b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7f64dd0143f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7f64dcf47c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7f64dced92de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7f64dcf3aafc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7f64dcf36749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7f64dcf401c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7f64dcee83fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f64dc4df9be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  39:     0x7f64dc277b43 - <unknown>
  40:     0x7f64dc309a00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/lint/unused/lint-unused-variables.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/lint-unused-variables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-variables" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-variables/auxiliary" "--cfg" "something" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7f40b1938d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7f40b19a8278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7f40b192a7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7f40b1938b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7f40b1938b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7f40b193be84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7f40b193bb49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7f40b2340b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f40b193c5d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7f40b193c337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7f40b1939254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7f40b193c002 - rust_begin_unwind
  11:     0x7f40b18ed963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7f40b534a9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7f40b534d49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7f40b3288662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7f40b33110d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7f40b3312228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7f40b32c1706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7f40b32cecca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7f40b33119fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7f40b32d85ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7f40b3311864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7f40b32c0e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7f40b3310e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7f40b327e9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7f40b4806e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7f40b4802513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7f40b480212b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7f40b245f31a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7f40b2495517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7f40b24fd14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7f40b247d3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7f40b23b0c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7f40b23422de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7f40b23a3afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7f40b239f749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7f40b23a91c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7f40b23513fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f40b16e0b43 - <unknown>
  40:     0x7f40b1772a00 - <unknown>
  41:                0x0 - <unknown>

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/lint/unused/unused-closure.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-closure/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("unused"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("unused"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Block))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7fbe47c86d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7fbe47cf6278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7fbe47c787d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7fbe47c86b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7fbe47c86b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7fbe47c89e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7fbe47c89b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7fbe4868eb94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fbe47c8a5d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7fbe47c8a337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7fbe47c87254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7fbe47c8a002 - rust_begin_unwind
  11:     0x7fbe47c3b963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7fbe4b6989fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7fbe4b69b49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7fbe495d6662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7fbe4965f0d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7fbe49660228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7fbe4961ccca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7fbe4965f9fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  19:     0x7fbe496265ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  20:     0x7fbe4965f864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  21:     0x7fbe4960ee1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  22:     0x7fbe4965ee9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  23:     0x7fbe495cc9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  24:     0x7fbe4ab54e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  25:     0x7fbe4ab50513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  26:     0x7fbe4ab5012b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  27:     0x7fbe487ad31a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  28:     0x7fbe487e3517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  29:     0x7fbe4884b14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  30:     0x7fbe487cb3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  31:     0x7fbe486fec93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  32:     0x7fbe486902de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7fbe486f1afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  34:     0x7fbe486ed749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7fbe486f71c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7fbe4869f3fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fbe47a2eb43 - <unknown>
  39:     0x7fbe47ac0a00 - <unknown>
  40:                0x0 - <unknown>

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/liveness/liveness-upvars.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-upvars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-upvars" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-upvars/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("f"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("f"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Block))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7f2d4d234d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7f2d4d2a4278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7f2d4d2267d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7f2d4d234b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7f2d4d234b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7f2d4d237e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7f2d4d237b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7f2d4dc3cb94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f2d4d2385d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7f2d4d238337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7f2d4d235254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7f2d4d238002 - rust_begin_unwind
  11:     0x7f2d4d1e9963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7f2d50c469fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7f2d50c4949a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7f2d4eb84662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7f2d4ec0d0d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7f2d4ec0e228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7f2d4ebbd706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7f2d4ebcacca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7f2d4ec0d9fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7f2d4ebd45ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7f2d4ec0d864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7f2d4ebbce1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7f2d4ec0ce9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7f2d4eb7a9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7f2d50102e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7f2d500fe513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7f2d500fe12b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7f2d4dd5b31a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7f2d4dd91517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7f2d4ddf914b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7f2d4dd793f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7f2d4dcacc93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7f2d4dc3e2de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7f2d4dc9fafc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7f2d4dc9b749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7f2d4dca51c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7f2d4dc4d3fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f2d4d2449be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  39:     0x7f2d4cfdcb43 - <unknown>
  40:     0x7f2d4d06ea00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/mir/issue-68841.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-68841.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-68841" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-68841/auxiliary" "-Z" "mir-opt-level=3" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("async_closure"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("async_closure"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
stack backtrace:
   0:     0x7fcf3c775d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7fcf3c7e5278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7fcf3c7677d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7fcf3c775b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7fcf3c778e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7fcf3c778b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7fcf3d17db94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fcf3c7795d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7fcf3c779337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7fcf3c776254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7fcf3c779002 - rust_begin_unwind
  11:     0x7fcf3c72a963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7fcf401879fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7fcf4018a49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7fcf3e0c5662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7fcf3e14e0d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7fcf3e14f228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7fcf3e14f23e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  18:     0x7fcf3e111dd0 - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7fcf3e14f23e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  20:     0x7fcf3e10bcca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7fcf3e14e9fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  22:     0x7fcf3e1155ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7fcf3e14e864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  24:     0x7fcf3e0fde1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  25:     0x7fcf3e14de9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  26:     0x7fcf3e0bb9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  27:     0x7fcf3f643e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  28:     0x7fcf3f63f513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  29:     0x7fcf3f63f12b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  30:     0x7fcf3d29c31a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  31:     0x7fcf3d2d2517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  32:     0x7fcf3d33a14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7fcf3d2ba3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  34:     0x7fcf3d1edc93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7fcf3d17f2de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  36:     0x7fcf3d1e0afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7fcf3d1dc749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  38:     0x7fcf3d1e61c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  39:     0x7fcf3d18e3fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7fcf3c7859be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  41:     0x7fcf3c51db43 - <unknown>
  42:     0x7fcf3c5afa00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=3
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/regions/closure-in-projection-issue-97405.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/closure-in-projection-issue-97405.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/closure-in-projection-issue-97405" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/closure-in-projection-issue-97405/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("good_generic_fn"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Block))), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("good_generic_fn"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7f13c2a68d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7f13c2ad8278 - core::fmt::write::h8dc0e6ab2c0956b1
   1:     0x7f13c2ad8278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7f13c2a5a7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7f13c2a68b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7f13c2a6be84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7f13c2a6bb49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7f13c3470b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f13c2a6c5d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7f13c2a6c337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7f13c2a69254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7f13c2a6c002 - rust_begin_unwind
  11:     0x7f13c2a1d963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7f13c647a9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7f13c647d49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7f13c43b8662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7f13c44410d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7f13c44421fa - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7f13c4404dfc - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7f13c444223e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  19:     0x7f13c4404e28 - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  20:     0x7f13c444223e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  21:     0x7f13c4404dfc - rustc_ast[e60304300e2a2326]::visit::walk_expr::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  22:     0x7f13c444223e - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  23:     0x7f13c43fecca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  24:     0x7f13c44419fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  25:     0x7f13c44085ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  26:     0x7f13c4441864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  27:     0x7f13c43f0e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  28:     0x7f13c4440e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  29:     0x7f13c43ae9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  30:     0x7f13c5936e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  31:     0x7f13c5932513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  32:     0x7f13c593212b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  33:     0x7f13c358f31a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  34:     0x7f13c35c5517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  35:     0x7f13c362d14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7f13c35ad3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  37:     0x7f13c34e0c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  38:     0x7f13c34722de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  39:     0x7f13c34d3afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  40:     0x7f13c34cf749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  41:     0x7f13c34d91c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  42:     0x7f13c34813fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:     0x7f13c2a789be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  44:     0x7f13c2810b43 - <unknown>
  45:     0x7f13c28a2a00 - <unknown>
  46:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/rfc-2565-param-attrs/param-attrs-cfg.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-cfg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/param-attrs-cfg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/param-attrs-cfg/auxiliary" "--cfg" "something" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7fdea0d00d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7fdea0d70278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7fdea0cf27d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7fdea0d00b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7fdea0d00b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7fdea0d03e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7fdea0d03b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7fdea1708b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fdea0d045d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7fdea0d04337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7fdea0d01254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7fdea0d04002 - rust_begin_unwind
  11:     0x7fdea0cb5963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7fdea47129fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7fdea471549a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7fdea2650662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7fdea26d90d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7fdea26da228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7fdea2689706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7fdea2696cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7fdea26d99fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7fdea26a05ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7fdea26d9864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7fdea2688e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7fdea26d8e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7fdea26469b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7fdea3bcee46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7fdea3bca513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7fdea3bca12b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7fdea182731a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7fdea185d517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7fdea18c514b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7fdea18453f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7fdea1778c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7fdea170a2de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7fdea176bafc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7fdea1767749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7fdea17711c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7fdea17193fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fdea0d109be - std::sys::unix::thread::Thread::new::thread_start::h385833286693ea15
  39:     0x7fdea0aa8b43 - <unknown>
  40:     0x7fdea0b3aa00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
stack backtrace:
   0:     0x7efe009d8d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7efe00a48278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7efe009ca7d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7efe009d8b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7efe009dbe84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7efe009dbb49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7efe013e0b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efe009dc5d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7efe009dc337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7efe009d9254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7efe009dc002 - rust_begin_unwind
  11:     0x7efe0098d963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7efe043ea9fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7efe043ed49a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7efe02328662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7efe023b10d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7efe023b2228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7efe02361706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7efe0236ecca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7efe023b19fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7efe023785ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7efe023b1864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7efe02360e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7efe023b0e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7efe0231e9b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7efe038a6e46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7efe038a2513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7efe038a212b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7efe014ff31a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7efe01535517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7efe0159d14b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7efe0151d3f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7efe01450c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7efe013e22de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7efe01443afc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7efe0143f749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7efe014491c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7efe013f13fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7efe00780b43 - <unknown>
  40:     0x7efe00812a00 - <unknown>
  41:                0x0 - <unknown>

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/suggestions/suggest-on-bare-closure-call.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-on-bare-closure-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-on-bare-closure-call" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-on-bare-closure-call/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'found DefPathHash collision between DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(None), disambiguator: 0 }], krate: crate0 } and DefPath { data: [DisambiguatedDefPathData { data: ValueNs("main"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr(Some(Async(Closure))), disambiguator: 0 }], krate: crate0 }. Compilation cannot continue.', compiler/rustc_hir/src/definitions.rs:56:13
   0:     0x7efdbb470d1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd3473f275146e606
   1:     0x7efdbb4e0278 - core::fmt::write::h8dc0e6ab2c0956b1
   2:     0x7efdbb4627d1 - std::io::Write::write_fmt::h85c7fceecd7cb438
   3:     0x7efdbb470b21 - std::sys_common::backtrace::print::h67f85735b811626f
   3:     0x7efdbb470b21 - std::sys_common::backtrace::print::h67f85735b811626f
   4:     0x7efdbb473e84 - std::panicking::default_hook::{{closure}}::h036854f796d22e14
   5:     0x7efdbb473b49 - std::panicking::default_hook::h462fd2412077a2bd
   6:     0x7efdbbe78b94 - rustc_driver[bd0dec8ca0f541cd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efdbb4745d4 - std::panicking::rust_panic_with_hook::h8fb7010f602d4bf3
   8:     0x7efdbb474337 - std::panicking::begin_panic_handler::{{closure}}::h068e4793b81f7f26
   9:     0x7efdbb471254 - std::sys_common::backtrace::__rust_end_short_backtrace::h43b53774ca66c40f
  10:     0x7efdbb474002 - rust_begin_unwind
  11:     0x7efdbb425963 - core::panicking::panic_fmt::h2e0be1a8d35a9f78
  12:     0x7efdbee829fb - <rustc_hir[b5914597f9daef82]::definitions::DefPathTable>::allocate
  13:     0x7efdbee8549a - <rustc_hir[b5914597f9daef82]::definitions::Definitions>::create_def
  14:     0x7efdbcdc0662 - <rustc_resolve[887e11ef887816f6]::Resolver>::create_def
  15:     0x7efdbce490d5 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>::create_def
  16:     0x7efdbce4a228 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_expr
  17:     0x7efdbcdf9706 - rustc_ast[e60304300e2a2326]::visit::walk_local::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  18:     0x7efdbce06cca - rustc_ast[e60304300e2a2326]::visit::walk_fn::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  19:     0x7efdbce499fe - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_fn
  20:     0x7efdbce105ce - rustc_ast[e60304300e2a2326]::visit::walk_item::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  21:     0x7efdbce49864 - <rustc_resolve[887e11ef887816f6]::def_collector::DefCollector as rustc_ast[e60304300e2a2326]::visit::Visitor>::visit_item
  22:     0x7efdbcdf8e1d - rustc_ast[e60304300e2a2326]::visit::walk_crate::<rustc_resolve[887e11ef887816f6]::def_collector::DefCollector>
  23:     0x7efdbce48e9b - rustc_resolve[887e11ef887816f6]::def_collector::collect_definitions
  24:     0x7efdbcdb69b9 - <rustc_resolve[887e11ef887816f6]::Resolver as rustc_expand[911de5a86fb11740]::base::ResolverExpand>::visit_ast_fragment_with_placeholders
  25:     0x7efdbe33ee46 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::collect_invocations
  26:     0x7efdbe33a513 - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7efdbe33a12b - <rustc_expand[911de5a86fb11740]::expand::MacroExpander>::expand_crate
  28:     0x7efdbbf9731a - <rustc_session[8cc7e42dbe3a9044]::session::Session>::time::<core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::passes::configure_and_expand::{closure#1}>
  29:     0x7efdbbfcd517 - rustc_interface[73de797534f19fe1]::passes::configure_and_expand
  30:     0x7efdbc03514b - <rustc_interface[73de797534f19fe1]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[73de797534f19fe1]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<rustc_ast[e60304300e2a2326]::ast::Crate, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  31:     0x7efdbbfb53f9 - <rustc_interface[73de797534f19fe1]::queries::Queries>::expansion
  32:     0x7efdbbee8c93 - <rustc_interface[73de797534f19fe1]::interface::Compiler>::enter::<rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}::{closure#2}, core[4722f99dc5a60d1f]::result::Result<core[4722f99dc5a60d1f]::option::Option<rustc_interface[73de797534f19fe1]::queries::Linker>, rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  33:     0x7efdbbe7a2de - rustc_span[8fec70f3493ed01]::with_source_map::<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  34:     0x7efdbbedbafc - <scoped_tls[cfca8e0b46bdcd8e]::ScopedKey<rustc_span[8fec70f3493ed01]::SessionGlobals>>::set::<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  35:     0x7efdbbed7749 - std[f9eb4fd9c51afe4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  36:     0x7efdbbee11c6 - std[f9eb4fd9c51afe4c]::panic::catch_unwind::<core[4722f99dc5a60d1f]::panic::unwind_safe::AssertUnwindSafe<<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>
  37:     0x7efdbbe893fa - <<std[f9eb4fd9c51afe4c]::thread::Builder>::spawn_unchecked_<rustc_interface[73de797534f19fe1]::util::run_in_thread_pool_with_globals<rustc_interface[73de797534f19fe1]::interface::run_compiler<core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>, rustc_driver[bd0dec8ca0f541cd]::run_compiler::{closure#1}>::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4722f99dc5a60d1f]::result::Result<(), rustc_errors[768108a9979e89b1]::ErrorGuaranteed>>::{closure#1} as core[4722f99dc5a60d1f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7efdbb218b43 - <unknown>
  40:     0x7efdbb2aaa00 - <unknown>
  41:                0x0 - <unknown>

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (061e76624 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

