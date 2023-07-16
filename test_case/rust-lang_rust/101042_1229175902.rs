plain

running 13439 tests
........................................................................................ 88/13439
..........................................................................iiiiiiiiiiiiii 176/13439
.....................i.................i.....................F.......................... 264/13439
....................................................F................................... 352/13439
......................................................F........................F........ 440/13439
............F........FF................................................................. 528/13439
............................................................FF.......................... 704/13439
............................................................FF.......................... 704/13439
.................................F...................F.F........i....................... 792/13439
........................................................................................ 968/13439
........................................................................................ 968/13439
..................................................F.................F..............F.... 1056/13439
F..F....................FF.F.FF....FF.F...F............................................. 1144/13439
...........F..........F...FF.........F........FFF.F.....F.F.FF....F.F..FF............... 1232/13439
....F...F.F.........F..F...........F..............i...........F..................F.F..FF 1320/13439
F...F...............F...................................F............................... 1408/13439
.......................................................................F................ 1584/13439
.....................................................F.................................. 1672/13439
.........................................................F.i......ii.................... 1760/13439
........................................................................................ 1848/13439
---
.............F.......................................................................... 4400/13439
........................................................................................ 4488/13439
........................................................................................ 4576/13439
........................................................................................ 4664/13439
........................F.F............................................................. 4752/13439
.......................................................F................................ 4928/13439
........................................................................................ 5016/13439
........................................................................................ 5104/13439
..........................................................................i............. 5192/13439
---
......................................i................................................. 6600/13439
........................................................................................ 6688/13439
...............i.......................................................ii.ii........i... 6776/13439
.i...............................................................i...................... 6864/13439
......................................................................F..F.............. 6952/13439
......F.F.....FF.FFFFFFFFFF.F.FFFF.FF.i...i.........................................i... 7040/13439
........................................................................................ 7216/13439
..................i..................................................................... 7304/13439
........................................................................................ 7392/13439
........................................................................................ 7392/13439
.......................F....................F........................................... 7480/13439
..................................i..................................................... 7656/13439
........................................................................................ 7744/13439
..........................................ii....................................FF...... 7832/13439
........................................................................................ 7920/13439
........................................................................................ 7920/13439
F....................................................................................... 8008/13439
........................................................ii................i....i..ii.... 8096/13439
...............F.......F.......F........................................................ 8184/13439
.................................................................F........F.FF.....FFF.. 8272/13439
..F..................................F................F.............F..F.F...F....F..... 8360/13439
...........................F..F......FF..........F........F..F..F....F......FF.......... 8448/13439
.........................F...F.F.......FF.......................F.....................FF 8536/13439
.....F......i..ii..............................................................ii....... 8624/13439
....................................F.....F..........................................iii 8712/13439
.......................................i........................................i....... 8888/13439
.............................................................i.......................... 8976/13439
........................................................................................ 9064/13439
........................................................i............................... 9152/13439
---
........................................................................................ 9856/13439
............................................................................ii.......... 9944/13439
.....i.................................................................................. 10032/13439
........................................................................................ 10120/13439
.....F...................F....F..F.....F........F...F........F.....F.FFFF........F.F.... 10208/13439
...FFF.....................FF.F...F................................F.F....F......F.F..FF 10296/13439
.........F........F.FF.F................................................................ 10384/13439
........................................................................................ 10560/13439
........................................................................................ 10648/13439
........................................................................................ 10648/13439
................iiiiiF.i....i.i......................................................... 10736/13439
..................................................................................iiiiii 10912/13439
..................................................................................iiiiii 10912/13439
.i..iiiiii..i.........................F..F......................F.F............FFFF.FF.F 11000/13439
.F...................................................................................... 11088/13439
........................................................................................ 11264/13439
........................................................................................ 11352/13439
........................................................................................ 11440/13439
........................................................................................ 11528/13439
---
...................................................................................i.... 12584/13439
........................................................................................ 12672/13439
........................................................................................ 12760/13439
........................................................................................ 12848/13439
..........................................F............................................. 12936/13439
..........F................F............................................................ 13024/13439
........................................................................................ 13200/13439
........................................................................................ 13288/13439
........................................................................................ 13288/13439
................FFF.FF..FFFF....F...........iiiF.FF..................................... 13376/13439
.....F.........................................................

---- [ui] src/test/ui/asm/x86_64/type-check-5.rs stdout ----


