plain

---- [ui (nll)] ui/issues/issue-65230.rs stdout ----
diff of stderr:

- error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
-   --> $DIR/issue-65230.rs:10:28
-    |
- LL |       fn f(&self) -> Self::U {
- LL | |         self.0
- LL | |     }
-    | |_____^
-    |
-    |
- note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 10:10...
-   --> $DIR/issue-65230.rs:10:10
-    |
- LL |     fn f(&self) -> Self::U {
-    |          ^^^^^
- note: ...so that reference does not outlive borrowed content
+ error: lifetime may not live long enough
17    |
- LL |         self.0
-    |         ^^^^^^
-    |         ^^^^^^
- note: but, the lifetime must be valid for the lifetime `'a` as defined on the impl at 8:6...
-   --> $DIR/issue-65230.rs:8:6
-    |
23 LL | impl<'a> T for X<'a> {
-    |      ^^
- note: ...so that the types are compatible
-   --> $DIR/issue-65230.rs:10:28
-    |
- LL |       fn f(&self) -> Self::U {
- LL | |         self.0
- LL | |     }
-    | |_____^
-    | |_____^
-    = note: expected `<X<'a> as T>`
-               found `<X<'_> as T>`
+    |      -- lifetime `'a` defined here
+ LL |     type U = &'a i32;
+ LL |     fn f(&self) -> Self::U {
+    |          - let's call the lifetime of this reference `'1`
+ LL |         self.0
+    |         ^^^^^^ returning this value requires that `'1` must outlive `'a`
36 error: aborting due to previous error
37 

- For more information about this error, try `rustc --explain E0495`.
- For more information about this error, try `rustc --explain E0495`.
39 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65230.nll/issue-65230.nll.stderr
To only update this specific test, also pass `--test-args issues/issue-65230.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-65230.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65230.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65230.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/issues/issue-65230.rs:11:9
   |
LL | impl<'a> T for X<'a> {
   |      -- lifetime `'a` defined here
LL |     type U = &'a i32;
LL |     fn f(&self) -> Self::U {
   |          - let's call the lifetime of this reference `'1`
LL |         self.0
   |         ^^^^^^ returning this value requires that `'1` must outlive `'a`
error: aborting due to previous error


------------------------------------------
---
    [ui (nll)] ui/issues/issue-65230.rs

test result: FAILED. 12125 passed; 1 failed; 142 ignored; 0 measured; 0 filtered out; finished in 113.32s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:20:14
