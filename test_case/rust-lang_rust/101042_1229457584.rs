plain

running 13441 tests
........................................................................................ 88/13441
..........................................................................iiiiiiii.iiiii 176/13441
i....................i.................i...................F............................ 264/13441
.................................................F...................................... 352/13441
.........................................................F....................F......... 440/13441
..............F.....F.F................................................................. 528/13441
........................................................................................ 616/13441
.........................................................F...F.......................... 704/13441
................................F...................F.......F..i........................ 792/13441
........................................................................................ 968/13441
........................................................................................ 968/13441
....................................................F.......F.................F......... 1056/13441
FF.......................F.FFF.F...F...F.F......F....................................... 1144/13441
.........F.........F.F..........F...F....F...F..F....FF...F....FF.....FF...FF........... 1232/13441
.F.....F...F.........FF....F......................i..............F............FFF....... 1320/13441
.F...F..F.......F......................................F................................ 1408/13441
.......................................................................F................ 1584/13441
.......................................................F................................ 1672/13441
.......................................................F................................ 1672/13441
...........................................................i...F..ii.................... 1760/13441
........................................................................................ 1936/13441
..................................i..................................................... 2024/13441
.......F................................................................................ 2112/13441
........................................................................................ 2200/13441
---
............F........................................................................... 4400/13441
........................................................................................ 4488/13441
........................................................................................ 4576/13441
........................................................................................ 4664/13441
...........................F.F.......................................................... 4752/13441
......................................................F................................. 4928/13441
........................................................................................ 5016/13441
........................................................................................ 5104/13441
............................................................................i........... 5192/13441
............................................................................i........... 5192/13441
.......................................................i................................ 5280/13441
........................................................................................ 5368/13441
........................................................................................ 5456/13441
...F.....................................................F.............................. 5544/13441
........................................................................................ 5720/13441
........................................................................................ 5808/13441
........................................................................................ 5896/13441
........................................................................................ 5984/13441
---
........................................i............................................... 6600/13441
........................................................................................ 6688/13441
.................i.......................................................ii.i.i.......i. 6776/13441
...i...............................................................i.................... 6864/13441
..........................................................................F.....F....... 6952/13441
.........F.F...F.F.FFFFFFFFFF.F..F.FFFFiF...i.........................................i. 7040/13441
.i...................................................................................... 7216/13441
....................i................................................................... 7304/13441
........................................................................................ 7392/13441
........................................................................................ 7392/13441
..........................F................F............................................ 7480/13441
....................................i................................................... 7656/13441
........................................................................................ 7744/13441
........................................................................................ 7744/13441
............................................ii.................................F..F..... 7832/13441
..F..................................................................................... 8008/13441
..........................................................ii................i....i..ii.. 8096/13441
..........................................................ii................i....i..ii.. 8096/13441
..................F.........F..F........................................................ 8184/13441
.......................................................................F.....FFF.......F 8272/13441
.F.F.F..................................F.........................F...F.....FF....FF.... 8360/13441
..........................F.........F.F.F......F..............FF......F.F.FF............ 8448/13441
........................F..F......F......F......F.................F..................... 8536/13441
...F.F..F.....i..ii..............................................................ii..... 8624/13441
......................................F.......F........................................i 8712/13441
.........................................i........................................i..... 8888/13441
..............................................................i......................... 8976/13441
........................................................................................ 9064/13441
...........................................................i............................ 9152/13441
---
........................................................................................ 9856/13441
..............................................................................ii........ 9944/13441
.......i................................................................................ 10032/13441
........................................................................................ 10120/13441
......F....................F..F...F......F.........F....F.........F.F..FFFF........F.F.. 10208/13441
F.......FF...................FFF....F................................FF........F....FF.F 10296/13441
F.........F........FF.F..F.............................................................. 10384/13441
........................................................................................ 10560/13441
........................................................................................ 10648/13441
...............F.iiiii...i....i..i...................................................... 10736/13441
..........................................................................i............. 10824/13441
..........................................................................i............. 10824/13441
....................................................................................iiii 10912/13441
ii.i..iiiiii.i............................F.F......................F.F..........FF.FFFF. 11000/13441
F.F..................................................................................... 11088/13441
........................................................................................ 11264/13441
........................................................................................ 11352/13441
........................................................................................ 11440/13441
........................................................................................ 11528/13441
---
.....................................................................................i.. 12584/13441
........................................................................................ 12672/13441
........................................................................................ 12760/13441
........................................................................................ 12848/13441
.........................................F.............................................. 12936/13441
.............F..............F........................................................... 13024/13441
........................................................................................ 13200/13441
........................................................................................ 13288/13441
........................................................................................ 13288/13441
...................FFFFF.FFFF.F...............iii.FF.F.................................. 13376/13441
.........F.......................................................

