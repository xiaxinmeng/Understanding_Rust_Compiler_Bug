plain
1 error: lifetime may not live long enough
-   --> $DIR/issue-76547.rs:19:14
+   --> $DIR/issue-76547.rs:20:13
3    |
4 LL | async fn fut(bufs: &mut [&mut [u8]]) {
-    |              ^^^^  -     - let's call the lifetime of this reference `'2`
-    |              |     |
-    |              |     let's call the lifetime of this reference `'1`
-    |              assignment requires that `'1` must outlive `'2`
+    |                    -     - let's call the lifetime of this reference `'2`
+    |                    |
+    |                    let's call the lifetime of this reference `'1`
+ LL |     ListFut(bufs).await
+    |             ^^^^ this usage requires that `'1` must outlive `'2`
10 error: lifetime may not live long enough
-   --> $DIR/issue-76547.rs:33:15
+   --> $DIR/issue-76547.rs:34:14
12    |
12    |
13 LL | async fn fut2(bufs: &mut [&mut [u8]]) -> i32 {
-    |               ^^^^  -     - let's call the lifetime of this reference `'2`
-    |               |     |
-    |               |     let's call the lifetime of this reference `'1`
-    |               assignment requires that `'1` must outlive `'2`
+    |                     -     - let's call the lifetime of this reference `'2`
+    |                     |
+    |                     let's call the lifetime of this reference `'1`
+ LL |     ListFut2(bufs).await
+    |              ^^^^ this usage requires that `'1` must outlive `'2`
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-76547.nll/issue-76547.nll.stderr
To only update this specific test, also pass `--test-args async-await/issue-76547.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-76547.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-76547.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-76547.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/async-await/issue-76547.rs:20:13
   |
LL | async fn fut(bufs: &mut [&mut [u8]]) {
   |                    -     - let's call the lifetime of this reference `'2`
   |                    |
   |                    let's call the lifetime of this reference `'1`
LL |     ListFut(bufs).await
   |             ^^^^ this usage requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/async-await/issue-76547.rs:34:14
   |
   |
LL | async fn fut2(bufs: &mut [&mut [u8]]) -> i32 {
   |                     -     - let's call the lifetime of this reference `'2`
   |                     |
   |                     let's call the lifetime of this reference `'1`
LL |     ListFut2(bufs).await
   |              ^^^^ this usage requires that `'1` must outlive `'2`
error: aborting due to 2 previous errors


------------------------------------------
---
    [ui (nll)] ui/async-await/issue-76547.rs

test result: FAILED. 12013 passed; 1 failed; 128 ignored; 0 measured; 0 filtered out; finished in 110.56s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:20:00
