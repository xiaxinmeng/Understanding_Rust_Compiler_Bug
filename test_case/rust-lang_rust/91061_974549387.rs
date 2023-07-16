plain

---- [ui (nll)] ui/mismatched_types/closure-arg-type-mismatch.rs stdout ----
diff of stderr:

5    |              ^^^ ------------------ found signature of `fn((u32, u32)) -> _`
6    |              |
7    |              expected signature of `fn(&(u32, u32)) -> _`
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ note: required by a bound in `map`
+   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
+    |
+    |
+ LL |         F: FnMut(Self::Item) -> B,
+    |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map`
9 error[E0631]: type mismatch in closure arguments
10   --> $DIR/closure-arg-type-mismatch.rs:4:14


13    |              ^^^ ------------------- found signature of `for<'r> fn(&'r (u16, u16)) -> _`
14    |              |
15    |              expected signature of `fn(&(u32, u32)) -> _`
+ note: required by a bound in `map`
+   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
+    |
+    |
+ LL |         F: FnMut(Self::Item) -> B,
+    |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map`
17 error[E0631]: type mismatch in closure arguments
18   --> $DIR/closure-arg-type-mismatch.rs:5:14


21    |              ^^^ ------------------ found signature of `fn((u16, u16)) -> _`
22    |              |
23    |              expected signature of `fn(&(u32, u32)) -> _`
+ note: required by a bound in `map`
+   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
+    |
+    |
+ LL |         F: FnMut(Self::Item) -> B,
+    |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map`
25 error: aborting due to 3 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch.nll/closure-arg-type-mismatch.nll.stderr
To only update this specific test, also pass `--test-args mismatched_types/closure-arg-type-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:3:14
   |
LL |     a.iter().map(|_: (u32, u32)| 45); //~ ERROR type mismatch
   |              ^^^ ------------------ found signature of `fn((u32, u32)) -> _`
   |              |
   |              expected signature of `fn(&(u32, u32)) -> _`
note: required by a bound in `map`
  --> /checkout/library/core/src/iter/traits/iterator.rs:685:12
   |
   |
LL |         F: FnMut(Self::Item) -> B,
   |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:4:14
   |
   |
LL |     a.iter().map(|_: &(u16, u16)| 45); //~ ERROR type mismatch
   |              ^^^ ------------------- found signature of `for<'r> fn(&'r (u16, u16)) -> _`
   |              |
   |              expected signature of `fn(&(u32, u32)) -> _`
note: required by a bound in `map`
  --> /checkout/library/core/src/iter/traits/iterator.rs:685:12
   |
   |
LL |         F: FnMut(Self::Item) -> B,
   |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map`
error[E0631]: type mismatch in closure arguments
  --> /checkout/src/test/ui/mismatched_types/closure-arg-type-mismatch.rs:5:14
   |
   |
LL |     a.iter().map(|_: (u16, u16)| 45); //~ ERROR type mismatch
   |              ^^^ ------------------ found signature of `fn((u16, u16)) -> _`
   |              |
   |              expected signature of `fn(&(u32, u32)) -> _`
note: required by a bound in `map`
  --> /checkout/library/core/src/iter/traits/iterator.rs:685:12
   |
   |
LL |         F: FnMut(Self::Item) -> B,
   |            ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `map`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0631`.

---
test result: FAILED. 12236 passed; 1 failed; 146 ignored; 0 measured; 0 filtered out; finished in 115.89s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:19:55
