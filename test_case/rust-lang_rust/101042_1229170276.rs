plain

running 13439 tests
........................................................................................ 88/13439
..........................................................................iiiiiiiiiiiiii 176/13439
.....................i.................i.....................F.......................... 264/13439
...................................................F.................................... 352/13439
........................................................F....................F.......... 440/13439
..............F......F..F............................................................... 528/13439
.............................................................FF......................... 704/13439
.............................................................FF......................... 704/13439
................................F...................F.......F..i........................ 792/13439
........................................................................................ 968/13439
........................................................................................ 968/13439
.................................................F.................F............F....... 1056/13439
FF.....................FF..F....F.F.FFF.F............................................... 1144/13439
...............F....FF.......F.....F........F..F..F..F.F.....FF.F.F....F.....F....F..... 1232/13439
.......FFF.........F...F...........F...............i.............F.................FF.FF 1320/13439
F..F.............F......................................F............................... 1408/13439
........................................................................F............... 1584/13439
......................................................F................................. 1672/13439
.........................................................F.i......ii.................... 1760/13439
........................................................................................ 1848/13439
---
......................................i................................................. 6600/13439
........................................................................................ 6688/13439
...............i.......................................................ii.ii........i... 6776/13439
.i...............................................................i...................... 6864/13439
....................................................................F.........F......... 6952/13439
......FF.F......FFFFFFFFFFFF.F...FFFFiF...i.........................................i... 7040/13439
........................................................................................ 7216/13439
..................i..................................................................... 7304/13439
........................................................................................ 7392/13439
........................................................................................ 7392/13439
.........................F.................F............................................ 7480/13439
..................................i..................................................... 7656/13439
........................................................................................ 7744/13439
........................................................................................ 7744/13439
...........................................ii..................................F..F..... 7832/13439
F....................................................................................... 8008/13439
........................................................ii................i....i..ii.... 8096/13439
........................................................ii................i....i..ii.... 8096/13439
...............F...........F.F.......................................................... 8184/13439
................................................................F.........FF...F...F.F.F 8272/13439
...F...................................F.................F............FFF....F...F...... 8360/13439
............................F....F...FF.......F...........F......F.F..F.F...F........... 8448/13439
........................F......FF........FF....................F........................ 8536/13439
.FF......F..i..ii..............................................................ii....... 8624/13439
....................................F.....F..........................................iii 8712/13439
.......................................i........................................i....... 8888/13439
............................................................i........................... 8976/13439
........................................................................................ 9064/13439
........................................................i............................... 9152/13439
---
........................................................................................ 9856/13439
............................................................................ii.......... 9944/13439
.....i.................................................................................. 10032/13439
........................................................................................ 10120/13439
....F......................F..F.F.........F......F.F............F...FFFFF........FF..... 10208/13439
.F...F..F...................FFF..F....................................F..FF.......FF.F.F 10296/13439
................F.FF.F.F................................................................ 10384/13439
........................................................................................ 10560/13439
........................................................................................ 10648/13439
......F........iiiii...i....i..i........................................................ 10736/13439
........................................................................i............... 10824/13439
........................................................................i............... 10824/13439
..................................................................................iiiiii 10912/13439
.i..iiiiii.i.............................FF....................F.F.............FFF.FF.F. 11000/13439
FF...................................................................................... 11088/13439
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
.........F.................F............................................................ 13024/13439
........................................................................................ 13200/13439
........................................................................................ 13288/13439
........................................................................................ 13288/13439
..................FFFFFF.F.F.F..F...........iii..FF...F................................. 13376/13439
.......F.......................................................

---- [ui] src/test/ui/asm/x86_64/type-check-5.rs stdout ----


