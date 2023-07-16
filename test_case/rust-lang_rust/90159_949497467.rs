plain
diff of stderr:

2   --> $DIR/mismatched_trait_impl.rs:9:5
3    |
4 LL |     fn foo(&self, x: &'a u32, y: &u32) -> &'a u32;
-    |     ---------------------------------------------- expected `fn(&i32, &'a u32, &u32) -> &'a u32`
+    |     ---------------------------------------------- expected `fn(&'1 i32, &'a u32, &'2 u32) -> &'a u32`
6 ...
7 LL |     fn foo(&self, x: &u32, y: &'a u32) -> &'a u32 {
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&i32, &u32, &u32) -> &u32`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&'1 i32, &'2 u32, &'3 u32) -> &'3 u32`
9    |
-    = note: expected `fn(&i32, &'a u32, &u32) -> &'a u32`
-               found `fn(&i32, &u32, &u32) -> &u32`
+    = note: expected `fn(&'1 i32, &'a u32, &'2 u32) -> &'a u32`
+               found `fn(&'1 i32, &'2 u32, &'3 u32) -> &'3 u32`
12    = help: the lifetime requirements from the `impl` do not correspond to the requirements in the `trait`
13    = help: verify the lifetime relationships in the `trait` and `impl` between the `self` argument, the other inputs and its output


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl.nll/mismatched_trait_impl.nll.stderr
To only update this specific test, also pass `--test-args in-band-lifetimes/mismatched_trait_impl.rs`

error: 1 errors occurred comparing output.
error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/in-band-lifetimes/mismatched_trait_impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `impl` item signature doesn't match `trait` item signature
   |
   |
LL |     fn foo(&self, x: &'a u32, y: &u32) -> &'a u32;
   |     ---------------------------------------------- expected `fn(&'1 i32, &'a u32, &'2 u32) -> &'a u32`
...
LL |     fn foo(&self, x: &u32, y: &'a u32) -> &'a u32 { //~ ERROR `impl` item signature doesn't match
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&'1 i32, &'2 u32, &'3 u32) -> &'3 u32`
   |
   = note: expected `fn(&'1 i32, &'a u32, &'2 u32) -> &'a u32`
              found `fn(&'1 i32, &'2 u32, &'3 u32) -> &'3 u32`
   = help: the lifetime requirements from the `impl` do not correspond to the requirements in the `trait`
   = help: verify the lifetime relationships in the `trait` and `impl` between the `self` argument, the other inputs and its output
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 12178 passed; 1 failed; 143 ignored; 0 measured; 0 filtered out; finished in 92.34s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:16:12
