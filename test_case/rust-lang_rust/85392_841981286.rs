plain
.......i......................................i..................................................... 10700/11879
.................................................................................................... 10800/11879
.................................................................................................... 10900/11879
.................................................................................................... 11000/11879
..FF..........................................................................F..................... 11100/11879
.................................................................................................... 11300/11879
.................................................................................................... 11400/11879
.................................................................................................... 11500/11879
.................................................................................................... 11600/11879
---
---- [ui] ui/consts/const-eval/const-eval-query-stack.rs stdout ----
diff of stderr:

19    |
20 LL |     let x: &'static i32 = &X;
21    |                            ^ referenced constant has errors
+ note: You've met with a terrible fate, haven't you?
22 query stack during panic:
22 query stack during panic:
23 #0 [normalize_mir_const_after_erasing_regions] normalizing `main::promoted[1]`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
24 #1 [optimized_mir] optimizing MIR for `main`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/const-eval-query-stack.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-query-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
   |
LL | const X: i32 = 1 / 0; //~WARN any use of this value will cause an error
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:19:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:24:28
   |
LL |     let x: &'static i32 = &X;
   |                            ^ referenced constant has errors
error: internal compiler error: ty::ConstKind::Error constructed but no error reported.


thread 'rustc' panicked at 'aborting after 2 errors due to `-Z treat-err-as-bug=2`', compiler/rustc_errors/src/lib.rs:1039:36
   0: rust_begin_unwind
   1: std::panicking::begin_panic_fmt
   2: rustc_errors::HandlerInner::emit_diagnostic
   3: rustc_errors::HandlerInner::emit_diag_at_span
   3: rustc_errors::HandlerInner::emit_diag_at_span
   4: rustc_errors::HandlerInner::span_bug
   5: rustc_session::session::Session::delay_span_bug
   6: rustc_middle::ty::context::TyCtxt::const_error
   7: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
   8: rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::super_fold_with
   9: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const
  10: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
  11: rustc_infer::infer::InferCtxtBuilder::enter
  12: core::ops::function::FnOnce::call_once
  13: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::normalize_mir_const_after_erasing_regions>::compute
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  16: rustc_data_structures::stack::ensure_sufficient_stack
  17: rustc_query_system::query::plumbing::force_query_with_job
  18: rustc_query_system::query::plumbing::get_query_impl
  19: rustc_query_system::query::plumbing::get_query
  20: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions
  21: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
  22: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions
  23: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand
  24: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
  25: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_statement
  26: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_body
  27: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  29: rustc_mir::transform::optimized_mir
  29: rustc_mir::transform::optimized_mir
  30: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  31: rustc_data_structures::stack::ensure_sufficient_stack
  32: rustc_query_system::query::plumbing::force_query_with_job
  33: rustc_query_system::query::plumbing::get_query_impl
  34: rustc_query_system::query::plumbing::get_query
  35: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir
  36: rustc_mir::monomorphize::collector::collect_neighbours
  37: rustc_data_structures::stack::ensure_sufficient_stack
  38: rustc_mir::monomorphize::collector::collect_items_rec
  39: rustc_session::utils::<impl rustc_session::session::Session>::time
  40: rustc_mir::monomorphize::collector::collect_crate_mono_items
  41: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  42: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::collect_and_partition_mono_items>::compute
  43: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  44: rustc_data_structures::stack::ensure_sufficient_stack
  45: rustc_query_system::query::plumbing::force_query_with_job
  46: rustc_query_system::query::plumbing::get_query_impl
  47: rustc_query_system::query::plumbing::get_query
  48: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  49: rustc_codegen_ssa::base::codegen_crate
  50: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  51: rustc_session::utils::<impl rustc_session::session::Session>::time
  52: rustc_interface::queries::Queries::ongoing_codegen
  53: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  54: rustc_span::with_source_map
  56: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic
error: internal compiler error: unexpected panic