error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/type-check-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-5" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-5/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f41dcacccfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   0:     0x7f41dcacccfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7f41dcb35748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7f41dcabd8d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7f41dcacfcfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7f41dcacf9c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7f41dd4562e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f41dcad04b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7f41dcad02d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7f41dcacd274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7f41dcacffa2 - rust_begin_unwind
  10:     0x7f41dca80e43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7f41dcb31dd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7f41dcb31d7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7f41dca80cb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7f41e01534d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7f41e0150812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f41e0148d3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f41e0146386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7f41e015eb22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f41e015d53c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7f41e01a32e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7f41e01a1be1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7f41de73d253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7f41de5038df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7f41de732407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7f41de6fd61e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7f41def9b749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7f41df06e183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7f41deecea04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f41dd5f15c6 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  30:     0x7f41dd572749 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f41dd55589d - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f41dd5e5855 - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}>
  33:     0x7f41dd5ac84b - rustc_interface[f2bb50d053b0635d]::passes::analysis
  34:     0x7f41defdd754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  35:     0x7f41df0c1564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  36:     0x7f41deeafd2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  37:     0x7f41dd4c1efd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  38:     0x7f41dd45e476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  39:     0x7f41dd440ba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f41dd479b41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  41:     0x7f41dd442be2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  42:     0x7f41dd4b8849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  43:     0x7f41dd44524e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7f41dd4bce90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f41dcadcd25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  46:     0x7f41dc87ab43 - <unknown>
  47:     0x7f41dc90ca00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

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
   0:     0x7efdb9c0dcfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7efdb9c76748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7efdb9bfe8d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   2:     0x7efdb9bfe8d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7efdb9c10cfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7efdb9c109c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7efdba5972e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7efdb9c114b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7efdb9c112d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7efdb9c0e274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7efdb9c10fa2 - rust_begin_unwind
  10:     0x7efdb9bc1e43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7efdb9c72dd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7efdb9c72d7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7efdb9bc1cb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7efdbd2944d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7efdbd291812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7efdbd289d3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7efdbd287386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7efdbd29fb22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7efdbd29e53c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7efdbd2e42e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7efdbd2e2be1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7efdbb87e253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7efdbb6448df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7efdbb873407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7efdbb83e61e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7efdbc0dc749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7efdbc1af183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7efdbc00fa04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7efdbd07acc5 - <rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt>::mir_borrowck_opt_const_arg
  30:     0x7efdbabc431d - rustc_mir_transform[1424d1e862e7ea41]::mir_drops_elaborated_and_const_checked
  31:     0x7efdbc0ca1a6 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_middle[d6b4fe199899b983]::ty::WithOptConstParam<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId>, &rustc_data_structures[233d53fdabe388f6]::steal::Steal<rustc_middle[d6b4fe199899b983]::mir::Body>>>
  32:     0x7efdbc1fd175 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  33:     0x7efdbbff7ee7 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  34:     0x7efdbabc3f17 - rustc_mir_transform[1424d1e862e7ea41]::inner_mir_for_ctfe
  35:     0x7efdbabc39b6 - rustc_mir_transform[1424d1e862e7ea41]::mir_for_ctfe
  36:     0x7efdbc103e19 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::DefId, &rustc_middle[d6b4fe199899b983]::mir::Body>>
  37:     0x7efdbc1af2e9 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_for_ctfe, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  38:     0x7efdbbff8429 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_for_ctfe
  39:     0x7efdbbb70b95 - <rustc_const_eval[a31a5dc1175addd2]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[a31a5dc1175addd2]::interpret::machine::Machine>::load_mir
  40:     0x7efdbbb25976 - <rustc_const_eval[a31a5dc1175addd2]::interpret::eval_context::InterpCx<rustc_const_eval[a31a5dc1175addd2]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  41:     0x7efdbbc5ea3c - rustc_const_eval[a31a5dc1175addd2]::const_eval::eval_queries::eval_to_allocation_raw_provider
  42:     0x7efdbc1d6bda - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::eval_to_allocation_raw, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  43:     0x7efdbc011fe1 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::eval_to_allocation_raw
  44:     0x7efdbbc6fd81 - rustc_const_eval[a31a5dc1175addd2]::const_eval::eval_to_valtree
  45:     0x7efdbbb8cc00 - <rustc_const_eval[a31a5dc1175addd2]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_middle[d6b4fe199899b983]::ty::ParamEnvAnd<rustc_middle[d6b4fe199899b983]::mir::interpret::GlobalId>)>>::call_once
  46:     0x7efdbc1b67c7 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::eval_to_valtree, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  47:     0x7efdbc012b81 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::eval_to_valtree
  48:     0x7efdbd092626 - <rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  49:     0x7efdbd090ffa - <rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  50:     0x7efdbcdbeefe - <rustc_infer[51989bb73817884b]::infer::InferCtxt>::const_eval_resolve
  51:     0x7efdbcb8be7a - rustc_trait_selection[43eb4e694f0b915c]::traits::const_evaluatable::is_const_evaluatable
  52:     0x7efdbccd1150 - <rustc_trait_selection[43eb4e694f0b915c]::traits::fulfill::FulfillProcessor as rustc_data_structures[233d53fdabe388f6]::obligation_forest::ObligationProcessor>::process_obligation
  53:     0x7efdbcb92fde - <rustc_data_structures[233d53fdabe388f6]::obligation_forest::ObligationForest<rustc_trait_selection[43eb4e694f0b915c]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[43eb4e694f0b915c]::traits::fulfill::FulfillProcessor, rustc_data_structures[233d53fdabe388f6]::obligation_forest::Outcome<rustc_trait_selection[43eb4e694f0b915c]::traits::fulfill::PendingPredicateObligation, rustc_infer[51989bb73817884b]::traits::FulfillmentErrorCode>>
  54:     0x7efdbcccb51a - <rustc_trait_selection[43eb4e694f0b915c]::traits::fulfill::FulfillmentContext as rustc_infer[51989bb73817884b]::traits::engine::TraitEngine>::select_where_possible
  55:     0x7efdbcccb3f8 - <rustc_trait_selection[43eb4e694f0b915c]::traits::fulfill::FulfillmentContext as rustc_infer[51989bb73817884b]::traits::engine::TraitEngine>::select_all_or_error
  56:     0x7efdbccfb44b - <rustc_trait_selection[43eb4e694f0b915c]::traits::engine::ObligationCtxt>::select_all_or_error
  57:     0x7efdbb0d0533 - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[bd51273264e26bf8]::check::wfcheck::enter_wf_checking_ctxt<rustc_typeck[bd51273264e26bf8]::check::wfcheck::check_type_defn<rustc_typeck[bd51273264e26bf8]::check::wfcheck::check_item::{closure#3}>::{closure#0}>::{closure#0}>
  58:     0x7efdbb191935 - rustc_typeck[bd51273264e26bf8]::check::wfcheck::check_well_formed
  59:     0x7efdbc0df2d9 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, ()>>
  60:     0x7efdbc1c0b56 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::check_well_formed, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  61:     0x7efdbc02b0a4 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::check_well_formed
  62:     0x7efdbafe69a5 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_hir[1a47a771f165fe0a]::hir::ItemId], <rustc_middle[d6b4fe199899b983]::hir::ModuleItems>::par_items<rustc_typeck[bd51273264e26bf8]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  63:     0x7efdbaf7b519 - std[690769e8f4744118]::panicking::try::<(), core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_hir[1a47a771f165fe0a]::hir::ItemId], <rustc_middle[d6b4fe199899b983]::hir::ModuleItems>::par_items<rustc_typeck[bd51273264e26bf8]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  64:     0x7efdbaf113cd - <rustc_middle[d6b4fe199899b983]::hir::ModuleItems>::par_items::<rustc_typeck[bd51273264e26bf8]::check::wfcheck::check_mod_type_wf::{closure#0}>
  65:     0x7efdbb19c329 - rustc_typeck[bd51273264e26bf8]::check::wfcheck::check_mod_type_wf
  66:     0x7efdbc0df2d9 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, ()>>
  67:     0x7efdbc1c0a26 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::check_mod_type_wf, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  68:     0x7efdbc00c454 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::check_mod_type_wf
  69:     0x7efdbafe6cc5 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_for_each_module<rustc_typeck[bd51273264e26bf8]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  70:     0x7efdbaf7b579 - std[690769e8f4744118]::panicking::try::<(), core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_for_each_module<rustc_typeck[bd51273264e26bf8]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  71:     0x7efdbaf140ad - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_for_each_module<rustc_typeck[bd51273264e26bf8]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  72:     0x7efdbb0026dd - <rustc_session[cb132996c5d5d750]::session::Session>::track_errors::<rustc_typeck[bd51273264e26bf8]::check_crate::{closure#5}, ()>
  73:     0x7efdbb232a10 - rustc_typeck[bd51273264e26bf8]::check_crate
  74:     0x7efdba6ed811 - rustc_interface[f2bb50d053b0635d]::passes::analysis
  75:     0x7efdbc11e754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  76:     0x7efdbc202564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  77:     0x7efdbbff0d2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  78:     0x7efdba602efd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  79:     0x7efdba59f476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  80:     0x7efdba581ba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  81:     0x7efdba5bab41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  82:     0x7efdba583be2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  83:     0x7efdba5f9849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  84:     0x7efdba58624e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  85:     0x7efdba5fde90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  86:     0x7efdb9c1dd25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  87:     0x7efdb99bbb43 - <unknown>
  88:     0x7efdb9a4da00 - <unknown>
  89:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

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
   0:     0x7fe8b469ecfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   0:     0x7fe8b469ecfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7fe8b4707748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7fe8b468f8d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7fe8b46a1cfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7fe8b46a19c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7fe8b50282e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe8b46a24b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7fe8b46a22d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7fe8b469f274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7fe8b46a1fa2 - rust_begin_unwind
  10:     0x7fe8b4652e43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7fe8b4703dd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7fe8b4703d7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7fe8b4652cb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7fe8b7d254d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7fe8b7d22812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fe8b7d1ad3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fe8b7d18386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7fe8b7d30b22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fe8b7d2f53c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7fe8b7d752e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7fe8b7d73be1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7fe8b630f253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7fe8b60d58df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7fe8b6304407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7fe8b62cf61e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7fe8b6b6d749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7fe8b6c40183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7fe8b6aa0a04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fe8b7b0bcc5 - <rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt>::mir_borrowck_opt_const_arg
  30:     0x7fe8b565531d - rustc_mir_transform[1424d1e862e7ea41]::mir_drops_elaborated_and_const_checked
  31:     0x7fe8b6b5b1a6 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_middle[d6b4fe199899b983]::ty::WithOptConstParam<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId>, &rustc_data_structures[233d53fdabe388f6]::steal::Steal<rustc_middle[d6b4fe199899b983]::mir::Body>>>
  32:     0x7fe8b6c8e175 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  33:     0x7fe8b6a88ee7 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  34:     0x7fe8b5654f17 - rustc_mir_transform[1424d1e862e7ea41]::inner_mir_for_ctfe
  35:     0x7fe8b56549b6 - rustc_mir_transform[1424d1e862e7ea41]::mir_for_ctfe
  36:     0x7fe8b6b94e19 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::DefId, &rustc_middle[d6b4fe199899b983]::mir::Body>>
  37:     0x7fe8b6c402e9 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_for_ctfe, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  38:     0x7fe8b6a89429 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_for_ctfe
  39:     0x7fe8b6601b95 - <rustc_const_eval[a31a5dc1175addd2]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[a31a5dc1175addd2]::interpret::machine::Machine>::load_mir
  40:     0x7fe8b65b6976 - <rustc_const_eval[a31a5dc1175addd2]::interpret::eval_context::InterpCx<rustc_const_eval[a31a5dc1175addd2]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  41:     0x7fe8b66efa3c - rustc_const_eval[a31a5dc1175addd2]::const_eval::eval_queries::eval_to_allocation_raw_provider
  42:     0x7fe8b6c67bda - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::eval_to_allocation_raw, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  43:     0x7fe8b6aa2fe1 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::eval_to_allocation_raw
  44:     0x7fe8b66ede0b - rustc_const_eval[a31a5dc1175addd2]::const_eval::eval_queries::eval_to_const_value_raw_provider
  45:     0x7fe8b6c6f38a - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::eval_to_const_value_raw, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  46:     0x7fe8b6aa35b1 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::eval_to_const_value_raw
  47:     0x7fe8b66eda12 - rustc_const_eval[a31a5dc1175addd2]::const_eval::eval_queries::eval_to_const_value_raw_provider
  48:     0x7fe8b6c6f38a - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::eval_to_const_value_raw, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  49:     0x7fe8b6aa35b1 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::eval_to_const_value_raw
  50:     0x7fe8b7b229a4 - <rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt>::const_eval_global_id
  51:     0x7fe8b7afa94e - <rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt>::const_eval_instance
  52:     0x7fe8b5ff7bd7 - <rustc_mir_build[bc6ce0f8a68f1d3d]::thir::pattern::PatCtxt>::lower_path
  53:     0x7fe8b5ff30ca - <rustc_mir_build[bc6ce0f8a68f1d3d]::thir::pattern::PatCtxt>::lower_pattern
  54:     0x7fe8b608cde4 - <rustc_mir_build[bc6ce0f8a68f1d3d]::thir::pattern::check_match::MatchVisitor>::lower_pattern
  55:     0x7fe8b608d935 - <rustc_mir_build[bc6ce0f8a68f1d3d]::thir::pattern::check_match::MatchVisitor>::check_irrefutable
  56:     0x7fe8b608cd4d - <rustc_mir_build[bc6ce0f8a68f1d3d]::thir::pattern::check_match::MatchVisitor as rustc_hir[1a47a771f165fe0a]::intravisit::Visitor>::visit_local
  57:     0x7fe8b606c1c5 - rustc_hir[1a47a771f165fe0a]::intravisit::walk_expr::<rustc_mir_build[bc6ce0f8a68f1d3d]::thir::pattern::check_match::MatchVisitor>
  58:     0x7fe8b608aa0c - <rustc_mir_build[bc6ce0f8a68f1d3d]::thir::pattern::check_match::MatchVisitor as rustc_hir[1a47a771f165fe0a]::intravisit::Visitor>::visit_expr
  59:     0x7fe8b608a7bf - rustc_mir_build[bc6ce0f8a68f1d3d]::thir::pattern::check_match::check_match
  60:     0x7fe8b6b9e50c - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::DefId, ()>>
  61:     0x7fe8b6c3a7da - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::check_match, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  62:     0x7fe8b6aa6979 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::check_match
  63:     0x7fe8b51c3285 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  64:     0x7fe8b51446e9 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  65:     0x7fe8b51275dd - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  66:     0x7fe8b51c574b - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#1}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  67:     0x7fe8b51449b9 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#1}::{closure#0}>, ()>
  68:     0x7fe8b51b875e - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#1}>
  69:     0x7fe8b517e834 - rustc_interface[f2bb50d053b0635d]::passes::analysis
  70:     0x7fe8b6baf754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  71:     0x7fe8b6c93564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  72:     0x7fe8b6a81d2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  73:     0x7fe8b5093efd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  74:     0x7fe8b5030476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  75:     0x7fe8b5012ba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  76:     0x7fe8b504bb41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  77:     0x7fe8b5014be2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  78:     0x7fe8b508a849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  79:     0x7fe8b501724e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  80:     0x7fe8b508ee90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  81:     0x7fe8b46aed25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  82:     0x7fe8b444cb43 - <unknown>
  83:     0x7fe8b44dea00 - <unknown>
  84:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

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
   0:     0x7fbcd126ecfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   0:     0x7fbcd126ecfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7fbcd12d7748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7fbcd125f8d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7fbcd1271cfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7fbcd12719c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7fbcd1bf82e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fbcd12724b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7fbcd12722d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7fbcd126f274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7fbcd1271fa2 - rust_begin_unwind
  10:     0x7fbcd1222e43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7fbcd12d3dd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7fbcd12d3d7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7fbcd1222cb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7fbcd48f54d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7fbcd48f2812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fbcd48ead3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fbcd48e8386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7fbcd4900b22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fbcd48ff53c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7fbcd49452e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7fbcd4943be1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7fbcd2edf253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7fbcd2ca58df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7fbcd2ed4407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7fbcd2e9f61e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7fbcd373d749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7fbcd3810183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7fbcd3670a04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fbcd1d935c6 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  30:     0x7fbcd1d14749 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7fbcd1cf789d - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7fbcd1d87855 - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}>
  33:     0x7fbcd1d4e84b - rustc_interface[f2bb50d053b0635d]::passes::analysis
  34:     0x7fbcd377f754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  35:     0x7fbcd3863564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  36:     0x7fbcd3651d2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  37:     0x7fbcd1c63efd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  38:     0x7fbcd1c00476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  39:     0x7fbcd1be2ba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7fbcd1c1bb41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  41:     0x7fbcd1be4be2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  42:     0x7fbcd1c5a849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  43:     0x7fbcd1be724e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7fbcd1c5ee90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7fbcd127ed25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  46:     0x7fbcd101cb43 - <unknown>
  47:     0x7fbcd10aea00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

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
   0:     0x7ff686e57cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7ff686ec0748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7ff686e488d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   2:     0x7ff686e488d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7ff686e5acfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7ff686e5a9c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7ff6877e12e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff686e5b4b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7ff686e5b2d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7ff686e58274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7ff686e5afa2 - rust_begin_unwind
  10:     0x7ff686e0be43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7ff686ebcdd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7ff686ebcd7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7ff686e0bcb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7ff68a4de4d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7ff68a4db812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7ff68a4d3d3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7ff68a4d1386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7ff68a4e9b22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7ff68a4e853c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7ff68a52e2e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7ff68a52cbe1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7ff688ac8253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7ff68888e8df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7ff688abd407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7ff688a8861e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7ff689326749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7ff6893f9183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7ff689259a04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7ff68797c5c6 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  30:     0x7ff6878fd749 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7ff6878e089d - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7ff687970855 - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}>
  33:     0x7ff68793784b - rustc_interface[f2bb50d053b0635d]::passes::analysis
  34:     0x7ff689368754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  35:     0x7ff68944c564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  36:     0x7ff68923ad2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  37:     0x7ff68784cefd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  38:     0x7ff6877e9476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  39:     0x7ff6877cbba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7ff687804b41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  41:     0x7ff6877cdbe2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  42:     0x7ff687843849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  43:     0x7ff6877d024e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7ff687847e90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7ff686e67d25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  46:     0x7ff686c05b43 - <unknown>
  47:     0x7ff686c97a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `uninit_1`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7ff686e57cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7ff686ec0748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7ff686e488d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   2:     0x7ff686e488d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7ff686e5acfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7ff686e5a9c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7ff6877e12e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff686e5b4b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7ff686e5b2d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7ff686e58274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7ff686e5afa2 - rust_begin_unwind
  10:     0x7ff686e0be43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7ff686ebcdd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7ff686ebcd7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7ff686e0bcb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7ff68a4de4d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7ff68a4db812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7ff68a4d3d3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7ff68a4d1386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7ff68a4e9b22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7ff68a4e853c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7ff68a52e2e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7ff68a52cbe1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7ff688ac8253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7ff68888e8df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7ff688abd407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7ff688a8861e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7ff689326749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7ff6893f9183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7ff689259a04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7ff68797c5c6 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  30:     0x7ff6878fd749 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7ff6878e089d - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7ff687970855 - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}>
  33:     0x7ff68793784b - rustc_interface[f2bb50d053b0635d]::passes::analysis
  34:     0x7ff689368754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  35:     0x7ff68944c564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  36:     0x7ff68923ad2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  37:     0x7ff68784cefd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  38:     0x7ff6877e9476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  39:     0x7ff6877cbba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7ff687804b41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  41:     0x7ff6877cdbe2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  42:     0x7ff687843849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  43:     0x7ff6877d024e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7ff687847e90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7ff686e67d25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  46:     0x7ff686c05b43 - <unknown>
  47:     0x7ff686c97a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

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
   0:     0x7f310bb05cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   0:     0x7f310bb05cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7f310bb6e748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7f310baf68d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7f310bb08cfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7f310bb089c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7f310c48f2e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f310bb094b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7f310bb092d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7f310bb06274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7f310bb08fa2 - rust_begin_unwind
  10:     0x7f310bab9e43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7f310bb6add1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7f310bb6ad7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7f310bab9cb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7f310f18c4d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7f310f189812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f310f181d3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f310f17f386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7f310f197b22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f310f19653c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7f310f1dc2e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7f310f1dabe1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7f310d776253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7f310d53c8df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7f310d76b407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7f310d73661e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7f310dfd4749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7f310e0a7183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7f310df07a04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f310c62a5c6 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  30:     0x7f310c5ab749 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f310c58e89d - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f310c61e855 - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}>
  33:     0x7f310c5e584b - rustc_interface[f2bb50d053b0635d]::passes::analysis
  34:     0x7f310e016754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  35:     0x7f310e0fa564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  36:     0x7f310dee8d2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  37:     0x7f310c4faefd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  38:     0x7f310c497476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  39:     0x7f310c479ba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f310c4b2b41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  41:     0x7f310c47bbe2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  42:     0x7f310c4f1849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  43:     0x7f310c47e24e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7f310c4f5e90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f310bb15d25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  46:     0x7f310b8b3b43 - <unknown>
  47:     0x7f310b945a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

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
   0:     0x7f1488e6ecfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7f1488ed7748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7f1488e5f8d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   2:     0x7f1488e5f8d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7f1488e71cfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7f1488e719c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7f14897f82e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1488e724b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7f1488e722d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7f1488e6f274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7f1488e71fa2 - rust_begin_unwind
  10:     0x7f1488e22e43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7f1488ed3dd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7f1488ed3d7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7f1488e22cb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7f148c4f54d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7f148c4f2812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f148c4ead3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f148c4e8386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7f148c500b22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f148c4ff53c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7f148c5452e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7f148c543be1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7f148aadf253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7f148a8a58df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7f148aad4407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7f148aa9f61e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7f148b33d749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7f148b410183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7f148b270a04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f14899935c6 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  30:     0x7f1489914749 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f14898f789d - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f1489987855 - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}>
  33:     0x7f148994e84b - rustc_interface[f2bb50d053b0635d]::passes::analysis
  34:     0x7f148b37f754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  35:     0x7f148b463564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  36:     0x7f148b251d2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  37:     0x7f1489863efd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  38:     0x7f1489800476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  39:     0x7f14897e2ba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f148981bb41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  41:     0x7f14897e4be2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  42:     0x7f148985a849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  43:     0x7f14897e724e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7f148985ee90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f1488e7ed25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  46:     0x7f1488c1cb43 - <unknown>
  47:     0x7f1488caea00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

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
---
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
   0:     0x7ff12cac1cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   0:     0x7ff12cac1cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7ff12cb2a748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7ff12cab28d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7ff12cac4cfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7ff12cac49c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7ff12d44b2e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff12cac54b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7ff12cac52d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7ff12cac2274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7ff12cac4fa2 - rust_begin_unwind
  10:     0x7ff12ca75e43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7ff12cb26dd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7ff12cb26d7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7ff12ca75cb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7ff1301484d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7ff130145812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7ff13013dd3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7ff13013b386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7ff130153b22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7ff13015253c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7ff1301982e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7ff130196be1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7ff12e732253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7ff12e4f88df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7ff12e727407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7ff12e6f261e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7ff12ef90749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7ff12f063183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7ff12eec3a04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7ff12d5e65c6 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  30:     0x7ff12d567749 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7ff12d54a89d - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7ff12d5da855 - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}>
  33:     0x7ff12d5a184b - rustc_interface[f2bb50d053b0635d]::passes::analysis
  34:     0x7ff12efd2754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  35:     0x7ff12f0b6564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  36:     0x7ff12eea4d2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  37:     0x7ff12d4b6efd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  38:     0x7ff12d453476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  39:     0x7ff12d435ba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7ff12d46eb41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  41:     0x7ff12d437be2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  42:     0x7ff12d4ad849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  43:     0x7ff12d43a24e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7ff12d4b1e90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7ff12cad1d25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  46:     0x7ff12c86fb43 - <unknown>
  47:     0x7ff12c901a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `cannot_partially_init_adt_with_drop`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7ff12cac1cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   0:     0x7ff12cac1cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7ff12cb2a748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7ff12cab28d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7ff12cac4cfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7ff12cac49c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7ff12d44b2e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff12cac54b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7ff12cac52d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7ff12cac2274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7ff12cac4fa2 - rust_begin_unwind
  10:     0x7ff12ca75e43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7ff12cb26dd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7ff12cb26d7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7ff12ca75cb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7ff1301484d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7ff130145812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7ff13013dd3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7ff13013b386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7ff130153b22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7ff13015253c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7ff1301982e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7ff130196be1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7ff12e732253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7ff12e4f88df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7ff12e727407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7ff12e6f261e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7ff12ef90749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7ff12f063183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7ff12eec3a04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7ff12d5e65c6 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  30:     0x7ff12d567749 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7ff12d54a89d - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7ff12d5da855 - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}>
  33:     0x7ff12d5a184b - rustc_interface[f2bb50d053b0635d]::passes::analysis
  34:     0x7ff12efd2754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  35:     0x7ff12f0b6564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  36:     0x7ff12eea4d2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  37:     0x7ff12d4b6efd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  38:     0x7ff12d453476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  39:     0x7ff12d435ba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7ff12d46eb41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  41:     0x7ff12d437be2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  42:     0x7ff12d4ad849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  43:     0x7ff12d43a24e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7ff12d4b1e90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7ff12cad1d25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  46:     0x7ff12c86fb43 - <unknown>
  47:     0x7ff12c901a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `cannot_partially_init_mutable_adt_with_drop`
