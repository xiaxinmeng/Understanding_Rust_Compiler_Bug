plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:42:10] ............................................................................i.......................
[00:42:16] ...................i................................................................................
---
[00:42:50] .................................................................................................i..
[00:42:57] .......................................................................i............................
---
[00:43:50] .............................................i......................................................
---
[00:47:34] .............................i......................................................................
[00:47:49] ..............................................................i.....................................
[00:48:04] ...............................................i....................................................
[00:48:23] ....................................................................................................
[00:48:44] ....................................................................................................
[00:49:05] ....................................................................................................
[00:49:29] ...i...............................................................................................i
[00:49:58] .......................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:50:04] .............
[00:50:33] ....................................................................................................
[00:51:08] .................................................................ii.................................
[00:51:54] ............................i....................................................i.ii..........test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:51:57] .....
[00:52:35] .........................................................................................iiiiiii....
---
[00:54:44] ...................i............................................................ii.iii..............
[00:54:51] ....................................................................................................
[00:54:59] .........i..............................i...........................................................
[00:55:07] ....................................................................................................
[00:55:13] .....................i..............................................................................
[00:55:21] ....................................................................................................
[00:55:31] ....................................................................................................
[00:55:40] ....................................................................................................
[00:55:51] ....................................................................................................
[00:56:03] ....................................................................................................
[00:56:11] ..............i.....................................................................................
[00:56:20] ..................i..ii.............................................................................
[00:56:30] ....................................................................................................
[00:56:39] ....................................................................................................
[00:56:48] .....................................................................................i..............
[00:56:58] ...............................i....................................................................
---
[00:57:33] ...........................i........................................................................
[00:57:35] ....................................................................i...............................
[00:57:36] ................i.......................................................
---
[00:57:51] .............i........................
---
[00:58:21] i...i..ii....i.............ii.........iii......i..i...i...ii..i..i..ii.....
---
[00:58:24] i.......i......................i.......
---
[00:59:03] iiii.......i..i........i..i.i.............i..........iiii...........i...i..........ii.i.i.......ii..
[00:59:04] ....ii...
---
[01:00:48] ........................F.......F.F.FF.F.FF.FFF..................................F..Ftest [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:02:54] .
[01:02:54] failures:
[01:02:54]
[01:02:54] ---- [run-pass] run-pass-fulldeps/issue-16723.rs stdout ----
[01:02:54]
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-16723.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-16723.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-16723.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-16723.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54] 29 |     reg.register_macro("multiple_items", expand)
[01:02:54]    |                                          ^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand}`
---
[01:02:54] thread '[run-pass] run-pass-fulldeps/issue-16723.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:02:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:54]
[01:02:54] ---- [run-pass] run-pass-fulldeps/macro-crate-does-hygiene-work.rs stdout ----
[01:02:54]
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-does-hygiene-work.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-does-hygiene-work.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:02:54]    |
[01:02:54] 33 | macro_rules! unexported_macro { () => (3) }
[01:02:54]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:02:54]    |
[01:02:54]    = note: #[warn(unused_macros)] on by default
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:37:36
[01:02:54]    |
[01:02:54] 37 |     reg.register_macro("make_a_1", expand_make_a_1);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_make_a_1}`
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:38:36
[01:02:54]    |
[01:02:54] 38 |     reg.register_macro("identity", expand_identity);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_identity}`
---
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:02:54]    |
[01:02:54] 33 | macro_rules! unexported_macro { () => (3) }
[01:02:54]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:02:54]    |
[01:02:54]    = note: #[warn(unused_macros)] on by default
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:37:36
[01:02:54]    |
[01:02:54] 37 |     reg.register_macro("make_a_1", expand_make_a_1);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_make_a_1}`
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:38:36
[01:02:54]    |
[01:02:54] 38 |     reg.register_macro("identity", expand_identity);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_identity}`
---
[01:02:54] thread '[run-pass] run-pass-fulldeps/macro-crate-multi-decorator-literals.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:02:54]
[01:02:54] ---- [run-pass] run-pass-fulldeps/macro-crate-multi-decorator.rs stdout ----
[01:02:54]
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:02:54]    |
[01:02:54] 33 | macro_rules! unexported_macro { () => (3) }
[01:02:54]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:02:54]    |
[01:02:54]    = note: #[warn(unused_macros)] on by default
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:37:36
[01:02:54]    |
[01:02:54] 37 |     reg.register_macro("make_a_1", expand_make_a_1);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_make_a_1}`
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:38:36
[01:02:54]    |
[01:02:54] 38 |     reg.register_macro("identity", expand_identity);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_identity}`
---
[01:02:54] thread '[run-pass] run-pass-fulldeps/macro-crate-multi-decorator.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:02:54]
[01:02:54] ---- [run-pass] run-pass-fulldeps/macro-crate.rs stdout ----
[01:02:54]
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:02:54]    |
[01:02:54] 33 | macro_rules! unexported_macro { () => (3) }
[01:02:54]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:02:54]    |
[01:02:54]    = note: #[warn(unused_macros)] on by default
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:37:36
[01:02:54]    |
[01:02:54] 37 |     reg.register_macro("make_a_1", expand_make_a_1);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_make_a_1}`
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:38:36
[01:02:54]    |
[01:02:54] 38 |     reg.register_macro("identity", expand_identity);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_identity}`
---
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/mbe_matching_test_macro.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/mbe_matching_test_macro.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54] warning: unused import: `std::cell::RefCell`
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs:35:5
---
[01:02:54] warning: unused import: `quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+)`
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs:40:23
[01:02:54]    |
[01:02:54] 40 |     let mbe_matcher = quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+);
[01:02:54]    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs:89:35
[01:02:54]    |
[01:02:54] 89 |     reg.register_macro("matches", expand_mbe_matches);
[01:02:54]    |                                   ^^^^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_mbe_matches}`
---
[01:02:54] thread '[run-pass] run-pass-fulldeps/mbe_matching_test_macro.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:02:54]
[01:02:54] ---- [run-pass] run-pass-fulldeps/plugin-args-1.rs stdout ----
[01:02:54]
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-1.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-1.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54] error[E0050]: method `expand` has 4 parameters but the declaration in trait `syntax::ext::base::TTMacroExpander::expand` has 5
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:40:23
[01:02:54]    |
[01:02:54] 40 |                    _: TokenStream) -> Box<MacResult+'cx> {
[01:02:54]    |                       ^^^^^^^^^^^ expected 5 parameters, found 4
[01:02:54]    |
[01:02:54]    = note: `expand` from trait: `fn(&Self, &'cx mut syntax::ext::base::ExtCtxt<'_>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, syntax::tokenstream::TokenStream) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
---
[01:02:54] thread '[run-pass] run-pass-fulldeps/plugin-args-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:02:54]
[01:02:54] ---- [run-pass] run-pass-fulldeps/plugin-args-2.rs stdout ----
[01:02:54]
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-2.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-2.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54] error[E0050]: method `expand` has 4 parameters but the declaration in trait `syntax::ext::base::TTMacroExpander::expand` has 5
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:40:23
[01:02:54]    |
[01:02:54] 40 |                    _: TokenStream) -> Box<MacResult+'cx> {
[01:02:54]    |                       ^^^^^^^^^^^ expected 5 parameters, found 4
[01:02:54]    |
[01:02:54]    = note: `expand` from trait: `fn(&Self, &'cx mut syntax::ext::base::ExtCtxt<'_>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, syntax::tokenstream::TokenStream) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
---
[01:02:54] thread '[run-pass] run-pass-fulldeps/plugin-args-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:02:54]
[01:02:54] ---- [run-pass] run-pass-fulldeps/plugin-args-3.rs stdout ----
[01:02:54]
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-3.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-3.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54] error[E0050]: method `expand` has 4 parameters but the declaration in trait `syntax::ext::base::TTMacroExpander::expand` has 5
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:40:23
[01:02:54]    |
[01:02:54] 40 |                    _: TokenStream) -> Box<MacResult+'cx> {
[01:02:54]    |                       ^^^^^^^^^^^ expected 5 parameters, found 4
[01:02:54]    |
[01:02:54]    = note: `expand` from trait: `fn(&Self, &'cx mut syntax::ext::base::ExtCtxt<'_>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, syntax::tokenstream::TokenStream) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
---
[01:02:54] thread '[run-pass] run-pass-fulldeps/plugin-args-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:02:54]
[01:02:54] ---- [run-pass] run-pass-fulldeps/plugin-lib-ok-in-plugin.rs stdout ----
[01:02:54]
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-lib-ok-in-plugin.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-lib-ok-in-plugin.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:02:54]    |
[01:02:54] 33 | macro_rules! unexported_macro { () => (3) }
[01:02:54]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:02:54]    |
[01:02:54]    = note: #[warn(unused_macros)] on by default
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:37:36
[01:02:54]    |
[01:02:54] 37 |     reg.register_macro("make_a_1", expand_make_a_1);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_make_a_1}`
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:38:36
[01:02:54]    |
[01:02:54] 38 |     reg.register_macro("identity", expand_identity);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_identity}`
---
[01:02:54] thread '[run-pass] run-pass-fulldeps/plugin-lib-ok-in-plugin.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:02:54]
[01:02:54] ---- [run-pass] run-pass-fulldeps/plugin-plus-extern-crate.rs stdout ----
[01:02:54]
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-plus-extern-crate.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-plus-extern-crate.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1
[01:02:54]    |
[01:02:54] 33 | macro_rules! unexported_macro { () => (3) }
[01:02:54]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:02:54]    |
[01:02:54]    = note: #[warn(unused_macros)] on by default
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:37:36
[01:02:54]    |
[01:02:54] 37 |     reg.register_macro("make_a_1", expand_make_a_1);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_make_a_1}`
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:38:36
[01:02:54]    |
[01:02:54] 38 |     reg.register_macro("identity", expand_identity);
[01:02:54]    |                                    ^^^^^^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_identity}`
---
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/roman_numerals.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/roman_numerals.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/roman-numerals-macro.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/roman-numerals-macro.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/roman_numerals.rs:78:30
[01:02:54]    |
[01:02:54] 78 |     reg.register_macro("rn", expand_rn);
[01:02:54]    |                              ^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_rn}`
---
[01:02:54] thread '[run-pass] run-pass-fulldeps/roman-numerals-macro.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[01:02:54]
[01:02:54] ---- [run-pass] run-pass-fulldeps/syntax-extension-with-dll-deps.rs stdout ----
[01:02:54]
[01:02:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/syntax_extension_with_dll_deps_2.rs" failed to compile:
[01:02:54] status: exit code: 101
[01:02:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/syntax_extension_with_dll_deps_2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/syntax-extension-with-dll-deps.stage2-x86_64-unknown-linux-gnu.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/syntax-extension-with-dll-deps.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:02:54] warning: unused imports: `Item`, `MetaItem`
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/syntax_extension_with_dll_deps_2.rs:22:19
[01:02:54]    |
[01:02:54] 22 | use syntax::ast::{Item, MetaItem};
[01:02:54]    |                   ^^^^  ^^^^^^^^
[01:02:54]    |
[01:02:54]    = note: #[warn(unused_imports)] on by default
[01:02:54]
[01:02:54] error[E0308]: mismatched types
[01:02:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/syntax_extension_with_dll_deps_2.rs:30:31
[01:02:54]    |
[01:02:54] 30 |     reg.register_macro("foo", expand_foo);
[01:02:54]    |                               ^^^^^^^^^^ incorrect number of function parameters
[01:02:54]    |
[01:02:54]    = note: expected type `for<'cx, 'r, 's> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, &'cx std::option::Option<syntax::ast::Path>, syntax_pos::Span, &'s [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'cx>`
[01:02:54]               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<syntax::ext::base::MacResult + 'static> {expand_foo}`
---
[01:02:54] thread '[run-pass] run-pass-fulldeps/syntax-extension-with-dll-deps.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
---
[01:02:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:54] expected success, got: exit code: 101
[01:02:54]
[01:02:54]
[01:02:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:54] Build completed unsuccessfully in 0:21:56
[01:02:54] Makefile:58: recipe for target 'check' failed
[01:02:54] make: *** [check] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:03ccbb38:start=1523315184471642173,finish=1523315184487260160,duration=15617987
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:063c4da6
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:063c4da6:start=1523315184496120400,finish=1523315184504741524,duration=8621124
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0efe36f8
$ dmesg | grep -i kill
[   11.185574] init: failsafe main process (1093) killed by TERM signal
