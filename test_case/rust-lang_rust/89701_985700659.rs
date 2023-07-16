plain
---- [ui] ui/feature-gates/issue-43106-gating-of-bench.rs stdout ----
diff of stderr:

6    |
7    = note: import resolution is stuck, try simplifying macro imports
- error: aborting due to previous error
- error: aborting due to previous error
+ error: `bench` attribute cannot be used at crate level
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+    |
+ LL | #![bench                   = "4100"]
+    |
+    |
+ help: perhaps you meant to use an outer attribute
+    |
+ LL | #[bench                   = "4100"]
+ 
+ error: aborting due to 2 previous errors
10 
11 
---
To only update this specific test, also pass `--test-args feature-gates/issue-43106-gating-of-bench.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/issue-43106-gating-of-bench.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-bench" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-bench/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot determine resolution for the attribute macro `bench`
   |
   |
LL | #![bench                   = "4100"]
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: `bench` attribute cannot be used at crate level
   |
   |
LL | #![bench                   = "4100"]
   |
help: perhaps you meant to use an outer attribute
   |
   |
LL | #[bench                   = "4100"]

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/feature-gates/issue-43106-gating-of-test.rs stdout ----
diff of stderr:

6    |
7    = note: import resolution is stuck, try simplifying macro imports
- error: aborting due to previous error
- error: aborting due to previous error
+ error: `test` attribute cannot be used at crate level
+    |
+    |
+ LL | #![test                    = "4200"]
+    |
+    |
+ help: perhaps you meant to use an outer attribute
+    |
+ LL | #[test                    = "4200"]
+ 
+ error: aborting due to 2 previous errors
10 
11 
---
To only update this specific test, also pass `--test-args feature-gates/issue-43106-gating-of-test.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/issue-43106-gating-of-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot determine resolution for the attribute macro `test`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-test.rs:4:4
   |
LL | #![test                    = "4200"]
   |    ^^^^
   |
   = note: import resolution is stuck, try simplifying macro imports
error: `test` attribute cannot be used at crate level
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-test.rs:4:1
   |
LL | #![test                    = "4200"]
LL | #![test                    = "4200"]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: perhaps you meant to use an outer attribute
   |
LL | #[test                    = "4200"]

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/imports/issue-28134.rs stdout ----
diff of stderr:

6    |
7    = note: import resolution is stuck, try simplifying macro imports
- error: aborting due to previous error
- error: aborting due to previous error
+ error: `test` attribute cannot be used at crate level
+    |
+ LL | #![test]
+    | ^^^^^^^^
+    |
+    |
+ help: perhaps you meant to use an outer attribute
+    |
+ LL | #[test]
+    | ~~~~~~~
+ error: aborting due to 2 previous errors
10 
11 

---
To only update this specific test, also pass `--test-args imports/issue-28134.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-28134.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-28134" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-28134/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot determine resolution for the attribute macro `test`
  --> /checkout/src/test/ui/imports/issue-28134.rs:4:4
   |
LL | #![test] //~ ERROR cannot determine resolution for the attribute macro `test`
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: `test` attribute cannot be used at crate level
  --> /checkout/src/test/ui/imports/issue-28134.rs:4:1
   |
   |
LL | #![test] //~ ERROR cannot determine resolution for the attribute macro `test`
   |
help: perhaps you meant to use an outer attribute
   |
   |
LL | #[test] //~ ERROR cannot determine resolution for the attribute macro `test`
   | ~~~~~~~
error: aborting due to 2 previous errors


------------------------------------------
---
test result: FAILED. 12326 passed; 3 failed; 115 ignored; 0 measured; 0 filtered out; finished in 147.93s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:57