---- [ui] src/test/ui/asm/x86_64/type-check-5.rs stdout ----


error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/type-check-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-5" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-5/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7fa0c5d95d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   0:     0x7fa0c5d95d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7fa0c5dfe848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7fa0c5d86901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7fa0c5d98d9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7fa0c5d98a67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7fa0c67204d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa0c5d99551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7fa0c5d99377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7fa0c5d96314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7fa0c5d99042 - rust_begin_unwind
  10:     0x7fa0c5d49e43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7fa0c5dfaed1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7fa0c5dfae7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7fa0c5d49cb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7fa0c941e4d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7fa0c941b812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fa0c9413d3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fa0c9411386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7fa0c9429b22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fa0c942853c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7fa0c946e2b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7fa0c946cbb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7fa0c7a06ca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7fa0c77d50d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7fa0c79fbe57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7fa0c79c6f1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7fa0c82673f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7fa0c8339e33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7fa0c819a6f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fa0c68bb656 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  30:     0x7fa0c683bf79 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7fa0c681fc8d - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7fa0c68af535 - <rustc_session[9aa1c3390f62384]::session::Session>::time::<(), rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}>
  33:     0x7fa0c687689b - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  34:     0x7fa0c82a9404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  35:     0x7fa0c838d214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  36:     0x7fa0c817ba1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  37:     0x7fa0c678c07d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  38:     0x7fa0c6728726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  39:     0x7fa0c670af15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7fa0c6743df1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  41:     0x7fa0c670cb42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  42:     0x7fa0c67829d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  43:     0x7fa0c670cf3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  44:     0x7fa0c6787010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7fa0c5da6265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  46:     0x7fa0c5b43b43 - <unknown>
  47:     0x7fa0c5bd5a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (26e596d41 2022-08-28) running on x86_64-unknown-linux-gnu

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
stack backtrace:
   0:     0x7fd133327d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7fd133390848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7fd133318901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7fd13332ad9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7fd13332aa67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7fd133cb24d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd13332b551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7fd13332b377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7fd133328314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7fd13332b042 - rust_begin_unwind
  10:     0x7fd1332dbe43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7fd13338ced1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7fd13338ce7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7fd1332dbcb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7fd1369b04d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7fd1369ad812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fd1369a5d3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fd1369a3386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7fd1369bbb22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fd1369ba53c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7fd136a002b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7fd1369febb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7fd134f98ca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7fd134d670d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7fd134f8de57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7fd134f58f1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7fd1357f93f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7fd1358cbe33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7fd13572c6f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fd1367994f5 - <rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt>::mir_borrowck_opt_const_arg
  30:     0x7fd1342ddf7d - rustc_mir_transform[435b04e39e146f6c]::mir_drops_elaborated_and_const_checked
  31:     0x7fd1357e5f16 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_middle[7c90c35f413fc7d9]::ty::WithOptConstParam<rustc_span[8da2952674cc1f00]::def_id::LocalDefId>, &rustc_data_structures[903518a891f28ab1]::steal::Steal<rustc_middle[7c90c35f413fc7d9]::mir::Body>>>
  32:     0x7fd135919e25 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  33:     0x7fd135714bd7 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  34:     0x7fd1342ddb77 - rustc_mir_transform[435b04e39e146f6c]::inner_mir_for_ctfe
  35:     0x7fd1342dd616 - rustc_mir_transform[435b04e39e146f6c]::mir_for_ctfe
  36:     0x7fd135820ac9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::DefId, &rustc_middle[7c90c35f413fc7d9]::mir::Body>>
  37:     0x7fd1358cbf99 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_for_ctfe, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  38:     0x7fd135715119 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_for_ctfe
  39:     0x7fd13528e5f5 - <rustc_const_eval[62f1c59fbbbe7ba7]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[62f1c59fbbbe7ba7]::interpret::machine::Machine>::load_mir
  40:     0x7fd1352436d6 - <rustc_const_eval[62f1c59fbbbe7ba7]::interpret::eval_context::InterpCx<rustc_const_eval[62f1c59fbbbe7ba7]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  41:     0x7fd13537c24c - rustc_const_eval[62f1c59fbbbe7ba7]::const_eval::eval_queries::eval_to_allocation_raw_provider
  42:     0x7fd1358f388a - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::eval_to_allocation_raw, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  43:     0x7fd13572ecd1 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::eval_to_allocation_raw
  44:     0x7fd13538d5f1 - rustc_const_eval[62f1c59fbbbe7ba7]::const_eval::eval_to_valtree
  45:     0x7fd1352aa9a0 - <rustc_const_eval[62f1c59fbbbe7ba7]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_middle[7c90c35f413fc7d9]::ty::ParamEnvAnd<rustc_middle[7c90c35f413fc7d9]::mir::interpret::GlobalId>)>>::call_once
  46:     0x7fd1358d3477 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::eval_to_valtree, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  47:     0x7fd13572f871 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::eval_to_valtree
  48:     0x7fd1367b0e56 - <rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  49:     0x7fd1367af82a - <rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  50:     0x7fd1364db00e - <rustc_infer[eca65feee8c4c344]::infer::InferCtxt>::const_eval_resolve
  51:     0x7fd1362a7eea - rustc_trait_selection[48b36461b9a40bde]::traits::const_evaluatable::is_const_evaluatable
  52:     0x7fd1363ed1a0 - <rustc_trait_selection[48b36461b9a40bde]::traits::fulfill::FulfillProcessor as rustc_data_structures[903518a891f28ab1]::obligation_forest::ObligationProcessor>::process_obligation
  53:     0x7fd1362f8d5e - <rustc_data_structures[903518a891f28ab1]::obligation_forest::ObligationForest<rustc_trait_selection[48b36461b9a40bde]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[48b36461b9a40bde]::traits::fulfill::FulfillProcessor, rustc_data_structures[903518a891f28ab1]::obligation_forest::Outcome<rustc_trait_selection[48b36461b9a40bde]::traits::fulfill::PendingPredicateObligation, rustc_infer[eca65feee8c4c344]::traits::FulfillmentErrorCode>>
  54:     0x7fd1363e756a - <rustc_trait_selection[48b36461b9a40bde]::traits::fulfill::FulfillmentContext as rustc_infer[eca65feee8c4c344]::traits::engine::TraitEngine>::select_where_possible
  55:     0x7fd1363e7448 - <rustc_trait_selection[48b36461b9a40bde]::traits::fulfill::FulfillmentContext as rustc_infer[eca65feee8c4c344]::traits::engine::TraitEngine>::select_all_or_error
  56:     0x7fd13641749b - <rustc_trait_selection[48b36461b9a40bde]::traits::engine::ObligationCtxt>::select_all_or_error
  57:     0x7fd1347eae13 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[566232d667df7ecc]::check::wfcheck::enter_wf_checking_ctxt<rustc_typeck[566232d667df7ecc]::check::wfcheck::check_type_defn<rustc_typeck[566232d667df7ecc]::check::wfcheck::check_item::{closure#3}>::{closure#0}>::{closure#0}>
  58:     0x7fd1348acba5 - rustc_typeck[566232d667df7ecc]::check::wfcheck::check_well_formed
  59:     0x7fd1357fbf89 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, ()>>
  60:     0x7fd1358dd806 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::check_well_formed, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  61:     0x7fd135747d94 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::check_well_formed
  62:     0x7fd134701235 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_hir[f560e551bc84b0fc]::hir::ItemId], <rustc_middle[7c90c35f413fc7d9]::hir::ModuleItems>::par_items<rustc_typeck[566232d667df7ecc]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  63:     0x7fd134693bc9 - std[aa0add2ff02ab889]::panicking::try::<(), core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_hir[f560e551bc84b0fc]::hir::ItemId], <rustc_middle[7c90c35f413fc7d9]::hir::ModuleItems>::par_items<rustc_typeck[566232d667df7ecc]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  64:     0x7fd13462b85d - <rustc_middle[7c90c35f413fc7d9]::hir::ModuleItems>::par_items::<rustc_typeck[566232d667df7ecc]::check::wfcheck::check_mod_type_wf::{closure#0}>
  65:     0x7fd1348b7599 - rustc_typeck[566232d667df7ecc]::check::wfcheck::check_mod_type_wf
  66:     0x7fd1357fbf89 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, ()>>
  67:     0x7fd1358dd6d6 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::check_mod_type_wf, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  68:     0x7fd135729144 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::check_mod_type_wf
  69:     0x7fd134700bf5 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_for_each_module<rustc_typeck[566232d667df7ecc]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  70:     0x7fd134693b09 - std[aa0add2ff02ab889]::panicking::try::<(), core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_for_each_module<rustc_typeck[566232d667df7ecc]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  71:     0x7fd13463092d - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_for_each_module<rustc_typeck[566232d667df7ecc]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  72:     0x7fd13471cc4d - <rustc_session[9aa1c3390f62384]::session::Session>::track_errors::<rustc_typeck[566232d667df7ecc]::check_crate::{closure#5}, ()>
  73:     0x7fd13494dd20 - rustc_typeck[566232d667df7ecc]::check_crate
  74:     0x7fd133e08861 - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  75:     0x7fd13583b404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  76:     0x7fd13591f214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  77:     0x7fd13570da1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  78:     0x7fd133d1e07d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  79:     0x7fd133cba726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  80:     0x7fd133c9cf15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  81:     0x7fd133cd5df1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  82:     0x7fd133c9eb42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  83:     0x7fd133d149d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  84:     0x7fd133c9ef3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  85:     0x7fd133d19010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  86:     0x7fd133338265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  87:     0x7fd1330d5b43 - <unknown>
  88:     0x7fd133167a00 - <unknown>
  89:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (26e596d41 2022-08-28) running on x86_64-unknown-linux-gnu

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
   0:     0x7fe020811d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   0:     0x7fe020811d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7fe02087a848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7fe020802901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7fe020814d9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7fe020814a67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7fe02119c4d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe020815551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7fe020815377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7fe020812314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7fe020815042 - rust_begin_unwind
  10:     0x7fe0207c5e43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7fe020876ed1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7fe020876e7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7fe0207c5cb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7fe023e9a4d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7fe023e97812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fe023e8fd3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fe023e8d386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7fe023ea5b22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fe023ea453c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7fe023eea2b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7fe023ee8bb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7fe022482ca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7fe0222510d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7fe022477e57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7fe022442f1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7fe022ce33f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7fe022db5e33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7fe022c166f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fe023c834f5 - <rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt>::mir_borrowck_opt_const_arg
  30:     0x7fe0217c7f7d - rustc_mir_transform[435b04e39e146f6c]::mir_drops_elaborated_and_const_checked
  31:     0x7fe022ccff16 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_middle[7c90c35f413fc7d9]::ty::WithOptConstParam<rustc_span[8da2952674cc1f00]::def_id::LocalDefId>, &rustc_data_structures[903518a891f28ab1]::steal::Steal<rustc_middle[7c90c35f413fc7d9]::mir::Body>>>
  32:     0x7fe022e03e25 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  33:     0x7fe022bfebd7 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  34:     0x7fe0217c7b77 - rustc_mir_transform[435b04e39e146f6c]::inner_mir_for_ctfe
  35:     0x7fe0217c7616 - rustc_mir_transform[435b04e39e146f6c]::mir_for_ctfe
  36:     0x7fe022d0aac9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::DefId, &rustc_middle[7c90c35f413fc7d9]::mir::Body>>
  37:     0x7fe022db5f99 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_for_ctfe, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  38:     0x7fe022bff119 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_for_ctfe
  39:     0x7fe0227785f5 - <rustc_const_eval[62f1c59fbbbe7ba7]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[62f1c59fbbbe7ba7]::interpret::machine::Machine>::load_mir
  40:     0x7fe02272d6d6 - <rustc_const_eval[62f1c59fbbbe7ba7]::interpret::eval_context::InterpCx<rustc_const_eval[62f1c59fbbbe7ba7]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  41:     0x7fe02286624c - rustc_const_eval[62f1c59fbbbe7ba7]::const_eval::eval_queries::eval_to_allocation_raw_provider
  42:     0x7fe022ddd88a - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::eval_to_allocation_raw, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  43:     0x7fe022c18cd1 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::eval_to_allocation_raw
  44:     0x7fe02286461b - rustc_const_eval[62f1c59fbbbe7ba7]::const_eval::eval_queries::eval_to_const_value_raw_provider
  45:     0x7fe022de503a - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::eval_to_const_value_raw, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  46:     0x7fe022c192a1 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::eval_to_const_value_raw
  47:     0x7fe022864222 - rustc_const_eval[62f1c59fbbbe7ba7]::const_eval::eval_queries::eval_to_const_value_raw_provider
  48:     0x7fe022de503a - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::eval_to_const_value_raw, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  49:     0x7fe022c192a1 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::eval_to_const_value_raw
  50:     0x7fe023c9a1d4 - <rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt>::const_eval_global_id
  51:     0x7fe023c7217e - <rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt>::const_eval_instance
  52:     0x7fe02216be37 - <rustc_mir_build[588fc3c3046c592e]::thir::pattern::PatCtxt>::lower_path
  53:     0x7fe02216732a - <rustc_mir_build[588fc3c3046c592e]::thir::pattern::PatCtxt>::lower_pattern
  54:     0x7fe022200ca4 - <rustc_mir_build[588fc3c3046c592e]::thir::pattern::check_match::MatchVisitor>::lower_pattern
  55:     0x7fe0222017f5 - <rustc_mir_build[588fc3c3046c592e]::thir::pattern::check_match::MatchVisitor>::check_irrefutable
  56:     0x7fe022200c0d - <rustc_mir_build[588fc3c3046c592e]::thir::pattern::check_match::MatchVisitor as rustc_hir[f560e551bc84b0fc]::intravisit::Visitor>::visit_local
  57:     0x7fe0221e4795 - rustc_hir[f560e551bc84b0fc]::intravisit::walk_expr::<rustc_mir_build[588fc3c3046c592e]::thir::pattern::check_match::MatchVisitor>
  58:     0x7fe0221fe8cc - <rustc_mir_build[588fc3c3046c592e]::thir::pattern::check_match::MatchVisitor as rustc_hir[f560e551bc84b0fc]::intravisit::Visitor>::visit_expr
  59:     0x7fe0221fe67f - rustc_mir_build[588fc3c3046c592e]::thir::pattern::check_match::check_match
  60:     0x7fe022d141bc - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::DefId, ()>>
  61:     0x7fe022db048a - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::check_match, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  62:     0x7fe022c1c669 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::check_match
  63:     0x7fe021337315 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  64:     0x7fe0212b7f19 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  65:     0x7fe02129b9cd - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  66:     0x7fe0213397db - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#1}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  67:     0x7fe0212b81e9 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#1}::{closure#0}>, ()>
  68:     0x7fe02132c43e - <rustc_session[9aa1c3390f62384]::session::Session>::time::<(), rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#1}>
  69:     0x7fe0212f2884 - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  70:     0x7fe022d25404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  71:     0x7fe022e09214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  72:     0x7fe022bf7a1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  73:     0x7fe02120807d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  74:     0x7fe0211a4726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  75:     0x7fe021186f15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  76:     0x7fe0211bfdf1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  77:     0x7fe021188b42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  78:     0x7fe0211fe9d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  79:     0x7fe021188f3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  80:     0x7fe021203010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  81:     0x7fe020822265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  82:     0x7fe0205bfb43 - <unknown>
  83:     0x7fe020651a00 - <unknown>
  84:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (26e596d41 2022-08-28) running on x86_64-unknown-linux-gnu

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
   0:     0x7fd20153bd9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   0:     0x7fd20153bd9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7fd2015a4848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7fd20152c901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7fd20153ed9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7fd20153ea67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7fd201ec64d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd20153f551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7fd20153f377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7fd20153c314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7fd20153f042 - rust_begin_unwind
  10:     0x7fd2014efe43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7fd2015a0ed1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7fd2015a0e7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7fd2014efcb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7fd204bc44d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7fd204bc1812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fd204bb9d3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fd204bb7386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7fd204bcfb22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fd204bce53c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7fd204c142b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7fd204c12bb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7fd2031acca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7fd202f7b0d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7fd2031a1e57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7fd20316cf1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7fd203a0d3f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7fd203adfe33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7fd2039406f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7fd202061656 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  30:     0x7fd201fe1f79 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7fd201fc5c8d - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7fd202055535 - <rustc_session[9aa1c3390f62384]::session::Session>::time::<(), rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}>
  33:     0x7fd20201c89b - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  34:     0x7fd203a4f404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  35:     0x7fd203b33214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  36:     0x7fd203921a1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  37:     0x7fd201f3207d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  38:     0x7fd201ece726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  39:     0x7fd201eb0f15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7fd201ee9df1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  41:     0x7fd201eb2b42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  42:     0x7fd201f289d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  43:     0x7fd201eb2f3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  44:     0x7fd201f2d010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7fd20154c265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  46:     0x7fd2012e9b43 - <unknown>
  47:     0x7fd20137ba00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (26e596d41 2022-08-28) running on x86_64-unknown-linux-gnu

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
   0:     0x7f297d2b2d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   0:     0x7f297d2b2d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7f297d31b848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7f297d2a3901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7f297d2b5d9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7f297d2b5a67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7f297dc3d4d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f297d2b6551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7f297d2b6377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7f297d2b3314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7f297d2b6042 - rust_begin_unwind
  10:     0x7f297d266e43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7f297d317ed1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7f297d317e7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7f297d266cb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7f298093b4d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7f2980938812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f2980930d3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f298092e386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7f2980946b22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f298094553c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7f298098b2b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7f2980989bb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7f297ef23ca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7f297ecf20d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7f297ef18e57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7f297eee3f1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7f297f7843f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7f297f856e33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7f297f6b76f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f297ddd8656 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  30:     0x7f297dd58f79 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f297dd3cc8d - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f297ddcc535 - <rustc_session[9aa1c3390f62384]::session::Session>::time::<(), rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}>
  33:     0x7f297dd9389b - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  34:     0x7f297f7c6404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  35:     0x7f297f8aa214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  36:     0x7f297f698a1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  37:     0x7f297dca907d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  38:     0x7f297dc45726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  39:     0x7f297dc27f15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f297dc60df1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  41:     0x7f297dc29b42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  42:     0x7f297dc9f9d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  43:     0x7f297dc29f3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  44:     0x7f297dca4010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f297d2c3265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  46:     0x7f297d060b43 - <unknown>
  47:     0x7f297d0f2a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (26e596d41 2022-08-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `uninit_1`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f297d2b2d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   0:     0x7f297d2b2d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7f297d31b848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7f297d2a3901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7f297d2b5d9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7f297d2b5a67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7f297dc3d4d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f297d2b6551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7f297d2b6377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7f297d2b3314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7f297d2b6042 - rust_begin_unwind
  10:     0x7f297d266e43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7f297d317ed1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7f297d317e7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7f297d266cb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7f298093b4d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7f2980938812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f2980930d3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f298092e386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7f2980946b22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f298094553c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7f298098b2b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7f2980989bb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7f297ef23ca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7f297ecf20d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7f297ef18e57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7f297eee3f1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7f297f7843f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7f297f856e33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7f297f6b76f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f297ddd8656 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  30:     0x7f297dd58f79 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f297dd3cc8d - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f297ddcc535 - <rustc_session[9aa1c3390f62384]::session::Session>::time::<(), rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}>
  33:     0x7f297dd9389b - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  34:     0x7f297f7c6404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  35:     0x7f297f8aa214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  36:     0x7f297f698a1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  37:     0x7f297dca907d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  38:     0x7f297dc45726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  39:     0x7f297dc27f15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f297dc60df1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  41:     0x7f297dc29b42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  42:     0x7f297dc9f9d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  43:     0x7f297dc29f3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  44:     0x7f297dca4010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f297d2c3265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  46:     0x7f297d060b43 - <unknown>
  47:     0x7f297d0f2a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (26e596d41 2022-08-28) running on x86_64-unknown-linux-gnu

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
   0:     0x7f1de1be3d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7f1de1c4c848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7f1de1bd4901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7f1de1be6d9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7f1de1be6a67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7f1de256e4d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1de1be7551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7f1de1be7377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7f1de1be4314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7f1de1be7042 - rust_begin_unwind
  10:     0x7f1de1b97e43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7f1de1c48ed1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7f1de1c48e7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7f1de1b97cb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7f1de526c4d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7f1de5269812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f1de5261d3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f1de525f386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7f1de5277b22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f1de527653c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7f1de52bc2b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7f1de52babb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7f1de3854ca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7f1de36230d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7f1de3849e57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7f1de3814f1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7f1de40b53f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7f1de4187e33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7f1de3fe86f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f1de2709656 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  30:     0x7f1de2689f79 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f1de266dc8d - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f1de26fd535 - <rustc_session[9aa1c3390f62384]::session::Session>::time::<(), rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}>
  33:     0x7f1de26c489b - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  34:     0x7f1de40f7404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  35:     0x7f1de41db214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  36:     0x7f1de3fc9a1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  37:     0x7f1de25da07d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  38:     0x7f1de2576726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  39:     0x7f1de2558f15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f1de2591df1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  41:     0x7f1de255ab42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  42:     0x7f1de25d09d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  43:     0x7f1de255af3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  44:     0x7f1de25d5010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f1de1bf4265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  46:     0x7f1de1991b43 - <unknown>
  47:     0x7f1de1a23a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (26e596d41 2022-08-28) running on x86_64-unknown-linux-gnu

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
   0:     0x7f3efcb33d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7f3efcb9c848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7f3efcb24901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7f3efcb36d9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7f3efcb36a67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7f3efd4be4d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f3efcb37551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7f3efcb37377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7f3efcb34314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7f3efcb37042 - rust_begin_unwind
  10:     0x7f3efcae7e43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7f3efcb98ed1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7f3efcb98e7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7f3efcae7cb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7f3f001bc4d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7f3f001b9812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f3f001b1d3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f3f001af386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7f3f001c7b22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f3f001c653c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7f3f0020c2b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7f3f0020abb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7f3efe7a4ca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7f3efe5730d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7f3efe799e57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7f3efe764f1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7f3eff0053f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7f3eff0d7e33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7f3efef386f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f3efd659656 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  30:     0x7f3efd5d9f79 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f3efd5bdc8d - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f3efd64d535 - <rustc_session[9aa1c3390f62384]::session::Session>::time::<(), rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}>
  33:     0x7f3efd61489b - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  34:     0x7f3eff047404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  35:     0x7f3eff12b214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  36:     0x7f3efef19a1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  37:     0x7f3efd52a07d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  38:     0x7f3efd4c6726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  39:     0x7f3efd4a8f15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f3efd4e1df1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  41:     0x7f3efd4aab42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  42:     0x7f3efd5209d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  43:     0x7f3efd4aaf3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  44:     0x7f3efd525010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f3efcb44265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  46:     0x7f3efc8e1b43 - <unknown>
  47:     0x7f3efc973a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (26e596d41 2022-08-28) running on x86_64-unknown-linux-gnu

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



