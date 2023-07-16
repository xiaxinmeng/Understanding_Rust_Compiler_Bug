plain
........................................................................................ 9240/13134
........................................................................................ 9328/13134
........................................................................................ 9416/13134
........................................................................................ 9504/13134
F.FF......F..F....FF.................................................................... 9592/13134
.....................................................................ii...............i. 9768/13134
...........................................................ii........................... 9856/13134
........................................................................................ 9944/13134
........................................................................................ 10032/13134
---
diff of stderr:

32    |                   ^^^^^^^^^^^^^^^^
33 
34 error: item has unused generic parameters
+    |
+    |
+ LL | pub fn used_parent<const T: usize>() -> usize {
+    |        ^^^^^^^^^^^ -------------- generic parameter `T` is unused
+ 
+ error: item has unused generic parameters
+    |
+    |
+ LL |   pub fn used_binding<const T: usize>() -> usize {
+    |                       -------------- generic parameter `T` is unused
+ LL |       let x = || {
+    |  _____________^
+ LL | |         let y: usize = T;
+ LL | |         y
+ LL | |     };
+ 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ error: item has unused generic parameters
+    |
+    |
+ LL | pub fn used_binding<const T: usize>() -> usize {
+    |        ^^^^^^^^^^^^ -------------- generic parameter `T` is unused
+ 
+ error: item has unused generic parameters
36    |
36    |
37 LL | pub fn unused_upvar<const T: usize>() -> usize {

40 LL |     let y = || x;
42 
- error: aborting due to 4 previous errors; 1 warning emitted
- error: aborting due to 4 previous errors; 1 warning emitted
+ error: item has unused generic parameters
+    |
+    |
+ LL | pub fn unused_upvar<const T: usize>() -> usize {
+    |        ^^^^^^^^^^^^ -------------- generic parameter `T` is unused
+ 
+ error: item has unused generic parameters
+    |
+    |
+ LL | pub fn used_substs<const T: usize>() -> usize {
+    |                    -------------- generic parameter `T` is unused
+ LL |     let x = || unused::<T>();
+ 
+ 
+ error: item has unused generic parameters
+    |
+    |
+ LL | pub fn used_substs<const T: usize>() -> usize {
+    |        ^^^^^^^^^^^ -------------- generic parameter `T` is unused
+ error: aborting due to 10 previous errors; 1 warning emitted
44 
45 

---
To only update this specific test, also pass `--test-args polymorphization/const_parameters/closures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/polymorphization/const_parameters/closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/const_parameters/closures" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zpolymorphize=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/const_parameters/closures/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(generic_const_exprs, rustc_attrs)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information


error: item has unused generic parameters
   |
   |
LL | pub fn unused<const T: usize>() -> usize {
   |               -------------- generic parameter `T` is unused
LL |     //~^ ERROR item has unused generic parameters
LL |     let add_one = |x: usize| x + 1;


error: item has unused generic parameters
   |
   |
LL | pub fn unused<const T: usize>() -> usize {
   |        ^^^^^^ -------------- generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | pub fn used_parent<const T: usize>() -> usize {
   |                    -------------- generic parameter `T` is unused
LL |     let x: usize = T;
LL |     let add_one = |x: usize| x + 1;


error: item has unused generic parameters
   |
   |
LL | pub fn used_parent<const T: usize>() -> usize {
   |        ^^^^^^^^^^^ -------------- generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL |   pub fn used_binding<const T: usize>() -> usize {
   |                       -------------- generic parameter `T` is unused
LL |       let x = || {
   |  _____________^
LL | |         let y: usize = T;
LL | |         y
LL | |     };


error: item has unused generic parameters
   |
   |
LL | pub fn used_binding<const T: usize>() -> usize {
   |        ^^^^^^^^^^^^ -------------- generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | pub fn unused_upvar<const T: usize>() -> usize {
   |                     -------------- generic parameter `T` is unused
LL |     let x: usize = T;
LL |     let y = || x;


error: item has unused generic parameters
   |
   |
LL | pub fn unused_upvar<const T: usize>() -> usize {
   |        ^^^^^^^^^^^^ -------------- generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | pub fn used_substs<const T: usize>() -> usize {
   |                    -------------- generic parameter `T` is unused
LL |     let x = || unused::<T>();


error: item has unused generic parameters
   |
   |
LL | pub fn used_substs<const T: usize>() -> usize {
   |        ^^^^^^^^^^^ -------------- generic parameter `T` is unused
error: aborting due to 10 previous errors; 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/polymorphization/const_parameters/functions.rs stdout ----
diff of stderr:

13 LL | pub fn unused<const T: usize>() {
14    |        ^^^^^^ -------------- generic parameter `T` is unused
- error: aborting due to previous error; 1 warning emitted
- error: aborting due to previous error; 1 warning emitted
+ error: item has unused generic parameters
+    |
+    |
+ LL | pub fn used_binding<const T: usize>() -> usize {
+    |        ^^^^^^^^^^^^ -------------- generic parameter `T` is unused
+ 
+ error: item has unused generic parameters
+    |
+    |
+ LL | pub fn used_substs<const T: usize>() {
+    |        ^^^^^^^^^^^ -------------- generic parameter `T` is unused
+ error: aborting due to 3 previous errors; 1 warning emitted
17 
18 

---
To only update this specific test, also pass `--test-args polymorphization/const_parameters/functions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/polymorphization/const_parameters/functions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/const_parameters/functions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zpolymorphize=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/const_parameters/functions/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(generic_const_exprs, rustc_attrs)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information


error: item has unused generic parameters
   |
   |
LL | pub fn unused<const T: usize>() {
   |        ^^^^^^ -------------- generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | pub fn used_binding<const T: usize>() -> usize {
   |        ^^^^^^^^^^^^ -------------- generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | pub fn used_substs<const T: usize>() {
   |        ^^^^^^^^^^^ -------------- generic parameter `T` is unused
error: aborting due to 3 previous errors; 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/polymorphization/issue-74614.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/polymorphization/issue-74614.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/issue-74614" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zpolymorphize=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/issue-74614/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: matches!(ty.kind(), ty :: Param(_))', compiler/rustc_const_eval/src/interpret/util.rs:55:37
   0:     0x7fbf667ef8cc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6da52b8c080fa25f
   1:     0x7fbf66853c98 - core::fmt::write::ha7f3ef05f1e932d7
   1:     0x7fbf66853c98 - core::fmt::write::ha7f3ef05f1e932d7
   2:     0x7fbf667df631 - std::io::Write::write_fmt::h33d8bc39df08f3c2
   3:     0x7fbf667f28be - std::panicking::default_hook::{{closure}}::h5b3670c933f77237
   4:     0x7fbf667f25a9 - std::panicking::default_hook::h0f4bd245be161a14
   5:     0x7fbf67323d84 - rustc_driver[ebce7bac9047669c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fbf667f3022 - std::panicking::rust_panic_with_hook::h761a46c9b5c81537
   7:     0x7fbf667f2e19 - std::panicking::begin_panic_handler::{{closure}}::h93362ec5efa634b5
   8:     0x7fbf667efde4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4da797a20e282334
   9:     0x7fbf667f2b39 - rust_begin_unwind
  10:     0x7fbf667a71c3 - core::panicking::panic_fmt::hb793bd16fb8ae423
  11:     0x7fbf667a708d - core::panicking::panic::hedef113afa3b979d
  12:     0x7fbf68913dda - <rustc_const_eval[65b78f989cd2dff0]::interpret::util::ensure_monomorphic_enough::UsedParamsNeedSubstVisitor as rustc_middle[e946ef83ef54db28]::ty::fold::TypeVisitor>::visit_ty
  13:     0x7fbf68913916 - rustc_const_eval[65b78f989cd2dff0]::interpret::util::ensure_monomorphic_enough::<rustc_middle[e946ef83ef54db28]::ty::Ty>
  14:     0x7fbf68811cb5 - <rustc_const_eval[65b78f989cd2dff0]::interpret::eval_context::InterpCx<rustc_const_eval[65b78f989cd2dff0]::const_eval::machine::CompileTimeInterpreter>>::cast
  15:     0x7fbf6883d904 - <rustc_const_eval[65b78f989cd2dff0]::interpret::eval_context::InterpCx<rustc_const_eval[65b78f989cd2dff0]::const_eval::machine::CompileTimeInterpreter>>::eval_rvalue_into_place
  16:     0x7fbf688387e5 - <rustc_const_eval[65b78f989cd2dff0]::interpret::eval_context::InterpCx<rustc_const_eval[65b78f989cd2dff0]::const_eval::machine::CompileTimeInterpreter>>::run
  17:     0x7fbf688c43ff - rustc_const_eval[65b78f989cd2dff0]::const_eval::eval_queries::eval_to_allocation_raw_provider
  18:     0x7fbf68ecf8b7 - rustc_query_system[76dcf74ca33e2690]::query::plumbing::get_query::<rustc_query_impl[24336930c871cb82]::queries::eval_to_allocation_raw, rustc_query_impl[24336930c871cb82]::plumbing::QueryCtxt>
  19:     0x7fbf68a46211 - <rustc_query_impl[24336930c871cb82]::Queries as rustc_middle[e946ef83ef54db28]::ty::query::QueryEngine>::eval_to_allocation_raw
  20:     0x7fbf688c25dd - rustc_const_eval[65b78f989cd2dff0]::const_eval::eval_queries::eval_to_const_value_raw_provider
  21:     0x7fbf68ed6fca - rustc_query_system[76dcf74ca33e2690]::query::plumbing::get_query::<rustc_query_impl[24336930c871cb82]::queries::eval_to_const_value_raw, rustc_query_impl[24336930c871cb82]::plumbing::QueryCtxt>
  22:     0x7fbf68a467e1 - <rustc_query_impl[24336930c871cb82]::Queries as rustc_middle[e946ef83ef54db28]::ty::query::QueryEngine>::eval_to_const_value_raw
  23:     0x7fbf688c2279 - rustc_const_eval[65b78f989cd2dff0]::const_eval::eval_queries::eval_to_const_value_raw_provider
  24:     0x7fbf68ed6fca - rustc_query_system[76dcf74ca33e2690]::query::plumbing::get_query::<rustc_query_impl[24336930c871cb82]::queries::eval_to_const_value_raw, rustc_query_impl[24336930c871cb82]::plumbing::QueryCtxt>
  25:     0x7fbf68a467e1 - <rustc_query_impl[24336930c871cb82]::Queries as rustc_middle[e946ef83ef54db28]::ty::query::QueryEngine>::eval_to_const_value_raw
  26:     0x7fbf69bed735 - <rustc_middle[e946ef83ef54db28]::ty::context::TyCtxt>::const_eval_global_id
  27:     0x7fbf69bec68a - <rustc_middle[e946ef83ef54db28]::ty::context::TyCtxt>::const_eval_resolve
  28:     0x7fbf6778a403 - <rustc_monomorphize[5b9db920a2623b75]::collector::MirNeighborCollector as rustc_middle[e946ef83ef54db28]::mir::visit::Visitor>::visit_constant
  29:     0x7fbf67782c3a - <rustc_monomorphize[5b9db920a2623b75]::collector::MirNeighborCollector as rustc_middle[e946ef83ef54db28]::mir::visit::Visitor>::visit_operand
  30:     0x7fbf67781541 - <rustc_monomorphize[5b9db920a2623b75]::collector::MirNeighborCollector as rustc_middle[e946ef83ef54db28]::mir::visit::Visitor>::visit_rvalue
  31:     0x7fbf6778c97e - rustc_monomorphize[5b9db920a2623b75]::collector::collect_neighbours
  32:     0x7fbf67788795 - rustc_monomorphize[5b9db920a2623b75]::collector::collect_items_rec
  33:     0x7fbf67788cec - rustc_monomorphize[5b9db920a2623b75]::collector::collect_items_rec
  34:     0x7fbf67788cec - rustc_monomorphize[5b9db920a2623b75]::collector::collect_items_rec
  35:     0x7fbf677a454f - <rustc_session[91e91aee7ac01910]::session::Session>::time::<(), rustc_monomorphize[5b9db920a2623b75]::collector::collect_crate_mono_items::{closure#1}>
  36:     0x7fbf67784c74 - rustc_monomorphize[5b9db920a2623b75]::collector::collect_crate_mono_items
  37:     0x7fbf67798ffd - rustc_monomorphize[5b9db920a2623b75]::partitioning::collect_and_partition_mono_items
  38:     0x7fbf68e29697 - rustc_query_system[76dcf74ca33e2690]::query::plumbing::try_execute_query::<rustc_query_impl[24336930c871cb82]::plumbing::QueryCtxt, rustc_query_system[76dcf74ca33e2690]::query::caches::DefaultCache<(), (&std[190549edf6963674]::collections::hash::set::HashSet<rustc_span[32d81c11a68aba39]::def_id::DefId, core[aabe0cdb53e3c9f7]::hash::BuildHasherDefault<rustc_hash[2a06c35db1eb9ece]::FxHasher>>, &[rustc_middle[e946ef83ef54db28]::mir::mono::CodegenUnit])>>
  39:     0x7fbf68ef1baa - rustc_query_system[76dcf74ca33e2690]::query::plumbing::get_query::<rustc_query_impl[24336930c871cb82]::queries::collect_and_partition_mono_items, rustc_query_impl[24336930c871cb82]::plumbing::QueryCtxt>
  40:     0x7fbf68a71729 - <rustc_query_impl[24336930c871cb82]::Queries as rustc_middle[e946ef83ef54db28]::ty::query::QueryEngine>::collect_and_partition_mono_items
  41:     0x7fbf674f37c0 - rustc_codegen_ssa[568687aab4b525e6]::base::codegen_crate::<rustc_codegen_llvm[c2e577d83cf32029]::LlvmCodegenBackend>
  42:     0x7fbf675b2c29 - <rustc_codegen_llvm[c2e577d83cf32029]::LlvmCodegenBackend as rustc_codegen_ssa[568687aab4b525e6]::traits::backend::CodegenBackend>::codegen_crate
  43:     0x7fbf67463281 - <rustc_session[91e91aee7ac01910]::session::Session>::time::<alloc[b22802118ad12fe4]::boxed::Box<dyn core[aabe0cdb53e3c9f7]::any::Any>, rustc_interface[d61b41473d75d5c7]::passes::start_codegen::{closure#0}>
  44:     0x7fbf6745221d - <rustc_interface[d61b41473d75d5c7]::passes::QueryContext>::enter::<<rustc_interface[d61b41473d75d5c7]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[aabe0cdb53e3c9f7]::result::Result<alloc[b22802118ad12fe4]::boxed::Box<dyn core[aabe0cdb53e3c9f7]::any::Any>, rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>>
  45:     0x7fbf6743d42e - <rustc_interface[d61b41473d75d5c7]::queries::Queries>::ongoing_codegen
  46:     0x7fbf67341907 - <rustc_interface[d61b41473d75d5c7]::interface::Compiler>::enter::<rustc_driver[ebce7bac9047669c]::run_compiler::{closure#1}::{closure#2}, core[aabe0cdb53e3c9f7]::result::Result<core[aabe0cdb53e3c9f7]::option::Option<rustc_interface[d61b41473d75d5c7]::queries::Linker>, rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>>
  47:     0x7fbf6739a09b - rustc_span[32d81c11a68aba39]::with_source_map::<core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>, rustc_interface[d61b41473d75d5c7]::interface::create_compiler_and_run<core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>, rustc_driver[ebce7bac9047669c]::run_compiler::{closure#1}>::{closure#1}>
  48:     0x7fbf67342a41 - <scoped_tls[7ba522b4189e977f]::ScopedKey<rustc_span[32d81c11a68aba39]::SessionGlobals>>::set::<rustc_interface[d61b41473d75d5c7]::interface::run_compiler<core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>, rustc_driver[ebce7bac9047669c]::run_compiler::{closure#1}>::{closure#0}, core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>>
  49:     0x7fbf67397099 - std[190549edf6963674]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d61b41473d75d5c7]::util::run_in_thread_pool_with_globals<rustc_interface[d61b41473d75d5c7]::interface::run_compiler<core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>, rustc_driver[ebce7bac9047669c]::run_compiler::{closure#1}>::{closure#0}, core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>>::{closure#0}, core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>>
  50:     0x7fbf67343aa1 - std[190549edf6963674]::panicking::try::<core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>, core[aabe0cdb53e3c9f7]::panic::unwind_safe::AssertUnwindSafe<<std[190549edf6963674]::thread::Builder>::spawn_unchecked_<rustc_interface[d61b41473d75d5c7]::util::run_in_thread_pool_with_globals<rustc_interface[d61b41473d75d5c7]::interface::run_compiler<core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>, rustc_driver[ebce7bac9047669c]::run_compiler::{closure#1}>::{closure#0}, core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>>::{closure#0}, core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  51:     0x7fbf6738e362 - <<std[190549edf6963674]::thread::Builder>::spawn_unchecked_<rustc_interface[d61b41473d75d5c7]::util::run_in_thread_pool_with_globals<rustc_interface[d61b41473d75d5c7]::interface::run_compiler<core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>, rustc_driver[ebce7bac9047669c]::run_compiler::{closure#1}>::{closure#0}, core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>>::{closure#0}, core[aabe0cdb53e3c9f7]::result::Result<(), rustc_errors[6e625ad0412fb7d9]::ErrorGuaranteed>>::{closure#1} as core[aabe0cdb53e3c9f7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7fbf667fe043 - std::sys::unix::thread::Thread::new::thread_start::h6b35487c00acf003
  53:     0x7fbf60d4d609 - start_thread
  54:     0x7fbf66660133 - clone
  55:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (e696ac544 2022-06-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z polymorphize=on
query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `foo::promoted[0]`
#1 [eval_to_const_value_raw] simplifying constant for the type system `foo::promoted[0]`
#2 [eval_to_const_value_raw] simplifying constant for the type system `foo::promoted[0]`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/polymorphization/predicates.rs stdout ----
---- [ui] src/test/ui/polymorphization/predicates.rs stdout ----
diff of stderr:

25   --> $DIR/predicates.rs:58:4
26    |
27 LL | fn quux<A, B, C: Default>() -> usize
-    |    ^^^^ -  - generic parameter `B` is unused
-    |         |
+    |    ^^^^ -  -  - generic parameter `C` is unused
+    |         |  |
+    |         |  generic parameter `B` is unused
30    |         generic parameter `A` is unused
31 
32 error: item has unused generic parameters
33   --> $DIR/predicates.rs:75:4
34    |
34    |
35 LL | fn foobar<F, G>() -> usize
-    |    ^^^^^^ - generic parameter `F` is unused
+    |    ^^^^^^ -  - generic parameter `G` is unused
+    |           |
+    |           generic parameter `F` is unused
37 
38 error: item has unused generic parameters


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/predicates/predicates.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/predicates/predicates.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args polymorphization/predicates.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/polymorphization/predicates.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/predicates" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zpolymorphize=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/predicates/auxiliary"
stdout: none
--- stderr -------------------------------
error: item has unused generic parameters
   |
   |
LL | fn foo<I, T>(_: I)
   |    ^^^    - generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | fn baz<I, T>(_: I)
   |    ^^^    - generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | impl<'a, I, T: 'a, E> Iterator for Foo<'a, I, E>
   |          -         - generic parameter `E` is unused
   |          |
   |          generic parameter `I` is unused
...
LL |         self.find(|_| true)


error: item has unused generic parameters
   |
   |
LL | fn quux<A, B, C: Default>() -> usize
   |    ^^^^ -  -  - generic parameter `C` is unused
   |         |  |
   |         |  generic parameter `B` is unused
   |         generic parameter `A` is unused

error: item has unused generic parameters
   |
   |
LL | fn foobar<F, G>() -> usize
   |    ^^^^^^ -  - generic parameter `G` is unused
   |           |
   |           generic parameter `F` is unused

error: item has unused generic parameters
   |
   |
LL | fn bar<I>() {
   |    ^^^ - generic parameter `I` is unused

note: the above error was encountered while instantiating `fn foo::<std::slice::Iter<u32>, T>`
   |
LL |     foo(x.iter());
   |     ^^^^^^^^^^^^^

---
43 LL |     finish(unused_const::<1u32>());
44    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
45 
- error: aborting due to 2 previous errors; 1 warning emitted
+ error: item has unused generic parameters
+    |
+    |
+ LL |   pub fn used_const_in_yield<const Y: u32>() -> impl Generator<(), Yield = u32, Return = u32> + Unpin
+    |                              ------------ generic parameter `Y` is unused
+ LL |   {
+ LL | /     || {
+ LL | |         yield Y;
+ LL | |         2
+ LL | |     }
+ 
+ 
+ note: the above error was encountered while instantiating `fn finish::<[generator@$DIR/generators.rs:70:5: 73:6], u32, u32>`
+    |
+    |
+ LL |     finish(used_const_in_yield::<1u32>());
+ 
+ 
+ error: item has unused generic parameters
+    |
+    |
+ LL |   pub fn used_const_in_return<const R: u32>() -> impl Generator<(), Yield = u32, Return = u32> + Unpin
+    |                               ------------ generic parameter `R` is unused
+ LL |   {
+ LL | /     || {
+ LL | |         yield 4;
+ LL | |         R
+ LL | |     }
+ 
+ 
+ note: the above error was encountered while instantiating `fn finish::<[generator@$DIR/generators.rs:79:5: 82:6], u32, u32>`
+    |
+    |
+ LL |     finish(used_const_in_return::<1u32>());
+ 
+ error: aborting due to 4 previous errors; 1 warning emitted
47 
48 
---
To only update this specific test, also pass `--test-args polymorphization/generators.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/polymorphization/generators.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/generators" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zpolymorphize=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/generators/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(generic_const_exprs, generators, generator_trait, rustc_attrs)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information


error: item has unused generic parameters
   |
   |
LL |   pub fn unused_type<T>() -> impl Generator<(), Yield = u32, Return = u32> + Unpin {
   |                      - generic parameter `T` is unused
LL | /     || {
LL | |         //~^ ERROR item has unused generic parameters
LL | |         yield 1;
LL | |         2
LL | |     }


note: the above error was encountered while instantiating `fn finish::<[generator@/checkout/src/test/ui/polymorphization/generators.rs:35:5: 39:6], u32, u32>`
   |
   |
LL |     finish(unused_type::<u32>());


error: item has unused generic parameters
   |
   |
LL |   pub fn unused_const<const T: u32>() -> impl Generator<(), Yield = u32, Return = u32> + Unpin {
   |                       ------------ generic parameter `T` is unused
LL | /     || {
LL | |         //~^ ERROR item has unused generic parameters
LL | |         yield 1;
LL | |         2
LL | |     }


note: the above error was encountered while instantiating `fn finish::<[generator@/checkout/src/test/ui/polymorphization/generators.rs:60:5: 64:6], u32, u32>`
   |
LL |     finish(unused_const::<1u32>());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: item has unused generic parameters
   |
   |
LL |   pub fn used_const_in_yield<const Y: u32>() -> impl Generator<(), Yield = u32, Return = u32> + Unpin
   |                              ------------ generic parameter `Y` is unused
LL |   {
LL | /     || {
LL | |         yield Y;
LL | |         2
LL | |     }


note: the above error was encountered while instantiating `fn finish::<[generator@/checkout/src/test/ui/polymorphization/generators.rs:70:5: 73:6], u32, u32>`
   |
   |
LL |     finish(used_const_in_yield::<1u32>());


error: item has unused generic parameters
   |
   |
LL |   pub fn used_const_in_return<const R: u32>() -> impl Generator<(), Yield = u32, Return = u32> + Unpin
   |                               ------------ generic parameter `R` is unused
LL |   {
LL | /     || {
LL | |         yield 4;
LL | |         R
LL | |     }


note: the above error was encountered while instantiating `fn finish::<[generator@/checkout/src/test/ui/polymorphization/generators.rs:79:5: 82:6], u32, u32>`
   |
   |
LL |     finish(used_const_in_return::<1u32>());

error: aborting due to 4 previous errors; 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/polymorphization/type_parameters/functions.rs stdout ----
diff of stderr:

5    |        ^^^^^^ - generic parameter `T` is unused
6 
7 error: item has unused generic parameters
+    |
+    |
+ LL | pub fn used_substs<T>() {
+    |        ^^^^^^^^^^^ - generic parameter `T` is unused
+ 
+ error: item has unused generic parameters
9    |
9    |
10 LL | impl<F: Default> Foo<F> {

31 LL |     pub fn used_fn<G: Default>() {
33 
- error: aborting due to 4 previous errors
- error: aborting due to 4 previous errors
+ error: item has unused generic parameters
+    |
+    |
+ LL | impl<F: Default> Foo<F> {
+    |      - generic parameter `F` is unused
+ LL |     pub fn used_substs() {
+    |            ^^^^^^^^^^^
+ 
+ error: aborting due to 6 previous errors
---
To only update this specific test, also pass `--test-args polymorphization/type_parameters/functions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/polymorphization/type_parameters/functions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/type_parameters/functions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zpolymorphize=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/type_parameters/functions/auxiliary"
stdout: none
--- stderr -------------------------------
error: item has unused generic parameters
   |
   |
LL | pub fn unused<T>() {
   |        ^^^^^^ - generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | pub fn used_substs<T>() {
   |        ^^^^^^^^^^^ - generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | impl<F: Default> Foo<F> {
   |      - generic parameter `F` is unused
LL |     pub fn unused_impl() {
   |            ^^^^^^^^^^^


error: item has unused generic parameters
   |
   |
LL | impl<F: Default> Foo<F> {
   |      - generic parameter `F` is unused
...
LL |     pub fn unused_both<G: Default>() {
   |            ^^^^^^^^^^^ - generic parameter `G` is unused

error: item has unused generic parameters
   |
   |
LL | impl<F: Default> Foo<F> {
   |      - generic parameter `F` is unused
...
LL |     pub fn used_fn<G: Default>() {


error: item has unused generic parameters
   |
   |
LL | impl<F: Default> Foo<F> {
   |      - generic parameter `F` is unused
LL |     pub fn used_substs() {
   |            ^^^^^^^^^^^

error: aborting due to 6 previous errors
---
diff of stderr:

23    |                   ^^^^^^^^^^^^^^
24 
25 error: item has unused generic parameters
+    |
+    |
+ LL | pub fn used_substs<T>() -> u32 {
+    |                    - generic parameter `T` is unused
+ LL |     let x = || unused::<T>();
+ 
+ 
+ error: item has unused generic parameters
+    |
+    |
+ LL | pub fn used_substs<T>() -> u32 {
+    |        ^^^^^^^^^^^ - generic parameter `T` is unused
+ 
+ error: item has unused generic parameters
27    |
27    |
28 LL | impl<F: Default> Foo<F> {

86 LL |     pub fn used_fn<G: Default>() -> u32 {
88 
- error: aborting due to 9 previous errors
- error: aborting due to 9 previous errors
+ error: item has unused generic parameters
+    |
+    |
+ LL | impl<F: Default> Foo<F> {
+    |      - generic parameter `F` is unused
+ ...
+ LL |         let x = || unused::<F>();
+ 
+ 
+ error: item has unused generic parameters
+    |
+    |
+ LL | impl<F: Default> Foo<F> {
+    |      - generic parameter `F` is unused
+ LL |     pub fn used_substs() -> u32 {
+    |            ^^^^^^^^^^^
+ 
+ error: aborting due to 13 previous errors
---
To only update this specific test, also pass `--test-args polymorphization/type_parameters/closures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/polymorphization/type_parameters/closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/type_parameters/closures" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zpolymorphize=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/type_parameters/closures/auxiliary"
stdout: none
--- stderr -------------------------------
error: item has unused generic parameters
   |
   |
LL | pub fn unused<T>() -> u32 {
   |               - generic parameter `T` is unused
...
LL |     let add_one = |x: u32| x + 1;


error: item has unused generic parameters
   |
   |
LL | pub fn unused<T>() -> u32 {
   |        ^^^^^^ - generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | pub fn used_parent<T: Default>() -> u32 {
   |                    - generic parameter `T` is unused
LL |     let _: T = Default::default();
LL |     let add_one = |x: u32| x + 1;


error: item has unused generic parameters
   |
   |
LL | pub fn used_substs<T>() -> u32 {
   |                    - generic parameter `T` is unused
LL |     let x = || unused::<T>();


error: item has unused generic parameters
   |
   |
LL | pub fn used_substs<T>() -> u32 {
   |        ^^^^^^^^^^^ - generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | impl<F: Default> Foo<F> {
   |      - generic parameter `F` is unused
...
LL |     pub fn unused_all<G: Default>() -> u32 {
   |                       - generic parameter `G` is unused
LL |         //~^ ERROR item has unused generic parameters
LL |         let add_one = |x: u32| x + 1;


error: item has unused generic parameters
   |
   |
LL | impl<F: Default> Foo<F> {
   |      - generic parameter `F` is unused
...
LL |     pub fn unused_all<G: Default>() -> u32 {
   |            ^^^^^^^^^^ - generic parameter `G` is unused

error: item has unused generic parameters
   |
   |
LL |       pub fn used_impl<G: Default>() -> u32 {
   |                        - generic parameter `G` is unused
LL |           //~^ ERROR item has unused generic parameters
LL |           let add_one = |x: u32| {
   |  _______________________^
LL | |             //~^ ERROR item has unused generic parameters
LL | |             let _: F = Default::default();
LL | |             x + 1
LL | |         };


error: item has unused generic parameters
   |
   |
LL |     pub fn used_impl<G: Default>() -> u32 {
   |            ^^^^^^^^^ - generic parameter `G` is unused

error: item has unused generic parameters
   |
   |
LL |   impl<F: Default> Foo<F> {
   |        - generic parameter `F` is unused
...
LL |           let add_one = |x: u32| {
   |  _______________________^
LL | |             //~^ ERROR item has unused generic parameters
LL | |             let _: G = Default::default();
LL | |             x + 1
LL | |         };


error: item has unused generic parameters
   |
   |
LL | impl<F: Default> Foo<F> {
   |      - generic parameter `F` is unused
...
LL |     pub fn used_fn<G: Default>() -> u32 {


error: item has unused generic parameters
   |
   |
LL | impl<F: Default> Foo<F> {
   |      - generic parameter `F` is unused
...
LL |         let x = || unused::<F>();


error: item has unused generic parameters
   |
   |
LL | impl<F: Default> Foo<F> {
   |      - generic parameter `F` is unused
LL |     pub fn used_substs() -> u32 {
   |            ^^^^^^^^^^^

error: aborting due to 13 previous errors
