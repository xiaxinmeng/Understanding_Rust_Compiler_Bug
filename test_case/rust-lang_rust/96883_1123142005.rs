plain
........................................................................................ 4664/13080
........................................................................................ 4752/13080
........................................................................................ 4840/13080
........................................................................................ 4928/13080
.....................................................................F.F.......i........ 5016/13080
........................................................................................ 5192/13080
........................................................................................ 5280/13080
........................................................................................ 5368/13080
........................................................................................ 5456/13080
---
diff of stderr:

12    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
13 
14 error[E0391]: cycle detected when computing type of `Foo::X`
-   --> $DIR/cycle-trait-default-type-trait.rs:4:23
16    |
16    |
17 LL | trait Foo<X = Box<dyn Foo>> {
+    |           ^^^^^^^^^^^^^^^^
19    |
19    |
20    = note: ...which immediately requires computing type of `Foo::X` again
21 note: cycle used when collecting item types in top-level module

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/cycle-trait-default-type-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cycle-trait/cycle-trait-default-type-trait.rs`

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `Foo::X`
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {
   |
   |
   = note: ...which immediately requires computing type of `Foo::X` again
note: cycle used when collecting item types in top-level module
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {


error[E0391]: cycle detected when computing type of `Foo::X`
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {
   |
   |
   = note: ...which immediately requires computing type of `Foo::X` again
note: cycle used when collecting item types in top-level module
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/trait-objects.rs#extended stdout ----

error in revision `extended`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/trait-objects.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "extended" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/trait-objects.extended" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/trait-objects.extended/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Normalizing Binder(TraitPredicate(<<dyn StreamingIterator<for<'a> Item = &'a i32> as StreamingIterator>::Item<'a> as std::marker::Sized>, polarity:Positive), [Region(BrNamed(DefId(0:5 ~ trait_objects[7260]::StreamingIterator::Item::'a), 'a))]) without wrapping in a `Binder`', compiler/rustc_trait_selection/src/traits/project.rs:439:9
stack backtrace:
   0:     0x7fad2c0c09ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h93197d43cde616c5
   1:     0x7fad2c126ee8 - core::fmt::write::h7b278aaf2f6cec6c
   2:     0x7fad2c0b0841 - std::io::Write::write_fmt::had3e315c45a7e7b6
   3:     0x7fad2c0c39de - std::panicking::default_hook::{{closure}}::h6d8f8512cff0b448
   4:     0x7fad2c0c360c - std::panicking::default_hook::h05a9a6700a75a4dc
   5:     0x7fad2cc43561 - rustc_driver[ccd6c9df767b67e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fad2c0c423e - std::panicking::rust_panic_with_hook::h0df9f97442b4a198
   7:     0x7fad2c0c4037 - std::panicking::begin_panic_handler::{{closure}}::h3767436a60be767b
   8:     0x7fad2c0c0ec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h20bca5830f007b20
   9:     0x7fad2c0c3d19 - rust_begin_unwind
  10:     0x7fad2c078073 - core::panicking::panic_fmt::hef91b1db86be0508
  11:     0x7fad2f11e9e5 - <rustc_trait_selection[4eb2330ee3b1af7d]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[825930897a6bfbc8]::ty::Predicate>
  12:     0x7fad2f1279a4 - rustc_trait_selection[4eb2330ee3b1af7d]::traits::project::normalize_with_depth_to::<rustc_middle[825930897a6bfbc8]::ty::Predicate>
  13:     0x7fad2f1ad9bc - <rustc_trait_selection[4eb2330ee3b1af7d]::traits::select::SelectionContext>::confirm_candidate
  14:     0x7fad2f1b5420 - <rustc_trait_selection[4eb2330ee3b1af7d]::traits::select::SelectionContext>::select
  15:     0x7fad2f1686cc - <rustc_trait_selection[4eb2330ee3b1af7d]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  16:     0x7fad2f162bd5 - <rustc_trait_selection[4eb2330ee3b1af7d]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  17:     0x7fad2f16c33e - <rustc_data_structures[a79bec5cc933c350]::obligation_forest::ObligationForest<rustc_trait_selection[4eb2330ee3b1af7d]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[4eb2330ee3b1af7d]::traits::fulfill::FulfillProcessor, rustc_data_structures[a79bec5cc933c350]::obligation_forest::Outcome<rustc_trait_selection[4eb2330ee3b1af7d]::traits::fulfill::PendingPredicateObligation, rustc_infer[ebb5cecda71be67a]::traits::FulfillmentErrorCode>>
  18:     0x7fad2f16160b - <rustc_trait_selection[4eb2330ee3b1af7d]::traits::fulfill::FulfillmentContext as rustc_infer[ebb5cecda71be67a]::traits::engine::TraitEngine>::select_where_possible
  19:     0x7fad2d4253fc - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_argument_types
  20:     0x7fad2d424a5b - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_method_argument_types
  21:     0x7fad2d46bb8a - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_expr_kind
  22:     0x7fad2d413607 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  23:     0x7fad2d467de9 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  24:     0x7fad2d415297 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_field
  25:     0x7fad2d469296 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_expr_kind
  26:     0x7fad2d413607 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  27:     0x7fad2d467de9 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  28:     0x7fad2d42d5e4 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  29:     0x7fad2d469263 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_expr_kind
  30:     0x7fad2d413607 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  31:     0x7fad2d467de9 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  32:     0x7fad2d414a07 - <rustc_typeck[c2352e000b75c3df]::check::fn_ctxt::FnCtxt>::check_return_expr
  33:     0x7fad2d76f7c2 - rustc_typeck[c2352e000b75c3df]::check::check::check_fn
  34:     0x7fad2d61998e - <rustc_infer[ebb5cecda71be67a]::infer::InferCtxtBuilder>::enter::<&rustc_middle[825930897a6bfbc8]::ty::context::TypeckResults, <rustc_typeck[c2352e000b75c3df]::check::inherited::InheritedBuilder>::enter<rustc_typeck[c2352e000b75c3df]::check::typeck_with_fallback<rustc_typeck[c2352e000b75c3df]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[825930897a6bfbc8]::ty::context::TypeckResults>::{closure#0}>
  35:     0x7fad2d73340e - <rustc_typeck[c2352e000b75c3df]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[c2352e000b75c3df]::check::typeck_with_fallback<rustc_typeck[c2352e000b75c3df]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[825930897a6bfbc8]::ty::context::TypeckResults>
  36:     0x7fad2d53dc31 - rustc_typeck[c2352e000b75c3df]::check::typeck
  37:     0x7fad2e5d8cf6 - rustc_query_system[10898e9872b56b08]::query::plumbing::try_execute_query::<rustc_query_impl[9f2ab3b914846481]::plumbing::QueryCtxt, rustc_query_system[10898e9872b56b08]::query::caches::DefaultCache<rustc_span[109da3edcb9ea423]::def_id::LocalDefId, &rustc_middle[825930897a6bfbc8]::ty::context::TypeckResults>>
  38:     0x7fad2e6eda87 - rustc_query_system[10898e9872b56b08]::query::plumbing::get_query::<rustc_query_impl[9f2ab3b914846481]::queries::typeck, rustc_query_impl[9f2ab3b914846481]::plumbing::QueryCtxt>
  39:     0x7fad2e52f134 - <rustc_query_impl[9f2ab3b914846481]::Queries as rustc_middle[825930897a6bfbc8]::ty::query::QueryEngine>::typeck
  40:     0x7fad2d70ab8a - <rustc_middle[825930897a6bfbc8]::hir::map::Map>::par_body_owners::<rustc_typeck[c2352e000b75c3df]::check::typeck_item_bodies::{closure#0}>
  41:     0x7fad2d542c9d - rustc_typeck[c2352e000b75c3df]::check::typeck_item_bodies
  42:     0x7fad2e62078a - rustc_query_system[10898e9872b56b08]::query::plumbing::try_execute_query::<rustc_query_impl[9f2ab3b914846481]::plumbing::QueryCtxt, rustc_query_system[10898e9872b56b08]::query::caches::DefaultCache<(), ()>>
  43:     0x7fad2e6b4365 - rustc_query_system[10898e9872b56b08]::query::plumbing::get_query::<rustc_query_impl[9f2ab3b914846481]::queries::typeck_item_bodies, rustc_query_impl[9f2ab3b914846481]::plumbing::QueryCtxt>
  44:     0x7fad2e52ebde - <rustc_query_impl[9f2ab3b914846481]::Queries as rustc_middle[825930897a6bfbc8]::ty::query::QueryEngine>::typeck_item_bodies
  45:     0x7fad2d69ca2a - <rustc_session[328fe83121afcbe3]::session::Session>::time::<(), rustc_typeck[c2352e000b75c3df]::check_crate::{closure#7}>
  46:     0x7fad2d59e46e - rustc_typeck[c2352e000b75c3df]::check_crate
  47:     0x7fad2cd15b51 - rustc_interface[c80a97f9d6efa2f1]::passes::analysis
  48:     0x7fad2e614e4e - rustc_query_system[10898e9872b56b08]::query::plumbing::try_execute_query::<rustc_query_impl[9f2ab3b914846481]::plumbing::QueryCtxt, rustc_query_system[10898e9872b56b08]::query::caches::DefaultCache<(), core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>>>
  49:     0x7fad2e6ede62 - rustc_query_system[10898e9872b56b08]::query::plumbing::get_query::<rustc_query_impl[9f2ab3b914846481]::queries::analysis, rustc_query_impl[9f2ab3b914846481]::plumbing::QueryCtxt>
  50:     0x7fad2e512aee - <rustc_query_impl[9f2ab3b914846481]::Queries as rustc_middle[825930897a6bfbc8]::ty::query::QueryEngine>::analysis
  51:     0x7fad2cc27ec4 - <rustc_interface[c80a97f9d6efa2f1]::passes::QueryContext>::enter::<rustc_driver[ccd6c9df767b67e]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>>
  52:     0x7fad2cbd412e - <rustc_interface[c80a97f9d6efa2f1]::interface::Compiler>::enter::<rustc_driver[ccd6c9df767b67e]::run_compiler::{closure#1}::{closure#2}, core[e18b907748a785ff]::result::Result<core[e18b907748a785ff]::option::Option<rustc_interface[c80a97f9d6efa2f1]::queries::Linker>, rustc_errors[a945e97e47866c65]::ErrorGuaranteed>>
  53:     0x7fad2cbb530b - rustc_span[109da3edcb9ea423]::with_source_map::<core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>, rustc_interface[c80a97f9d6efa2f1]::interface::create_compiler_and_run<core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>, rustc_driver[ccd6c9df767b67e]::run_compiler::{closure#1}>::{closure#1}>
  54:     0x7fad2cbd52c9 - <scoped_tls[b31b07fe48cccd78]::ScopedKey<rustc_span[109da3edcb9ea423]::SessionGlobals>>::set::<rustc_interface[c80a97f9d6efa2f1]::interface::run_compiler<core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>, rustc_driver[ccd6c9df767b67e]::run_compiler::{closure#1}>::{closure#0}, core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>>
  55:     0x7fad2cc2b379 - std[9e97cdcd04846c67]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c80a97f9d6efa2f1]::util::run_in_thread_pool_with_globals<rustc_interface[c80a97f9d6efa2f1]::interface::run_compiler<core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>, rustc_driver[ccd6c9df767b67e]::run_compiler::{closure#1}>::{closure#0}, core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>>::{closure#0}, core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>>
  56:     0x7fad2cbd65d1 - std[9e97cdcd04846c67]::panicking::try::<core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>, core[e18b907748a785ff]::panic::unwind_safe::AssertUnwindSafe<<std[9e97cdcd04846c67]::thread::Builder>::spawn_unchecked_<rustc_interface[c80a97f9d6efa2f1]::util::run_in_thread_pool_with_globals<rustc_interface[c80a97f9d6efa2f1]::interface::run_compiler<core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>, rustc_driver[ccd6c9df767b67e]::run_compiler::{closure#1}>::{closure#0}, core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>>::{closure#0}, core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  57:     0x7fad2cc2c072 - <<std[9e97cdcd04846c67]::thread::Builder>::spawn_unchecked_<rustc_interface[c80a97f9d6efa2f1]::util::run_in_thread_pool_with_globals<rustc_interface[c80a97f9d6efa2f1]::interface::run_compiler<core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>, rustc_driver[ccd6c9df767b67e]::run_compiler::{closure#1}>::{closure#0}, core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>>::{closure#0}, core[e18b907748a785ff]::result::Result<(), rustc_errors[a945e97e47866c65]::ErrorGuaranteed>>::{closure#1} as core[e18b907748a785ff]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7fad2c0cf553 - std::sys::unix::thread::Thread::new::thread_start::hecb8c3f1ea7e9a0b
  59:     0x7fad26622609 - start_thread
  60:     0x7fad2bf35163 - clone
  61:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (bebae151a 2022-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `min_size`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/infinite/infinite-type-alias-mutual-recursion.rs stdout ----
---- [ui] src/test/ui/infinite/infinite-type-alias-mutual-recursion.rs stdout ----
diff of stderr:

1 error[E0391]: cycle detected when expanding type alias `X1`
-   --> $DIR/infinite-type-alias-mutual-recursion.rs:1:11
3    |
4 LL | type X1 = X2;
-    |           ^^
+    | ^^^^^^^^^^^^^
+    | ^^^^^^^^^^^^^
6    |
7 note: ...which requires expanding type alias `X2`...
-   --> $DIR/infinite-type-alias-mutual-recursion.rs:3:11
9    |
10 LL | type X2 = X3;
-    |           ^^
+    | ^^^^^^^^^^^^^
+    | ^^^^^^^^^^^^^
12 note: ...which requires expanding type alias `X3`...
-   --> $DIR/infinite-type-alias-mutual-recursion.rs:4:11
14    |
15 LL | type X3 = X1;
-    |           ^^
+    | ^^^^^^^^^^^^^
+    | ^^^^^^^^^^^^^
17    = note: ...which again requires expanding type alias `X1`, completing the cycle
18    = note: type aliases cannot be recursive
19    = help: consider using a struct, enum, or union instead to break the cycle

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-type-alias-mutual-recursion/infinite-type-alias-mutual-recursion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-type-alias-mutual-recursion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-type-alias-mutual-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-type-alias-mutual-recursion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-type-alias-mutual-recursion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when expanding type alias `X1`
   |
LL | type X1 = X2;
   | ^^^^^^^^^^^^^
   |
   |
note: ...which requires expanding type alias `X2`...
   |
LL | type X2 = X3;
   | ^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^
note: ...which requires expanding type alias `X3`...
   |
LL | type X3 = X1;
   | ^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^
   = note: ...which again requires expanding type alias `X1`, completing the cycle
   = note: type aliases cannot be recursive
   = help: consider using a struct, enum, or union instead to break the cycle
   = help: see <https://doc.rust-lang.org/reference/types.html#recursive-types> for more information
note: cycle used when collecting item types in top-level module
   |
   |
LL | / type X1 = X2;
LL | | //~^ ERROR cycle detected when expanding type alias `X1`
LL | | type X2 = X3;
LL | | type X3 = X1;
LL | |
LL | | fn main() {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/infinite/infinite-vec-type-recursion.rs stdout ----
diff of stderr:

1 error[E0391]: cycle detected when expanding type alias `X`
-   --> $DIR/infinite-vec-type-recursion.rs:1:14
3    |
3    |
4 LL | type X = Vec<X>;
+    | ^^^^^^^^^^^^^^^^
6    |
6    |
7    = note: ...which immediately requires expanding type alias `X` again
8    = note: type aliases cannot be recursive

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-vec-type-recursion/infinite-vec-type-recursion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-vec-type-recursion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-vec-type-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-vec-type-recursion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-vec-type-recursion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when expanding type alias `X`
   |
   |
LL | type X = Vec<X>;
   |
   |
   = note: ...which immediately requires expanding type alias `X` again
   = note: type aliases cannot be recursive
   = help: consider using a struct, enum, or union instead to break the cycle
   = help: see <https://doc.rust-lang.org/reference/types.html#recursive-types> for more information
note: cycle used when collecting item types in top-level module
   |
   |
LL | / type X = Vec<X>;
LL | | //~^ ERROR cycle detected
LL | |
LL | | fn main() { let b: X = Vec::new(); }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-34373.rs stdout ----
diff of stderr:

1 error[E0391]: cycle detected when computing type of `Foo::T`
-   --> $DIR/issue-34373.rs:7:30
3    |
3    |
4 LL | pub struct Foo<T = Box<Trait<DefaultFoo>>>;
+    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7 note: ...which requires expanding type alias `DefaultFoo`...
-   --> $DIR/issue-34373.rs:8:19
9    |
9    |
10 LL | type DefaultFoo = Foo;
+    | ^^^^^^^^^^^^^^^^^^^^^^
+    | ^^^^^^^^^^^^^^^^^^^^^^
12    = note: ...which again requires computing type of `Foo::T`, completing the cycle
13 note: cycle used when collecting item types in top-level module


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/issue-34373.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/issue-34373.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-34373.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34373.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `Foo::T`
   |
   |
LL | pub struct Foo<T = Box<Trait<DefaultFoo>>>;  //~ ERROR cycle detected
   |
   |
note: ...which requires expanding type alias `DefaultFoo`...
   |
   |
LL | type DefaultFoo = Foo;
   | ^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires computing type of `Foo::T`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
LL | / #![allow(warnings)]
LL | |
LL | |
LL | | trait Trait<T> {
LL | |     fn foo(_: T) {}
LL | | fn main() {
LL | | }
   | |_^

