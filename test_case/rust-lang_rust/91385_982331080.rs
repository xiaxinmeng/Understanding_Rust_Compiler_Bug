plain
diff of stderr:

2   --> $DIR/macro-pat2021-pattern-followed-by-or.rs:4:28
3    |
4 LL | macro_rules! foo { ($x:pat | $y:pat) => {} }
-    |                            ^ not allowed after `pat` fragments
+    |                     ------ ^ not allowed after `pat` fragments
+    |                     |
+    |                     help: try a `pat_param` fragment specifier instead: `$x:pat_param`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
7    = note: allowed there are: `=>`, `,`, `=`, `if` or `in`

10   --> $DIR/macro-pat2021-pattern-followed-by-or.rs:7:28
11    |
11    |
12 LL | macro_rules! ogg { ($x:pat | $y:pat_param) => {} }
-    |                            ^ not allowed after `pat` fragments
+    |                     ------ ^ not allowed after `pat` fragments
+    |                     |
+    |                     help: try a `pat_param` fragment specifier instead: `$x:pat_param`
14    |
15    = note: allowed there are: `=>`, `,`, `=`, `if` or `in`

18   --> $DIR/macro-pat2021-pattern-followed-by-or.rs:9:35
19    |
19    |
20 LL |     ( $expr:expr , $( $( $pat:pat)|+ => $expr_arm:pat),+ ) => {
-    |                                   ^ not allowed after `pat` fragments
+    |                          -------- ^ not allowed after `pat` fragments
+    |                          |
+    |                          help: try a `pat_param` fragment specifier instead: `$pat:pat_param`
22    |
23    = note: allowed there are: `=>`, `,`, `=`, `if` or `in`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-pat2021-pattern-followed-by-or/macro-pat2021-pattern-followed-by-or.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-pat2021-pattern-followed-by-or/macro-pat2021-pattern-followed-by-or.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-pat2021-pattern-followed-by-or.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-pat2021-pattern-followed-by-or.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-pat2021-pattern-followed-by-or" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-pat2021-pattern-followed-by-or/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `$x:pat` is followed by `|`, which is not allowed for `pat` fragments
   |
   |
LL | macro_rules! foo { ($x:pat | $y:pat) => {} } //~ ERROR `$x:pat` is followed by `|`, which is not allowed for `pat` fragments
   |                     ------ ^ not allowed after `pat` fragments
   |                     |
   |                     help: try a `pat_param` fragment specifier instead: `$x:pat_param`
   |
   = note: allowed there are: `=>`, `,`, `=`, `if` or `in`

error: `$x:pat` is followed by `|`, which is not allowed for `pat` fragments
   |
   |
LL | macro_rules! ogg { ($x:pat | $y:pat_param) => {} } //~ ERROR `$x:pat` is followed by `|`, which is not allowed for `pat` fragments
   |                     ------ ^ not allowed after `pat` fragments
   |                     |
   |                     help: try a `pat_param` fragment specifier instead: `$x:pat_param`
   |
   = note: allowed there are: `=>`, `,`, `=`, `if` or `in`

error: `$pat:pat` may be followed by `|`, which is not allowed for `pat` fragments
   |
   |
LL |     ( $expr:expr , $( $( $pat:pat)|+ => $expr_arm:pat),+ ) => { //~ ERROR  `$pat:pat` may be followed by `|`, which is not allowed for `p...
   |                          -------- ^ not allowed after `pat` fragments
   |                          |
   |                          help: try a `pat_param` fragment specifier instead: `$pat:pat_param`
   |
   = note: allowed there are: `=>`, `,`, `=`, `if` or `in`
error: aborting due to 3 previous errors


------------------------------------------
---
test result: FAILED. 12306 passed; 1 failed; 115 ignored; 0 measured; 0 filtered out; finished in 141.31s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:08
