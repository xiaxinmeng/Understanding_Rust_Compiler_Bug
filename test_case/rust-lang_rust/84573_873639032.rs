plain
.................................................................................................... 1000/12046
.................................................................................................... 1100/12046
.................................i.................................................................. 1200/12046
.................................................................................................... 1300/12046
....................................................................F............F.................. 1400/12046
.................................................................................................... 1600/12046
................................................................................i................... 1700/12046
.................................................................................................... 1800/12046
.................................................................................................... 1900/12046
---

---- [ui] ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs stdout ----
diff of stderr:

17    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
18 help: add a dummy let to cause `f` to be fully captured
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL |     let result = panic::catch_unwind(move || { let _ = &f;
+ LL |     let result = panic::catch_unwind(move || { let _ = &f; 
22 LL |
23 LL |         f.0()



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims/mir_calls_to_shims.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `UnwindSafe`, `RefUnwindSafe` trait implementation will change in Rust 2021
   |
LL |       let result = panic::catch_unwind(move || {
   |  ______________________________________^
   |  ______________________________________^
LL | |         //~^ ERROR: `UnwindSafe`, `RefUnwindSafe` trait implementation
LL | |         //~| HELP: add a dummy let to cause `f` to be fully captured
LL | |         f.0()
LL | |     });
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/mir_calls_to_shims.rs:3:9
   |
   |
LL | #![deny(disjoint_capture_migration)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `f` to be fully captured
   |
LL |     let result = panic::catch_unwind(move || { let _ = &f; 
LL |         //~^ ERROR: `UnwindSafe`, `RefUnwindSafe` trait implementation
LL |         //~| HELP: add a dummy let to cause `f` to be fully captured
LL |         f.0()
LL |     });

error: aborting due to previous error


---
1 error[E0004]: non-exhaustive patterns: type `u8` is non-empty
-   --> $DIR/pattern-matching-should-fail.rs:70:23
+   --> $DIR/pattern-matching-should-fail.rs:68:23
3    |
4 LL |     let c1 = || match x { };

8    = note: the matched value is of type `u8`
9 
9 
10 error[E0381]: use of possibly-uninitialized variable: `x`
-   --> $DIR/pattern-matching-should-fail.rs:8:23
12    |
12    |
13 LL |     let c1 = || match x { };
14    |                       ^ use of possibly-uninitialized `x`
15 
15 
16 error[E0381]: borrow of possibly-uninitialized variable: `x`
-   --> $DIR/pattern-matching-should-fail.rs:15:14
18    |
18    |
19 LL |     let c2 = || match x { _ => () };
20    |              ^^       - borrow occurs due to use in closure

22    |              use of possibly-uninitialized `x`
23 
24 error[E0381]: borrow of possibly-uninitialized variable: `variant`
-   --> $DIR/pattern-matching-should-fail.rs:27:13
26    |
27 LL |     let c = || {
27 LL |     let c = || {
28    |             ^^ use of possibly-uninitialized `variant`

31    |               ------- borrow occurs due to use in closure
32 
33 error[E0381]: borrow of possibly-uninitialized variable: `variant`
-   --> $DIR/pattern-matching-should-fail.rs:39:13
35    |
36 LL |     let c = || {
36 LL |     let c = || {
37    |             ^^ use of possibly-uninitialized `variant`

40    |               ------- borrow occurs due to use in closure
41 
42 error[E0381]: use of possibly-uninitialized variable: `g`
-   --> $DIR/pattern-matching-should-fail.rs:55:15
44    |
44    |
45 LL |         match g { };
46    |               ^ use of possibly-uninitialized `g`
47 
48 error[E0381]: use of possibly-uninitialized variable: `t`
-   --> $DIR/pattern-matching-should-fail.rs:58:19
+   --> $DIR/pattern-matching-should-fail.rs:56:19
+   --> $DIR/pattern-matching-should-fail.rs:56:19
50    |
51 LL |             match t { };
52    |                   ^ use of possibly-uninitialized `t`
53 
53 
54 error[E0381]: use of possibly-uninitialized variable: `x`
-   --> $DIR/pattern-matching-should-fail.rs:70:23
56    |
56    |
57 LL |     let c1 = || match x { };
58    |                       ^ use of possibly-uninitialized `x`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail/pattern-matching-should-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/pattern-matching-should-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0004]: non-exhaustive patterns: type `u8` is non-empty
  --> /checkout/src/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail.rs:68:23
   |
LL |     let c1 = || match x { };
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `u8`


error[E0381]: use of possibly-uninitialized variable: `x`
   |
   |
LL |     let c1 = || match x { };
   |                       ^ use of possibly-uninitialized `x`

error[E0381]: borrow of possibly-uninitialized variable: `x`
   |
   |
LL |     let c2 = || match x { _ => () };
   |              ^^       - borrow occurs due to use in closure
   |              |
   |              use of possibly-uninitialized `x`

error[E0381]: borrow of possibly-uninitialized variable: `variant`
   |
LL |     let c = || {
LL |     let c = || {
   |             ^^ use of possibly-uninitialized `variant`
LL |     //~^ ERROR: borrow of possibly-uninitialized variable: `variant`
LL |         match variant {
   |               ------- borrow occurs due to use in closure

error[E0381]: borrow of possibly-uninitialized variable: `variant`
   |
LL |     let c = || {
LL |     let c = || {
   |             ^^ use of possibly-uninitialized `variant`
LL |     //~^ ERROR: borrow of possibly-uninitialized variable: `variant`
LL |         match variant {
   |               ------- borrow occurs due to use in closure

error[E0381]: use of possibly-uninitialized variable: `g`
   |
   |
LL |         match g { };
   |               ^ use of possibly-uninitialized `g`
error[E0381]: use of possibly-uninitialized variable: `t`
  --> /checkout/src/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail.rs:56:19
   |
   |
LL |             match t { };
   |                   ^ use of possibly-uninitialized `t`

error[E0381]: use of possibly-uninitialized variable: `x`
   |
   |
LL |     let c1 = || match x { };
   |                       ^ use of possibly-uninitialized `x`
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0004, E0381.
For more information about an error, try `rustc --explain E0004`.
---
test result: FAILED. 11947 passed; 2 failed; 97 ignored; 0 measured; 0 filtered out; finished in 104.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:10:43
