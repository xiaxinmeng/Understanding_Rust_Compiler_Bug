plain
.................................................................................................... 9400/11731
.................................................................................................... 9500/11731
.........................................................................i.....i.................... 9600/11731
.................................................................................................... 9700/11731
..................iiiiiii..iiiiii.i................................................................. 9800/11731
.................................................................................................... 10000/11731
.................................................................................................... 10100/11731
.................................................................................................... 10200/11731
.................................................................................................... 10300/11731
---
.......................ii........................................................................... 11700/11731
...............................
failures:

---- [ui] ui/macros/macro-or-patterns-back-compat.rs stdout ----


- error: the meaning of the pat fragment specific is changing in Rust 2021, which may affect this macro
+ error: the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
2   --> $DIR/macro-or-patterns-back-compat.rs:7:21
3    |
4 LL | macro_rules! foo { ($x:pat | $y:pat) => {} }

10 LL | #![deny(or_patterns_back_compat)]
12 
12 
- error: the meaning of the pat fragment specific is changing in Rust 2021, which may affect this macro
+ error: the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
14   --> $DIR/macro-or-patterns-back-compat.rs:8:23
15    |
16 LL | macro_rules! bar { ($($x:pat)+ | $($y:pat)+) => {} }

17    |                       ^^^^^^ help: use pat2015 to preserve semantics: `$x:pat2015`
18 
- error: the meaning of the pat fragment specific is changing in Rust 2021, which may affect this macro
+ error: the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
20   --> $DIR/macro-or-patterns-back-compat.rs:11:21
21    |
22 LL | macro_rules! ogg { ($x:pat | $y:pat2015) => {} }

23    |                     ^^^^^^ help: use pat2015 to preserve semantics: `$x:pat2015`
24 
- error: the meaning of the pat fragment specific is changing in Rust 2021, which may affect this macro
+ error: the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
26   --> $DIR/macro-or-patterns-back-compat.rs:13:26
27    |
28 LL |     ( $expr:expr , $( $( $pat:pat )|+ => $expr_arm:expr ),+ ) => {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-or-patterns-back-compat/macro-or-patterns-back-compat.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-or-patterns-back-compat.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-or-patterns-back-compat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-or-patterns-back-compat" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-or-patterns-back-compat/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
  --> /checkout/src/test/ui/macros/macro-or-patterns-back-compat.rs:7:21
   |
LL | macro_rules! foo { ($x:pat | $y:pat) => {} } //~ ERROR the meaning of the pat fragment specific is changing in Rust 2021, which may affec...
   |                     ^^^^^^ help: use pat2015 to preserve semantics: `$x:pat2015`
note: the lint level is defined here
  --> /checkout/src/test/ui/macros/macro-or-patterns-back-compat.rs:5:9
   |
   |
LL | #![deny(or_patterns_back_compat)]


error: the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
  --> /checkout/src/test/ui/macros/macro-or-patterns-back-compat.rs:8:23
   |
LL | macro_rules! bar { ($($x:pat)+ | $($y:pat)+) => {} } //~ ERROR the meaning of the pat fragment specific is changing in Rust 2021, which m...
   |                       ^^^^^^ help: use pat2015 to preserve semantics: `$x:pat2015`

error: the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
  --> /checkout/src/test/ui/macros/macro-or-patterns-back-compat.rs:11:21
   |
LL | macro_rules! ogg { ($x:pat | $y:pat2015) => {} } //~ ERROR the meaning of the pat fragment specific is changing in Rust 2021, which may a...
   |                     ^^^^^^ help: use pat2015 to preserve semantics: `$x:pat2015`

error: the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
  --> /checkout/src/test/ui/macros/macro-or-patterns-back-compat.rs:13:26
   |
LL |     ( $expr:expr , $( $( $pat:pat )|+ => $expr_arm:expr ),+ ) => { //~ ERROR the meaning of the pat fragment specific is changing in Rust...
   |                          ^^^^^^^^ help: use pat2015 to preserve semantics: `$pat:pat2015`
error: aborting due to 4 previous errors


------------------------------------------
---
test result: FAILED. 11634 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 134.80s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:08