#1 [analysis] running analysis passes on this crate
error[E0382]: assign of moved value: `d`
  --> /checkout/src/test/ui/nll/issue-21232-partial-init-and-erroneous-use.rs:39:5
   |
   |
LL |     let mut d = D { x: 0, s: S{ y: 0, z: 0 } };
   |         ----- move occurs because `d` has type `D`, which does not implement the `Copy` trait
LL |     drop(d);
   |          - value moved here
LL |     d.x = 10;


thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7ff12cac1cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   0:     0x7ff12cac1cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7ff12cb2a748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7ff12cab28d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7ff12cac4cfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7ff12cac49c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7ff12d44b2e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff12cac54b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7ff12cac52d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7ff12cac2274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7ff12cac4fa2 - rust_begin_unwind
  10:     0x7ff12ca75e43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7ff12cb26dd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7ff12cb26d7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7ff12ca75cb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7ff1301484d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7ff130145812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7ff13013dd3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7ff13013b386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7ff130153b22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7ff13015253c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7ff1301982e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7ff130196be1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7ff12e732253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7ff12e4f88df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7ff12e727407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7ff12e6f261e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7ff12ef90749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7ff12f063183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7ff12eec3a04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7ff12d5e65c6 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  30:     0x7ff12d567749 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7ff12d54a89d - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7ff12d5da855 - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}>
  33:     0x7ff12d5a184b - rustc_interface[f2bb50d053b0635d]::passes::analysis
  34:     0x7ff12efd2754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  35:     0x7ff12f0b6564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  36:     0x7ff12eea4d2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  37:     0x7ff12d4b6efd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  38:     0x7ff12d453476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  39:     0x7ff12d435ba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7ff12d46eb41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  41:     0x7ff12d437be2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  42:     0x7ff12d4ad849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  43:     0x7ff12d43a24e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7ff12d4b1e90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7ff12cad1d25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  46:     0x7ff12c86fb43 - <unknown>
  47:     0x7ff12c901a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `cannot_partially_init_inner_adt_via_outer_with_drop`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7ff12cac1cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   0:     0x7ff12cac1cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e9ff2e3a2e53de9
   1:     0x7ff12cb2a748 - core::fmt::write::h0d5bc1b9e7db07ad
   2:     0x7ff12cab28d1 - std::io::Write::write_fmt::h53ddf2471edc2549
   3:     0x7ff12cac4cfe - std::panicking::default_hook::{{closure}}::h1affe3551583197b
   4:     0x7ff12cac49c7 - std::panicking::default_hook::h6b8d25c66f651012
   5:     0x7ff12d44b2e4 - rustc_driver[7d698f9cfa99bdfd]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff12cac54b1 - std::panicking::rust_panic_with_hook::h2b5b4f4d45fc7d56
   7:     0x7ff12cac52d7 - std::panicking::begin_panic_handler::{{closure}}::hbff4e95c61b02db3
   8:     0x7ff12cac2274 - std::sys_common::backtrace::__rust_end_short_backtrace::hac943f3472a5904c
   9:     0x7ff12cac4fa2 - rust_begin_unwind
  10:     0x7ff12ca75e43 - core::panicking::panic_fmt::h73a242a5fecc5d83
  11:     0x7ff12cb26dd1 - core::panicking::panic_display::heca389ad9f429667
  12:     0x7ff12cb26d7b - core::panicking::panic_str::h45dec565d6032617
  13:     0x7ff12ca75cb6 - core::option::expect_failed::h785c0b8aacc9f236
  14:     0x7ff1301484d2 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::translation::Translate>::translate_message
  15:     0x7ff130145812 - <rustc_errors[ff22c7c98990f232]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7ff13013dd3a - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter>::emit_message_default
  17:     0x7ff13013b386 - <rustc_errors[ff22c7c98990f232]::emitter::EmitterWriter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  18:     0x7ff130153b22 - <rustc_errors[ff22c7c98990f232]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7ff13015253c - <rustc_errors[ff22c7c98990f232]::json::JsonEmitter as rustc_errors[ff22c7c98990f232]::emitter::Emitter>::emit_diagnostic
  20:     0x7ff1301982e8 - <rustc_errors[ff22c7c98990f232]::HandlerInner>::emit_diagnostic
  21:     0x7ff130196be1 - <rustc_errors[ff22c7c98990f232]::Handler>::emit_diagnostic
  22:     0x7ff12e732253 - rustc_borrowck[8e20c45d50ec34a]::do_mir_borrowck
  23:     0x7ff12e4f88df - <rustc_infer[51989bb73817884b]::infer::InferCtxtBuilder>::enter::<rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult, rustc_borrowck[8e20c45d50ec34a]::mir_borrowck::{closure#0}>
  24:     0x7ff12e727407 - rustc_borrowck[8e20c45d50ec34a]::mir_borrowck
  25:     0x7ff12e6f261e - <rustc_borrowck[8e20c45d50ec34a]::provide::{closure#0} as core[4a7bf389a38ff77b]::ops::function::FnOnce<(rustc_middle[d6b4fe199899b983]::ty::context::TyCtxt, rustc_span[a89f37427d6f91c3]::def_id::LocalDefId)>>::call_once
  26:     0x7ff12ef90749 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<rustc_span[a89f37427d6f91c3]::def_id::LocalDefId, &rustc_middle[d6b4fe199899b983]::mir::query::BorrowCheckResult>>
  27:     0x7ff12f063183 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::mir_borrowck, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  28:     0x7ff12eec3a04 - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7ff12d5e65c6 - <core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once
  30:     0x7ff12d567749 - std[690769e8f4744118]::panic::catch_unwind::<core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7ff12d54a89d - rustc_data_structures[233d53fdabe388f6]::sync::par_for_each_in::<&[rustc_span[a89f37427d6f91c3]::def_id::LocalDefId], <rustc_middle[d6b4fe199899b983]::hir::map::Map>::par_body_owners<rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7ff12d5da855 - <rustc_session[cb132996c5d5d750]::session::Session>::time::<(), rustc_interface[f2bb50d053b0635d]::passes::analysis::{closure#2}>
  33:     0x7ff12d5a184b - rustc_interface[f2bb50d053b0635d]::passes::analysis
  34:     0x7ff12efd2754 - rustc_query_system[82784919c3846bd0]::query::plumbing::try_execute_query::<rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt, rustc_query_system[82784919c3846bd0]::query::caches::DefaultCache<(), core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>>
  35:     0x7ff12f0b6564 - rustc_query_system[82784919c3846bd0]::query::plumbing::get_query::<rustc_query_impl[f09998aacc60c94e]::queries::analysis, rustc_query_impl[f09998aacc60c94e]::plumbing::QueryCtxt>
  36:     0x7ff12eea4d2e - <rustc_query_impl[f09998aacc60c94e]::Queries as rustc_middle[d6b4fe199899b983]::ty::query::QueryEngine>::analysis
  37:     0x7ff12d4b6efd - <rustc_interface[f2bb50d053b0635d]::passes::QueryContext>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  38:     0x7ff12d453476 - <rustc_interface[f2bb50d053b0635d]::interface::Compiler>::enter::<rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}::{closure#2}, core[4a7bf389a38ff77b]::result::Result<core[4a7bf389a38ff77b]::option::Option<rustc_interface[f2bb50d053b0635d]::queries::Linker>, rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  39:     0x7ff12d435ba5 - rustc_span[a89f37427d6f91c3]::with_source_map::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7ff12d46eb41 - rustc_interface[f2bb50d053b0635d]::interface::create_compiler_and_run::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>
  41:     0x7ff12d437be2 - <scoped_tls[bbf8c8c9759dc420]::ScopedKey<rustc_span[a89f37427d6f91c3]::SessionGlobals>>::set::<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  42:     0x7ff12d4ad849 - std[690769e8f4744118]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>
  43:     0x7ff12d43a24e - std[690769e8f4744118]::panicking::try::<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, core[4a7bf389a38ff77b]::panic::unwind_safe::AssertUnwindSafe<<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7ff12d4b1e90 - <<std[690769e8f4744118]::thread::Builder>::spawn_unchecked_<rustc_interface[f2bb50d053b0635d]::util::run_in_thread_pool_with_globals<rustc_interface[f2bb50d053b0635d]::interface::run_compiler<core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>, rustc_driver[7d698f9cfa99bdfd]::run_compiler::{closure#1}>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#0}, core[4a7bf389a38ff77b]::result::Result<(), rustc_errors[ff22c7c98990f232]::ErrorGuaranteed>>::{closure#1} as core[4a7bf389a38ff77b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7ff12cad1d25 - std::sys::unix::thread::Thread::new::thread_start::h5580cd95718bfb58
  46:     0x7ff12c86fb43 - <unknown>
  47:     0x7ff12c901a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7da4a0081 2022-08-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `cannot_partially_init_inner_adt_via_mutable_outer_with_drop`
#1 [analysis] running analysis passes on this crate
error[E0382]: assign to part of moved value: `d`
  --> /checkout/src/test/ui/nll/issue-21232-partial-init-and-erroneous-use.rs:56:5
   |
   |
LL |     let mut d = D { x: 0, s: S{ y: 0, z: 0} };
   |         ----- move occurs because `d` has type `D`, which does not implement the `Copy` trait
LL |     drop(d);
   |          - value moved here
LL |     d.s.y = 20;

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0381, E0382.
Some errors have detailed explanations: E0381, E0382.
For more information about an error, try `rustc --explain E0381`.
------------------------------------------


---- [ui] src/test/ui/nll/issue-21232-partial-init-and-use.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-21232-partial-init-and-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-21232-partial-init-and-use" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-21232-partial-init-and-use/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
---
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



---- [ui] src/test/ui/nll/issue-52113.rs stdout ----
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


