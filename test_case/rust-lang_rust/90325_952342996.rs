plain
test [ui (nll)] ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui (nll)] ui/lifetimes/issue-90170-elision-mismatch.rs stdout ----

- error[E0623]: lifetime mismatch
-   --> $DIR/issue-90170-elision-mismatch.rs:3:47
+ error: lifetime may not live long enough
+ error: lifetime may not live long enough
+   --> $DIR/issue-90170-elision-mismatch.rs:3:40
3    |
4 LL | pub fn foo(x: &mut Vec<&u8>, y: &u8) { x.push(y); }
-    |                        ---      ---           ^ ...but data from `y` flows into `x` here
-    |                                 these two types are declared with different lifetimes...
-    |
-    |
-    = note: each elided lifetime in input position becomes a distinct lifetime
- help: consider introducing a named lifetime parameter
-    |
- LL | pub fn foo<'a>(x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); }
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |           ++++              ++          ++
+    |                        -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |                        |        |
+    |                        |        let's call the lifetime of this reference `'1`
+    |                        let's call the lifetime of this reference `'2`
- error[E0623]: lifetime mismatch
-   --> $DIR/issue-90170-elision-mismatch.rs:5:51
+ error: lifetime may not live long enough
+   --> $DIR/issue-90170-elision-mismatch.rs:5:44
+   --> $DIR/issue-90170-elision-mismatch.rs:5:44
17    |
18 LL | pub fn foo2(x: &mut Vec<&'_ u8>, y: &u8) { x.push(y); }
-    |                         ------      ---           ^ ...but data from `y` flows into `x` here
-    |                                     these two types are declared with different lifetimes...
-    |
-    |
-    = note: each elided lifetime in input position becomes a distinct lifetime
- help: consider introducing a named lifetime parameter
-    |
- LL | pub fn foo2<'a>(x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); }
-    |            ++++              ~~          ++
+    |                         -           -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |                         |           |
+    |                         |           let's call the lifetime of this reference `'1`
+    |                         let's call the lifetime of this reference `'2`
- error[E0623]: lifetime mismatch
-   --> $DIR/issue-90170-elision-mismatch.rs:7:70
+ error: lifetime may not live long enough
+   --> $DIR/issue-90170-elision-mismatch.rs:7:63
+   --> $DIR/issue-90170-elision-mismatch.rs:7:63
31    |
32 LL | pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&u8>, y: &u8) { x.push(y); }
-    |                                               ---      ---           ^ ...but data from `y` flows into `x` here
-    |                                                        these two types are declared with different lifetimes...
-    |
-    |
-    = note: each elided lifetime in input position becomes a distinct lifetime
- help: consider introducing a named lifetime parameter
-    |
- LL | pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); }
-    |                                                ++          ++
+    |                                               -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |                                               |        |
+    |                                               |        let's call the lifetime of this reference `'1`
+    |                                               let's call the lifetime of this reference `'2`
43 error: aborting due to 3 previous errors
44 

- For more information about this error, try `rustc --explain E0623`.
- For more information about this error, try `rustc --explain E0623`.
46 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch.nll/issue-90170-elision-mismatch.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-90170-elision-mismatch.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs:3:40
   |
LL | pub fn foo(x: &mut Vec<&u8>, y: &u8) { x.push(y); } //~ ERROR lifetime mismatch
   |                        -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |                        |        |
   |                        |        let's call the lifetime of this reference `'1`
   |                        let's call the lifetime of this reference `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs:5:44
   |
   |
LL | pub fn foo2(x: &mut Vec<&'_ u8>, y: &u8) { x.push(y); } //~ ERROR lifetime mismatch
   |                         -           -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |                         |           |
   |                         |           let's call the lifetime of this reference `'1`
   |                         let's call the lifetime of this reference `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs:7:63
   |
   |
LL | pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&u8>, y: &u8) { x.push(y); } //~ ERROR lifetime mismatch
   |                                               -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |                                               |        |
   |                                               |        let's call the lifetime of this reference `'1`
   |                                               let's call the lifetime of this reference `'2`
error: aborting due to 3 previous errors


------------------------------------------
---
test result: FAILED. 12205 passed; 1 failed; 143 ignored; 0 measured; 0 filtered out; finished in 111.37s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:18:58
