plain
.................................................................................................... 9000/11191
.................................................................................................... 9100/11191
..................................................................................i......i.......... 9200/11191
.................................................................................................... 9300/11191
.....................iiiiii..iiiiii.i............................................................... 9400/11191
..............test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
...................................................................................... 9600/11191
.................................................................................................... 9700/11191
.................................................................................................... 9800/11191
---
 finished in 0.484 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.075 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 126 tests
.........................................................F.F..F..F.F..............FFF.F.......F..... 100/126
......F..FF........FF.....
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [incremental] incremental/issue-35593.rs stdout ----


error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-35593.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/issue-35593.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(issue_35593[317d]::main)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
------------------------------------------


---- [incremental] incremental/inlined_hir_34991/main.rs stdout ----
---- [incremental] incremental/inlined_hir_34991/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/inlined_hir_34991/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/inlined_hir_34991/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/inlined_hir_34991/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/inlined_hir_34991/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(main[317d]::foo)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `foo`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
------------------------------------------


---- [incremental] incremental/hygiene/load_cached_hygiene.rs stdout ----
---- [incremental] incremental/hygiene/load_cached_hygiene.rs stdout ----

error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs" failed to compile: 
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/load_cached_hygiene.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(cached_hygiene[8787]::unchanged_fn)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `unchanged_fn`
end of query stack
------------------------------------------


---- [incremental] incremental/issue-42602.rs stdout ----
---- [incremental] incremental/issue-42602.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-42602.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/issue-42602.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(issue_42602[317d]::b::bar)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `b::bar`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
------------------------------------------


---- [incremental] incremental/issue-49595/issue-49595.rs stdout ----
---- [incremental] incremental/issue-49595/issue-49595.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-49595/issue-49595.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue-49595/issue-49595.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue-49595" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue-49595/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(issue_49595[317d]::lit_test::lit_test)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `lit_test::lit_test`
#1 [eval_to_allocation_raw] const-evaluating + checking `lit_test::lit_test`
end of query stack
------------------------------------------


---- [incremental] incremental/issue-79890-imported-crates-changed.rs stdout ----
---- [incremental] incremental/issue-79890-imported-crates-changed.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-79890-imported-crates-changed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-79890-imported-crates-changed/issue-79890-imported-crates-changed.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-79890-imported-crates-changed/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "issue_79890" "--test" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-79890-imported-crates-changed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused import: `issue_79890::MyTrait`
   |
   |
LL | #[cfg(rpass2)] use issue_79890::MyTrait;
   |
   = note: `#[warn(unused_imports)]` on by default


thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(issue_79890_imported_crates_changed[317d]::main)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack


------------------------------------------



---- [incremental] incremental/krate-inlined.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate-inlined.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/krate-inlined.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(krate_inlined[317d]::x::method)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `x::method`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
------------------------------------------


---- [incremental] incremental/incremental_proc_macro.rs stdout ----
---- [incremental] incremental/incremental_proc_macro.rs stdout ----

error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs" failed to compile: 
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/incremental_proc_macro.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/incremental_proc_macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused variable: `input`
  --> /checkout/src/test/incremental/auxiliary/incremental_proc_macro_aux.rs:15:15
   |
LL | pub fn derive(input: TokenStream) -> TokenStream {
   |               ^^^^^ help: if this is intentional, prefix it with an underscore: `_input`
   = note: `#[warn(unused_variables)]` on by default


thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(incremental_proc_macro_aux[8787]::_)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `_`
end of query stack
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
 right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1437:21
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: 1 warning emitted



------------------------------------------


---- [incremental] incremental/remapped_paths_cc/main.rs stdout ----

error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/remapped_paths_cc/auxiliary/extern_crate.rs" failed to compile: 
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remapped_paths_cc/auxiliary/extern_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(extern_crate[8787]::inline_fn)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `inline_fn`
end of query stack
------------------------------------------


---- [incremental] incremental/remove-private-item-cross-crate/main.rs stdout ----
---- [incremental] incremental/remove-private-item-cross-crate/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remove-private-item-cross-crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(main[317d]::main)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
------------------------------------------


---- [incremental] incremental/static_refering_to_other_static3/issue.rs stdout ----
---- [incremental] incremental/static_refering_to_other_static3/issue.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/static_refering_to_other_static3/issue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_refering_to_other_static3/issue/issue.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_refering_to_other_static3/issue/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_refering_to_other_static3/issue/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for promoted_mir(issue[317d]::main)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [promoted_mir] optimizing promoted MIR for `main`
end of query stack
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
 right: `Codegenning`', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1437:21
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [incremental] incremental/thinlto/cgu_invalidated_when_import_added.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_invalidated_when_import_added.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_when_import_added/cgu_invalidated_when_import_added.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_when_import_added" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-Zhuman-readable-cgu-names" "-Cllvm-args=-import-instr-limit=10" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_when_import_added/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(cgu_invalidated_when_import_added[317d]::foo::bar)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z human-readable-cgu-names -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 -C llvm-args=-import-instr-limit=10
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `foo::bar`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
------------------------------------------


---- [incremental] incremental/thinlto/cgu_invalidated_when_import_removed.rs stdout ----
---- [incremental] incremental/thinlto/cgu_invalidated_when_import_removed.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_invalidated_when_import_removed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_when_import_removed/cgu_invalidated_when_import_removed.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_when_import_removed" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-Zhuman-readable-cgu-names" "-Cllvm-args=-import-instr-limit=10" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_when_import_removed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(cgu_invalidated_when_import_removed[317d]::foo::bar)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (671502ef1 2020-12-21) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z human-readable-cgu-names -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 -C llvm-args=-import-instr-limit=10
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `foo::bar`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
------------------------------------------


---- [incremental] incremental/thinlto/cgu_invalidated_when_export_added.rs stdout ----
---- [incremental] incremental/thinlto/cgu_invalidated_when_export_added.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_invalidated_when_export_added.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_when_export_added/cgu_invalidated_when_export_added.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_when_export_added" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_when_export_added/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'found unstable fingerprints for optimized_mir(cgu_invalidated_when_export_added[317d]::run::FOO::__getit)', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:566:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
---
test result: FAILED. 111 passed; 15 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.14s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "incremental" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:06