note: You've met with a terrible fate, haven't you?
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.54.0-nightly (5eb7427d2 2021-05-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -Z treat-err-as-bug=2 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [normalize_mir_const_after_erasing_regions] normalizing `main::promoted[1]`
#1 [optimized_mir] optimizing MIR for `main`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items

------------------------------------------



---- [ui] ui/proc-macro/load-panic-backtrace.rs stdout ----
diff of stderr:

1 at 'panic-derive', $DIR/auxiliary/test-macros.rs:43:5
+ note: You've met with a terrible fate, haven't you?
2 error: proc-macro derive panicked
3   --> $DIR/load-panic-backtrace.rs:20:10
4    |

---
To only update this specific test, also pass `--test-args proc-macro/load-panic-backtrace.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/load-panic-backtrace.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/load-panic-backtrace" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "proc-macro-backtrace" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/load-panic-backtrace/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'panic-derive', /checkout/src/test/ui/proc-macro/auxiliary/test-macros.rs:43:5

error: internal compiler error: unexpected panic


note: You've met with a terrible fate, haven't you?
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.54.0-nightly (5eb7427d2 2021-05-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -Z proc-macro-backtrace -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: proc-macro derive panicked
  --> /checkout/src/test/ui/proc-macro/load-panic-backtrace.rs:20:10
  --> /checkout/src/test/ui/proc-macro/load-panic-backtrace.rs:20:10
   |
LL | #[derive(Panic)]
   |
   = help: message: panic-derive

error: aborting due to previous error
---

6 
7 error: internal compiler error: unexpected panic
8 
+ note: 
+ There was a night when winds from unknown spaces whirled us irresistibly into
+ limitless vacuum beyond all thought and entity. Perceptions of the most
+ maddeningly untransmissible sort thronged upon us; perceptions of infinity
+ which at the time convulsed us with joy, yet which are now partly lost to my
+ memory and partly incapable of presentation to others.
9 query stack during panic:
9 query stack during panic:
10 #0 [eval_to_allocation_raw] const-evaluating + checking `C`
11 #1 [eval_to_allocation_raw] const-evaluating + checking `C`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/err.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args treat-err-as-bug/err.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/treat-err-as-bug/err.rs:11:21
   |
LL | pub static C: u32 = 0 - 1;
   |                     ^^^^^ attempt to compute `0_u32 - 1_u32`, which would overflow

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1037:27

error: internal compiler error: unexpected panic

note: 
note: 
There was a night when winds from unknown spaces whirled us irresistibly into
limitless vacuum beyond all thought and entity. Perceptions of the most
maddeningly untransmissible sort thronged upon us; perceptions of infinity
which at the time convulsed us with joy, yet which are now partly lost to my
memory and partly incapable of presentation to others.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.54.0-nightly (5eb7427d2 2021-05-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -Z treat-err-as-bug -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `C`
#1 [eval_to_allocation_raw] const-evaluating + checking `C`

------------------------------------------



---- [ui] ui/treat-err-as-bug/delay_span_bug.rs stdout ----
diff of stderr:

6 
7 error: internal compiler error: unexpected panic
8 
+ note: 
+ There was a night when winds from unknown spaces whirled us irresistibly into
+ limitless vacuum beyond all thought and entity. Perceptions of the most
+ maddeningly untransmissible sort thronged upon us; perceptions of infinity
+ which at the time convulsed us with joy, yet which are now partly lost to my
+ memory and partly incapable of presentation to others.
9 query stack during panic:
9 query stack during panic:
10 #0 [trigger_delay_span_bug] trigger a delay span bug


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/delay_span_bug.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/delay_span_bug.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args treat-err-as-bug/delay_span_bug.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/delay_span_bug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: delayed span bug triggered by #[rustc_error(delay_span_bug_from_inside_query)]
   |
LL | fn main() {}
   | ^^^^^^^^^


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1037:27

error: internal compiler error: unexpected panic

note: 
note: 
There was a night when winds from unknown spaces whirled us irresistibly into
limitless vacuum beyond all thought and entity. Perceptions of the most
maddeningly untransmissible sort thronged upon us; perceptions of infinity
which at the time convulsed us with joy, yet which are now partly lost to my
memory and partly incapable of presentation to others.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.54.0-nightly (5eb7427d2 2021-05-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -Z treat-err-as-bug -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [trigger_delay_span_bug] trigger a delay span bug

------------------------------------------



---- [ui] ui/type-alias-impl-trait/associated-type-lifetime-ice.rs stdout ----
diff of stderr:

9    |
10    = error: internal compiler error: unexpected panic
11 
+ note: 
+ There was a night when winds from unknown spaces whirled us irresistibly into
+ limitless vacuum beyond all thought and entity. Perceptions of the most
+ maddeningly untransmissible sort thronged upon us; perceptions of infinity
+ which at the time convulsed us with joy, yet which are now partly lost to my
+ memory and partly incapable of presentation to others.
12 query stack during panic:
13 end of query stack
14 

---
To only update this specific test, also pass `--test-args type-alias-impl-trait/associated-type-lifetime-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/associated-type-lifetime-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/associated-type-lifetime-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/associated-type-lifetime-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: unexpected concrete region in borrowck: ReEarlyBound(0, 'a)
   |
   |
LL | /     fn f() -> Self::ImplTrait {
LL | |     //~^ ERROR unexpected concrete region in borrowck: ReEarlyBound(0, 'a)
LL | |         ()
LL | |     }
   |
   = note: delayed at compiler/rustc_mir/src/borrow_check/region_infer/opaque_types.rs:83:44


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1021:13

error: internal compiler error: unexpected panic

note: 
note: 
There was a night when winds from unknown spaces whirled us irresistibly into
limitless vacuum beyond all thought and entity. Perceptions of the most
maddeningly untransmissible sort thronged upon us; perceptions of infinity
which at the time convulsed us with joy, yet which are now partly lost to my
memory and partly incapable of presentation to others.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.54.0-nightly (5eb7427d2 2021-05-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type rlib
query stack during panic:
end of query stack

------------------------------------------
---
test result: FAILED. 11777 passed; 5 failed; 97 ignored; 0 measured; 0 filtered out; finished in 115.32s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:46
