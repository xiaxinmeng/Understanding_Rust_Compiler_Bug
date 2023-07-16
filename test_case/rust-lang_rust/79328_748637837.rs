plain
.................................................................................................... 1900/11189
.................................................................................................... 2000/11189
.................................................................................................... 2100/11189
.................................................................................................... 2200/11189
...........................F..F..................................................................... 2300/11189
...........................................................F..F..................................... 2400/11189
..................................i..i.............................................................. 2500/11189
.....................................iiiii.......................................................... 2700/11189
.................................................................................................... 2800/11189
.................................................................................................... 2900/11189
.................................................................................................... 3000/11189
---
failures:

---- [ui] ui/consts/control-flow/assert.rs#stock stdout ----

error in revision `stock`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/assert.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/assert.stock" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/assert.stock/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:488:41: couldn't find hir id HirId { owner: DefId(0:0 ~ assert[317d]), local_id: 0 } in the HIR map
thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (a34176597 2020-12-20) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `_`
#1 [check_mod_item_types] checking item types in top-level module
end of query stack


------------------------------------------



---- [ui] ui/consts/control-flow/assert.rs#const_panic stdout ----

error in revision `const_panic`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/assert.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "const_panic" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/assert.const_panic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/assert.const_panic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:488:41: couldn't find hir id HirId { owner: DefId(0:0 ~ assert[317d]), local_id: 0 } in the HIR map
thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (a34176597 2020-12-20) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `_`
#1 [check_mod_item_types] checking item types in top-level module
end of query stack


------------------------------------------



---- [ui] ui/consts/ptr_comparisons.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/ptr_comparisons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:488:41: couldn't find hir id HirId { owner: DefId(0:0 ~ ptr_comparisons[8787]), local_id: 0 } in the HIR map
thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (a34176597 2020-12-20) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `_`
#1 [check_mod_item_types] checking item types in top-level module
end of query stack


------------------------------------------



---- [ui] ui/consts/ptr_is_null.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/ptr_is_null.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_is_null" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_is_null/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:488:41: couldn't find hir id HirId { owner: DefId(0:0 ~ ptr_is_null[8787]), local_id: 0 } in the HIR map
thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.50.0-nightly (a34176597 2020-12-20) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `_`
#1 [check_mod_item_types] checking item types in top-level module
end of query stack


------------------------------------------



---- [ui] ui/expr/if/if-without-else-as-fn-expr.rs stdout ----
diff of stderr:

1 error[E0317]: `if` may be missing an `else` clause
3    |
3    |
- LL |   fn foo(bar: usize) -> usize {
-    |                         ----- expected `usize` because of this return type
6 LL | /     if bar % 5 == 0 {
7 LL | |         return 3;
8 LL | |     }

-    | |_____^ expected `usize`, found `()`
+    | |_____^ expected `()`, found `usize`
10    |
11    = note: `if` expressions without `else` evaluate to `()`
12    = help: consider adding an `else` block that evaluates to the expected type
15   --> $DIR/if-without-else-as-fn-expr.rs:9:20
16    |
16    |
17 LL |       let x: usize = if bar % 5 == 0 {
-    |  _________-__________^
-    | |         expected because of this assignment
+    |  ____________________^
21 LL | |         return 3;
22 LL | |     };
22 LL | |     };
-    | |_____^ expected `usize`, found `()`
+    | |_____^ expected `()`, found `usize`
24    |
25    = note: `if` expressions without `else` evaluate to `()`
26    = help: consider adding an `else` block that evaluates to the expected type

28 error[E0317]: `if` may be missing an `else` clause
30    |
30    |
- LL |   fn foo3(bar: usize) -> usize {
-    |                          ----- expected `usize` because of this return type
33 LL | /     if bar % 5 == 0 {
34 LL | |         3
+    | |         - found here
35 LL | |     }
-    | |_____^ expected `usize`, found `()`
+    | |_____^ expected `()`, found `usize`
37    |
38    = note: `if` expressions without `else` evaluate to `()`
39    = help: consider adding an `else` block that evaluates to the expected type

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/if/if-without-else-as-fn-expr/if-without-else-as-fn-expr.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/if/if-without-else-as-fn-expr/if-without-else-as-fn-expr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args expr/if/if-without-else-as-fn-expr.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/expr/if/if-without-else-as-fn-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/if/if-without-else-as-fn-expr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/if/if-without-else-as-fn-expr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0317]: `if` may be missing an `else` clause
   |
   |
LL | /     if bar % 5 == 0 {
LL | |         return 3;
LL | |     }
   | |_____^ expected `()`, found `usize`
   |
   = note: `if` expressions without `else` evaluate to `()`
   = help: consider adding an `else` block that evaluates to the expected type

error[E0317]: `if` may be missing an `else` clause
   |
   |
LL |       let x: usize = if bar % 5 == 0 {
LL | |         return 3;
LL | |     };
LL | |     };
   | |_____^ expected `()`, found `usize`
   |
   = note: `if` expressions without `else` evaluate to `()`
   = help: consider adding an `else` block that evaluates to the expected type

error[E0317]: `if` may be missing an `else` clause
   |
   |
LL | /     if bar % 5 == 0 {
LL | |         3
   | |         - found here
LL | |     }
   | |_____^ expected `()`, found `usize`
   |
   = note: `if` expressions without `else` evaluate to `()`
   = help: consider adding an `else` block that evaluates to the expected type

error[E0317]: `if` may be missing an `else` clause
   |
   |
LL |   fn foo_let(bar: usize) -> usize {
   |                             ----- expected `usize` because of this return type
LL | /     if let 0 = 1 {
LL | |         return 3;
LL | |     }
   | |_____^ expected `usize`, found `()`
   |
   = note: `if` expressions without `else` evaluate to `()`
   = help: consider adding an `else` block that evaluates to the expected type

error[E0317]: `if` may be missing an `else` clause
   |
   |
LL |       let x: usize = if let 0 = 1 {
   |  _________-__________^
   | |         expected because of this assignment
LL | |         return 3;
LL | |     };
LL | |     };
   | |_____^ expected `usize`, found `()`
   |
   = note: `if` expressions without `else` evaluate to `()`
   = help: consider adding an `else` block that evaluates to the expected type

error[E0317]: `if` may be missing an `else` clause
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |   fn foo3_let(bar: usize) -> usize {
   |                              ----- expected `usize` because of this return type
LL | /     if let 0 = 1 {
LL | |         3
LL | |     }
   | |_____^ expected `usize`, found `()`
   |
   = note: `if` expressions without `else` evaluate to `()`
   = help: consider adding an `else` block that evaluates to the expected type
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0317`.

---
test result: FAILED. 11100 passed; 5 failed; 84 ignored; 0 measured; 0 filtered out; finished in 144.81s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:16
