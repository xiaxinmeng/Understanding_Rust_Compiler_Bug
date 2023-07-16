plain
.................................................................................................... 9100/11269
.................................................................................................... 9200/11269
..................................................................i......i.......................... 9300/11269
.................................................................................................... 9400/11269
....iiiiii..iiiiii..i............................................................................... 9500/11269
.................................................................................................... 9700/11269
.................................................................................................... 9800/11269
.................................................................................................... 9900/11269
.................................................................................................... 10000/11269
---
.................................................................................................... 10500/11269
.................................................................................................... 10600/11269
...........................i........................................................................ 10700/11269
.................................................................................................... 10800/11269
.........F..F.F..................................................................................... 10900/11269
.................................................................................................... 11100/11269
...........................................................i.i...................................... 11200/11269
.....................................................................
failures:
failures:

---- [ui] ui/borrowck/borrow-raw-address-of-mutability.rs stdout ----
diff of stderr:

20    |
21 LL |     let f = || {
22    |         - help: consider changing this to be mutable: `mut f`
- ...
+ LL |         let y = &raw mut x;
+    |                          - calling `f` requires mutable borrow due to mutation of `x`
+ LL |     };
24 LL |     f();
25    |     ^ cannot borrow as mutable


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-raw-address-of-mutability/borrow-raw-address-of-mutability.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-raw-address-of-mutability/borrow-raw-address-of-mutability.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrow-raw-address-of-mutability.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrow-raw-address-of-mutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-raw-address-of-mutability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-raw-address-of-mutability/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
LL |     let x = 0;
LL |     let x = 0;
   |         - help: consider changing this to be mutable: `mut x`
LL |     let y = &raw mut x;                 //~ ERROR cannot borrow
   |             ^^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
LL |     let x = 0;
LL |     let x = 0;
   |         - help: consider changing this to be mutable: `mut x`
LL |     let mut f = || {
LL |         let y = &raw mut x;             //~ ERROR cannot borrow
   |                 ^^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `f` as mutable, as it is not declared as mutable
   |
   |
LL |     let f = || {
   |         - help: consider changing this to be mutable: `mut f`
LL |         let y = &raw mut x;
   |                          - calling `f` requires mutable borrow due to mutation of `x`
LL |     };
LL |     f();                                //~ ERROR cannot borrow
   |     ^ cannot borrow as mutable

error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn make_fn<F: Fn()>(f: F) -> F { f }
   |                          - change this to accept `FnMut` instead of `Fn`
...
LL |       let f = make_fn(|| {
   |  _____________-------_-
   | |             |
   | |             expects `Fn` instead of `FnMut`
LL | |         let y = &raw mut x;             //~ ERROR cannot borrow
   | |                 ^^^^^^^^^^ cannot borrow as mutable
LL | |     });
   | |_____- in this closure

error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn make_fn<F: Fn()>(f: F) -> F { f }
   |                          - change this to accept `FnMut` instead of `Fn`
...
LL |       let f = make_fn(move || {
   |  _____________-------_-
   | |             |
   | |             expects `Fn` instead of `FnMut`
LL | |         let y = &raw mut x;             //~ ERROR cannot borrow
   | |                 ^^^^^^^^^^ cannot borrow as mutable
LL | |     });
   | |_____- in this closure
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0596`.


------------------------------------------


---- [ui] ui/unboxed-closures/unboxed-closures-infer-fnmut-calling-fnmut-no-mut.rs stdout ----
diff of stderr:

12    |
13 LL |     let tick2 = || {
14    |         ----- help: consider changing this to be mutable: `mut tick2`
+ LL |         tick1();
+    |         ----- calling `tick2` requires mutable borrow due to mutation of `tick1`
15 ...
16 LL |     tick2();
17    |     ^^^^^ cannot borrow as mutable

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-calling-fnmut-no-mut/unboxed-closures-infer-fnmut-calling-fnmut-no-mut.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-infer-fnmut-calling-fnmut-no-mut.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-calling-fnmut-no-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-calling-fnmut-no-mut" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-calling-fnmut-no-mut/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0596]: cannot borrow `tick1` as mutable, as it is not declared as mutable
   |
   |
LL |     let tick1 = || {
   |         ----- help: consider changing this to be mutable: `mut tick1`
...
LL |         tick1(); //~ ERROR cannot borrow `tick1` as mutable
   |         ^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `tick2` as mutable, as it is not declared as mutable
   |
   |
LL |     let tick2 = || {
   |         ----- help: consider changing this to be mutable: `mut tick2`
LL |         tick1(); //~ ERROR cannot borrow `tick1` as mutable
   |         ----- calling `tick2` requires mutable borrow due to mutation of `tick1`
...
LL |     tick2(); //~ ERROR cannot borrow
   |     ^^^^^ cannot borrow as mutable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.

---
diff of stderr:

2   --> $DIR/unboxed-closures-infer-fnmut-missing-mut.rs:7:5
3    |
4 LL |     let tick = || counter += 1;
-    |         ---- help: consider changing this to be mutable: `mut tick`
+    |         ----      ------- calling `tick` requires mutable borrow due to mutation of `counter`
+    |         |
+    |         help: consider changing this to be mutable: `mut tick`
6 LL |     tick();
7    |     ^^^^ cannot borrow as mutable


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-missing-mut/unboxed-closures-infer-fnmut-missing-mut.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-infer-fnmut-missing-mut.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-missing-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-missing-mut" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-missing-mut/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0596]: cannot borrow `tick` as mutable, as it is not declared as mutable
   |
   |
LL |     let tick = || counter += 1;
   |         ----      ------- calling `tick` requires mutable borrow due to mutation of `counter`
   |         |
   |         help: consider changing this to be mutable: `mut tick`
LL |     tick(); //~ ERROR cannot borrow `tick` as mutable, as it is not declared as mutable
   |     ^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.

---
diff of stderr:

2   --> $DIR/unboxed-closures-infer-fnmut-move-missing-mut.rs:7:5
3    |
4 LL |     let tick = move || counter += 1;
-    |         ---- help: consider changing this to be mutable: `mut tick`
+    |         ----           ------- calling `tick` requires mutable borrow due to mutation of `counter`
+    |         |
+    |         help: consider changing this to be mutable: `mut tick`
6 LL |     tick();
7    |     ^^^^ cannot borrow as mutable


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-move-missing-mut/unboxed-closures-infer-fnmut-move-missing-mut.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-infer-fnmut-move-missing-mut.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-move-missing-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-move-missing-mut" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-infer-fnmut-move-missing-mut/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0596]: cannot borrow `tick` as mutable, as it is not declared as mutable
   |
   |
LL |     let tick = move || counter += 1;
   |         ----           ------- calling `tick` requires mutable borrow due to mutation of `counter`
   |         |
   |         help: consider changing this to be mutable: `mut tick`
LL |     tick(); //~ ERROR cannot borrow `tick` as mutable, as it is not declared as mutable
   |     ^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:07
