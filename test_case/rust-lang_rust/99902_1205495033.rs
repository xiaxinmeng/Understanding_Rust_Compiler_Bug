plain
.............................................i..ii...................................... 8536/13320
........................ii.............................................................. 8624/13320
..............................iiii...................................................... 8712/13320
...................................................................i.................... 8800/13320
...F..F.F...........i........FF......................................................... 8888/13320
.............................................................................i.......... 9064/13320
........................................................................................ 9152/13320
...................................................................................i.... 9240/13320
........................................................................................ 9328/13320
........................................................................................ 9328/13320
........................................................................................ 9416/13320
........................................................................................ 9504/13320
........................................................................................ 9592/13320
........................................................................................ 9680/13320
........................................................................................ 9768/13320
......................................................................................ii 9856/13320
....F..........i............................................................ii..F....... 9944/13320
........................................................................................ 10120/13320
........................................................................................ 10208/13320
........................................................................................ 10296/13320
........................................................................................ 10384/13320
---
........................................................................................ 11528/13320
........................................................................................ 11616/13320
........................................................................................ 11704/13320
...................................i........i........i.....i.....................i...... 11792/13320
.......................F..F.F........................................................... 11880/13320
........................................................................................ 12056/13320
........................................................................................ 12144/13320
........................................................................................ 12232/13320
................................................................................FF...... 12320/13320
---
28    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
29    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
30 
- query stack during panic:
+ RUST_BACKTRACE=full` for a verbose backtrace.query stack during panic:
32 #0 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
33 #1 [optimized_mir] optimizing MIR for `main`
34 #2 [collect_and_partition_mono_items] collect_and_partition_mono_items

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/const-eval-query-stack.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-query-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:19:16
   |
   |
LL | const X: i32 = 1 / 0; //~WARN any use of this value will cause an error
   | ------------   ^^^^^ attempt to divide `1_i32` by zero
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:18:8
   |
LL | #[warn(const_err)]
---
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

thread 'rustc' panicked at 'aborting after 2 errors due to `-Z treat-err-as-bug=2`', compiler/rustc_errors/src/lib.rs:1452:36
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   1: core::panicking::panic_fmt
   2: <rustc_errors::HandlerInner>::panic_if_treat_err_as_bug
   4: <rustc_errors::Handler>::emit_diagnostic
   4: <rustc_errors::Handler>::emit_diagnostic
   5: <() as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
   6: <<rustc_const_eval::const_eval::error::ConstEvalErr>::struct_generic<<rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint::{closure#0}>::{closure#2} as core::ops::function::FnOnce<(rustc_errors::diagnostic_builder::LintDiagnosticBuilder<()>,)>>::call_once::{shim:vtable#0}
   7: <alloc::boxed::Box<dyn for<'a> core::ops::function::FnOnce<(rustc_errors::diagnostic_builder::LintDiagnosticBuilder<'a, ()>,), Output = ()>> as core::ops::function::FnOnce<(rustc_errors::diagnostic_builder::LintDiagnosticBuilder<()>,)>>::call_once
   8: rustc_middle::lint::struct_lint_level::struct_lint_level_impl
   9: rustc_middle::lint::struct_lint_level::<<rustc_const_eval::const_eval::error::ConstEvalErr>::struct_generic<<rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint::{closure#0}>::{closure#2}>
  10: <rustc_middle::ty::context::TyCtxt>::struct_span_lint_hir::<rustc_span::span_encoding::Span, <rustc_const_eval::const_eval::error::ConstEvalErr>::struct_generic<<rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint::{closure#0}>::{closure#2}>
  11: <rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint
  12: <rustc_mir_transform::const_prop_lint::ConstPropagator>::eval_constant
  13: <rustc_mir_transform::const_prop_lint::ConstPropagator as rustc_middle::mir::visit::Visitor>::visit_constant
  14: <rustc_mir_transform::const_prop_lint::ConstPropagator as rustc_middle::mir::visit::Visitor>::visit_statement
  15: <rustc_mir_transform::const_prop_lint::ConstPropagator as rustc_middle::mir::visit::Visitor>::visit_body
  16: <rustc_mir_transform::const_prop_lint::ConstProp as rustc_mir_transform::pass_manager::MirLint>::run_lint
  17: rustc_mir_transform::pass_manager::run_passes
  18: rustc_mir_transform::run_post_borrowck_cleanup_passes
  19: rustc_mir_transform::mir_drops_elaborated_and_const_checked
  20: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>>
  21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::mir::Body>>
  25: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
  26: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
  27: <rustc_middle::ty::context::TyCtxt>::instance_mir
  27: <rustc_middle::ty::context::TyCtxt>::instance_mir
  28: rustc_monomorphize::collector::collect_neighbours
  29: rustc_monomorphize::collector::collect_items_rec
  30: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
  31: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  32: rustc_data_structures::sync::par_for_each_in::<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  33: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
  34: rustc_monomorphize::collector::collect_crate_mono_items
  35: rustc_monomorphize::partitioning::collect_and_partition_mono_items
  36: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>>
  37: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
  38: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  39: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
  40: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  41: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  42: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorGuaranteed>>
  43: <rustc_interface::queries::Queries>::ongoing_codegen
  45: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  46: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
  47: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (cd42edd68 2022-08-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=2
query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#1 [optimized_mir] optimizing MIR for `main`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
Future incompatibility report: Future breakage diagnostic:
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:19:16
   |
   |
LL | const X: i32 = 1 / 0; //~WARN any use of this value will cause an error
   | ------------   ^^^^^ attempt to divide `1_i32` by zero
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:18:8
   |
LL | #[warn(const_err)]
---
To only update this specific test, also pass `--test-args hrtb/issue-95034.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-95034.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-95034" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-95034/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/hrtb/issue-95034.rs:93:49: 95:6]>', compiler/rustc_traits/src/normalize_erasing_regions.rs:51:17
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (cd42edd68 2022-08-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
query stack during panic:
#0 [try_normalize_generic_arg_after_erasing_regions] normalizing `impl core::future::future::Future<Output = ()>`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `test::{closure#0}`
------------------------------------------


---- [ui] src/test/ui/issues/issue-87707.rs stdout ----
---- [ui] src/test/ui/issues/issue-87707.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'Here Once instance is poisoned.', $DIR/issue-87707.rs:13:24
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
- thread 'main' panicked at 'Once instance has previously been poisoned', $DIR/issue-87707.rs:15:7
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.thread 'main' panicked at 'Once instance has previously been poisoned', $DIR/issue-87707.rs:15:7


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87707/issue-87707.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87707/a"
stdout: none
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'Here Once instance is poisoned.', /checkout/src/test/ui/issues/issue-87707.rs:13:24
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.thread 'main' panicked at 'Once instance has previously been poisoned', /checkout/src/test/ui/issues/issue-87707.rs:15:7


---- [ui] src/test/ui/panics/issue-47429-short-backtraces.rs#legacy stdout ----
diff of run.stderr:
---
+ RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.legacy/issue-47429-short-backtraces.legacy.run.stderr

error in revision `legacy`: 1 errors occurred comparing run output.
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.legacy/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/issue-47429-short-backtraces.rs:22:5
---
RUST_BACKTRACE=full` for a verbose backtrace.
------------------------------------------


---- [ui] src/test/ui/panics/issue-47429-short-backtraces.rs#v0 stdout ----
diff of run.stderr:
2 stack backtrace:
3    0: std::panicking::begin_panic::<&str>
4    1: issue_47429_short_backtraces::main
- note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
- note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
+ note: Some details are omitted, run with `
+ RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.v0/issue-47429-short-backtraces.v0.run.stderr

error in revision `v0`: 1 errors occurred comparing run output.
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.v0/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/issue-47429-short-backtraces.rs:22:5
---

---- [ui] src/test/ui/panics/location-detail-panic-no-location-info.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'no location info', <redacted>:0:0
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
3 



The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-location-info/location-detail-panic-no-location-info.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-location-info/a"
stdout: none
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'no location info', <redacted>:0:0
------------------------------------------


---- [ui] src/test/ui/panics/runtime-switch.rs#legacy stdout ----
---
+ RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.legacy/runtime-switch.legacy.run.stderr

error in revision `legacy`: 1 errors occurred comparing run output.
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.legacy/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/runtime-switch.rs:25:5
---
RUST_BACKTRACE=full` for a verbose backtrace.
------------------------------------------


---- [ui] src/test/ui/panics/runtime-switch.rs#v0 stdout ----
diff of run.stderr:
2 stack backtrace:
3    0: std::panicking::begin_panic::<&str>
4    1: runtime_switch::main
- note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
- note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
+ note: Some details are omitted, run with `
+ RUST_BACKTRACE=full` for a verbose backtrace.
6 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.v0/runtime-switch.v0.run.stderr

error in revision `v0`: 1 errors occurred comparing run output.
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/runtime-switch.v0/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/runtime-switch.rs:25:5
---
1 at 'panic-derive', $DIR/auxiliary/test-macros.rs:43:5
- error: proc-macro derive panicked
-   --> $DIR/load-panic-backtrace.rs:10:10
-    |
- LL | #[derive(Panic)]
-    |
-    = help: message: panic-derive
- 
10 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args proc-macro/load-panic-backtrace.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/load-panic-backtrace.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/load-panic-backtrace" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "proc-macro-backtrace" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/load-panic-backtrace/auxiliary"
stdout: none
--- stderr -------------------------------
thread '<unnamed>' panicked at 'panic-derive', /checkout/src/test/ui/proc-macro/auxiliary/test-macros.rs:43:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.{"message":"proc-macro derive panicked","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/load-panic-backtrace.rs","byte_start":266,"byte_end":271,"line_start":10,"line_end":10,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"#[derive(Panic)]","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"message: panic-derive","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: proc-macro derive panicked\n  --> /checkout/src/test/ui/proc-macro/load-panic-backtrace.rs:10:10\n   |\nLL | #[derive(Panic)]\n   |          ^^^^^\n   |\n   = help: message: panic-derive\n\n"}
------------------------------------------


---- [ui] src/test/ui/process/multi-panic.rs stdout ----
---- [ui] src/test/ui/process/multi-panic.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/multi-panic/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Some("note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/process/multi-panic.rs:25:9")`,
 right: `Some("note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.")`', /checkout/src/test/ui/process/multi-panic.rs:12:5
------------------------------------------


---- [ui] src/test/ui/test-attrs/test-panic-abort-nocapture.rs stdout ----
---
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.testing321
10 


The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/test-panic-abort-nocapture.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/a" "--test-threads=1" "--nocapture"
running 4 tests
test it_fails ... about to fail
FAILED
test it_panics - should panic ... about to panic
---
ok

failures:

---- it_fails stdout ----
---- it_fails stderr ----

failures:
    it_fails

---
26     it_exits
27     it_fails


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/test-panic-abort.run.stdout
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/a" "--test-threads=1"
running 5 tests
test it_exits ... FAILED
test it_fails ... FAILED
test it_panics - should panic ... ok
test it_panics - should panic ... ok
test it_works ... ok
test no_residual_environment ... ok

failures:

---- it_exits stdout ----
---- it_exits stderr ----
note: got unexpected return code 123
---- it_fails stdout ----
testing123
testing123
---- it_fails stderr ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `5`', /checkout/src/test/ui/test-attrs/test-panic-abort.rs:34:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
18     thready_fail
19 


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-capture/test-thread-capture.run.stdout
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-capture/a" "--test-threads=1"
running 2 tests
test thready_fail ... FAILED
test thready_pass ... ok

---
To only update this specific test, also pass `--test-args treat-err-as-bug/delay_span_bug.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/delay_span_bug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: delayed span bug triggered by #[rustc_error(delay_span_bug_from_inside_query)]
   |
LL | fn main() {}
   | ^^^^^^^^^


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1450:27
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (cd42edd68 2022-08-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug
query stack during panic:
query stack during panic:
#0 [trigger_delay_span_bug] trigger a delay span bug
------------------------------------------


---- [ui] src/test/ui/treat-err-as-bug/err.rs stdout ----
---- [ui] src/test/ui/treat-err-as-bug/err.rs stdout ----
diff of stderr:

4 LL | pub static C: u32 = 0 - 1;
6 
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
7 error: internal compiler error: unexpected panic
8 
---
To only update this specific test, also pass `--test-args treat-err-as-bug/err.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: could not evaluate static initializer
   |
   |
LL | pub static C: u32 = 0 - 1;


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1450:27
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (cd42edd68 2022-08-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug
query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `C`
#1 [eval_to_allocation_raw] const-evaluating + checking `C`
------------------------------------------



