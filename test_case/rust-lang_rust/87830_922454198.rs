plain
....................................i.i............................................................. 12100/12148
................................................
failures:

---- [ui] ui/did_you_mean/issue-87830-try-brackets-for-arrays.rs stdout ----


18 error: this code is interpreted as a block expression, not an array
19   --> $DIR/issue-87830-try-brackets-for-arrays.rs:7:24
20    |
- LL | const BAR: [&str; 3] = {"one", "two", "three"}; 
+ LL | const BAR: [&str; 3] = {"one", "two", "three"};
23    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
24    = note: to define an array, one would use square brackets instead of curly braces

25 help: try using [] instead of {}
26    |
- LL | const BAR: [&str; 3] = ["one", "two", "three"]; 
+ LL | const BAR: [&str; 3] = ["one", "two", "three"];
29 
29 
30 error: this code is interpreted as a block expression, not an array

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-87830-try-brackets-for-arrays/issue-87830-try-brackets-for-arrays.stderr
To only update this specific test, also pass `--test-args did_you_mean/issue-87830-try-brackets-for-arrays.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-87830-try-brackets-for-arrays.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-87830-try-brackets-for-arrays" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-87830-try-brackets-for-arrays/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this code is interpreted as a block expression, not an array
  --> /checkout/src/test/ui/did_you_mean/issue-87830-try-brackets-for-arrays.rs:3:22
   |
LL |   const FOO: [u8; 3] = { //~ ERROR this code is interpreted as a block expression
LL | |     1, 2, 3
LL | | };
   | |_^
   |
   |
   = note: to define an array, one would use square brackets instead of curly braces
help: try using [] instead of {}
   |
LL ~ const FOO: [u8; 3] = [ //~ ERROR this code is interpreted as a block expression
LL |     1, 2, 3
LL ~ ];


error: this code is interpreted as a block expression, not an array
  --> /checkout/src/test/ui/did_you_mean/issue-87830-try-brackets-for-arrays.rs:7:24
   |
LL | const BAR: [&str; 3] = {"one", "two", "three"};
   |
   |
   = note: to define an array, one would use square brackets instead of curly braces
help: try using [] instead of {}
   |
LL | const BAR: [&str; 3] = ["one", "two", "three"];


error: this code is interpreted as a block expression, not an array
  --> /checkout/src/test/ui/did_you_mean/issue-87830-try-brackets-for-arrays.rs:11:5
LL |     {1, 2, 3};
   |     ^^^^^^^^^
   |
   |
   = note: to define an array, one would use square brackets instead of curly braces
help: try using [] instead of {}
LL |     [1, 2, 3];
   |     ~       ~


error: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`
  --> /checkout/src/test/ui/did_you_mean/issue-87830-try-brackets-for-arrays.rs:16:6
   |
LL |     1, 2, 3 //~ ERROR expected one of
   |      ^ expected one of `.`, `;`, `?`, `}`, or an operator
error: aborting due to 4 previous errors


------------------------------------------
---
test result: FAILED. 12045 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 128.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:55