---- [ui] src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs stdout ----
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
   0:     0x7f61d8eb0d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7f61d8f19848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7f61d8ea1901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7f61d8eb3d9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7f61d8eb3a67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7f61d983b4d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f61d8eb4551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7f61d8eb4377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7f61d8eb1314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7f61d8eb4042 - rust_begin_unwind
  10:     0x7f61d8e64e43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7f61d8f15ed1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7f61d8f15e7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7f61d8e64cb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7f61dc5394d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7f61dc536812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f61dc52ed3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f61dc52c386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7f61dc544b22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f61dc54353c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7f61dc5892b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7f61dc587bb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7f61dab21ca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7f61da8f00d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7f61dab16e57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7f61daae1f1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7f61db3823f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7f61db454e33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7f61db2b56f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f61d99d6656 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  30:     0x7f61d9956f79 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f61d993ac8d - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f61d99ca535 - <rustc_session[9aa1c3390f62384]::session::Session>::time::<(), rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}>
  33:     0x7f61d999189b - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  34:     0x7f61db3c4404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  35:     0x7f61db4a8214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  36:     0x7f61db296a1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  37:     0x7f61d98a707d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  38:     0x7f61d9843726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  39:     0x7f61d9825f15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f61d985edf1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  41:     0x7f61d9827b42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  42:     0x7f61d989d9d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  43:     0x7f61d9827f3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  44:     0x7f61d98a2010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f61d8ec1265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  46:     0x7f61d8c5eb43 - <unknown>
  47:     0x7f61d8cf0a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (26e596d41 2022-08-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `cannot_partially_init_adt_with_drop`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7f61d8eb0d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7f61d8f19848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7f61d8ea1901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7f61d8eb3d9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7f61d8eb3a67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7f61d983b4d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f61d8eb4551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7f61d8eb4377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7f61d8eb1314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7f61d8eb4042 - rust_begin_unwind
  10:     0x7f61d8e64e43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7f61d8f15ed1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7f61d8f15e7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7f61d8e64cb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7f61dc5394d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7f61dc536812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f61dc52ed3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f61dc52c386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7f61dc544b22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f61dc54353c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7f61dc5892b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7f61dc587bb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7f61dab21ca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7f61da8f00d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7f61dab16e57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7f61daae1f1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7f61db3823f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7f61db454e33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7f61db2b56f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f61d99d6656 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  30:     0x7f61d9956f79 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f61d993ac8d - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f61d99ca535 - <rustc_session[9aa1c3390f62384]::session::Session>::time::<(), rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}>
  33:     0x7f61d999189b - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  34:     0x7f61db3c4404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  35:     0x7f61db4a8214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  36:     0x7f61db296a1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  37:     0x7f61d98a707d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  38:     0x7f61d9843726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  39:     0x7f61d9825f15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f61d985edf1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  41:     0x7f61d9827b42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  42:     0x7f61d989d9d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  43:     0x7f61d9827f3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  44:     0x7f61d98a2010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f61d8ec1265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  46:     0x7f61d8c5eb43 - <unknown>
  47:     0x7f61d8cf0a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (26e596d41 2022-08-28) running on x86_64-unknown-linux-gnu

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
stack backtrace:
   0:     0x7f61d8eb0d9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc09286111e572d39
   1:     0x7f61d8f19848 - core::fmt::write::h0b3b3116cd99b6c5
   2:     0x7f61d8ea1901 - std::io::Write::write_fmt::h44fe3a5a762c6d4c
   3:     0x7f61d8eb3d9e - std::panicking::default_hook::{{closure}}::h085efc95531cca5e
   4:     0x7f61d8eb3a67 - std::panicking::default_hook::h141bf036904fbc4c
   5:     0x7f61d983b4d4 - rustc_driver[bfa76eff64a5c0e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f61d8eb4551 - std::panicking::rust_panic_with_hook::h90ab57157093ed6a
   7:     0x7f61d8eb4377 - std::panicking::begin_panic_handler::{{closure}}::he755aacb8e67557a
   8:     0x7f61d8eb1314 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ecfdbc10d62ef5b
   9:     0x7f61d8eb4042 - rust_begin_unwind
  10:     0x7f61d8e64e43 - core::panicking::panic_fmt::h6230a26a7ad979d7
  11:     0x7f61d8f15ed1 - core::panicking::panic_display::h781ed9178c7122c1
  12:     0x7f61d8f15e7b - core::panicking::panic_str::h83370df4ca56c377
  13:     0x7f61d8e64cb6 - core::option::expect_failed::h7db4dd805bdcdd6a
  14:     0x7f61dc5394d2 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::translation::Translate>::translate_message
  15:     0x7f61dc536812 - <rustc_errors[c8754a17d738d237]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f61dc52ed3a - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f61dc52c386 - <rustc_errors[c8754a17d738d237]::emitter::EmitterWriter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  18:     0x7f61dc544b22 - <rustc_errors[c8754a17d738d237]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f61dc54353c - <rustc_errors[c8754a17d738d237]::json::JsonEmitter as rustc_errors[c8754a17d738d237]::emitter::Emitter>::emit_diagnostic
  20:     0x7f61dc5892b8 - <rustc_errors[c8754a17d738d237]::HandlerInner>::emit_diagnostic
  21:     0x7f61dc587bb1 - <rustc_errors[c8754a17d738d237]::Handler>::emit_diagnostic
  22:     0x7f61dab21ca3 - rustc_borrowck[d3e8aaeabe134467]::do_mir_borrowck
  23:     0x7f61da8f00d7 - <rustc_infer[eca65feee8c4c344]::infer::InferCtxtBuilder>::enter::<rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult, rustc_borrowck[d3e8aaeabe134467]::mir_borrowck::{closure#0}>
  24:     0x7f61dab16e57 - rustc_borrowck[d3e8aaeabe134467]::mir_borrowck
  25:     0x7f61daae1f1e - <rustc_borrowck[d3e8aaeabe134467]::provide::{closure#0} as core[c0ec586217c735e6]::ops::function::FnOnce<(rustc_middle[7c90c35f413fc7d9]::ty::context::TyCtxt, rustc_span[8da2952674cc1f00]::def_id::LocalDefId)>>::call_once
  26:     0x7f61db3823f9 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<rustc_span[8da2952674cc1f00]::def_id::LocalDefId, &rustc_middle[7c90c35f413fc7d9]::mir::query::BorrowCheckResult>>
  27:     0x7f61db454e33 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::mir_borrowck, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  28:     0x7f61db2b56f4 - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::mir_borrowck
  29:     0x7f61d99d6656 - <core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once
  30:     0x7f61d9956f79 - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  31:     0x7f61d993ac8d - rustc_data_structures[903518a891f28ab1]::sync::par_for_each_in::<&[rustc_span[8da2952674cc1f00]::def_id::LocalDefId], <rustc_middle[7c90c35f413fc7d9]::hir::map::Map>::par_body_owners<rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f61d99ca535 - <rustc_session[9aa1c3390f62384]::session::Session>::time::<(), rustc_interface[f7c6e6e311a814b4]::passes::analysis::{closure#2}>
  33:     0x7f61d999189b - rustc_interface[f7c6e6e311a814b4]::passes::analysis
  34:     0x7f61db3c4404 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::try_execute_query::<rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt, rustc_query_system[44d21f4c8bda61d4]::query::caches::DefaultCache<(), core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>>
  35:     0x7f61db4a8214 - rustc_query_system[44d21f4c8bda61d4]::query::plumbing::get_query::<rustc_query_impl[3badc20e7186c283]::queries::analysis, rustc_query_impl[3badc20e7186c283]::plumbing::QueryCtxt>
  36:     0x7f61db296a1e - <rustc_query_impl[3badc20e7186c283]::Queries as rustc_middle[7c90c35f413fc7d9]::ty::query::QueryEngine>::analysis
  37:     0x7f61d98a707d - <rustc_interface[f7c6e6e311a814b4]::passes::QueryContext>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  38:     0x7f61d9843726 - <rustc_interface[f7c6e6e311a814b4]::interface::Compiler>::enter::<rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}::{closure#2}, core[c0ec586217c735e6]::result::Result<core[c0ec586217c735e6]::option::Option<rustc_interface[f7c6e6e311a814b4]::queries::Linker>, rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  39:     0x7f61d9825f15 - rustc_span[8da2952674cc1f00]::with_source_map::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f61d985edf1 - rustc_interface[f7c6e6e311a814b4]::interface::create_compiler_and_run::<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>
  41:     0x7f61d9827b42 - <scoped_tls[39541f8b2c8068a8]::ScopedKey<rustc_span[8da2952674cc1f00]::SessionGlobals>>::set::<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  42:     0x7f61d989d9d9 - std[aa0add2ff02ab889]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  43:     0x7f61d9827f3e - std[aa0add2ff02ab889]::panic::catch_unwind::<core[c0ec586217c735e6]::panic::unwind_safe::AssertUnwindSafe<<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>
  44:     0x7f61d98a2010 - <<std[aa0add2ff02ab889]::thread::Builder>::spawn_unchecked_<rustc_interface[f7c6e6e311a814b4]::util::run_in_thread_pool_with_globals<rustc_interface[f7c6e6e311a814b4]::interface::run_compiler<core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>, rustc_driver[bfa76eff64a5c0e3]::run_compiler::{closure#1}>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#0}, core[c0ec586217c735e6]::result::Result<(), rustc_errors[c8754a17d738d237]::ErrorGuaranteed>>::{closure#1} as core[c0ec586217c735e6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f61d8ec1265 - std::sys::unix::thread::Thread::new::thread_start::h2b31fd39f02c33b6
  46:     0x7f61d8c5eb43 - <unknown>
  47:     0x7f61d8cf0a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic
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
