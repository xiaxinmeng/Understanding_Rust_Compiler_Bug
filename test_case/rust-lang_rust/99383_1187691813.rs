plain
---- [ui] src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs stdout ----
diff of stderr:

- error[E0308]: mismatched types
+ error[E0271]: type mismatch resolving `<Concrete as Bar>::Other == Concrete`
3    |
4 LL | type Tait = impl Sized;

-    |             ---------- the expected opaque type
-    |             ---------- the expected opaque type
+    |             ---------- the found opaque type
6 ...
7 LL |     type Item = Concrete;
-    |                 ^^^^^^^^ types differ
+    |                 ^^^^^^^^ type mismatch resolving `<Concrete as Bar>::Other == Concrete`
-    = note: expected opaque type `Tait`
-                    found struct `Concrete`
-                    found struct `Concrete`
+ note: expected this to be `Concrete`
+    |
+    |
+ LL |     type Other = Tait;
+    = note:   expected struct `Concrete`
+            found opaque type `Tait`
+            found opaque type `Tait`
+ note: required by a bound in `Foo::Item`
+    |
+    |
+ LL |     type Item: Bar<Other = Self>;
+    |                    ^^^^^^^^^^^^ required by this bound in `Foo::Item`
13 error: aborting due to previous error
14 

- For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args impl-trait/issues/issue-99348-impl-compatibility.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-99348-impl-compatibility" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-99348-impl-compatibility/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `<Concrete as Bar>::Other == Concrete`
   |
LL | type Tait = impl Sized;
   |             ---------- the found opaque type
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...
LL |     type Item = Concrete;
   |                 ^^^^^^^^ type mismatch resolving `<Concrete as Bar>::Other == Concrete`
note: expected this to be `Concrete`
  --> /checkout/src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs:13:18
   |
   |
LL |     type Other = Tait;
   = note:   expected struct `Concrete`
           found opaque type `Tait`
note: required by a bound in `Foo::Item`
  --> /checkout/src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs:17:20
  --> /checkout/src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs:17:20
   |
