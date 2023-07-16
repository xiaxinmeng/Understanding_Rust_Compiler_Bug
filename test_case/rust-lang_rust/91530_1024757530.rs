plain

---- [ui] ui/suggestions/issue-86100-tuple-paren-comma.rs stdout ----
diff of stderr:

11 help: use a trailing comma to create a tuple with one element
12    |
13 LL |     let _x: (i32,) = (5,);
+    |                        +
15 
16 error[E0308]: mismatched types
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
17   --> $DIR/issue-86100-tuple-paren-comma.rs:13:9

24 help: use a trailing comma to create a tuple with one element
25    |
26 LL |     foo((Some(3),));
+    |                 +
28 
29 error[E0308]: mismatched types
30   --> $DIR/issue-86100-tuple-paren-comma.rs:17:22
30   --> $DIR/issue-86100-tuple-paren-comma.rs:17:22

37 help: use a trailing comma to create a tuple with one element
38    |
39 LL |     let _s = S { _s: ("abc".to_string(),) };
+    |                                        +
41 
42 error[E0308]: mismatched types
43   --> $DIR/issue-86100-tuple-paren-comma.rs:23:22
43   --> $DIR/issue-86100-tuple-paren-comma.rs:23:22


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-86100-tuple-paren-comma/issue-86100-tuple-paren-comma.stderr
To only update this specific test, also pass `--test-args suggestions/issue-86100-tuple-paren-comma.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-86100-tuple-paren-comma.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-86100-tuple-paren-comma" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-86100-tuple-paren-comma/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/issue-86100-tuple-paren-comma.rs:9:22
   |
LL |     let _x: (i32,) = (5);
   |             ------   ^^^ expected tuple, found integer
   |             expected due to this
   |
   = note: expected tuple `(i32,)`
               found type `{integer}`
               found type `{integer}`
help: use a trailing comma to create a tuple with one element
   |
LL |     let _x: (i32,) = (5,);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/issue-86100-tuple-paren-comma.rs:13:9
   |
   |
LL |     foo((Some(3)));
   |         ^^^^^^^^^ expected tuple, found enum `Option`
   |
   = note: expected tuple `(_,)`
               found enum `Option<{integer}>`
help: use a trailing comma to create a tuple with one element
   |
LL |     foo((Some(3),));

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/issue-86100-tuple-paren-comma.rs:17:22
   |
   |
LL |     let _s = S { _s: ("abc".to_string()) };
   |
   |
   = note: expected tuple `(String,)`
             found struct `String`
help: use a trailing comma to create a tuple with one element
   |
LL |     let _s = S { _s: ("abc".to_string(),) };

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/issue-86100-tuple-paren-comma.rs:23:22
   |
   |
LL |     let _x: (i32,) = (t);
   |             ------   ^^^ expected a tuple with 1 element, found one with 2 elements
   |             expected due to this
   |
   = note: expected tuple `(i32,)`
   = note: expected tuple `(i32,)`
              found tuple `({integer}, {integer})`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 12435 passed; 1 failed; 121 ignored; 0 measured; 0 filtered out; finished in 118.05s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:37