error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/type-check-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-5" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-5/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7f4b4af91cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   1:     0x7f4b4affa748 - core::fmt::write::hd17183098a2b8f9a
   2:     0x7f4b4af82971 - std::io::Write::write_fmt::h8c18b55141e95391
   3:     0x7f4b4af94cfe - std::panicking::default_hook::{{closure}}::hbc7e1819e45e1439
   4:     0x7f4b4af949c7 - std::panicking::default_hook::hf5c60f92db8187ea
   5:     0x7f4b4b91c1d4 - rustc_driver[2f4eb290599bf9d6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4b4af954b1 - std::panicking::rust_panic_with_hook::hf49d10582a3f829d
   7:     0x7f4b4af952d7 - std::panicking::begin_panic_handler::{{closure}}::h3b40746384dd0adb
   8:     0x7f4b4af92274 - std::sys_common::backtrace::__rust_end_short_backtrace::ha0d6fcb399f81d73
   9:     0x7f4b4af94fa2 - rust_begin_unwind
  10:     0x7f4b4af45e43 - core::panicking::panic_fmt::h3a49f258a41d598f
  11:     0x7f4b4aff6dd1 - core::panicking::panic_display::h018d75316b91e3d7
  12:     0x7f4b4aff6d7b - core::panicking::panic_str::h7263301f4633b2c9
  13:     0x7f4b4af45cb6 - core::option::expect_failed::h61b9de2d1355e3a8
  14:     0x7f4b4e618b22 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::translation::Translate>::translate_message
  15:     0x7f4b4e615e62 - <rustc_errors[ce55abf701337603]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f4b4e60e38a - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f4b4e60b9d6 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  18:     0x7f4b4e6241a2 - <rustc_errors[ce55abf701337603]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f4b4e622bbc - <rustc_errors[ce55abf701337603]::json::JsonEmitter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  20:     0x7f4b4e6686f8 - <rustc_errors[ce55abf701337603]::HandlerInner>::emit_diagnostic
  21:     0x7f4b4e666ff1 - <rustc_errors[ce55abf701337603]::Handler>::emit_diagnostic
  22:     0x7f4b4cc03023 - rustc_borrowck[5b86fdeed8fcfb8b]::do_mir_borrowck
  23:     0x7f4b4c9c96ff - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult, rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck::{closure#0}>
  24:     0x7f4b4cbf81d7 - rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck
  25:     0x7f4b4cbc33ee - <rustc_borrowck[5b86fdeed8fcfb8b]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_span[15af97997a9557ab]::def_id::LocalDefId)>>::call_once
  26:     0x7f4b4d461209 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, &rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult>>
  27:     0x7f4b4d533c43 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_borrowck, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  28:     0x7f4b4d3944c4 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f4b4bab7896 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  30:     0x7f4b4ba380f9 - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f4b4ba250cd - rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in::<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f4b4baab6f5 - <rustc_session[769c1969a479696c]::session::Session>::time::<(), rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}>
  33:     0x7f4b4ba72a4b - rustc_interface[efb2de776eb96ea0]::passes::analysis
  34:     0x7f4b4d4a3214 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<(), core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>>
  35:     0x7f4b4d587024 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::analysis, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  36:     0x7f4b4d3757ee - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::analysis
  37:     0x7f4b4b987dad - <rustc_interface[efb2de776eb96ea0]::passes::QueryContext>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  38:     0x7f4b4b924426 - <rustc_interface[efb2de776eb96ea0]::interface::Compiler>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}, core[c2b73153b1d687cd]::result::Result<core[c2b73153b1d687cd]::option::Option<rustc_interface[efb2de776eb96ea0]::queries::Linker>, rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  39:     0x7f4b4b906c05 - rustc_span[15af97997a9557ab]::with_source_map::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f4b4b93faf1 - rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>
  41:     0x7f4b4b908a82 - <scoped_tls[66056ed50dce875b]::ScopedKey<rustc_span[15af97997a9557ab]::SessionGlobals>>::set::<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  42:     0x7f4b4b97e709 - std[11dc701f356fae7c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  43:     0x7f4b4b908e2e - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  44:     0x7f4b4b982d40 - <<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1} as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f4b4afa1d25 - std::sys::unix::thread::Thread::new::thread_start::h6e088341e63c9a12
  46:     0x7f4b4ad3fb43 - <unknown>
  47:     0x7f4b4add1a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (cf7a15686 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main`
#1 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/associated-type-bounds/implied-region-constraints.rs stdout ----
---- [ui] src/test/ui/associated-type-bounds/implied-region-constraints.rs stdout ----
diff of stderr:

2   --> $DIR/implied-region-constraints.rs:17:56
3    |
4 LL | fn _bad_st<'a, 'b, T>(x: St<'a, 'b, T>)
-    |            --  -- lifetime `'b` defined here
+    |            --  -- lifetime `'a` defined here
6    |            |
7    |            lifetime `'a` defined here

15   --> $DIR/implied-region-constraints.rs:38:64
16    |
16    |
17 LL | fn _bad_en7<'a, 'b, T>(x: En7<'a, 'b, T>)
-    |             --  -- lifetime `'b` defined here
+    |             --  -- lifetime `'a` defined here
19    |             |
20    |             lifetime `'a` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/implied-region-constraints/implied-region-constraints.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/implied-region-constraints/implied-region-constraints.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-type-bounds/implied-region-constraints.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/implied-region-constraints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/implied-region-constraints" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/implied-region-constraints/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn _bad_st<'a, 'b, T>(x: St<'a, 'b, T>)
   |            --  -- lifetime `'a` defined here
   |            |
   |            lifetime `'a` defined here
...
LL |     let _failure_proves_not_implied_outlives_region_b: &'b T = &x.f0;
   |                                                        ^^^^^ type annotation requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-type-bounds/implied-region-constraints.rs:38:64
   |
   |
LL | fn _bad_en7<'a, 'b, T>(x: En7<'a, 'b, T>)
   |             --  -- lifetime `'a` defined here
   |             |
   |             lifetime `'a` defined here
...
LL |             let _failure_proves_not_implied_outlives_region_b: &'b T = &x;
   |                                                                ^^^^^ type annotation requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body.rs stdout ----
diff of stderr:

2   --> $DIR/associated-types-project-from-hrtb-in-fn-body.rs:22:29
3    |
4 LL | fn bar<'a, 'b, I : for<'x> Foo<&'x isize>>(
-    |        --  -- lifetime `'b` defined here
+    |        --  -- lifetime `'a` defined here
6    |        |
7    |        lifetime `'a` defined here


17 LL | fn bar<'a, 'b, I : for<'x> Foo<&'x isize>>(
18    |        --  -- lifetime `'b` defined here
19    |        |
-    |        lifetime `'a` defined here
+    |        lifetime `'b` defined here
21 ...
22 LL |     let z: I::A = if cond { x } else { y };
23    |                                        ^ assignment requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body/associated-types-project-from-hrtb-in-fn-body.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/associated-types-project-from-hrtb-in-fn-body.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn bar<'a, 'b, I : for<'x> Foo<&'x isize>>(
   |        --  -- lifetime `'a` defined here
   |        |
   |        lifetime `'a` defined here
...
LL |     let z: I::A = if cond { x } else { y };
   |                             ^ assignment requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body.rs:22:40
   |
   |
LL | fn bar<'a, 'b, I : for<'x> Foo<&'x isize>>(
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'b` defined here
...
LL |     let z: I::A = if cond { x } else { y };
   |                                        ^ assignment requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/associated-types-subtyping-1.rs stdout ----
diff of stderr:

4 LL | fn method2<'a,'b,T>(x: &'a T, y: &'b T)
5    |            -- -- lifetime `'b` defined here
6    |            |
-    |            lifetime `'a` defined here
+    |            lifetime `'b` defined here
8 ...
9 LL |     let a: <T as Trait<'a>>::Type = make_any();
10    |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`

17 LL | fn method3<'a,'b,T>(x: &'a T, y: &'b T)
18    |            -- -- lifetime `'b` defined here
19    |            |
-    |            lifetime `'a` defined here
+    |            lifetime `'b` defined here
21 ...
22 LL |     let _c: <T as Trait<'a>>::Type = b;
23    |             ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1/associated-types-subtyping-1.stderr
To only update this specific test, also pass `--test-args associated-types/associated-types-subtyping-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn method2<'a,'b,T>(x: &'a T, y: &'b T)
   |            -- -- lifetime `'b` defined here
   |            |
   |            lifetime `'b` defined here
...
LL |     let a: <T as Trait<'a>>::Type = make_any();
   |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs:36:13
   |
   |
LL | fn method3<'a,'b,T>(x: &'a T, y: &'b T)
   |            -- -- lifetime `'b` defined here
   |            |
   |            lifetime `'b` defined here
...
LL |     let _c: <T as Trait<'a>>::Type = b;
   |             ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs#krisskross stdout ----
diff of stderr:

2   --> $DIR/project-fn-ret-contravariant.rs:46:4
3    |
4 LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
-    |              -- -- lifetime `'b` defined here
+    |              -- -- lifetime `'a` defined here
6    |              |
7    |              lifetime `'a` defined here


17 LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
18    |              -- -- lifetime `'b` defined here
19    |              |
-    |              lifetime `'a` defined here
+    |              lifetime `'b` defined here
22 LL |    (a, b)
22 LL |    (a, b)
23    |    ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.krisskross/project-fn-ret-contravariant.krisskross.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-contravariant.rs`


error in revision `krisskross`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "krisskross" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.krisskross" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.krisskross/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
   |              -- -- lifetime `'a` defined here
   |              |
   |              lifetime `'a` defined here
...
LL |    (a, b) //[krisskross]~ ERROR lifetime may not live long enough
   |    ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs:46:4
   |
   |
LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
   |              -- -- lifetime `'b` defined here
   |              |
   |              lifetime `'b` defined here
...
LL |    (a, b) //[krisskross]~ ERROR lifetime may not live long enough
   |    ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/cache/project-fn-ret-invariant.rs#krisskross stdout ----
diff of stderr:

2   --> $DIR/project-fn-ret-invariant.rs:59:5
3    |
4 LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
-    |              --  -- lifetime `'b` defined here
+    |              --  -- lifetime `'a` defined here
6    |              |
7    |              lifetime `'a` defined here


20 LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
21    |              --  -- lifetime `'b` defined here
22    |              |
-    |              lifetime `'a` defined here
+    |              lifetime `'b` defined here
25 LL |     (a, b)
25 LL |     (a, b)
26    |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross/project-fn-ret-invariant.krisskross.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`


error in revision `krisskross`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "krisskross" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |              --  -- lifetime `'a` defined here
   |              |
   |              lifetime `'a` defined here
LL |     (a, b)
LL |     (a, b)
   |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:59:5
   |
   |
LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |              --  -- lifetime `'b` defined here
   |              |
   |              lifetime `'b` defined here
LL |     (a, b)
LL |     (a, b)
   |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/cache/project-fn-ret-invariant.rs#oneuse stdout ----
diff of stderr:

2   --> $DIR/project-fn-ret-invariant.rs:40:13
3    |
4 LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
-    |        --  -- lifetime `'b` defined here
+    |        --  -- lifetime `'a` defined here
6    |        |
7    |        lifetime `'a` defined here
8 LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.

20 LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
21    |        --  -- lifetime `'b` defined here
22    |        |
-    |        lifetime `'a` defined here
+    |        lifetime `'b` defined here
24 LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
25 LL |     let a = bar(f, x);
26    |             ^^^^^^^^^ argument requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse/project-fn-ret-invariant.oneuse.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`

error in revision `oneuse`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "oneuse" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |        --  -- lifetime `'a` defined here
   |        |
   |        lifetime `'a` defined here
LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
LL |     let a = bar(f, x);
   |             ^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:40:13
   |
   |
LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'b` defined here
LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
LL |     let a = bar(f, x);
   |             ^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a function pointer to `foo`
   = note: the function `foo` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/async-await/issue-74497-lifetime-in-opaque.rs stdout ----
diff of stderr:

5    |                  -- ^^^^^^ returning this value requires that `'1` must outlive `'2`
6    |                  ||
7    |                  |return type of closure `impl Future<Output = ()>` contains a lifetime `'2`
-    |                  has type `&'1 u8`
+    |                  has type `impl Future<Output = ()>`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74497-lifetime-in-opaque/issue-74497-lifetime-in-opaque.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-74497-lifetime-in-opaque.rs`

error: 1 errors occurred comparing output.
---
diff of stderr:

2   --> $DIR/expect-region-supply-region-2.rs:14:30
3    |
4 LL | fn expect_bound_supply_named<'x>() {
-    |                              -- lifetime `'x` defined here
+    |                              -- lifetime `'1` defined here
6 ...
7 LL |     closure_expecting_bound(|x: &'x u32| {
8    |                              ^  - let's call the lifetime of this reference `'1`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region-2/expect-region-supply-region-2.stderr
To only update this specific test, also pass `--test-args closures/closure-expected-type/expect-region-supply-region-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn expect_bound_supply_named<'x>() {
   |                              -- lifetime `'1` defined here
...
LL |     closure_expecting_bound(|x: &'x u32| {
   |                              ^  - let's call the lifetime of this reference `'1`
   |                              |
   |                              requires that `'1` must outlive `'x`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region-2.rs:14:30
   |
   |
LL | fn expect_bound_supply_named<'x>() {
   |                              -- lifetime `'x` defined here
...
LL |     closure_expecting_bound(|x: &'x u32| {
   |                              ^ requires that `'x` must outlive `'static`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/codemap_tests/one_line.rs stdout ----
diff of stderr:

8    |     | first borrow later used by call
9    |     first mutable borrow occurs here
- help: try adding a local storing this argument...
- help: try adding a local storing this argument...
+ help: try adding a local storing this  argument...
12   --> $DIR/one_line.rs:3:12
13    |
14 LL |     v.push(v.pop().unwrap());
15    |            ^^^^^^^
15    |            ^^^^^^^
- help: ...and then using that local as the argument to this call
+ help: and then using that local `as the argument to this call`
17   --> $DIR/one_line.rs:3:5
18    |
19 LL |     v.push(v.pop().unwrap());

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/one_line/one_line.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args codemap_tests/one_line.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/one_line.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/one_line" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/one_line/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0499]: cannot borrow `v` as mutable more than once at a time
   |
   |
LL |     v.push(v.pop().unwrap()); //~ ERROR cannot borrow
   |     | |    |
   |     | |    |
   |     | |    second mutable borrow occurs here
   |     | first borrow later used by call
   |     first mutable borrow occurs here
help: try adding a local storing this  argument...
  --> /checkout/src/test/ui/codemap_tests/one_line.rs:3:12
   |
   |
LL |     v.push(v.pop().unwrap()); //~ ERROR cannot borrow
   |            ^^^^^^^
help: and then using that local `as the argument to this call`
   |
   |
LL |     v.push(v.pop().unwrap()); //~ ERROR cannot borrow

error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.
For more information about this error, try `rustc --explain E0499`.
------------------------------------------


---- [ui] src/test/ui/const-generics/const-generic-default-wont-borrowck.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-generic-default-wont-borrowck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-generic-default-wont-borrowck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-generic-default-wont-borrowck/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7fd17a156cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   0:     0x7fd17a156cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   1:     0x7fd17a1bf748 - core::fmt::write::hd17183098a2b8f9a
   2:     0x7fd17a147971 - std::io::Write::write_fmt::h8c18b55141e95391
   3:     0x7fd17a159cfe - std::panicking::default_hook::{{closure}}::hbc7e1819e45e1439
   4:     0x7fd17a1599c7 - std::panicking::default_hook::hf5c60f92db8187ea
   5:     0x7fd17aae11d4 - rustc_driver[2f4eb290599bf9d6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd17a15a4b1 - std::panicking::rust_panic_with_hook::hf49d10582a3f829d
   7:     0x7fd17a15a2d7 - std::panicking::begin_panic_handler::{{closure}}::h3b40746384dd0adb
   8:     0x7fd17a157274 - std::sys_common::backtrace::__rust_end_short_backtrace::ha0d6fcb399f81d73
   9:     0x7fd17a159fa2 - rust_begin_unwind
  10:     0x7fd17a10ae43 - core::panicking::panic_fmt::h3a49f258a41d598f
  11:     0x7fd17a1bbdd1 - core::panicking::panic_display::h018d75316b91e3d7
  12:     0x7fd17a1bbd7b - core::panicking::panic_str::h7263301f4633b2c9
  13:     0x7fd17a10acb6 - core::option::expect_failed::h61b9de2d1355e3a8
  14:     0x7fd17d7ddb22 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::translation::Translate>::translate_message
  15:     0x7fd17d7dae62 - <rustc_errors[ce55abf701337603]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fd17d7d338a - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fd17d7d09d6 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  18:     0x7fd17d7e91a2 - <rustc_errors[ce55abf701337603]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fd17d7e7bbc - <rustc_errors[ce55abf701337603]::json::JsonEmitter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  20:     0x7fd17d82d6f8 - <rustc_errors[ce55abf701337603]::HandlerInner>::emit_diagnostic
  21:     0x7fd17d82bff1 - <rustc_errors[ce55abf701337603]::Handler>::emit_diagnostic
  22:     0x7fd17bdc8023 - rustc_borrowck[5b86fdeed8fcfb8b]::do_mir_borrowck
  23:     0x7fd17bb8e6ff - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult, rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck::{closure#0}>
  24:     0x7fd17bdbd1d7 - rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck
  25:     0x7fd17bd883ee - <rustc_borrowck[5b86fdeed8fcfb8b]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_span[15af97997a9557ab]::def_id::LocalDefId)>>::call_once
  26:     0x7fd17c626209 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, &rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult>>
  27:     0x7fd17c6f8c43 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_borrowck, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  28:     0x7fd17c5594c4 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fd17d5c4295 - <rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt>::mir_borrowck_opt_const_arg
  30:     0x7fd17b10e4cd - rustc_mir_transform[f2d2d02a4d4cc651]::mir_drops_elaborated_and_const_checked
  31:     0x7fd17c612d26 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_middle[3a3d34b0b0b7c294]::ty::WithOptConstParam<rustc_span[15af97997a9557ab]::def_id::LocalDefId>, &rustc_data_structures[c71a9283324d9f00]::steal::Steal<rustc_middle[3a3d34b0b0b7c294]::mir::Body>>>
  32:     0x7fd17c746c35 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  33:     0x7fd17c5419a7 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  34:     0x7fd17b10e0c7 - rustc_mir_transform[f2d2d02a4d4cc651]::inner_mir_for_ctfe
  35:     0x7fd17b10db66 - rustc_mir_transform[f2d2d02a4d4cc651]::mir_for_ctfe
  36:     0x7fd17c64d8d9 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::DefId, &rustc_middle[3a3d34b0b0b7c294]::mir::Body>>
  37:     0x7fd17c6f8da9 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_for_ctfe, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  38:     0x7fd17c541ee9 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_for_ctfe
  39:     0x7fd17c0bab35 - <rustc_const_eval[11fed4fe39b2827e]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[11fed4fe39b2827e]::interpret::machine::Machine>::load_mir
  40:     0x7fd17c06f966 - <rustc_const_eval[11fed4fe39b2827e]::interpret::eval_context::InterpCx<rustc_const_eval[11fed4fe39b2827e]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  41:     0x7fd17c1a8abc - rustc_const_eval[11fed4fe39b2827e]::const_eval::eval_queries::eval_to_allocation_raw_provider
  42:     0x7fd17c72069a - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::eval_to_allocation_raw, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  43:     0x7fd17c55baa1 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::eval_to_allocation_raw
  44:     0x7fd17c1b9e61 - rustc_const_eval[11fed4fe39b2827e]::const_eval::eval_to_valtree
  45:     0x7fd17c0d6b20 - <rustc_const_eval[11fed4fe39b2827e]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_middle[3a3d34b0b0b7c294]::ty::ParamEnvAnd<rustc_middle[3a3d34b0b0b7c294]::mir::interpret::GlobalId>)>>::call_once
  46:     0x7fd17c700287 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::eval_to_valtree, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  47:     0x7fd17c55c641 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::eval_to_valtree
  48:     0x7fd17d5dbbf6 - <rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  49:     0x7fd17d5da5ca - <rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  50:     0x7fd17d30855e - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxt>::const_eval_resolve
  51:     0x7fd17d0d561a - rustc_trait_selection[d0b6b0c32cf2d47d]::traits::const_evaluatable::is_const_evaluatable
  52:     0x7fd17d21a7b0 - <rustc_trait_selection[d0b6b0c32cf2d47d]::traits::fulfill::FulfillProcessor as rustc_data_structures[c71a9283324d9f00]::obligation_forest::ObligationProcessor>::process_obligation
  53:     0x7fd17d0dc8d6 - <rustc_data_structures[c71a9283324d9f00]::obligation_forest::ObligationForest<rustc_trait_selection[d0b6b0c32cf2d47d]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[d0b6b0c32cf2d47d]::traits::fulfill::FulfillProcessor, rustc_data_structures[c71a9283324d9f00]::obligation_forest::Outcome<rustc_trait_selection[d0b6b0c32cf2d47d]::traits::fulfill::PendingPredicateObligation, rustc_infer[49ae84ee19bed59e]::traits::FulfillmentErrorCode>>
  54:     0x7fd17d214b7a - <rustc_trait_selection[d0b6b0c32cf2d47d]::traits::fulfill::FulfillmentContext as rustc_infer[49ae84ee19bed59e]::traits::engine::TraitEngine>::select_where_possible
  55:     0x7fd17d214a58 - <rustc_trait_selection[d0b6b0c32cf2d47d]::traits::fulfill::FulfillmentContext as rustc_infer[49ae84ee19bed59e]::traits::engine::TraitEngine>::select_all_or_error
  56:     0x7fd17d244aab - <rustc_trait_selection[d0b6b0c32cf2d47d]::traits::engine::ObligationCtxt>::select_all_or_error
  57:     0x7fd17b61a413 - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[6b2f6e7b8e3d7577]::check::wfcheck::enter_wf_checking_ctxt<rustc_typeck[6b2f6e7b8e3d7577]::check::wfcheck::check_type_defn<rustc_typeck[6b2f6e7b8e3d7577]::check::wfcheck::check_item::{closure#2}>::{closure#0}>::{closure#0}>
  58:     0x7fd17b6db705 - rustc_typeck[6b2f6e7b8e3d7577]::check::wfcheck::check_well_formed
  59:     0x7fd17c628d99 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, ()>>
  60:     0x7fd17c70a616 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::check_well_formed, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  61:     0x7fd17c574b64 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::check_well_formed
  62:     0x7fd17b530dc5 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_hir[cf0363beb2848b9e]::hir::ItemId], <rustc_middle[3a3d34b0b0b7c294]::hir::ModuleItems>::par_items<rustc_typeck[6b2f6e7b8e3d7577]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  63:     0x7fd17b4c35d9 - std[11dc701f356fae7c]::panicking::try::<(), core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_hir[cf0363beb2848b9e]::hir::ItemId], <rustc_middle[3a3d34b0b0b7c294]::hir::ModuleItems>::par_items<rustc_typeck[6b2f6e7b8e3d7577]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  64:     0x7fd17b45b50d - <rustc_middle[3a3d34b0b0b7c294]::hir::ModuleItems>::par_items::<rustc_typeck[6b2f6e7b8e3d7577]::check::wfcheck::check_mod_type_wf::{closure#0}>
  65:     0x7fd17b6e60f9 - rustc_typeck[6b2f6e7b8e3d7577]::check::wfcheck::check_mod_type_wf
  66:     0x7fd17c628d99 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, ()>>
  67:     0x7fd17c70a4e6 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::check_mod_type_wf, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  68:     0x7fd17c555f14 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::check_mod_type_wf
  69:     0x7fd17b530785 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_for_each_module<rustc_typeck[6b2f6e7b8e3d7577]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  70:     0x7fd17b4c3519 - std[11dc701f356fae7c]::panicking::try::<(), core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_for_each_module<rustc_typeck[6b2f6e7b8e3d7577]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  71:     0x7fd17b46298d - rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in::<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_for_each_module<rustc_typeck[6b2f6e7b8e3d7577]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  72:     0x7fd17b54c7dd - <rustc_session[769c1969a479696c]::session::Session>::track_errors::<rustc_typeck[6b2f6e7b8e3d7577]::check_crate::{closure#5}, ()>
  73:     0x7fd17b77cb20 - rustc_typeck[6b2f6e7b8e3d7577]::check_crate
  74:     0x7fd17ac37a11 - rustc_interface[efb2de776eb96ea0]::passes::analysis
  75:     0x7fd17c668214 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<(), core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>>
  76:     0x7fd17c74c024 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::analysis, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  77:     0x7fd17c53a7ee - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::analysis
  78:     0x7fd17ab4cdad - <rustc_interface[efb2de776eb96ea0]::passes::QueryContext>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  79:     0x7fd17aae9426 - <rustc_interface[efb2de776eb96ea0]::interface::Compiler>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}, core[c2b73153b1d687cd]::result::Result<core[c2b73153b1d687cd]::option::Option<rustc_interface[efb2de776eb96ea0]::queries::Linker>, rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  80:     0x7fd17aacbc05 - rustc_span[15af97997a9557ab]::with_source_map::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#1}>
  81:     0x7fd17ab04af1 - rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>
  82:     0x7fd17aacda82 - <scoped_tls[66056ed50dce875b]::ScopedKey<rustc_span[15af97997a9557ab]::SessionGlobals>>::set::<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  83:     0x7fd17ab43709 - std[11dc701f356fae7c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  84:     0x7fd17aacde2e - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  85:     0x7fd17ab47d40 - <<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1} as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  86:     0x7fd17a166d25 - std::sys::unix::thread::Thread::new::thread_start::h6e088341e63c9a12
  87:     0x7fd179f04b43 - <unknown>
  88:     0x7fd179f96a00 - <unknown>
  89:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (cf7a15686 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `X::{constant#0}`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `X::{constant#0}`
#2 [mir_for_ctfe] caching mir of `X::{constant#0}` for CTFE
#3 [eval_to_allocation_raw] const-evaluating + checking `X::{constant#0}`
#4 [eval_to_valtree] evaluating type-level constant
#5 [check_well_formed] checking that `X` is well-formed
#6 [check_mod_type_wf] checking that types are well-formed in top-level module
#7 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/consts/issue-78655.rs stdout ----
---- [ui] src/test/ui/consts/issue-78655.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-78655.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-78655" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-78655/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7f24c8f4ecfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   1:     0x7f24c8fb7748 - core::fmt::write::hd17183098a2b8f9a
   2:     0x7f24c8f3f971 - std::io::Write::write_fmt::h8c18b55141e95391
   3:     0x7f24c8f51cfe - std::panicking::default_hook::{{closure}}::hbc7e1819e45e1439
   4:     0x7f24c8f519c7 - std::panicking::default_hook::hf5c60f92db8187ea
   5:     0x7f24c98d91d4 - rustc_driver[2f4eb290599bf9d6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f24c8f524b1 - std::panicking::rust_panic_with_hook::hf49d10582a3f829d
   7:     0x7f24c8f522d7 - std::panicking::begin_panic_handler::{{closure}}::h3b40746384dd0adb
   8:     0x7f24c8f4f274 - std::sys_common::backtrace::__rust_end_short_backtrace::ha0d6fcb399f81d73
   9:     0x7f24c8f51fa2 - rust_begin_unwind
  10:     0x7f24c8f02e43 - core::panicking::panic_fmt::h3a49f258a41d598f
  11:     0x7f24c8fb3dd1 - core::panicking::panic_display::h018d75316b91e3d7
  12:     0x7f24c8fb3d7b - core::panicking::panic_str::h7263301f4633b2c9
  13:     0x7f24c8f02cb6 - core::option::expect_failed::h61b9de2d1355e3a8
  14:     0x7f24cc5d5b22 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::translation::Translate>::translate_message
  15:     0x7f24cc5d2e62 - <rustc_errors[ce55abf701337603]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f24cc5cb38a - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f24cc5c89d6 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  18:     0x7f24cc5e11a2 - <rustc_errors[ce55abf701337603]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f24cc5dfbbc - <rustc_errors[ce55abf701337603]::json::JsonEmitter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  20:     0x7f24cc6256f8 - <rustc_errors[ce55abf701337603]::HandlerInner>::emit_diagnostic
  21:     0x7f24cc623ff1 - <rustc_errors[ce55abf701337603]::Handler>::emit_diagnostic
  22:     0x7f24cabc0023 - rustc_borrowck[5b86fdeed8fcfb8b]::do_mir_borrowck
  23:     0x7f24ca9866ff - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult, rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck::{closure#0}>
  24:     0x7f24cabb51d7 - rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck
  25:     0x7f24cab803ee - <rustc_borrowck[5b86fdeed8fcfb8b]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_span[15af97997a9557ab]::def_id::LocalDefId)>>::call_once
  26:     0x7f24cb41e209 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, &rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult>>
  27:     0x7f24cb4f0c43 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_borrowck, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  28:     0x7f24cb3514c4 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f24cc3bc295 - <rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt>::mir_borrowck_opt_const_arg
  30:     0x7f24c9f064cd - rustc_mir_transform[f2d2d02a4d4cc651]::mir_drops_elaborated_and_const_checked
  31:     0x7f24cb40ad26 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_middle[3a3d34b0b0b7c294]::ty::WithOptConstParam<rustc_span[15af97997a9557ab]::def_id::LocalDefId>, &rustc_data_structures[c71a9283324d9f00]::steal::Steal<rustc_middle[3a3d34b0b0b7c294]::mir::Body>>>
  32:     0x7f24cb53ec35 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  33:     0x7f24cb3399a7 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  34:     0x7f24c9f060c7 - rustc_mir_transform[f2d2d02a4d4cc651]::inner_mir_for_ctfe
  35:     0x7f24c9f05b66 - rustc_mir_transform[f2d2d02a4d4cc651]::mir_for_ctfe
  36:     0x7f24cb4458d9 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::DefId, &rustc_middle[3a3d34b0b0b7c294]::mir::Body>>
  37:     0x7f24cb4f0da9 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_for_ctfe, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  38:     0x7f24cb339ee9 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_for_ctfe
  39:     0x7f24caeb2b35 - <rustc_const_eval[11fed4fe39b2827e]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[11fed4fe39b2827e]::interpret::machine::Machine>::load_mir
  40:     0x7f24cae67966 - <rustc_const_eval[11fed4fe39b2827e]::interpret::eval_context::InterpCx<rustc_const_eval[11fed4fe39b2827e]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  41:     0x7f24cafa0abc - rustc_const_eval[11fed4fe39b2827e]::const_eval::eval_queries::eval_to_allocation_raw_provider
  42:     0x7f24cb51869a - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::eval_to_allocation_raw, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  43:     0x7f24cb353aa1 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::eval_to_allocation_raw
  44:     0x7f24caf9ee8b - rustc_const_eval[11fed4fe39b2827e]::const_eval::eval_queries::eval_to_const_value_raw_provider
  45:     0x7f24cb51fe4a - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::eval_to_const_value_raw, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  46:     0x7f24cb354071 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::eval_to_const_value_raw
  47:     0x7f24caf9ea92 - rustc_const_eval[11fed4fe39b2827e]::const_eval::eval_queries::eval_to_const_value_raw_provider
  48:     0x7f24cb51fe4a - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::eval_to_const_value_raw, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  49:     0x7f24cb354071 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::eval_to_const_value_raw
  50:     0x7f24cc3d2f74 - <rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt>::const_eval_global_id
  51:     0x7f24cc3aaf1e - <rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt>::const_eval_instance
  52:     0x7f24ca8a88b7 - <rustc_mir_build[1dd737905aae26f4]::thir::pattern::PatCtxt>::lower_path
  53:     0x7f24ca8a3daa - <rustc_mir_build[1dd737905aae26f4]::thir::pattern::PatCtxt>::lower_pattern
  54:     0x7f24ca93dc14 - <rustc_mir_build[1dd737905aae26f4]::thir::pattern::check_match::MatchVisitor>::lower_pattern
  55:     0x7f24ca93e765 - <rustc_mir_build[1dd737905aae26f4]::thir::pattern::check_match::MatchVisitor>::check_irrefutable
  56:     0x7f24ca93db7d - <rustc_mir_build[1dd737905aae26f4]::thir::pattern::check_match::MatchVisitor as rustc_hir[cf0363beb2848b9e]::intravisit::Visitor>::visit_local
  57:     0x7f24ca9208c5 - rustc_hir[cf0363beb2848b9e]::intravisit::walk_expr::<rustc_mir_build[1dd737905aae26f4]::thir::pattern::check_match::MatchVisitor>
  58:     0x7f24ca93b83c - <rustc_mir_build[1dd737905aae26f4]::thir::pattern::check_match::MatchVisitor as rustc_hir[cf0363beb2848b9e]::intravisit::Visitor>::visit_expr
  59:     0x7f24ca93b5ef - rustc_mir_build[1dd737905aae26f4]::thir::pattern::check_match::check_match
  60:     0x7f24cb44efcc - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::DefId, ()>>
  61:     0x7f24cb4eb29a - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::check_match, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  62:     0x7f24cb357439 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::check_match
  63:     0x7f24c9a74555 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  64:     0x7f24c99f5099 - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  65:     0x7f24c99e1e0d - rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in::<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  66:     0x7f24c9a76a1b - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#1}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  67:     0x7f24c99f5369 - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#1}::{closure#0}>, ()>
  68:     0x7f24c9a6967e - <rustc_session[769c1969a479696c]::session::Session>::time::<(), rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#1}>
  69:     0x7f24c9a2fa34 - rustc_interface[efb2de776eb96ea0]::passes::analysis
  70:     0x7f24cb460214 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<(), core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>>
  71:     0x7f24cb544024 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::analysis, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  72:     0x7f24cb3327ee - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::analysis
  73:     0x7f24c9944dad - <rustc_interface[efb2de776eb96ea0]::passes::QueryContext>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  74:     0x7f24c98e1426 - <rustc_interface[efb2de776eb96ea0]::interface::Compiler>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}, core[c2b73153b1d687cd]::result::Result<core[c2b73153b1d687cd]::option::Option<rustc_interface[efb2de776eb96ea0]::queries::Linker>, rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  75:     0x7f24c98c3c05 - rustc_span[15af97997a9557ab]::with_source_map::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#1}>
  76:     0x7f24c98fcaf1 - rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>
  77:     0x7f24c98c5a82 - <scoped_tls[66056ed50dce875b]::ScopedKey<rustc_span[15af97997a9557ab]::SessionGlobals>>::set::<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  78:     0x7f24c993b709 - std[11dc701f356fae7c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  79:     0x7f24c98c5e2e - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  80:     0x7f24c993fd40 - <<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1} as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  81:     0x7f24c8f5ed25 - std::sys::unix::thread::Thread::new::thread_start::h6e088341e63c9a12
  82:     0x7f24c8cfcb43 - <unknown>
  83:     0x7f24c8d8ea00 - <unknown>
  84:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (cf7a15686 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `FOO`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `FOO`
#2 [mir_for_ctfe] caching mir of `FOO` for CTFE
#3 [eval_to_allocation_raw] const-evaluating + checking `FOO`
#4 [eval_to_const_value_raw] simplifying constant for the type system `FOO`
#5 [eval_to_const_value_raw] simplifying constant for the type system `FOO`
#6 [check_match] match-checking `main`
#7 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/drop/repeat-drop-2.rs stdout ----
---- [ui] src/test/ui/drop/repeat-drop-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/repeat-drop-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/repeat-drop-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/repeat-drop-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/drop/repeat-drop-2.rs:4:17
   |
LL |     let foo = String::new();
LL |     let foo = String::new();
   |         --- move occurs because `foo` has type `String`, which does not implement the `Copy` trait
LL |     let _bar = foo;
   |                --- value moved here
LL |     let _baz = [foo; 0]; //~ ERROR use of moved value: `foo` [E0382]
   |                 ^^^ value used here after move
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/drop/repeat-drop-2.rs:7:25
   |
   |
LL | const _: [String; 0] = [String::new(); 0];
   |                        -^^^^^^^^^^^^^----
   |                        ||
   |                        |constants cannot evaluate destructors
   |                        value is dropped here

thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f43b2e7acfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   0:     0x7f43b2e7acfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   1:     0x7f43b2ee3748 - core::fmt::write::hd17183098a2b8f9a
   2:     0x7f43b2e6b971 - std::io::Write::write_fmt::h8c18b55141e95391
   3:     0x7f43b2e7dcfe - std::panicking::default_hook::{{closure}}::hbc7e1819e45e1439
   4:     0x7f43b2e7d9c7 - std::panicking::default_hook::hf5c60f92db8187ea
   5:     0x7f43b38051d4 - rustc_driver[2f4eb290599bf9d6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f43b2e7e4b1 - std::panicking::rust_panic_with_hook::hf49d10582a3f829d
   7:     0x7f43b2e7e2d7 - std::panicking::begin_panic_handler::{{closure}}::h3b40746384dd0adb
   8:     0x7f43b2e7b274 - std::sys_common::backtrace::__rust_end_short_backtrace::ha0d6fcb399f81d73
   9:     0x7f43b2e7dfa2 - rust_begin_unwind
  10:     0x7f43b2e2ee43 - core::panicking::panic_fmt::h3a49f258a41d598f
  11:     0x7f43b2edfdd1 - core::panicking::panic_display::h018d75316b91e3d7
  12:     0x7f43b2edfd7b - core::panicking::panic_str::h7263301f4633b2c9
  13:     0x7f43b2e2ecb6 - core::option::expect_failed::h61b9de2d1355e3a8
  14:     0x7f43b6501b22 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::translation::Translate>::translate_message
  15:     0x7f43b64fee62 - <rustc_errors[ce55abf701337603]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f43b64f738a - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f43b64f49d6 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  18:     0x7f43b650d1a2 - <rustc_errors[ce55abf701337603]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f43b650bbbc - <rustc_errors[ce55abf701337603]::json::JsonEmitter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  20:     0x7f43b65516f8 - <rustc_errors[ce55abf701337603]::HandlerInner>::emit_diagnostic
  21:     0x7f43b654fff1 - <rustc_errors[ce55abf701337603]::Handler>::emit_diagnostic
  22:     0x7f43b4aec023 - rustc_borrowck[5b86fdeed8fcfb8b]::do_mir_borrowck
  23:     0x7f43b48b26ff - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult, rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck::{closure#0}>
  24:     0x7f43b4ae11d7 - rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck
  25:     0x7f43b4aac3ee - <rustc_borrowck[5b86fdeed8fcfb8b]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_span[15af97997a9557ab]::def_id::LocalDefId)>>::call_once
  26:     0x7f43b534a209 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, &rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult>>
  27:     0x7f43b541cc43 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_borrowck, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  28:     0x7f43b527d4c4 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f43b39a0896 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  30:     0x7f43b39210f9 - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f43b390e0cd - rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in::<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f43b39946f5 - <rustc_session[769c1969a479696c]::session::Session>::time::<(), rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}>
  33:     0x7f43b395ba4b - rustc_interface[efb2de776eb96ea0]::passes::analysis
  34:     0x7f43b538c214 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<(), core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>>
  35:     0x7f43b5470024 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::analysis, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  36:     0x7f43b525e7ee - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::analysis
  37:     0x7f43b3870dad - <rustc_interface[efb2de776eb96ea0]::passes::QueryContext>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  38:     0x7f43b380d426 - <rustc_interface[efb2de776eb96ea0]::interface::Compiler>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}, core[c2b73153b1d687cd]::result::Result<core[c2b73153b1d687cd]::option::Option<rustc_interface[efb2de776eb96ea0]::queries::Linker>, rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  39:     0x7f43b37efc05 - rustc_span[15af97997a9557ab]::with_source_map::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f43b3828af1 - rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>
  41:     0x7f43b37f1a82 - <scoped_tls[66056ed50dce875b]::ScopedKey<rustc_span[15af97997a9557ab]::SessionGlobals>>::set::<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  42:     0x7f43b3867709 - std[11dc701f356fae7c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  43:     0x7f43b37f1e2e - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  44:     0x7f43b386bd40 - <<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1} as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f43b2e8ad25 - std::sys::unix::thread::Thread::new::thread_start::h6e088341e63c9a12
  46:     0x7f43b2c28b43 - <unknown>
  47:     0x7f43b2cbaa00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (cf7a15686 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `must_be_init`
#1 [analysis] running analysis passes on this crate
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0381, E0382, E0493.
For more information about an error, try `rustc --explain E0381`.
For more information about an error, try `rustc --explain E0381`.
------------------------------------------


---- [ui] src/test/ui/error-codes/E0621-does-not-trigger-for-closures.rs stdout ----
diff of stderr:

4 LL |     invoke(&x, |a, b| if a > b { a } else { b });
5    |                    --                       ^ returning this value requires that `'1` must outlive `'2`
6    |                    ||
-    |                    |return type of closure is &'2 i32
+    |                    |return type of closure is &'1 i32
8    |                    has type `&'1 i32`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0621-does-not-trigger-for-closures/E0621-does-not-trigger-for-closures.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0621-does-not-trigger-for-closures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0621-does-not-trigger-for-closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0621-does-not-trigger-for-closures" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0621-does-not-trigger-for-closures/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     invoke(&x, |a, b| if a > b { a } else { b }); //~ ERROR lifetime may not live long enough
   |                    --                       ^ returning this value requires that `'1` must outlive `'2`
   |                    ||
   |                    |return type of closure is &'1 i32
   |                    has type `&'1 i32`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/fn/implied-bounds-unnorm-associated-type-2.rs stdout ----
diff of stderr:

4 LL | fn g<'a, 'b>() {
5    |      --  -- lifetime `'b` defined here
6    |      |
-    |      lifetime `'a` defined here
+    |      lifetime `'b` defined here
8 LL |     f::<'a, 'b>(());
9    |     ^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-2/implied-bounds-unnorm-associated-type-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-2/implied-bounds-unnorm-associated-type-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/implied-bounds-unnorm-associated-type-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/implied-bounds-unnorm-associated-type-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-2/auxiliary"
---

error[E0382]: use of moved value: `self.0`
  --> /checkout/src/test/ui/mir/drop-elaboration-after-borrowck-error.rs:18:9
   |
LL |     pub const fn f(mut self, other: T) -> Self {
   |                    -------- move occurs because `self` has type `B<T>`, which does not implement the `Copy` trait
LL |         let _this = self;
   |                     ---- value moved here
LL |         //~^ ERROR destructors cannot be evaluated at compile-time
LL |         self.0[0] = other;
   |         ^^^^^^^^^ value used here after move
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0381, E0382, E0493.
For more information about an error, try `rustc --explain E0381`.
For more information about an error, try `rustc --explain E0381`.
------------------------------------------


---- [ui] src/test/ui/moves/issue-72649-uninit-in-loop.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/issue-72649-uninit-in-loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/issue-72649-uninit-in-loop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/issue-72649-uninit-in-loop/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `value`
   |
LL |         let value = NonCopy{};
LL |         let value = NonCopy{};
   |             ----- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
LL |         //~^ NOTE move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
LL |         let _used = value;
   |                     ----- value moved here
LL |         //~^ NOTE value moved here
LL |         let _used2 = value; //~ ERROR use of moved value: `value`
   |                      ^^^^^ value used here after move
error[E0382]: use of moved value: `value`
  --> /checkout/src/test/ui/moves/issue-72649-uninit-in-loop.rs:32:26
   |
LL |     let value = NonCopy{};
LL |     let value = NonCopy{};
   |         ----- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
...
LL |         let _used = value;
   |                     ----- value moved here
...
LL |             let _used2 = value; //~ ERROR use of moved value: `value`
   |                          ^^^^^ value used here after move
error[E0382]: use of moved value: `value`
  --> /checkout/src/test/ui/moves/issue-72649-uninit-in-loop.rs:42:21
   |
LL |     let value = NonCopy{};
LL |     let value = NonCopy{};
   |         ----- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
...
LL |         let _used = value; //~ ERROR use of moved value: `value`
   |                     ^^^^^ value moved here, in previous iteration of loop
error[E0382]: use of moved value: `value`
  --> /checkout/src/test/ui/moves/issue-72649-uninit-in-loop.rs:53:22
   |
LL |     let mut value = NonCopy{};
LL |     let mut value = NonCopy{};
   |         --------- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
...
LL |         let _used2 = value; //~ ERROR use of moved value: `value`
   |                      ^^^^^ value moved here, in previous iteration of loop

thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7f22130d9cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   1:     0x7f2213142748 - core::fmt::write::hd17183098a2b8f9a
   2:     0x7f22130ca971 - std::io::Write::write_fmt::h8c18b55141e95391
   3:     0x7f22130dccfe - std::panicking::default_hook::{{closure}}::hbc7e1819e45e1439
   4:     0x7f22130dc9c7 - std::panicking::default_hook::hf5c60f92db8187ea
   5:     0x7f2213a641d4 - rustc_driver[2f4eb290599bf9d6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f22130dd4b1 - std::panicking::rust_panic_with_hook::hf49d10582a3f829d
   7:     0x7f22130dd2d7 - std::panicking::begin_panic_handler::{{closure}}::h3b40746384dd0adb
   8:     0x7f22130da274 - std::sys_common::backtrace::__rust_end_short_backtrace::ha0d6fcb399f81d73
   9:     0x7f22130dcfa2 - rust_begin_unwind
  10:     0x7f221308de43 - core::panicking::panic_fmt::h3a49f258a41d598f
  11:     0x7f221313edd1 - core::panicking::panic_display::h018d75316b91e3d7
  12:     0x7f221313ed7b - core::panicking::panic_str::h7263301f4633b2c9
  13:     0x7f221308dcb6 - core::option::expect_failed::h61b9de2d1355e3a8
  14:     0x7f2216760b22 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::translation::Translate>::translate_message
  15:     0x7f221675de62 - <rustc_errors[ce55abf701337603]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f221675638a - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f22167539d6 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  18:     0x7f221676c1a2 - <rustc_errors[ce55abf701337603]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f221676abbc - <rustc_errors[ce55abf701337603]::json::JsonEmitter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  20:     0x7f22167b06f8 - <rustc_errors[ce55abf701337603]::HandlerInner>::emit_diagnostic
  21:     0x7f22167aeff1 - <rustc_errors[ce55abf701337603]::Handler>::emit_diagnostic
  22:     0x7f2214d4b023 - rustc_borrowck[5b86fdeed8fcfb8b]::do_mir_borrowck
  23:     0x7f2214b116ff - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult, rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck::{closure#0}>
  24:     0x7f2214d401d7 - rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck
  25:     0x7f2214d0b3ee - <rustc_borrowck[5b86fdeed8fcfb8b]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_span[15af97997a9557ab]::def_id::LocalDefId)>>::call_once
  26:     0x7f22155a9209 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, &rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult>>
  27:     0x7f221567bc43 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_borrowck, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  28:     0x7f22154dc4c4 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f2213bff896 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  30:     0x7f2213b800f9 - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f2213b6d0cd - rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in::<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f2213bf36f5 - <rustc_session[769c1969a479696c]::session::Session>::time::<(), rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}>
  33:     0x7f2213bbaa4b - rustc_interface[efb2de776eb96ea0]::passes::analysis
  34:     0x7f22155eb214 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<(), core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>>
  35:     0x7f22156cf024 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::analysis, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  36:     0x7f22154bd7ee - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::analysis
  37:     0x7f2213acfdad - <rustc_interface[efb2de776eb96ea0]::passes::QueryContext>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  38:     0x7f2213a6c426 - <rustc_interface[efb2de776eb96ea0]::interface::Compiler>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}, core[c2b73153b1d687cd]::result::Result<core[c2b73153b1d687cd]::option::Option<rustc_interface[efb2de776eb96ea0]::queries::Linker>, rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  39:     0x7f2213a4ec05 - rustc_span[15af97997a9557ab]::with_source_map::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f2213a87af1 - rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>
  41:     0x7f2213a50a82 - <scoped_tls[66056ed50dce875b]::ScopedKey<rustc_span[15af97997a9557ab]::SessionGlobals>>::set::<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  42:     0x7f2213ac6709 - std[11dc701f356fae7c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  43:     0x7f2213a50e2e - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  44:     0x7f2213acad40 - <<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1} as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f22130e9d25 - std::sys::unix::thread::Thread::new::thread_start::h6e088341e63c9a12
  46:     0x7f2212e87b43 - <unknown>
  47:     0x7f2212f19a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (cf7a15686 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `uninit_1`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7f22130d9cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   1:     0x7f2213142748 - core::fmt::write::hd17183098a2b8f9a
   2:     0x7f22130ca971 - std::io::Write::write_fmt::h8c18b55141e95391
   3:     0x7f22130dccfe - std::panicking::default_hook::{{closure}}::hbc7e1819e45e1439
   4:     0x7f22130dc9c7 - std::panicking::default_hook::hf5c60f92db8187ea
   5:     0x7f2213a641d4 - rustc_driver[2f4eb290599bf9d6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f22130dd4b1 - std::panicking::rust_panic_with_hook::hf49d10582a3f829d
   7:     0x7f22130dd2d7 - std::panicking::begin_panic_handler::{{closure}}::h3b40746384dd0adb
   8:     0x7f22130da274 - std::sys_common::backtrace::__rust_end_short_backtrace::ha0d6fcb399f81d73
   9:     0x7f22130dcfa2 - rust_begin_unwind
  10:     0x7f221308de43 - core::panicking::panic_fmt::h3a49f258a41d598f
  11:     0x7f221313edd1 - core::panicking::panic_display::h018d75316b91e3d7
  12:     0x7f221313ed7b - core::panicking::panic_str::h7263301f4633b2c9
  13:     0x7f221308dcb6 - core::option::expect_failed::h61b9de2d1355e3a8
  14:     0x7f2216760b22 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::translation::Translate>::translate_message
  15:     0x7f221675de62 - <rustc_errors[ce55abf701337603]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f221675638a - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f22167539d6 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  18:     0x7f221676c1a2 - <rustc_errors[ce55abf701337603]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f221676abbc - <rustc_errors[ce55abf701337603]::json::JsonEmitter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  20:     0x7f22167b06f8 - <rustc_errors[ce55abf701337603]::HandlerInner>::emit_diagnostic
  21:     0x7f22167aeff1 - <rustc_errors[ce55abf701337603]::Handler>::emit_diagnostic
  22:     0x7f2214d4b023 - rustc_borrowck[5b86fdeed8fcfb8b]::do_mir_borrowck
  23:     0x7f2214b116ff - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult, rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck::{closure#0}>
  24:     0x7f2214d401d7 - rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck
  25:     0x7f2214d0b3ee - <rustc_borrowck[5b86fdeed8fcfb8b]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_span[15af97997a9557ab]::def_id::LocalDefId)>>::call_once
  26:     0x7f22155a9209 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, &rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult>>
  27:     0x7f221567bc43 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_borrowck, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  28:     0x7f22154dc4c4 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f2213bff896 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  30:     0x7f2213b800f9 - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f2213b6d0cd - rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in::<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f2213bf36f5 - <rustc_session[769c1969a479696c]::session::Session>::time::<(), rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}>
  33:     0x7f2213bbaa4b - rustc_interface[efb2de776eb96ea0]::passes::analysis
  34:     0x7f22155eb214 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<(), core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>>
  35:     0x7f22156cf024 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::analysis, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  36:     0x7f22154bd7ee - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::analysis
  37:     0x7f2213acfdad - <rustc_interface[efb2de776eb96ea0]::passes::QueryContext>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  38:     0x7f2213a6c426 - <rustc_interface[efb2de776eb96ea0]::interface::Compiler>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}, core[c2b73153b1d687cd]::result::Result<core[c2b73153b1d687cd]::option::Option<rustc_interface[efb2de776eb96ea0]::queries::Linker>, rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  39:     0x7f2213a4ec05 - rustc_span[15af97997a9557ab]::with_source_map::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f2213a87af1 - rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>
  41:     0x7f2213a50a82 - <scoped_tls[66056ed50dce875b]::ScopedKey<rustc_span[15af97997a9557ab]::SessionGlobals>>::set::<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  42:     0x7f2213ac6709 - std[11dc701f356fae7c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  43:     0x7f2213a50e2e - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  44:     0x7f2213acad40 - <<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1} as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f22130e9d25 - std::sys::unix::thread::Thread::new::thread_start::h6e088341e63c9a12
  46:     0x7f2212e87b43 - <unknown>
  47:     0x7f2212f19a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (cf7a15686 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `uninit_2`
#1 [analysis] running analysis passes on this crate
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0381, E0382.
For more information about an error, try `rustc --explain E0381`.
For more information about an error, try `rustc --explain E0381`.
------------------------------------------


---- [ui] src/test/ui/moves/move-into-dead-array-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/move-into-dead-array-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-into-dead-array-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-into-dead-array-1/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7fc585f00cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   1:     0x7fc585f69748 - core::fmt::write::hd17183098a2b8f9a
   2:     0x7fc585ef1971 - std::io::Write::write_fmt::h8c18b55141e95391
   3:     0x7fc585f03cfe - std::panicking::default_hook::{{closure}}::hbc7e1819e45e1439
   4:     0x7fc585f039c7 - std::panicking::default_hook::hf5c60f92db8187ea
   5:     0x7fc58688b1d4 - rustc_driver[2f4eb290599bf9d6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc585f044b1 - std::panicking::rust_panic_with_hook::hf49d10582a3f829d
   7:     0x7fc585f042d7 - std::panicking::begin_panic_handler::{{closure}}::h3b40746384dd0adb
   8:     0x7fc585f01274 - std::sys_common::backtrace::__rust_end_short_backtrace::ha0d6fcb399f81d73
   9:     0x7fc585f03fa2 - rust_begin_unwind
  10:     0x7fc585eb4e43 - core::panicking::panic_fmt::h3a49f258a41d598f
  11:     0x7fc585f65dd1 - core::panicking::panic_display::h018d75316b91e3d7
  12:     0x7fc585f65d7b - core::panicking::panic_str::h7263301f4633b2c9
  13:     0x7fc585eb4cb6 - core::option::expect_failed::h61b9de2d1355e3a8
  14:     0x7fc589587b22 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::translation::Translate>::translate_message
  15:     0x7fc589584e62 - <rustc_errors[ce55abf701337603]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fc58957d38a - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fc58957a9d6 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  18:     0x7fc5895931a2 - <rustc_errors[ce55abf701337603]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fc589591bbc - <rustc_errors[ce55abf701337603]::json::JsonEmitter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  20:     0x7fc5895d76f8 - <rustc_errors[ce55abf701337603]::HandlerInner>::emit_diagnostic
  21:     0x7fc5895d5ff1 - <rustc_errors[ce55abf701337603]::Handler>::emit_diagnostic
  22:     0x7fc587b72023 - rustc_borrowck[5b86fdeed8fcfb8b]::do_mir_borrowck
  23:     0x7fc5879386ff - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult, rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck::{closure#0}>
  24:     0x7fc587b671d7 - rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck
  25:     0x7fc587b323ee - <rustc_borrowck[5b86fdeed8fcfb8b]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_span[15af97997a9557ab]::def_id::LocalDefId)>>::call_once
  26:     0x7fc5883d0209 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, &rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult>>
  27:     0x7fc5884a2c43 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_borrowck, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  28:     0x7fc5883034c4 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fc586a26896 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  30:     0x7fc5869a70f9 - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7fc5869940cd - rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in::<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7fc586a1a6f5 - <rustc_session[769c1969a479696c]::session::Session>::time::<(), rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}>
  33:     0x7fc5869e1a4b - rustc_interface[efb2de776eb96ea0]::passes::analysis
  34:     0x7fc588412214 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<(), core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>>
  35:     0x7fc5884f6024 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::analysis, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  36:     0x7fc5882e47ee - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::analysis
  37:     0x7fc5868f6dad - <rustc_interface[efb2de776eb96ea0]::passes::QueryContext>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  38:     0x7fc586893426 - <rustc_interface[efb2de776eb96ea0]::interface::Compiler>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}, core[c2b73153b1d687cd]::result::Result<core[c2b73153b1d687cd]::option::Option<rustc_interface[efb2de776eb96ea0]::queries::Linker>, rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  39:     0x7fc586875c05 - rustc_span[15af97997a9557ab]::with_source_map::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7fc5868aeaf1 - rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>
  41:     0x7fc586877a82 - <scoped_tls[66056ed50dce875b]::ScopedKey<rustc_span[15af97997a9557ab]::SessionGlobals>>::set::<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  42:     0x7fc5868ed709 - std[11dc701f356fae7c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  43:     0x7fc586877e2e - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  44:     0x7fc5868f1d40 - <<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1} as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7fc585f10d25 - std::sys::unix::thread::Thread::new::thread_start::h6e088341e63c9a12
  46:     0x7fc585caeb43 - <unknown>
  47:     0x7fc585d40a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (cf7a15686 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `foo`
#1 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/moves/move-of-addr-of-mut.rs stdout ----
---- [ui] src/test/ui/moves/move-of-addr-of-mut.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/move-of-addr-of-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-of-addr-of-mut" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-of-addr-of-mut/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7fcd5e810cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   1:     0x7fcd5e879748 - core::fmt::write::hd17183098a2b8f9a
   2:     0x7fcd5e801971 - std::io::Write::write_fmt::h8c18b55141e95391
   3:     0x7fcd5e813cfe - std::panicking::default_hook::{{closure}}::hbc7e1819e45e1439
   4:     0x7fcd5e8139c7 - std::panicking::default_hook::hf5c60f92db8187ea
   5:     0x7fcd5f19b1d4 - rustc_driver[2f4eb290599bf9d6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fcd5e8144b1 - std::panicking::rust_panic_with_hook::hf49d10582a3f829d
   7:     0x7fcd5e8142d7 - std::panicking::begin_panic_handler::{{closure}}::h3b40746384dd0adb
   8:     0x7fcd5e811274 - std::sys_common::backtrace::__rust_end_short_backtrace::ha0d6fcb399f81d73
   9:     0x7fcd5e813fa2 - rust_begin_unwind
  10:     0x7fcd5e7c4e43 - core::panicking::panic_fmt::h3a49f258a41d598f
  11:     0x7fcd5e875dd1 - core::panicking::panic_display::h018d75316b91e3d7
  12:     0x7fcd5e875d7b - core::panicking::panic_str::h7263301f4633b2c9
  13:     0x7fcd5e7c4cb6 - core::option::expect_failed::h61b9de2d1355e3a8
  14:     0x7fcd61e97b22 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::translation::Translate>::translate_message
  15:     0x7fcd61e94e62 - <rustc_errors[ce55abf701337603]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fcd61e8d38a - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fcd61e8a9d6 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  18:     0x7fcd61ea31a2 - <rustc_errors[ce55abf701337603]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fcd61ea1bbc - <rustc_errors[ce55abf701337603]::json::JsonEmitter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  20:     0x7fcd61ee76f8 - <rustc_errors[ce55abf701337603]::HandlerInner>::emit_diagnostic
  21:     0x7fcd61ee5ff1 - <rustc_errors[ce55abf701337603]::Handler>::emit_diagnostic
  22:     0x7fcd60482023 - rustc_borrowck[5b86fdeed8fcfb8b]::do_mir_borrowck
  23:     0x7fcd602486ff - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult, rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck::{closure#0}>
  24:     0x7fcd604771d7 - rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck
  25:     0x7fcd604423ee - <rustc_borrowck[5b86fdeed8fcfb8b]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_span[15af97997a9557ab]::def_id::LocalDefId)>>::call_once
  26:     0x7fcd60ce0209 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, &rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult>>
  27:     0x7fcd60db2c43 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_borrowck, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  28:     0x7fcd60c134c4 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fcd5f336896 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  30:     0x7fcd5f2b70f9 - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7fcd5f2a40cd - rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in::<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7fcd5f32a6f5 - <rustc_session[769c1969a479696c]::session::Session>::time::<(), rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}>
  33:     0x7fcd5f2f1a4b - rustc_interface[efb2de776eb96ea0]::passes::analysis
  34:     0x7fcd60d22214 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<(), core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>>
  35:     0x7fcd60e06024 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::analysis, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  36:     0x7fcd60bf47ee - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::analysis
  37:     0x7fcd5f206dad - <rustc_interface[efb2de776eb96ea0]::passes::QueryContext>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  38:     0x7fcd5f1a3426 - <rustc_interface[efb2de776eb96ea0]::interface::Compiler>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}, core[c2b73153b1d687cd]::result::Result<core[c2b73153b1d687cd]::option::Option<rustc_interface[efb2de776eb96ea0]::queries::Linker>, rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  39:     0x7fcd5f185c05 - rustc_span[15af97997a9557ab]::with_source_map::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7fcd5f1beaf1 - rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>
  41:     0x7fcd5f187a82 - <scoped_tls[66056ed50dce875b]::ScopedKey<rustc_span[15af97997a9557ab]::SessionGlobals>>::set::<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  42:     0x7fcd5f1fd709 - std[11dc701f356fae7c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  43:     0x7fcd5f187e2e - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  44:     0x7fcd5f201d40 - <<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1} as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7fcd5e820d25 - std::sys::unix::thread::Thread::new::thread_start::h6e088341e63c9a12
  46:     0x7fcd5e5beb43 - <unknown>
  47:     0x7fcd5e650a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (cf7a15686 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main`
#1 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/nll/closure-requirements/escape-argument-callee.rs stdout ----
---- [ui] src/test/ui/nll/closure-requirements/escape-argument-callee.rs stdout ----
diff of stderr:

17    |                                       -  -  ^^^^^^ assignment requires that `'1` must outlive `'2`
18    |                                       |  |
19    |                                       |  has type `&'1 i32`
-    |                                       has type `&'_#2r mut &'2 i32`
+    |                                       has type `&'1 i32`
22 note: no external requirements
23   --> $DIR/escape-argument-callee.rs:20:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee/escape-argument-callee.stderr
To only update this specific test, also pass `--test-args nll/closure-requirements/escape-argument-callee.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |         let mut closure = expect_sig(|p, y| *p = y);
   |
   |
   = note: defining type: test::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) mut &ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) i32, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) i32)),
               (),

error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs:26:45
   |
   |
LL |         let mut closure = expect_sig(|p, y| *p = y);
   |                                       -  -  ^^^^^^ assignment requires that `'1` must outlive `'2`
   |                                       |  |
   |                                       |  has type `&'1 i32`
   |                                       has type `&'1 i32`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs:20:1
   |
LL | / fn test() {
LL | / fn test() {
LL | |     let x = 44;
LL | |     let mut p = &x;
LL | |
LL | |     deref(p);
LL | | }
   | |_^
   |
---
diff of stderr:

17   --> $DIR/propagate-approximated-fail-no-postdom.rs:46:13
18    |
19 LL |         |_outlives1, _outlives2, _outlives3, x, y| {
-    |          ----------              ---------- has type `Cell<&'2 &'_#3r u32>`
+    |          ----------              ---------- has type `Cell<&'_#1r &'1 u32>`
21    |          |
22    |          has type `Cell<&'_#1r &'1 u32>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/propagate-approximated-fail-no-postdom.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/propagate-approximated-fail-no-postdom.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-fail-no-postdom.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |         |_outlives1, _outlives2, _outlives3, x, y| {
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#3r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>)),
               (),
   = note: late-bound region is '_#4r
   = note: late-bound region is '_#5r
   = note: late-bound region is '_#6r


error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs:46:13
   |
LL |         |_outlives1, _outlives2, _outlives3, x, y| {
   |          ----------              ---------- has type `Cell<&'_#1r &'1 u32>`
   |          |
   |          has type `Cell<&'_#1r &'1 u32>`
...
LL |             demand_y(x, y, p) //~ ERROR
   |             ^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs:38:1
   |
   |
LL | / fn supply<'a, 'b, 'c>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>, cell_c: Cell<&'c u32>) {
LL | |     establish_relationships(
LL | |         cell_a,
LL | |         cell_b,
LL | |     );
LL | | }
   | |_^
   |
---
diff of stderr:

32   --> $DIR/propagate-approximated-ref.rs:45:9
33    |
34 LL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
-    |           --  -- lifetime `'b` defined here
+    |           --  -- lifetime `'a` defined here
36    |           |
37    |           lifetime `'a` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/propagate-approximated-ref.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/propagate-approximated-ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-ref.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:43:47
   |
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 4, kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 5, kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) u32>)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r
   = note: late-bound region is '_#4r
   = note: number of external vids: 5
   = note: where '_#1r: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:42:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR lifetime may not live long enough
LL | |     });
LL | | }
   |
   = note: defining type: supply

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:45:9
   |
LL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
   |           --  -- lifetime `'a` defined here
   |           |
   |           lifetime `'a` defined here
...
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2.rs stdout ----
diff of stderr:

2   --> $DIR/region-lbr1-does-not-outlive-ebr2.rs:9:5
3    |
4 LL | fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'b u32 {
-    |        --  -- lifetime `'b` defined here
+    |        --  -- lifetime `'a` defined here
6    |        |
7    |        lifetime `'a` defined here
8 LL |     &*x

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2/region-lbr1-does-not-outlive-ebr2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/region-lbr1-does-not-outlive-ebr2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'b u32 {
   |        --  -- lifetime `'a` defined here
   |        |
   |        lifetime `'a` defined here
LL |     &*x
   |     ^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/propagate-approximated-val.rs stdout ----
diff of stderr:

32   --> $DIR/propagate-approximated-val.rs:38:9
33    |
34 LL | fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
-    |         --  -- lifetime `'b` defined here
+    |         --  -- lifetime `'a` defined here
36    |         |
37    |         lifetime `'a` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/propagate-approximated-val.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/propagate-approximated-val.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-val.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs:36:45
   |
   |
LL |     establish_relationships(cell_a, cell_b, |outlives1, outlives2, x, y| {
   |
   |
   = note: defining type: test::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r
   = note: late-bound region is '_#4r
   = note: number of external vids: 5
   = note: where '_#1r: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs:35:1
   |
   |
LL | / fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(cell_a, cell_b, |outlives1, outlives2, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(outlives1, outlives2, x.get())
LL | |         //~^ ERROR lifetime may not live long enough
LL | |     });
LL | | }
   |
   = note: defining type: test

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs:38:9
   |
LL | fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
   |         --  -- lifetime `'a` defined here
   |         |
   |         lifetime `'a` defined here
...
LL |         demand_y(outlives1, outlives2, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs stdout ----
diff of stderr:

16   --> $DIR/propagate-fail-to-approximate-longer-wrong-bounds.rs:41:9
17    |
18 LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
-    |                                                ----------  ---------- has type `&'_#8r Cell<&'2 &'_#2r u32>`
+    |                                                ----------  ---------- has type `&'_#6r Cell<&'1 &'_#1r u32>`
20    |                                                |
21    |                                                has type `&'_#6r Cell<&'1 &'_#1r u32>`
22 LL |         // Only works if 'x: 'y:

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/propagate-fail-to-approximate-longer-wrong-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 4, kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 5, kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) u32>)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:41:9
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |                                                ----------  ---------- has type `&'_#6r Cell<&'1 &'_#1r u32>`
   |                                                |
   |                                                has type `&'_#6r Cell<&'1 &'_#1r u32>`
LL |         // Only works if 'x: 'y:
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:38:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR
LL | |     });
LL | | }
   |
   = note: defining type: supply

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs stdout ----
diff of stderr:

18 LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
19    |                                                ---------  - has type `&'_#7r Cell<&'1 u32>`
20    |                                                |
-    |                                                has type `&'_#5r Cell<&'2 &'_#1r u32>`
+    |                                                has type `&'_#7r Cell<&'1 u32>`
22 LL |         // Only works if 'x: 'y:
23 LL |         demand_y(x, y, x.get())
24    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/propagate-fail-to-approximate-longer-no-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 4, kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>)),
               (),
   = note: late-bound region is '_#2r
   = note: late-bound region is '_#3r

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:37:9
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
   |                                                ---------  - has type `&'_#7r Cell<&'1 u32>`
   |                                                |
   |                                                has type `&'_#7r Cell<&'1 u32>`
LL |         // Only works if 'x: 'y:
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:34:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR
LL | |     });
LL | | }
   |
   = note: defining type: supply

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs stdout ----
diff of stderr:

