plain
running 15019 tests
..........................................ii............................................    88/15019
...................................................................................iiiii   176/15019
iiiiiiiiii.....................i..................i.....................................   264/15019
..............................F.....................F...................................   352/15019
..............................FF........................................................   440/15019
........................................................................................   616/15019
........................................................................................   704/15019
........................................................................................   792/15019
........................................................................................   880/15019
---
failures:

---- [ui] tests/ui/associated-consts/issue-105330.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-consts/issue-105330.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-105330" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-105330/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected one of `!` or `::`, found `A`
  --> fake-test-src-base/associated-consts/issue-105330.rs:8:14
   |
LL | impl TraitWAssocConst for impl Demo { //~ ERROR E0404
   |                                     - while parsing this item list starting here
LL |     //~^ ERROR E0562
LL |     pubconst A: str = 32; //~ ERROR expected one of
   |              ^ expected one of `!` or `::`
LL | }
   | - the item list ends here
error[E0404]: expected trait, found struct `Demo`
  --> fake-test-src-base/associated-consts/issue-105330.rs:6:32
   |
   |
LL | impl TraitWAssocConst for impl Demo { //~ ERROR E0404

error[E0658]: associated const equality is incomplete
  --> fake-test-src-base/associated-consts/issue-105330.rs:11:28
   |
   |
LL | fn foo<A: TraitWAssocConst<A=32>>() { //~ ERROR E0658
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
error[E0658]: associated const equality is incomplete
  --> fake-test-src-base/associated-consts/issue-105330.rs:17:29
   |
   |
LL | fn main<A: TraitWAssocConst<A=32>>() { //~ ERROR E0131
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
Build completed unsuccessfully in 0:12:24
Build completed unsuccessfully in 0:12:24
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable

error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in impl headers
  --> fake-test-src-base/associated-consts/issue-105330.rs:6:27
   |
LL | impl TraitWAssocConst for impl Demo { //~ ERROR E0404


error[E0277]: the trait bound `Demo: TraitWAssocConst` is not satisfied
  --> fake-test-src-base/associated-consts/issue-105330.rs:12:11
   |
LL |     foo::<Demo>()(); //~ ERROR E0271
   |           ^^^^ the trait `TraitWAssocConst` is not implemented for `Demo`
note: required by a bound in `foo`
  --> fake-test-src-base/associated-consts/issue-105330.rs:11:11
   |
   |
LL | fn foo<A: TraitWAssocConst<A=32>>() { //~ ERROR E0658
   |           ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `foo`

error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:1234:21: unexpected DefKind in AliasTy: AssocConst
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:     0x7f809dbd6494 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha3c226363dc7f2a7
   1:     0x7f809dc3d148 - core::fmt::write::h57859af84dfe13ec
   1:     0x7f809dc3d148 - core::fmt::write::h57859af84dfe13ec
   2:     0x7f809dbcaba1 - std::io::Write::write_fmt::h671743467e8a4e4f
   3:     0x7f809dbd62a1 - std::sys_common::backtrace::print::hc72bffc6192a421c
   4:     0x7f809dbd941a - std::panicking::default_hook::{{closure}}::he0d7258ad98f6ac4
   5:     0x7f809dbd90fc - std::panicking::default_hook::h99295760cbd95b6c
   6:     0x7f809e6861fb - rustc_driver_impl[63ab30e802b86487]::install_ice_hook::{closure#0}
   7:     0x7f809dbd9b27 - std::panicking::rust_panic_with_hook::h33ebcf184eb7cbc6
   8:     0x7f80a12ec5f3 - std[46087f1e662846f8]::panicking::begin_panic::<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>::{closure#0}
   9:     0x7f80a12e83b6 - std[46087f1e662846f8]::sys_common::backtrace::__rust_end_short_backtrace::<std[46087f1e662846f8]::panicking::begin_panic<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>::{closure#0}, !>
  10:     0x7f809e61e856 - std[46087f1e662846f8]::panicking::begin_panic::<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>
  11:     0x7f80a131f2d7 - <rustc_errors[6d6bacf36537e5a7]::HandlerInner>::bug::<alloc[9a03455bde5eff86]::string::String>
  12:     0x7f80a131f029 - <rustc_errors[6d6bacf36537e5a7]::Handler>::bug::<alloc[9a03455bde5eff86]::string::String>
  13:     0x7f80a14774c7 - rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt::<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}
  14:     0x7f80a1475f7c - rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_opt::<rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f80a1475f04 - rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_context_opt::<rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_opt<rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f809e627f62 - rustc_middle[ec06c754c4392f9d]::util::bug::bug_fmt
  17:     0x7f80a138088c - <rustc_middle[ec06c754c4392f9d]::ty::sty::AliasTy>::kind
  18:     0x7f80a0db1a2e - <rustc_middle[ec06c754c4392f9d]::ty::sty::AliasTy as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  19:     0x7f80a0f88acd - <rustc_middle[ec06c754c4392f9d]::ty::ProjectionPredicate as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  20:     0x7f80a0f892b6 - <rustc_middle[ec06c754c4392f9d]::ty::PredicateKind as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  21:     0x7f80a0dd9548 - <rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter as rustc_middle[ec06c754c4392f9d]::ty::print::pretty::PrettyPrinter>::in_binder::<rustc_middle[ec06c754c4392f9d]::ty::PredicateKind>
  22:     0x7f80a0fb458e - <rustc_infer[535559aeccef3cf9]::infer::InferCtxt>::probe::<(), <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_projection_error::{closure#0}>
  23:     0x7f80a0e57756 - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_projection_error
  24:     0x7f80a0e57015 - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  25:     0x7f80a0e44e1b - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  26:     0x7f809eb88ae3 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_argument_types
  27:     0x7f809eb57965 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::confirm_builtin_call
  28:     0x7f809eb56530 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_call
  29:     0x7f809ebd864f - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_kind
  30:     0x7f809eb70144 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  31:     0x7f809ebd7032 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  32:     0x7f809eb5582a - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_call
  33:     0x7f809ebd864f - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_kind
  34:     0x7f809eb70144 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  35:     0x7f809ebd7032 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  36:     0x7f809eb9139a - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_stmt
  37:     0x7f809eb918f5 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_block_with_expected
  38:     0x7f809ebd8212 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_kind
  39:     0x7f809eb70144 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  40:     0x7f809ebd7032 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  41:     0x7f809eb71fed - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_return_expr
  42:     0x7f809ed52664 - rustc_hir_typeck[f8516401557f8d10]::check::check_fn
  43:     0x7f809ec6181c - rustc_hir_typeck[f8516401557f8d10]::typeck
  44:     0x7f80a0353288 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  45:     0x7f80a014428a - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::typeck
  46:     0x7f809ec4b649 - rustc_middle[ec06c754c4392f9d]::ty::query::query_get_at::<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>>
  47:     0x7f809ec611d2 - rustc_hir_typeck[f8516401557f8d10]::used_trait_imports
  48:     0x7f80a0353288 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  49:     0x7f80a0144faa - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::used_trait_imports
  50:     0x7f809ef4e039 - rustc_middle[ec06c754c4392f9d]::ty::query::query_get_at::<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>>
  51:     0x7f809ef6455f - rustc_hir_analysis[fa67bc547203e004]::check_crate
  52:     0x7f809e77a1e9 - rustc_interface[91d420b75f0e655f]::passes::analysis
  53:     0x7f80a0428087 - <rustc_query_impl[4cfdfbf748ec14c6]::dynamic_query::analysis::{closure#2} as core[8dfb209407a6be03]::ops::function::FnOnce<(rustc_middle[ec06c754c4392f9d]::ty::context::TyCtxt, ())>>::call_once
  54:     0x7f80a02c4fe2 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::SingleCache<rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  55:     0x7f80a0120566 - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::analysis
  56:     0x7f809e6a285d - <rustc_middle[ec06c754c4392f9d]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  57:     0x7f809e6d8848 - <rustc_interface[91d420b75f0e655f]::interface::Compiler>::enter::<rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}::{closure#2}, core[8dfb209407a6be03]::result::Result<core[8dfb209407a6be03]::option::Option<rustc_interface[91d420b75f0e655f]::queries::Linker>, rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  58:     0x7f809e6a14b0 - rustc_span[363adaf74270032a]::set_source_map::<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  59:     0x7f809e693ab9 - <scoped_tls[36523a48917882f7]::ScopedKey<rustc_span[363adaf74270032a]::SessionGlobals>>::set::<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  60:     0x7f809e6ab946 - std[46087f1e662846f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  61:     0x7f809e6d9046 - std[46087f1e662846f8]::panicking::try::<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, core[8dfb209407a6be03]::panic::unwind_safe::AssertUnwindSafe<<std[46087f1e662846f8]::thread::Builder>::spawn_unchecked_<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  62:     0x7f809e699c8d - <<std[46087f1e662846f8]::thread::Builder>::spawn_unchecked_<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#1} as core[8dfb209407a6be03]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  63:     0x7f809dbe645e - std::sys::unix::thread::Thread::new::thread_start::he160ce89251d709f
  64:     0x7f809d981b43 - <unknown>
  65:     0x7f809da13a00 - <unknown>
  66:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (88f831626 2023-05-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `foo`
#1 [used_trait_imports] finding used_trait_imports `foo`
#2 [analysis] running analysis passes on this crate
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0277, E0404, E0562, E0658.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] tests/ui/associated-consts/projection-unspecified-but-bounded.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-consts/projection-unspecified-but-bounded.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/projection-unspecified-but-bounded" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/projection-unspecified-but-bounded/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:1234:21: unexpected DefKind in AliasTy: AssocConst
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:     0x7fc6607d5494 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha3c226363dc7f2a7
   1:     0x7fc66083c148 - core::fmt::write::h57859af84dfe13ec
   1:     0x7fc66083c148 - core::fmt::write::h57859af84dfe13ec
   2:     0x7fc6607c9ba1 - std::io::Write::write_fmt::h671743467e8a4e4f
   3:     0x7fc6607d52a1 - std::sys_common::backtrace::print::hc72bffc6192a421c
   4:     0x7fc6607d841a - std::panicking::default_hook::{{closure}}::he0d7258ad98f6ac4
   5:     0x7fc6607d80fc - std::panicking::default_hook::h99295760cbd95b6c
   6:     0x7fc6612851fb - rustc_driver_impl[63ab30e802b86487]::install_ice_hook::{closure#0}
   7:     0x7fc6607d8b27 - std::panicking::rust_panic_with_hook::h33ebcf184eb7cbc6
   8:     0x7fc663eeb5f3 - std[46087f1e662846f8]::panicking::begin_panic::<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>::{closure#0}
   9:     0x7fc663ee73b6 - std[46087f1e662846f8]::sys_common::backtrace::__rust_end_short_backtrace::<std[46087f1e662846f8]::panicking::begin_panic<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>::{closure#0}, !>
  10:     0x7fc66121d856 - std[46087f1e662846f8]::panicking::begin_panic::<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>
  11:     0x7fc663f1e2d7 - <rustc_errors[6d6bacf36537e5a7]::HandlerInner>::bug::<alloc[9a03455bde5eff86]::string::String>
  12:     0x7fc663f1e029 - <rustc_errors[6d6bacf36537e5a7]::Handler>::bug::<alloc[9a03455bde5eff86]::string::String>
  13:     0x7fc6640764c7 - rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt::<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}
  14:     0x7fc664074f7c - rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_opt::<rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7fc664074f04 - rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_context_opt::<rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_opt<rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7fc661226f62 - rustc_middle[ec06c754c4392f9d]::util::bug::bug_fmt
  17:     0x7fc663f7f88c - <rustc_middle[ec06c754c4392f9d]::ty::sty::AliasTy>::kind
  18:     0x7fc6639b0a2e - <rustc_middle[ec06c754c4392f9d]::ty::sty::AliasTy as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  19:     0x7fc663b87acd - <rustc_middle[ec06c754c4392f9d]::ty::ProjectionPredicate as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  20:     0x7fc663b882b6 - <rustc_middle[ec06c754c4392f9d]::ty::PredicateKind as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  21:     0x7fc6639d8548 - <rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter as rustc_middle[ec06c754c4392f9d]::ty::print::pretty::PrettyPrinter>::in_binder::<rustc_middle[ec06c754c4392f9d]::ty::PredicateKind>
  22:     0x7fc663bb358e - <rustc_infer[535559aeccef3cf9]::infer::InferCtxt>::probe::<(), <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_projection_error::{closure#0}>
  23:     0x7fc663a56756 - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_projection_error
  24:     0x7fc663a56015 - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  25:     0x7fc663a43e1b - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  26:     0x7fc661787ae3 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_argument_types
  27:     0x7fc661756965 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::confirm_builtin_call
  28:     0x7fc661755530 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_call
  29:     0x7fc6617d764f - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_kind
  30:     0x7fc66176f144 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  31:     0x7fc6617d6032 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  32:     0x7fc66179039a - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_stmt
  33:     0x7fc6617908f5 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_block_with_expected
  34:     0x7fc6617d7212 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_kind
  35:     0x7fc66176f144 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  36:     0x7fc6617d6032 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  37:     0x7fc661770fed - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_return_expr
  38:     0x7fc661951664 - rustc_hir_typeck[f8516401557f8d10]::check::check_fn
  39:     0x7fc66186081c - rustc_hir_typeck[f8516401557f8d10]::typeck
  40:     0x7fc662f52288 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  41:     0x7fc662d4328a - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::typeck
  42:     0x7fc66184a649 - rustc_middle[ec06c754c4392f9d]::ty::query::query_get_at::<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>>
  43:     0x7fc6618601d2 - rustc_hir_typeck[f8516401557f8d10]::used_trait_imports
  44:     0x7fc662f52288 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  45:     0x7fc662d43faa - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::used_trait_imports
  46:     0x7fc661b4d039 - rustc_middle[ec06c754c4392f9d]::ty::query::query_get_at::<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>>
  47:     0x7fc661b6355f - rustc_hir_analysis[fa67bc547203e004]::check_crate
  48:     0x7fc6613791e9 - rustc_interface[91d420b75f0e655f]::passes::analysis
  49:     0x7fc663027087 - <rustc_query_impl[4cfdfbf748ec14c6]::dynamic_query::analysis::{closure#2} as core[8dfb209407a6be03]::ops::function::FnOnce<(rustc_middle[ec06c754c4392f9d]::ty::context::TyCtxt, ())>>::call_once
  50:     0x7fc662ec3fe2 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::SingleCache<rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  51:     0x7fc662d1f566 - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::analysis
  52:     0x7fc6612a185d - <rustc_middle[ec06c754c4392f9d]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  53:     0x7fc6612d7848 - <rustc_interface[91d420b75f0e655f]::interface::Compiler>::enter::<rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}::{closure#2}, core[8dfb209407a6be03]::result::Result<core[8dfb209407a6be03]::option::Option<rustc_interface[91d420b75f0e655f]::queries::Linker>, rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  54:     0x7fc6612a04b0 - rustc_span[363adaf74270032a]::set_source_map::<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  55:     0x7fc661292ab9 - <scoped_tls[36523a48917882f7]::ScopedKey<rustc_span[363adaf74270032a]::SessionGlobals>>::set::<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  56:     0x7fc6612aa946 - std[46087f1e662846f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  57:     0x7fc6612d8046 - std[46087f1e662846f8]::panicking::try::<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, core[8dfb209407a6be03]::panic::unwind_safe::AssertUnwindSafe<<std[46087f1e662846f8]::thread::Builder>::spawn_unchecked_<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  58:     0x7fc661298c8d - <<std[46087f1e662846f8]::thread::Builder>::spawn_unchecked_<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#1} as core[8dfb209407a6be03]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  59:     0x7fc6607e545e - std::sys::unix::thread::Thread::new::thread_start::he160ce89251d709f
  60:     0x7fc660580b43 - <unknown>
  61:     0x7fc660612a00 - <unknown>
  62:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (88f831626 2023-05-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `bar`
#1 [used_trait_imports] finding used_trait_imports `bar`
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/associated-type-bounds/const-projection-err.rs#stock stdout ----

error in revision `stock`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-type-bounds/const-projection-err.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/const-projection-err.stock" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/const-projection-err.stock/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:1234:21: unexpected DefKind in AliasTy: AssocConst
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:     0x7f1c438a4494 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha3c226363dc7f2a7
   1:     0x7f1c4390b148 - core::fmt::write::h57859af84dfe13ec
   1:     0x7f1c4390b148 - core::fmt::write::h57859af84dfe13ec
   2:     0x7f1c43898ba1 - std::io::Write::write_fmt::h671743467e8a4e4f
   3:     0x7f1c438a42a1 - std::sys_common::backtrace::print::hc72bffc6192a421c
   4:     0x7f1c438a741a - std::panicking::default_hook::{{closure}}::he0d7258ad98f6ac4
   5:     0x7f1c438a70fc - std::panicking::default_hook::h99295760cbd95b6c
   6:     0x7f1c443541fb - rustc_driver_impl[63ab30e802b86487]::install_ice_hook::{closure#0}
   7:     0x7f1c438a7b27 - std::panicking::rust_panic_with_hook::h33ebcf184eb7cbc6
   8:     0x7f1c46fba5f3 - std[46087f1e662846f8]::panicking::begin_panic::<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>::{closure#0}
   9:     0x7f1c46fb63b6 - std[46087f1e662846f8]::sys_common::backtrace::__rust_end_short_backtrace::<std[46087f1e662846f8]::panicking::begin_panic<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>::{closure#0}, !>
  10:     0x7f1c442ec856 - std[46087f1e662846f8]::panicking::begin_panic::<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>
  11:     0x7f1c46fed2d7 - <rustc_errors[6d6bacf36537e5a7]::HandlerInner>::bug::<alloc[9a03455bde5eff86]::string::String>
  12:     0x7f1c46fed029 - <rustc_errors[6d6bacf36537e5a7]::Handler>::bug::<alloc[9a03455bde5eff86]::string::String>
  13:     0x7f1c471454c7 - rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt::<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}
  14:     0x7f1c47143f7c - rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_opt::<rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f1c47143f04 - rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_context_opt::<rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_opt<rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f1c442f5f62 - rustc_middle[ec06c754c4392f9d]::util::bug::bug_fmt
  17:     0x7f1c4704e88c - <rustc_middle[ec06c754c4392f9d]::ty::sty::AliasTy>::kind
  18:     0x7f1c46a7fa2e - <rustc_middle[ec06c754c4392f9d]::ty::sty::AliasTy as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  19:     0x7f1c46c56acd - <rustc_middle[ec06c754c4392f9d]::ty::ProjectionPredicate as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  20:     0x7f1c46c572b6 - <rustc_middle[ec06c754c4392f9d]::ty::PredicateKind as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  21:     0x7f1c46aa7548 - <rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter as rustc_middle[ec06c754c4392f9d]::ty::print::pretty::PrettyPrinter>::in_binder::<rustc_middle[ec06c754c4392f9d]::ty::PredicateKind>
  22:     0x7f1c46c8258e - <rustc_infer[535559aeccef3cf9]::infer::InferCtxt>::probe::<(), <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_projection_error::{closure#0}>
  23:     0x7f1c46b25756 - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_projection_error
  24:     0x7f1c46b25015 - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  25:     0x7f1c46b12e1b - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  26:     0x7f1c44856ae3 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_argument_types
  27:     0x7f1c44825965 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::confirm_builtin_call
  28:     0x7f1c44824530 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_call
  29:     0x7f1c448a664f - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_kind
  30:     0x7f1c4483e144 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  31:     0x7f1c448a5032 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  32:     0x7f1c4485f39a - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_stmt
  33:     0x7f1c4485f8f5 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_block_with_expected
  34:     0x7f1c448a6212 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_kind
  35:     0x7f1c4483e144 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  36:     0x7f1c448a5032 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  37:     0x7f1c4483ffed - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_return_expr
  38:     0x7f1c44a20664 - rustc_hir_typeck[f8516401557f8d10]::check::check_fn
  39:     0x7f1c4492f81c - rustc_hir_typeck[f8516401557f8d10]::typeck
  40:     0x7f1c46021288 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  41:     0x7f1c45e1228a - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::typeck
  42:     0x7f1c44919649 - rustc_middle[ec06c754c4392f9d]::ty::query::query_get_at::<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>>
  43:     0x7f1c4492f1d2 - rustc_hir_typeck[f8516401557f8d10]::used_trait_imports
  44:     0x7f1c46021288 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  45:     0x7f1c45e12faa - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::used_trait_imports
  46:     0x7f1c44c1c039 - rustc_middle[ec06c754c4392f9d]::ty::query::query_get_at::<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>>
  47:     0x7f1c44c3255f - rustc_hir_analysis[fa67bc547203e004]::check_crate
  48:     0x7f1c444481e9 - rustc_interface[91d420b75f0e655f]::passes::analysis
  49:     0x7f1c460f6087 - <rustc_query_impl[4cfdfbf748ec14c6]::dynamic_query::analysis::{closure#2} as core[8dfb209407a6be03]::ops::function::FnOnce<(rustc_middle[ec06c754c4392f9d]::ty::context::TyCtxt, ())>>::call_once
  50:     0x7f1c45f92fe2 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::SingleCache<rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  51:     0x7f1c45dee566 - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::analysis
  52:     0x7f1c4437085d - <rustc_middle[ec06c754c4392f9d]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  53:     0x7f1c443a6848 - <rustc_interface[91d420b75f0e655f]::interface::Compiler>::enter::<rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}::{closure#2}, core[8dfb209407a6be03]::result::Result<core[8dfb209407a6be03]::option::Option<rustc_interface[91d420b75f0e655f]::queries::Linker>, rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  54:     0x7f1c4436f4b0 - rustc_span[363adaf74270032a]::set_source_map::<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  55:     0x7f1c44361ab9 - <scoped_tls[36523a48917882f7]::ScopedKey<rustc_span[363adaf74270032a]::SessionGlobals>>::set::<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  56:     0x7f1c44379946 - std[46087f1e662846f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  57:     0x7f1c443a7046 - std[46087f1e662846f8]::panicking::try::<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, core[8dfb209407a6be03]::panic::unwind_safe::AssertUnwindSafe<<std[46087f1e662846f8]::thread::Builder>::spawn_unchecked_<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  58:     0x7f1c44367c8d - <<std[46087f1e662846f8]::thread::Builder>::spawn_unchecked_<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#1} as core[8dfb209407a6be03]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  59:     0x7f1c438b445e - std::sys::unix::thread::Thread::new::thread_start::he160ce89251d709f
  60:     0x7f1c4364fb43 - <unknown>
  61:     0x7f1c436e1a00 - <unknown>
  62:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (88f831626 2023-05-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `bar`
#1 [used_trait_imports] finding used_trait_imports `bar`
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/associated-type-bounds/const-projection-err.rs#gce stdout ----

error in revision `gce`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-type-bounds/const-projection-err.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "gce" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/const-projection-err.gce" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/const-projection-err.gce/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/associated-type-bounds/const-projection-err.rs:4:26
   |
LL | #![cfg_attr(gce, feature(generic_const_exprs))]
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = note: `#[warn(incomplete_features)]` on by default


error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:1234:21: unexpected DefKind in AliasTy: AssocConst
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:     0x7f4090fd1494 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha3c226363dc7f2a7
   1:     0x7f4091038148 - core::fmt::write::h57859af84dfe13ec
   1:     0x7f4091038148 - core::fmt::write::h57859af84dfe13ec
   2:     0x7f4090fc5ba1 - std::io::Write::write_fmt::h671743467e8a4e4f
   3:     0x7f4090fd12a1 - std::sys_common::backtrace::print::hc72bffc6192a421c
   4:     0x7f4090fd441a - std::panicking::default_hook::{{closure}}::he0d7258ad98f6ac4
   5:     0x7f4090fd40fc - std::panicking::default_hook::h99295760cbd95b6c
   6:     0x7f4091a811fb - rustc_driver_impl[63ab30e802b86487]::install_ice_hook::{closure#0}
   7:     0x7f4090fd4b27 - std::panicking::rust_panic_with_hook::h33ebcf184eb7cbc6
   8:     0x7f40946e75f3 - std[46087f1e662846f8]::panicking::begin_panic::<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>::{closure#0}
   9:     0x7f40946e33b6 - std[46087f1e662846f8]::sys_common::backtrace::__rust_end_short_backtrace::<std[46087f1e662846f8]::panicking::begin_panic<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>::{closure#0}, !>
  10:     0x7f4091a19856 - std[46087f1e662846f8]::panicking::begin_panic::<rustc_errors[6d6bacf36537e5a7]::ExplicitBug>
  11:     0x7f409471a2d7 - <rustc_errors[6d6bacf36537e5a7]::HandlerInner>::bug::<alloc[9a03455bde5eff86]::string::String>
  12:     0x7f409471a029 - <rustc_errors[6d6bacf36537e5a7]::Handler>::bug::<alloc[9a03455bde5eff86]::string::String>
  13:     0x7f40948724c7 - rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt::<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}
  14:     0x7f4094870f7c - rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_opt::<rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f4094870f04 - rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_context_opt::<rustc_middle[ec06c754c4392f9d]::ty::context::tls::with_opt<rustc_middle[ec06c754c4392f9d]::util::bug::opt_span_bug_fmt<rustc_span[363adaf74270032a]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f4091a22f62 - rustc_middle[ec06c754c4392f9d]::util::bug::bug_fmt
  17:     0x7f409477b88c - <rustc_middle[ec06c754c4392f9d]::ty::sty::AliasTy>::kind
  18:     0x7f40941aca2e - <rustc_middle[ec06c754c4392f9d]::ty::sty::AliasTy as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  19:     0x7f4094383acd - <rustc_middle[ec06c754c4392f9d]::ty::ProjectionPredicate as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  20:     0x7f40943842b6 - <rustc_middle[ec06c754c4392f9d]::ty::PredicateKind as rustc_middle[ec06c754c4392f9d]::ty::print::Print<rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter>>::print
  21:     0x7f40941d4548 - <rustc_middle[ec06c754c4392f9d]::ty::print::pretty::FmtPrinter as rustc_middle[ec06c754c4392f9d]::ty::print::pretty::PrettyPrinter>::in_binder::<rustc_middle[ec06c754c4392f9d]::ty::PredicateKind>
  22:     0x7f40943af58e - <rustc_infer[535559aeccef3cf9]::infer::InferCtxt>::probe::<(), <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_projection_error::{closure#0}>
  23:     0x7f4094252756 - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_projection_error
  24:     0x7f4094252015 - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  25:     0x7f409423fe1b - <rustc_infer[535559aeccef3cf9]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1d0a803fa0be77f7]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  26:     0x7f4091f83ae3 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_argument_types
  27:     0x7f4091f52965 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::confirm_builtin_call
  28:     0x7f4091f51530 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_call
  29:     0x7f4091fd364f - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_kind
  30:     0x7f4091f6b144 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  31:     0x7f4091fd2032 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  32:     0x7f4091f8c39a - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_stmt
  33:     0x7f4091f8c8f5 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_block_with_expected
  34:     0x7f4091fd3212 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_kind
  35:     0x7f4091f6b144 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  36:     0x7f4091fd2032 - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  37:     0x7f4091f6cfed - <rustc_hir_typeck[f8516401557f8d10]::fn_ctxt::FnCtxt>::check_return_expr
  38:     0x7f409214d664 - rustc_hir_typeck[f8516401557f8d10]::check::check_fn
  39:     0x7f409205c81c - rustc_hir_typeck[f8516401557f8d10]::typeck
  40:     0x7f409374e288 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  41:     0x7f409353f28a - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::typeck
  42:     0x7f4092046649 - rustc_middle[ec06c754c4392f9d]::ty::query::query_get_at::<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>>
  43:     0x7f409205c1d2 - rustc_hir_typeck[f8516401557f8d10]::used_trait_imports
  44:     0x7f409374e288 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  45:     0x7f409353ffaa - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::used_trait_imports
  46:     0x7f4092349039 - rustc_middle[ec06c754c4392f9d]::ty::query::query_get_at::<rustc_query_system[56dc6edff29f1586]::query::caches::VecCache<rustc_span[363adaf74270032a]::def_id::LocalDefId, rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 8usize]>>>
  47:     0x7f409235f55f - rustc_hir_analysis[fa67bc547203e004]::check_crate
  48:     0x7f4091b751e9 - rustc_interface[91d420b75f0e655f]::passes::analysis
  49:     0x7f4093823087 - <rustc_query_impl[4cfdfbf748ec14c6]::dynamic_query::analysis::{closure#2} as core[8dfb209407a6be03]::ops::function::FnOnce<(rustc_middle[ec06c754c4392f9d]::ty::context::TyCtxt, ())>>::call_once
  50:     0x7f40936bffe2 - rustc_query_system[56dc6edff29f1586]::query::plumbing::try_execute_query::<rustc_query_impl[4cfdfbf748ec14c6]::DynamicConfig<rustc_query_system[56dc6edff29f1586]::query::caches::SingleCache<rustc_middle[ec06c754c4392f9d]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[4cfdfbf748ec14c6]::plumbing::QueryCtxt, false>
  51:     0x7f409351b566 - rustc_query_impl[4cfdfbf748ec14c6]::get_query_non_incr::analysis
  52:     0x7f4091a9d85d - <rustc_middle[ec06c754c4392f9d]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  53:     0x7f4091ad3848 - <rustc_interface[91d420b75f0e655f]::interface::Compiler>::enter::<rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}::{closure#2}, core[8dfb209407a6be03]::result::Result<core[8dfb209407a6be03]::option::Option<rustc_interface[91d420b75f0e655f]::queries::Linker>, rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  54:     0x7f4091a9c4b0 - rustc_span[363adaf74270032a]::set_source_map::<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  55:     0x7f4091a8eab9 - <scoped_tls[36523a48917882f7]::ScopedKey<rustc_span[363adaf74270032a]::SessionGlobals>>::set::<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  56:     0x7f4091aa6946 - std[46087f1e662846f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>
  57:     0x7f4091ad4046 - std[46087f1e662846f8]::panicking::try::<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, core[8dfb209407a6be03]::panic::unwind_safe::AssertUnwindSafe<<std[46087f1e662846f8]::thread::Builder>::spawn_unchecked_<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  58:     0x7f4091a94c8d - <<std[46087f1e662846f8]::thread::Builder>::spawn_unchecked_<rustc_interface[91d420b75f0e655f]::util::run_in_thread_pool_with_globals<rustc_interface[91d420b75f0e655f]::interface::run_compiler<core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>, rustc_driver_impl[63ab30e802b86487]::run_compiler::{closure#1}>::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8dfb209407a6be03]::result::Result<(), rustc_span[363adaf74270032a]::ErrorGuaranteed>>::{closure#1} as core[8dfb209407a6be03]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  59:     0x7f4090fe145e - std::sys::unix::thread::Thread::new::thread_start::he160ce89251d709f
  60:     0x7f4090d7cb43 - <unknown>
  61:     0x7f4090e0ea00 - <unknown>
  62:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (88f831626 2023-05-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `bar`
#1 [used_trait_imports] finding used_trait_imports `bar`
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error; 1 warning emitted
------------------------------------------