LL |     type Item: Bar<Other = Self>;
   |                    ^^^^^^^^^^^^ required by this bound in `Foo::Item`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/issue-96572-unconstrained-upvar-2021.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-96572-unconstrained-upvar-2021.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-96572-unconstrained-upvar-2021" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-96572-unconstrained-upvar-2021/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/closure.rs:185:25: Unexpected type Opaque(DefId(0:5 ~ issue_96572_unconstrained_upvar_2021[acc1]::main::T::{opaque#0}), []) for `Field` projection
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
   0:     0x7f665f29638c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he3e8bda708a19a9f
   1:     0x7f665f2fcef8 - core::fmt::write::ha13b60526f68606a
   1:     0x7f665f2fcef8 - core::fmt::write::ha13b60526f68606a
   2:     0x7f665f286531 - std::io::Write::write_fmt::hbda2db30b7cc30ac
   3:     0x7f665f29932e - std::panicking::default_hook::{{closure}}::h560dad6840a8e7ea
   4:     0x7f665f298ff6 - std::panicking::default_hook::h34041a7941db9f5c
   5:     0x7f665fc64fe4 - rustc_driver[fd89767628f3c01e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f665f299ae2 - std::panicking::rust_panic_with_hook::hbc815e57be5cd56c
   7:     0x7f666263fac3 - std[da272c3ef198aa80]::panicking::begin_panic::<rustc_errors[dc2ff1671bbffc75]::ExplicitBug>::{closure#0}
   8:     0x7f666263d716 - std[da272c3ef198aa80]::sys_common::backtrace::__rust_end_short_backtrace::<std[da272c3ef198aa80]::panicking::begin_panic<rustc_errors[dc2ff1671bbffc75]::ExplicitBug>::{closure#0}, !>
   9:     0x7f665fbfbaf6 - std[da272c3ef198aa80]::panicking::begin_panic::<rustc_errors[dc2ff1671bbffc75]::ExplicitBug>
  10:     0x7f6662637ef6 - std[da272c3ef198aa80]::panic::panic_any::<rustc_errors[dc2ff1671bbffc75]::ExplicitBug>
  11:     0x7f66626348b6 - <rustc_errors[dc2ff1671bbffc75]::HandlerInner>::bug::<&alloc[70118cb2c626a6fe]::string::String>
  12:     0x7f6662634570 - <rustc_errors[dc2ff1671bbffc75]::Handler>::bug::<&alloc[70118cb2c626a6fe]::string::String>
  13:     0x7f66627e21c5 - rustc_middle[6f5264a6afce6a65]::util::bug::opt_span_bug_fmt::<rustc_span[14d2494bb02e1473]::span_encoding::Span>::{closure#0}
  14:     0x7f66627d931b - rustc_middle[6f5264a6afce6a65]::ty::context::tls::with_opt::<rustc_middle[6f5264a6afce6a65]::util::bug::opt_span_bug_fmt<rustc_span[14d2494bb02e1473]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f66627d92c6 - rustc_middle[6f5264a6afce6a65]::ty::context::tls::with_context_opt::<rustc_middle[6f5264a6afce6a65]::ty::context::tls::with_opt<rustc_middle[6f5264a6afce6a65]::util::bug::opt_span_bug_fmt<rustc_span[14d2494bb02e1473]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f66627e2109 - rustc_middle[6f5264a6afce6a65]::util::bug::opt_span_bug_fmt::<rustc_span[14d2494bb02e1473]::span_encoding::Span>
  17:     0x7f665fc01f75 - rustc_middle[6f5264a6afce6a65]::util::bug::bug_fmt
  18:     0x7f66626cad38 - <&mut rustc_middle[6f5264a6afce6a65]::ty::closure::symbols_for_closure_captures::{closure#0} as core[c40c890f2341d509]::ops::function::FnOnce<(&rustc_middle[6f5264a6afce6a65]::ty::closure::CapturedPlace,)>>::call_once
  19:     0x7f66626b80d2 - <alloc[70118cb2c626a6fe]::vec::Vec<rustc_span[14d2494bb02e1473]::symbol::Symbol> as alloc[70118cb2c626a6fe]::vec::spec_from_iter::SpecFromIter<rustc_span[14d2494bb02e1473]::symbol::Symbol, core[c40c890f2341d509]::iter::adapters::map::Map<core[c40c890f2341d509]::iter::adapters::flatten::Flatten<core[c40c890f2341d509]::option::IntoIter<core[c40c890f2341d509]::iter::adapters::flatten::FlatMap<indexmap[3408c94a95748c9a]::map::Values<rustc_hir[d77988aaf7e77eb4]::hir_id::HirId, alloc[70118cb2c626a6fe]::vec::Vec<rustc_middle[6f5264a6afce6a65]::ty::closure::CapturedPlace>>, core[c40c890f2341d509]::slice::iter::Iter<rustc_middle[6f5264a6afce6a65]::ty::closure::CapturedPlace>, <rustc_middle[6f5264a6afce6a65]::ty::context::TypeckResults>::closure_min_captures_flattened::{closure#0}::{closure#0}>>>, rustc_middle[6f5264a6afce6a65]::ty::closure::symbols_for_closure_captures::{closure#0}>>>::from_iter
  20:     0x7f66626e4cc2 - rustc_middle[6f5264a6afce6a65]::ty::closure::symbols_for_closure_captures
  21:     0x7f66619a412e - rustc_query_system[34387b37f05e90a]::query::plumbing::get_query::<rustc_query_impl[267cf340949f4406]::queries::symbols_for_closure_captures, rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt>
  22:     0x7f6661488f1a - <rustc_query_impl[267cf340949f4406]::Queries as rustc_middle[6f5264a6afce6a65]::ty::query::QueryEngine>::symbols_for_closure_captures
  23:     0x7f6660c5e562 - rustc_mir_build[95509cd634411e8b]::build::construct_fn::<core[c40c890f2341d509]::iter::adapters::chain::Chain<alloc[70118cb2c626a6fe]::vec::into_iter::IntoIter<rustc_mir_build[95509cd634411e8b]::build::ArgInfo>, core[c40c890f2341d509]::iter::adapters::map::Map<core[c40c890f2341d509]::iter::adapters::enumerate::Enumerate<core[c40c890f2341d509]::slice::iter::Iter<rustc_hir[d77988aaf7e77eb4]::hir::Param>>, rustc_mir_build[95509cd634411e8b]::build::mir_build::{closure#1}::{closure#1}>>>
  24:     0x7f6660cee918 - <rustc_infer[a7222fc338a6533f]::infer::InferCtxtBuilder>::enter::<rustc_middle[6f5264a6afce6a65]::mir::Body, rustc_mir_build[95509cd634411e8b]::build::mir_build::{closure#1}>
  25:     0x7f6660c5c8ec - rustc_mir_build[95509cd634411e8b]::build::mir_built
  26:     0x7f6661881ee4 - rustc_query_system[34387b37f05e90a]::query::plumbing::try_execute_query::<rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt, rustc_query_system[34387b37f05e90a]::query::caches::DefaultCache<rustc_middle[6f5264a6afce6a65]::ty::WithOptConstParam<rustc_span[14d2494bb02e1473]::def_id::LocalDefId>, &rustc_data_structures[a9eb48882c3fd18e]::steal::Steal<rustc_middle[6f5264a6afce6a65]::mir::Body>>>
  27:     0x7f66619b6456 - rustc_query_system[34387b37f05e90a]::query::plumbing::get_query::<rustc_query_impl[267cf340949f4406]::queries::mir_built, rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt>
  28:     0x7f6661485d6a - <rustc_query_impl[267cf340949f4406]::Queries as rustc_middle[6f5264a6afce6a65]::ty::query::QueryEngine>::mir_built
  29:     0x7f666030ef17 - rustc_mir_transform[47db56a177475bc2]::check_unsafety::unsafety_check_result
  30:     0x7f666030a9a6 - <rustc_mir_transform[47db56a177475bc2]::check_unsafety::provide::{closure#0} as core[c40c890f2341d509]::ops::function::FnOnce<(rustc_middle[6f5264a6afce6a65]::ty::context::TyCtxt, rustc_span[14d2494bb02e1473]::def_id::LocalDefId)>>::call_once
  31:     0x7f6661894b7d - rustc_query_system[34387b37f05e90a]::query::plumbing::try_execute_query::<rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt, rustc_query_system[34387b37f05e90a]::query::caches::DefaultCache<rustc_span[14d2494bb02e1473]::def_id::LocalDefId, &rustc_middle[6f5264a6afce6a65]::mir::query::UnsafetyCheckResult>>
  32:     0x7f66619857c7 - rustc_query_system[34387b37f05e90a]::query::plumbing::get_query::<rustc_query_impl[267cf340949f4406]::queries::unsafety_check_result, rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt>
  33:     0x7f6661495944 - <rustc_query_impl[267cf340949f4406]::Queries as rustc_middle[6f5264a6afce6a65]::ty::query::QueryEngine>::unsafety_check_result
  34:     0x7f666030cf93 - <rustc_mir_transform[47db56a177475bc2]::check_unsafety::UnsafetyChecker as rustc_middle[6f5264a6afce6a65]::mir::visit::Visitor>::visit_rvalue
  35:     0x7f666030f1be - rustc_mir_transform[47db56a177475bc2]::check_unsafety::unsafety_check_result
  36:     0x7f666030a9a6 - <rustc_mir_transform[47db56a177475bc2]::check_unsafety::provide::{closure#0} as core[c40c890f2341d509]::ops::function::FnOnce<(rustc_middle[6f5264a6afce6a65]::ty::context::TyCtxt, rustc_span[14d2494bb02e1473]::def_id::LocalDefId)>>::call_once
  37:     0x7f6661894b7d - rustc_query_system[34387b37f05e90a]::query::plumbing::try_execute_query::<rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt, rustc_query_system[34387b37f05e90a]::query::caches::DefaultCache<rustc_span[14d2494bb02e1473]::def_id::LocalDefId, &rustc_middle[6f5264a6afce6a65]::mir::query::UnsafetyCheckResult>>
  38:     0x7f66619857c7 - rustc_query_system[34387b37f05e90a]::query::plumbing::get_query::<rustc_query_impl[267cf340949f4406]::queries::unsafety_check_result, rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt>
  39:     0x7f6661495944 - <rustc_query_impl[267cf340949f4406]::Queries as rustc_middle[6f5264a6afce6a65]::ty::query::QueryEngine>::unsafety_check_result
  40:     0x7f66602bb511 - rustc_mir_transform[47db56a177475bc2]::mir_const
  41:     0x7f6661881ee4 - rustc_query_system[34387b37f05e90a]::query::plumbing::try_execute_query::<rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt, rustc_query_system[34387b37f05e90a]::query::caches::DefaultCache<rustc_middle[6f5264a6afce6a65]::ty::WithOptConstParam<rustc_span[14d2494bb02e1473]::def_id::LocalDefId>, &rustc_data_structures[a9eb48882c3fd18e]::steal::Steal<rustc_middle[6f5264a6afce6a65]::mir::Body>>>
  42:     0x7f66619b6593 - rustc_query_system[34387b37f05e90a]::query::plumbing::get_query::<rustc_query_impl[267cf340949f4406]::queries::mir_const, rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt>
  43:     0x7f66614862ea - <rustc_query_impl[267cf340949f4406]::Queries as rustc_middle[6f5264a6afce6a65]::ty::query::QueryEngine>::mir_const
  44:     0x7f66602bc3a8 - rustc_mir_transform[47db56a177475bc2]::mir_promoted
  45:     0x7f666196102f - rustc_query_system[34387b37f05e90a]::query::plumbing::get_query::<rustc_query_impl[267cf340949f4406]::queries::mir_promoted, rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt>
  46:     0x7f666148899a - <rustc_query_impl[267cf340949f4406]::Queries as rustc_middle[6f5264a6afce6a65]::ty::query::QueryEngine>::mir_promoted
  47:     0x7f6660f5cd6f - rustc_borrowck[4a3e050f671a743c]::mir_borrowck
  48:     0x7f6660f23716 - <rustc_borrowck[4a3e050f671a743c]::provide::{closure#0} as core[c40c890f2341d509]::ops::function::FnOnce<(rustc_middle[6f5264a6afce6a65]::ty::context::TyCtxt, rustc_span[14d2494bb02e1473]::def_id::LocalDefId)>>::call_once
  49:     0x7f6661893e1d - rustc_query_system[34387b37f05e90a]::query::plumbing::try_execute_query::<rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt, rustc_query_system[34387b37f05e90a]::query::caches::DefaultCache<rustc_span[14d2494bb02e1473]::def_id::LocalDefId, &rustc_middle[6f5264a6afce6a65]::mir::query::BorrowCheckResult>>
  50:     0x7f6661960908 - rustc_query_system[34387b37f05e90a]::query::plumbing::get_query::<rustc_query_impl[267cf340949f4406]::queries::mir_borrowck, rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt>
  51:     0x7f666149f064 - <rustc_query_impl[267cf340949f4406]::Queries as rustc_middle[6f5264a6afce6a65]::ty::query::QueryEngine>::mir_borrowck
  52:     0x7f666069242d - <rustc_typeck[e7f071e1bcd98f63]::collect::type_of::find_opaque_ty_constraints::ConstraintLocator>::check
  53:     0x7f6660691802 - <rustc_typeck[e7f071e1bcd98f63]::collect::type_of::find_opaque_ty_constraints::ConstraintLocator as rustc_hir[d77988aaf7e77eb4]::intravisit::Visitor>::visit_item
  54:     0x7f6660680a1a - rustc_typeck[e7f071e1bcd98f63]::collect::type_of::type_of
  55:     0x7f66618aa080 - rustc_query_system[34387b37f05e90a]::query::plumbing::try_execute_query::<rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt, rustc_query_system[34387b37f05e90a]::query::caches::DefaultCache<rustc_span[14d2494bb02e1473]::def_id::DefId, rustc_middle[6f5264a6afce6a65]::ty::Ty>>
  56:     0x7f66619b4191 - rustc_query_system[34387b37f05e90a]::query::plumbing::get_query::<rustc_query_impl[267cf340949f4406]::queries::type_of, rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt>
  57:     0x7f6661480139 - <rustc_query_impl[267cf340949f4406]::Queries as rustc_middle[6f5264a6afce6a65]::ty::query::QueryEngine>::type_of
  58:     0x7f666080ad37 - rustc_typeck[e7f071e1bcd98f63]::check::check::check_opaque
  59:     0x7f666080f0dd - rustc_typeck[e7f071e1bcd98f63]::check::check::check_item_type
  60:     0x7f666081b77a - rustc_typeck[e7f071e1bcd98f63]::check::check::check_mod_item_types
  61:     0x7f6661896670 - rustc_query_system[34387b37f05e90a]::query::plumbing::try_execute_query::<rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt, rustc_query_system[34387b37f05e90a]::query::caches::DefaultCache<rustc_span[14d2494bb02e1473]::def_id::LocalDefId, ()>>
  62:     0x7f666197e6f4 - rustc_query_system[34387b37f05e90a]::query::plumbing::get_query::<rustc_query_impl[267cf340949f4406]::queries::check_mod_item_types, rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt>
  63:     0x7f6661499a34 - <rustc_query_impl[267cf340949f4406]::Queries as rustc_middle[6f5264a6afce6a65]::ty::query::QueryEngine>::check_mod_item_types
  64:     0x7f66608281ea - <rustc_middle[6f5264a6afce6a65]::hir::map::Map>::for_each_module::<rustc_typeck[e7f071e1bcd98f63]::check_crate::{closure#6}::{closure#0}>
  65:     0x7f66607476c2 - <rustc_session[35f02101f843a957]::session::Session>::time::<(), rustc_typeck[e7f071e1bcd98f63]::check_crate::{closure#6}>
  66:     0x7f66608f28cc - rustc_typeck[e7f071e1bcd98f63]::check_crate
  67:     0x7f665fda2ea1 - rustc_interface[2c997dfd7fc71c8c]::passes::analysis
  68:     0x7f66618ce2c0 - rustc_query_system[34387b37f05e90a]::query::plumbing::try_execute_query::<rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt, rustc_query_system[34387b37f05e90a]::query::caches::DefaultCache<(), core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>>>
  69:     0x7f66619b42b2 - rustc_query_system[34387b37f05e90a]::query::plumbing::get_query::<rustc_query_impl[267cf340949f4406]::queries::analysis, rustc_query_impl[267cf340949f4406]::plumbing::QueryCtxt>
  70:     0x7f666148069e - <rustc_query_impl[267cf340949f4406]::Queries as rustc_middle[6f5264a6afce6a65]::ty::query::QueryEngine>::analysis
  71:     0x7f665fcbace5 - <rustc_interface[2c997dfd7fc71c8c]::passes::QueryContext>::enter::<rustc_driver[fd89767628f3c01e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>>
  72:     0x7f665fc66c14 - <rustc_interface[2c997dfd7fc71c8c]::interface::Compiler>::enter::<rustc_driver[fd89767628f3c01e]::run_compiler::{closure#1}::{closure#2}, core[c40c890f2341d509]::result::Result<core[c40c890f2341d509]::option::Option<rustc_interface[2c997dfd7fc71c8c]::queries::Linker>, rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>>
  73:     0x7f665fc54af8 - rustc_span[14d2494bb02e1473]::with_source_map::<core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>, rustc_interface[2c997dfd7fc71c8c]::interface::create_compiler_and_run<core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>, rustc_driver[fd89767628f3c01e]::run_compiler::{closure#1}>::{closure#1}>
  74:     0x7f665fc6969a - rustc_interface[2c997dfd7fc71c8c]::interface::create_compiler_and_run::<core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>, rustc_driver[fd89767628f3c01e]::run_compiler::{closure#1}>
  75:     0x7f665fc4f01f - <scoped_tls[a5a6ed0cc2b7e300]::ScopedKey<rustc_span[14d2494bb02e1473]::SessionGlobals>>::set::<rustc_interface[2c997dfd7fc71c8c]::interface::run_compiler<core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>, rustc_driver[fd89767628f3c01e]::run_compiler::{closure#1}>::{closure#0}, core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>>
  76:     0x7f665fc58739 - std[da272c3ef198aa80]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2c997dfd7fc71c8c]::util::run_in_thread_pool_with_globals<rustc_interface[2c997dfd7fc71c8c]::interface::run_compiler<core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>, rustc_driver[fd89767628f3c01e]::run_compiler::{closure#1}>::{closure#0}, core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>>::{closure#0}, core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>>
  77:     0x7f665fcc2809 - <<std[da272c3ef198aa80]::thread::Builder>::spawn_unchecked_<rustc_interface[2c997dfd7fc71c8c]::util::run_in_thread_pool_with_globals<rustc_interface[2c997dfd7fc71c8c]::interface::run_compiler<core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>, rustc_driver[fd89767628f3c01e]::run_compiler::{closure#1}>::{closure#0}, core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>>::{closure#0}, core[c40c890f2341d509]::result::Result<(), rustc_errors[dc2ff1671bbffc75]::ErrorGuaranteed>>::{closure#1} as core[c40c890f2341d509]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  78:     0x7f665f2a5185 - std::sys::unix::thread::Thread::new::thread_start::h6495dc7843bca201
  79:     0x7f66597f0609 - start_thread
  80:     0x7f665f103133 - clone
  81:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (0a1b98395 2022-07-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbols_for_closure_captures] symbols for captures of closure `main::{closure#0}` in `main::{closure#0}`
#1 [mir_built] building MIR for `main::{closure#0}`
#2 [unsafety_check_result] unsafety-checking `main::{closure#0}`
#3 [unsafety_check_result] unsafety-checking `main`
#4 [mir_const] processing MIR for `main`
#5 [mir_promoted] processing `main`
#6 [mir_borrowck] borrow-checking `main`
#7 [type_of] computing type of `main::T::{opaque#0}`
#8 [check_mod_item_types] checking item types in top-level module
#9 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