17    |                 -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
18    |                 |  |
19    |                 |  has type `&'1 i32`
-    |                 has type `&'2 i32`
+    |                 has type `&'1 i32`
22 note: no external requirements
23   --> $DIR/return-wrong-bound-region.rs:10:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/return-wrong-bound-region.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/return-wrong-bound-region.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |     expect_sig(|a, b| b); // ought to return `a`
   |
   |
   = note: defining type: test::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) i32, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) i32)) -> &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) i32,
               (),

error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:11:23
   |
   |
LL |     expect_sig(|a, b| b); // ought to return `a`
   |                 -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
   |                 |  |
   |                 |  has type `&'1 i32`
   |                 has type `&'1 i32`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:10:1
   |
LL | / fn test() {
LL | / fn test() {
LL | |     expect_sig(|a, b| b); // ought to return `a`
LL | |     //~^ ERROR
LL | | }
   |
   = note: defining type: test

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/nll/issue-21232-partial-init-and-erroneous-use.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-21232-partial-init-and-erroneous-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-21232-partial-init-and-erroneous-use" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-21232-partial-init-and-erroneous-use/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7fc5d65ebcfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   1:     0x7fc5d6654748 - core::fmt::write::hd17183098a2b8f9a
   2:     0x7fc5d65dc971 - std::io::Write::write_fmt::h8c18b55141e95391
   3:     0x7fc5d65eecfe - std::panicking::default_hook::{{closure}}::hbc7e1819e45e1439
   4:     0x7fc5d65ee9c7 - std::panicking::default_hook::hf5c60f92db8187ea
   5:     0x7fc5d6f761d4 - rustc_driver[2f4eb290599bf9d6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc5d65ef4b1 - std::panicking::rust_panic_with_hook::hf49d10582a3f829d
   7:     0x7fc5d65ef2d7 - std::panicking::begin_panic_handler::{{closure}}::h3b40746384dd0adb
   8:     0x7fc5d65ec274 - std::sys_common::backtrace::__rust_end_short_backtrace::ha0d6fcb399f81d73
   9:     0x7fc5d65eefa2 - rust_begin_unwind
  10:     0x7fc5d659fe43 - core::panicking::panic_fmt::h3a49f258a41d598f
  11:     0x7fc5d6650dd1 - core::panicking::panic_display::h018d75316b91e3d7
  12:     0x7fc5d6650d7b - core::panicking::panic_str::h7263301f4633b2c9
  13:     0x7fc5d659fcb6 - core::option::expect_failed::h61b9de2d1355e3a8
  14:     0x7fc5d9c72b22 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::translation::Translate>::translate_message
  15:     0x7fc5d9c6fe62 - <rustc_errors[ce55abf701337603]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fc5d9c6838a - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fc5d9c659d6 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  18:     0x7fc5d9c7e1a2 - <rustc_errors[ce55abf701337603]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fc5d9c7cbbc - <rustc_errors[ce55abf701337603]::json::JsonEmitter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  20:     0x7fc5d9cc26f8 - <rustc_errors[ce55abf701337603]::HandlerInner>::emit_diagnostic
  21:     0x7fc5d9cc0ff1 - <rustc_errors[ce55abf701337603]::Handler>::emit_diagnostic
  22:     0x7fc5d825d023 - rustc_borrowck[5b86fdeed8fcfb8b]::do_mir_borrowck
  23:     0x7fc5d80236ff - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult, rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck::{closure#0}>
  24:     0x7fc5d82521d7 - rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck
  25:     0x7fc5d821d3ee - <rustc_borrowck[5b86fdeed8fcfb8b]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_span[15af97997a9557ab]::def_id::LocalDefId)>>::call_once
  26:     0x7fc5d8abb209 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, &rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult>>
  27:     0x7fc5d8b8dc43 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_borrowck, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  28:     0x7fc5d89ee4c4 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fc5d7111896 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  30:     0x7fc5d70920f9 - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7fc5d707f0cd - rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in::<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7fc5d71056f5 - <rustc_session[769c1969a479696c]::session::Session>::time::<(), rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}>
  33:     0x7fc5d70cca4b - rustc_interface[efb2de776eb96ea0]::passes::analysis
  34:     0x7fc5d8afd214 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<(), core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>>
  35:     0x7fc5d8be1024 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::analysis, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  36:     0x7fc5d89cf7ee - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::analysis
  37:     0x7fc5d6fe1dad - <rustc_interface[efb2de776eb96ea0]::passes::QueryContext>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  38:     0x7fc5d6f7e426 - <rustc_interface[efb2de776eb96ea0]::interface::Compiler>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}, core[c2b73153b1d687cd]::result::Result<core[c2b73153b1d687cd]::option::Option<rustc_interface[efb2de776eb96ea0]::queries::Linker>, rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  39:     0x7fc5d6f60c05 - rustc_span[15af97997a9557ab]::with_source_map::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7fc5d6f99af1 - rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>
  41:     0x7fc5d6f62a82 - <scoped_tls[66056ed50dce875b]::ScopedKey<rustc_span[15af97997a9557ab]::SessionGlobals>>::set::<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  42:     0x7fc5d6fd8709 - std[11dc701f356fae7c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  43:     0x7fc5d6f62e2e - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  44:     0x7fc5d6fdcd40 - <<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1} as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7fc5d65fbd25 - std::sys::unix::thread::Thread::new::thread_start::h6e088341e63c9a12
  46:     0x7fc5d6399b43 - <unknown>
  47:     0x7fc5d642ba00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (cf7a15686 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `cannot_partially_init_adt_with_drop`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7fc5d65ebcfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5628b4b45d6c30de
   1:     0x7fc5d6654748 - core::fmt::write::hd17183098a2b8f9a
   2:     0x7fc5d65dc971 - std::io::Write::write_fmt::h8c18b55141e95391
   3:     0x7fc5d65eecfe - std::panicking::default_hook::{{closure}}::hbc7e1819e45e1439
   4:     0x7fc5d65ee9c7 - std::panicking::default_hook::hf5c60f92db8187ea
   5:     0x7fc5d6f761d4 - rustc_driver[2f4eb290599bf9d6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc5d65ef4b1 - std::panicking::rust_panic_with_hook::hf49d10582a3f829d
   7:     0x7fc5d65ef2d7 - std::panicking::begin_panic_handler::{{closure}}::h3b40746384dd0adb
   8:     0x7fc5d65ec274 - std::sys_common::backtrace::__rust_end_short_backtrace::ha0d6fcb399f81d73
   9:     0x7fc5d65eefa2 - rust_begin_unwind
  10:     0x7fc5d659fe43 - core::panicking::panic_fmt::h3a49f258a41d598f
  11:     0x7fc5d6650dd1 - core::panicking::panic_display::h018d75316b91e3d7
  12:     0x7fc5d6650d7b - core::panicking::panic_str::h7263301f4633b2c9
  13:     0x7fc5d659fcb6 - core::option::expect_failed::h61b9de2d1355e3a8
  14:     0x7fc5d9c72b22 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::translation::Translate>::translate_message
  15:     0x7fc5d9c6fe62 - <rustc_errors[ce55abf701337603]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fc5d9c6838a - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fc5d9c659d6 - <rustc_errors[ce55abf701337603]::emitter::EmitterWriter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  18:     0x7fc5d9c7e1a2 - <rustc_errors[ce55abf701337603]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fc5d9c7cbbc - <rustc_errors[ce55abf701337603]::json::JsonEmitter as rustc_errors[ce55abf701337603]::emitter::Emitter>::emit_diagnostic
  20:     0x7fc5d9cc26f8 - <rustc_errors[ce55abf701337603]::HandlerInner>::emit_diagnostic
  21:     0x7fc5d9cc0ff1 - <rustc_errors[ce55abf701337603]::Handler>::emit_diagnostic
  22:     0x7fc5d825d023 - rustc_borrowck[5b86fdeed8fcfb8b]::do_mir_borrowck
  23:     0x7fc5d80236ff - <rustc_infer[49ae84ee19bed59e]::infer::InferCtxtBuilder>::enter::<rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult, rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck::{closure#0}>
  24:     0x7fc5d82521d7 - rustc_borrowck[5b86fdeed8fcfb8b]::mir_borrowck
  25:     0x7fc5d821d3ee - <rustc_borrowck[5b86fdeed8fcfb8b]::provide::{closure#0} as core[c2b73153b1d687cd]::ops::function::FnOnce<(rustc_middle[3a3d34b0b0b7c294]::ty::context::TyCtxt, rustc_span[15af97997a9557ab]::def_id::LocalDefId)>>::call_once
  26:     0x7fc5d8abb209 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<rustc_span[15af97997a9557ab]::def_id::LocalDefId, &rustc_middle[3a3d34b0b0b7c294]::mir::query::BorrowCheckResult>>
  27:     0x7fc5d8b8dc43 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::mir_borrowck, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  28:     0x7fc5d89ee4c4 - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fc5d7111896 - <core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once
  30:     0x7fc5d70920f9 - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7fc5d707f0cd - rustc_data_structures[c71a9283324d9f00]::sync::par_for_each_in::<&[rustc_span[15af97997a9557ab]::def_id::LocalDefId], <rustc_middle[3a3d34b0b0b7c294]::hir::map::Map>::par_body_owners<rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7fc5d71056f5 - <rustc_session[769c1969a479696c]::session::Session>::time::<(), rustc_interface[efb2de776eb96ea0]::passes::analysis::{closure#2}>
  33:     0x7fc5d70cca4b - rustc_interface[efb2de776eb96ea0]::passes::analysis
  34:     0x7fc5d8afd214 - rustc_query_system[328747d3786d955d]::query::plumbing::try_execute_query::<rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt, rustc_query_system[328747d3786d955d]::query::caches::DefaultCache<(), core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>>
  35:     0x7fc5d8be1024 - rustc_query_system[328747d3786d955d]::query::plumbing::get_query::<rustc_query_impl[fafaade1cf861eb9]::queries::analysis, rustc_query_impl[fafaade1cf861eb9]::plumbing::QueryCtxt>
  36:     0x7fc5d89cf7ee - <rustc_query_impl[fafaade1cf861eb9]::Queries as rustc_middle[3a3d34b0b0b7c294]::ty::query::QueryEngine>::analysis
  37:     0x7fc5d6fe1dad - <rustc_interface[efb2de776eb96ea0]::passes::QueryContext>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  38:     0x7fc5d6f7e426 - <rustc_interface[efb2de776eb96ea0]::interface::Compiler>::enter::<rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}::{closure#2}, core[c2b73153b1d687cd]::result::Result<core[c2b73153b1d687cd]::option::Option<rustc_interface[efb2de776eb96ea0]::queries::Linker>, rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  39:     0x7fc5d6f60c05 - rustc_span[15af97997a9557ab]::with_source_map::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7fc5d6f99af1 - rustc_interface[efb2de776eb96ea0]::interface::create_compiler_and_run::<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>
  41:     0x7fc5d6f62a82 - <scoped_tls[66056ed50dce875b]::ScopedKey<rustc_span[15af97997a9557ab]::SessionGlobals>>::set::<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  42:     0x7fc5d6fd8709 - std[11dc701f356fae7c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  43:     0x7fc5d6f62e2e - std[11dc701f356fae7c]::panic::catch_unwind::<core[c2b73153b1d687cd]::panic::unwind_safe::AssertUnwindSafe<<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>
  44:     0x7fc5d6fdcd40 - <<std[11dc701f356fae7c]::thread::Builder>::spawn_unchecked_<rustc_interface[efb2de776eb96ea0]::util::run_in_thread_pool_with_globals<rustc_interface[efb2de776eb96ea0]::interface::run_compiler<core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>, rustc_driver[2f4eb290599bf9d6]::run_compiler::{closure#1}>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#0}, core[c2b73153b1d687cd]::result::Result<(), rustc_errors[ce55abf701337603]::ErrorGuaranteed>>::{closure#1} as core[c2b73153b1d687cd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7fc5d65fbd25 - std::sys::unix::thread::Thread::new::thread_start::h6e088341e63c9a12
  46:     0x7fc5d6399b43 - <unknown>
  47:     0x7fc5d642ba00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

---
diff of stderr:

2   --> $DIR/issue-52113.rs:30:9
3    |
4 LL | fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {
-    |                --  -- lifetime `'b` defined here
+    |                --  -- lifetime `'a` defined here
6    |                |
7    |                lifetime `'a` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52113/issue-52113.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52113/issue-52113.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52113.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52113.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52113" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52113/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {
   |                --  -- lifetime `'a` defined here
   |                |
   |                lifetime `'a` defined here
...
LL |         data.push(value); //~ ERROR lifetime may not live long enough
   |         ^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-52213.rs stdout ----
diff of stderr:

2   --> $DIR/issue-52213.rs:3:20
3    |
4 LL | fn transmute_lifetime<'a, 'b, T>(t: &'a (T,)) -> &'b T {
-    |                       --  -- lifetime `'b` defined here
+    |                       --  -- lifetime `'a` defined here
6    |                       |
7    |                       lifetime `'a` defined here
8 LL |     match (&t,) {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52213/issue-52213.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52213.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52213.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52213" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52213/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn transmute_lifetime<'a, 'b, T>(t: &'a (T,)) -> &'b T {
   |                       --  -- lifetime `'a` defined here
   |                       |
   |                       lifetime `'a` defined here
LL |     match (&t,) {
LL |         ((u,),) => u,
   |                    ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-52533-1.rs stdout ----
diff of stderr:

5    |            -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
6    |            |  |
7    |            |  has type `&Foo<'_, '1, u32>`
-    |            has type `&Foo<'_, '2, u32>`
+    |            has type `&Foo<'_, '1, u32>`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52533-1/issue-52533-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52533-1.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52533-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52533-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52533-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     gimme(|x, y| y)
   |            -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
   |            |  |
   |            |  has type `&Foo<'_, '1, u32>`
   |            has type `&Foo<'_, '1, u32>`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-52534.rs stdout ----
diff of stderr:

2   --> $DIR/issue-52534.rs:9:14
3    |
4 LL |     foo(|a| &x)
-    |          -   ^ `x` would have to be valid for `'0`...
-    |          has type `&'0 u32`
-    |          has type `&'0 u32`
+    |              ^ `x` would have to be valid for `'0`...
9 LL | }
9 LL | }
10    | - ...but `x` will be dropped here, when the function `bar` returns
11    |
11    |
+ help: has type `&'0 u32`
+    |
+    |
+ LL |     foo(|a| &x)
+    |          ^
12    = note: functions cannot return a borrow to data owned within the function's scope, functions can only return borrows to data passed as arguments
13    = note: to learn more, visit <https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references>

16   --> $DIR/issue-52534.rs:15:26
17    |
17    |
18 LL |     baz(|first, second| &y)
-    |          -----           ^ `y` would have to be valid for `'0`...
-    |          has type `&'0 u32`
-    |          has type `&'0 u32`
+    |                          ^ `y` would have to be valid for `'0`...
23 LL | }
23 LL | }
24    | - ...but `y` will be dropped here, when the function `foobar` returns
25    |
25    |
+ help: has type `&'0 u32`
+    |
+    |
+ LL |     baz(|first, second| &y)
+    |          ^^^^^
26    = note: functions cannot return a borrow to data owned within the function's scope, functions can only return borrows to data passed as arguments
27    = note: to learn more, visit <https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references>


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52534/issue-52534.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52534/issue-52534.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52534.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52534.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52534" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52534/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL |     foo(|a| &x)
   |              ^ `x` would have to be valid for `'0`...
LL | //~^ ERROR does not live long enough
LL | }
   | - ...but `x` will be dropped here, when the function `bar` returns
help: has type `&'0 u32`
  --> /checkout/src/test/ui/nll/issue-52534.rs:9:10
   |
   |
LL |     foo(|a| &x)
   |          ^
   = note: functions cannot return a borrow to data owned within the function's scope, functions can only return borrows to data passed as arguments
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references>

error[E0597]: `y` does not live long enough
   |
   |
LL |     baz(|first, second| &y)
   |                          ^ `y` would have to be valid for `'0`...
LL | //~^ ERROR does not live long enough
LL | }
   | - ...but `y` will be dropped here, when the function `foobar` returns
help: has type `&'0 u32`
  --> /checkout/src/test/ui/nll/issue-52534.rs:15:10
   |
   |
LL |     baz(|first, second| &y)
   |          ^^^^^
   = note: functions cannot return a borrow to data owned within the function's scope, functions can only return borrows to data passed as arguments
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references>
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/issue-52742.rs stdout ----
diff of stderr:

4 LL |     fn take_bar(&mut self, b: Bar<'_>) {
5    |                 ---------  - has type `Bar<'1>`
-    |                 has type `&mut Foo<'_, '2>`
+    |                 has type `Bar<'1>`
+    |                 has type `Bar<'1>`
8 LL |         self.y = b.z
9    |         ^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/issue-52742.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/issue-52742.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52742.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     fn take_bar(&mut self, b: Bar<'_>) {
   |                 ---------  - has type `Bar<'1>`
   |                 has type `Bar<'1>`
   |                 has type `Bar<'1>`
LL |         self.y = b.z
   |         ^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-58053.rs stdout ----
diff of stderr:

4 LL |     let f = |x: &i32| -> &i32 { x };
5    |                 -        -      ^ returning this value requires that `'1` must outlive `'2`
6    |                 |        |
-    |                 |        let's call the lifetime of this reference `'2`
+    |                 |        let's call the lifetime of this reference `'1`
8    |                 let's call the lifetime of this reference `'1`
10 error: lifetime may not live long enough


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-58053/issue-58053.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-58053.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-58053.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-58053" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-58053/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     let f = |x: &i32| -> &i32 { x };
   |                 -        -      ^ returning this value requires that `'1` must outlive `'2`
   |                 |        |
   |                 |        let's call the lifetime of this reference `'1`
   |                 let's call the lifetime of this reference `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-58053.rs:8:25
   |
   |
LL |     let g = |x: &i32| { x };
   |                 -   -   ^ returning this value requires that `'1` must outlive `'2`
   |                 |   |
   |                 |   return type of closure is &'2 i32
   |                 let's call the lifetime of this reference `'1`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/nll/issue-67007-escaping-data.rs stdout ----
diff of stderr:

2   --> $DIR/issue-67007-escaping-data.rs:15:21
3    |
4 LL | impl<'tcx> Consumer<'tcx> {
-    |      ---- lifetime `'tcx` defined here
+    |      ---- lifetime `'a` defined here
6 LL |     fn bad_method<'a>(&self, fcx: &FnCtxt<'a, 'tcx>) {
7    |                   -- lifetime `'a` defined here
8 LL |         let other = self.use_fcx(fcx);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-67007-escaping-data/issue-67007-escaping-data.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-67007-escaping-data.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-67007-escaping-data.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-67007-escaping-data" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-67007-escaping-data/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-67007-escaping-data.rs:15:21
   |
LL | impl<'tcx> Consumer<'tcx> {
   |      ---- lifetime `'a` defined here
LL |     fn bad_method<'a>(&self, fcx: &FnCtxt<'a, 'tcx>) {
   |                   -- lifetime `'a` defined here
LL |         let other = self.use_fcx(fcx); //~ ERROR lifetime may not live long enough
   |                     ^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
   |
   = help: consider adding the following bound: `'a: 'tcx`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-95272.rs stdout ----
diff of stderr:

2   --> $DIR/issue-95272.rs:10:13
3    |
4 LL | fn test<'a, 'b>(x: Cell<&'a ()>, y: Cell<&'b ()>) {
-    |         --  -- lifetime `'b` defined here
+    |         --  -- lifetime `'a` defined here
6    |         |
7    |         lifetime `'a` defined here
8 LL |     let f = check;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-95272/issue-95272.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-95272.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-95272.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-95272" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-95272/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn test<'a, 'b>(x: Cell<&'a ()>, y: Cell<&'b ()>) {
   |         --  -- lifetime `'a` defined here
   |         |
   |         lifetime `'a` defined here
LL |     let f = check;
   |             ^^^^^ assignment requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a function pointer to `check`
   = note: the function `check` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-98170.rs stdout ----
diff of stderr:

4 LL | impl MyStruct<'_> {
5    |               -- lifetime `'1` appears in the `impl`'s self type
6 LL |     pub fn new<'a>(field: &'a [u32]) -> MyStruct<'a> {
-    |                -- lifetime `'a` defined here
+    |                -- lifetime `'1` defined here
8 LL |         Self { field }
9    |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`

12   --> $DIR/issue-98170.rs:7:16
13    |
13    |
14 LL | impl MyStruct<'_> {
-    |               -- lifetime `'1` appears in the `impl`'s self type
+    |               -- lifetime `'a` appears in the `impl`'s self type
16 LL |     pub fn new<'a>(field: &'a [u32]) -> MyStruct<'a> {
17    |                -- lifetime `'a` defined here
18 LL |         Self { field }

24 LL | impl<'a> Trait<'a> for MyStruct<'_> {
25    |      --                         -- lifetime `'1` appears in the `impl`'s self type
26    |      |
-    |      lifetime `'a` defined here
+    |      lifetime `'1` defined here
28 LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {
29 LL |         Self { field }
30    |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
33   --> $DIR/issue-98170.rs:19:16
34    |
34    |
35 LL | impl<'a> Trait<'a> for MyStruct<'_> {
-    |      --                         -- lifetime `'1` appears in the `impl`'s self type
+    |      --                         -- lifetime `'a` appears in the `impl`'s self type
37    |      |
38    |      lifetime `'a` defined here
39 LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98170/issue-98170.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-98170.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-98170.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98170" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98170/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | impl MyStruct<'_> {
   |               -- lifetime `'1` appears in the `impl`'s self type
LL |     pub fn new<'a>(field: &'a [u32]) -> MyStruct<'a> {
   |                -- lifetime `'1` defined here
LL |         Self { field }
   |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-98170.rs:7:16
   |
   |
LL | impl MyStruct<'_> {
   |               -- lifetime `'a` appears in the `impl`'s self type
LL |     pub fn new<'a>(field: &'a [u32]) -> MyStruct<'a> {
   |                -- lifetime `'a` defined here
LL |         Self { field }
   |                ^^^^^ this usage requires that `'a` must outlive `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-98170.rs:19:9
   |
   |
LL | impl<'a> Trait<'a> for MyStruct<'_> {
   |      --                         -- lifetime `'1` appears in the `impl`'s self type
   |      |
   |      lifetime `'1` defined here
LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {
LL |         Self { field }
   |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-98170.rs:19:16
   |
   |
LL | impl<'a> Trait<'a> for MyStruct<'_> {
   |      --                         -- lifetime `'a` appears in the `impl`'s self type
   |      |
   |      lifetime `'a` defined here
LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {
LL |         Self { field }
   |                ^^^^^ this usage requires that `'a` must outlive `'1`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/nll/issue-98589-closures-relate-named-regions.rs stdout ----
diff of stderr:

4 LL | fn test_early_early<'a: 'a, 'b: 'b>() {
5    |                     --      -- lifetime `'b` defined here
6    |                     |
-    |                     lifetime `'a` defined here
+    |                     lifetime `'b` defined here
8 LL |     || { None::<&'a &'b ()>; };
9    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`


16 LL | fn test_early_late<'a: 'a, 'b>() {
17    |                    --      -- lifetime `'b` defined here
18    |                    |
-    |                    lifetime `'a` defined here
+    |                    lifetime `'b` defined here
20 LL |     || { None::<&'a &'b ()>; };
21    |          ^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`


28 LL | fn test_late_late<'a, 'b>() {
29    |                   --  -- lifetime `'b` defined here
30    |                   |
-    |                   lifetime `'a` defined here
+    |                   lifetime `'b` defined here
32 LL |     || { None::<&'a &'b ()>; };
33    |          ^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`


---
diff of stderr:

2   --> $DIR/outlives-suggestion-more.rs:5:5
3    |
4 LL | fn foo1<'a, 'b, 'c, 'd>(x: &'a usize, y: &'b usize) -> (&'c usize, &'d usize) {
-    |         --      -- lifetime `'c` defined here
+    |         --      -- lifetime `'a` defined here
6    |         |
7    |         lifetime `'a` defined here
8 LL |     (x, y)
14   --> $DIR/outlives-suggestion-more.rs:5:5
15    |
15    |
16 LL | fn foo1<'a, 'b, 'c, 'd>(x: &'a usize, y: &'b usize) -> (&'c usize, &'d usize) {
-    |             --      -- lifetime `'d` defined here
+    |             --      -- lifetime `'b` defined here
18    |             |
19    |             lifetime `'b` defined here
20 LL |     (x, y)
31   --> $DIR/outlives-suggestion-more.rs:11:5
32    |
32    |
33 LL | fn foo2<'a, 'b, 'c>(x: &'a usize, y: &'b usize) -> (&'c usize, &'static usize) {
-    |         --      -- lifetime `'c` defined here
+    |         --      -- lifetime `'a` defined here
35    |         |
36    |         lifetime `'a` defined here
37 LL |     (x, y)
56   --> $DIR/outlives-suggestion-more.rs:21:5
57    |
57    |
58 LL | fn foo3<'a, 'b, 'c, 'd, 'e>(
-    |         --  -- lifetime `'b` defined here
+    |         --  -- lifetime `'a` defined here
60    |         |
61    |         lifetime `'a` defined here


71 LL | fn foo3<'a, 'b, 'c, 'd, 'e>(
72    |         --  -- lifetime `'b` defined here
73    |         |
-    |         lifetime `'a` defined here
+    |         lifetime `'b` defined here
76 LL |     (x, y, z)
76 LL |     (x, y, z)
77    |     ^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-more/outlives-suggestion-more.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/outlives-suggestion-more.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/outlives-suggestion-more.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-more" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-more/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo1<'a, 'b, 'c, 'd>(x: &'a usize, y: &'b usize) -> (&'c usize, &'d usize) {
   |         --      -- lifetime `'a` defined here
   |         |
   |         lifetime `'a` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'c` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'c`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:5:5
   |
   |
LL | fn foo1<'a, 'b, 'c, 'd>(x: &'a usize, y: &'b usize) -> (&'c usize, &'d usize) {
   |             --      -- lifetime `'b` defined here
   |             |
   |             lifetime `'b` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'d` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'd`
help: the following changes may resolve your lifetime errors
   |
   |
   = help: add bound `'a: 'c`
   = help: add bound `'b: 'd`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:11:5
   |
   |
LL | fn foo2<'a, 'b, 'c>(x: &'a usize, y: &'b usize) -> (&'c usize, &'static usize) {
   |         --      -- lifetime `'a` defined here
   |         |
   |         lifetime `'a` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'c` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'c`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:11:5
   |
   |
LL | fn foo2<'a, 'b, 'c>(x: &'a usize, y: &'b usize) -> (&'c usize, &'static usize) {
   |             -- lifetime `'b` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ returning this value requires that `'b` must outlive `'static`
help: the following changes may resolve your lifetime errors
   |
   |
   = help: add bound `'a: 'c`
   = help: replace `'b` with `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:21:5
   |
   |
LL | fn foo3<'a, 'b, 'c, 'd, 'e>(
   |         --  -- lifetime `'a` defined here
   |         |
   |         lifetime `'a` defined here
...
LL |     (x, y, z) //~ERROR lifetime may not live long enough
   |     ^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:21:5
   |
   |
LL | fn foo3<'a, 'b, 'c, 'd, 'e>(
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'b` defined here
...
LL |     (x, y, z) //~ERROR lifetime may not live long enough
   |     ^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:21:5
   |
   |
LL | fn foo3<'a, 'b, 'c, 'd, 'e>(
   |                 -- lifetime `'c` defined here
...
LL |     (x, y, z) //~ERROR lifetime may not live long enough
   |     ^^^^^^^^^ returning this value requires that `'c` must outlive `'static`
help: the following changes may resolve your lifetime errors
   |
   |
   = help: `'a` and `'b` must be the same: replace one with the other
   = help: replace `'c` with `'static`
error: aborting due to 7 previous errors
------------------------------------------



---- [ui] src/test/ui/nll/polonius/subset-relations.rs stdout ----
diff of stderr:

4 LL | fn missing_subset<'a, 'b>(x: &'a u32, y: &'b u32) -> &'a u32 {
5    |                   --  -- lifetime `'b` defined here
6    |                   |
-    |                   lifetime `'a` defined here
+    |                   lifetime `'b` defined here
8 LL |     y
9    |     ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/subset-relations/subset-relations.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/subset-relations/subset-relations.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/polonius/subset-relations.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/polonius/subset-relations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/subset-relations" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "polonius" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/subset-relations/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn missing_subset<'a, 'b>(x: &'a u32, y: &'b u32) -> &'a u32 {
   |                   --  -- lifetime `'b` defined here
   |                   |
   |                   lifetime `'b` defined here
LL |     y //~ ERROR
   |     ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/outlives-suggestion-simple.rs stdout ----
diff of stderr:

2   --> $DIR/outlives-suggestion-simple.rs:4:5
3    |
4 LL | fn foo1<'a, 'b>(x: &'a usize) -> &'b usize {
-    |         --  -- lifetime `'b` defined here
+    |         --  -- lifetime `'a` defined here
6    |         |
7    |         lifetime `'a` defined here

22   --> $DIR/outlives-suggestion-simple.rs:12:5
23    |
23    |
24 LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
-    |         --  -- lifetime `'b` defined here
+    |         --  -- lifetime `'a` defined here
26    |         |
27    |         lifetime `'a` defined here
28 LL |     (x, y)

36 LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
37    |         --  -- lifetime `'b` defined here
38    |         |
-    |         lifetime `'a` defined here
+    |         lifetime `'b` defined here
40 LL |     (x, y)
41    |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`

48   --> $DIR/outlives-suggestion-simple.rs:20:5
49    |
49    |
50 LL | fn foo4<'a, 'b, 'c>(x: &'a usize) -> (&'b usize, &'c usize) {
-    |         --  -- lifetime `'b` defined here
+    |         --  -- lifetime `'a` defined here
52    |         |
53    |         lifetime `'a` defined here


71 LL | impl<'a> Bar<'a> {
72    |      -- lifetime `'a` defined here
73 LL |     pub fn get<'b>(&self) -> &'b usize {
-    |                -- lifetime `'b` defined here
+    |                -- lifetime `'a` defined here
75 LL |         self.x
76    |         ^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`

81   --> $DIR/outlives-suggestion-simple.rs:50:9
82    |
82    |
83 LL | impl<'a> Baz<'a> {
-    |      -- lifetime `'a` defined here
+    |      -- lifetime `'b` defined here
85 LL |     fn get<'b>(&'b self) -> &'a i32 {
86    |            -- lifetime `'b` defined here
87 LL |         self.x
93   --> $DIR/outlives-suggestion-simple.rs:71:9
94    |
94    |
95 LL | impl<'a> Foo2<'a> {
-    |      -- lifetime `'a` defined here
+    |      -- lifetime `'1` defined here
97 LL |     // should not produce outlives suggestions to name 'self
98 LL |     fn get_bar(&self) -> Bar2 {
99    |                - let's call the lifetime of this reference `'1`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/outlives-suggestion-simple.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/outlives-suggestion-simple.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/outlives-suggestion-simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo1<'a, 'b>(x: &'a usize) -> &'b usize {
   |         --  -- lifetime `'a` defined here
   |         |
   |         lifetime `'a` defined here
LL |     x //~ERROR lifetime may not live long enough
   |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:8:5
   |
   |
LL | fn foo2<'a>(x: &'a usize) -> &'static usize {
   |         -- lifetime `'a` defined here
LL |     x //~ERROR lifetime may not live long enough
   |     ^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:12:5
   |
   |
LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
   |         --  -- lifetime `'a` defined here
   |         |
   |         lifetime `'a` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:12:5
   |
   |
LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'b` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`

help: `'a` and `'b` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:20:5
   |
   |
LL | fn foo4<'a, 'b, 'c>(x: &'a usize) -> (&'b usize, &'c usize) {
   |         --  -- lifetime `'a` defined here
   |         |
   |         lifetime `'a` defined here
...
LL |     (x, x) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:29:9
   |
   |
LL |     pub fn foo<'a>(x: &'a usize) -> Self {
   |                -- lifetime `'a` defined here
LL |         Foo { x } //~ERROR lifetime may not live long enough
   |         ^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:39:9
   |
   |
LL | impl<'a> Bar<'a> {
   |      -- lifetime `'a` defined here
LL |     pub fn get<'b>(&self) -> &'b usize {
   |                -- lifetime `'a` defined here
LL |         self.x //~ERROR lifetime may not live long enough
   |         ^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:50:9
   |
   |
LL | impl<'a> Baz<'a> {
   |      -- lifetime `'b` defined here
LL |     fn get<'b>(&'b self) -> &'a i32 {
   |            -- lifetime `'b` defined here
LL |         self.x //~ERROR lifetime may not live long enough
   |         ^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:71:9
   |
   |
LL | impl<'a> Foo2<'a> {
   |      -- lifetime `'1` defined here
LL |     // should not produce outlives suggestions to name 'self
LL |     fn get_bar(&self) -> Bar2 {
   |                - let's call the lifetime of this reference `'1`
LL |         Bar2::new(&self) //~ERROR lifetime may not live long enough
   |         ^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'a`
error: aborting due to 9 previous errors
------------------------------------------



---- [ui] src/test/ui/nll/ty-outlives/projection-one-region-closure.rs stdout ----
diff of stderr:

45 LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
46    |                          --  -- lifetime `'b` defined here
47    |                          |
-    |                          lifetime `'a` defined here
+    |                          lifetime `'b` defined here
49 ...
50 LL |     with_signature(cell, t, |cell, t| require(cell, t));
51    |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`

98 LL | fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
99    |                           --  -- lifetime `'b` defined here
100    |                           |
-    |                           lifetime `'a` defined here
+    |                           lifetime `'b` defined here
102 ...
103 LL |     with_signature(cell, t, |cell, t| require(cell, t));
104    |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/projection-one-region-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/projection-one-region-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:45:29
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: no_relationships_late::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#2r ()>, T)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#3r
   = note: number of external vids: 4
   = note: where T: '_#2r
   = note: where '_#1r: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:41:1
   |
   |
LL | / fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | | {
...  |
LL | |     //~| ERROR
LL | | }
   |
   |
   = note: defining type: no_relationships_late::<'_#1r, T>
error[E0309]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:45:29
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     T: Anything<'b> + 'a,

error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:45:39
   |
   |
LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
   |                          --  -- lifetime `'b` defined here
   |                          |
   |                          lifetime `'b` defined here
...
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:56:29
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: no_relationships_early::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
               (),
           ]
   = note: number of external vids: 4
   = note: where T: '_#3r
   = note: where '_#2r: '_#3r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:51:1
   |
   |
LL | / fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | |     'a: 'a,
...  |
LL | |     //~| ERROR
LL | | }
   |
   |
   = note: defining type: no_relationships_early::<'_#1r, '_#2r, T>
error[E0309]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:56:29
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
