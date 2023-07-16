plain
.................................................................................................... 7000/12257
...........ii.......................................................i............................... 7100/12257
.................................................................................................... 7200/12257
.................................................................................................... 7300/12257
................................F..FFF.............................................................. 7400/12257
.................................................................................................... 7600/12257
.................................................................................................... 7700/12257
.................................................................................................... 7800/12257
.................................................................................................... 7900/12257
---
failures:

---- [ui] ui/consts/const_in_pattern/issue-73431.rs stdout ----
normalized stderr:
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.


Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/issue-73431.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_in_pattern/issue-73431.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_in_pattern/issue-73431.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
------------------------------------------


---- [ui] ui/mir-dataflow/indirect-mutation-offset.rs stdout ----
---- [ui] ui/mir-dataflow/indirect-mutation-offset.rs stdout ----
diff of stderr:

+ WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
1 error: rustc_peek: bit not set
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/indirect-mutation-offset.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mir-dataflow/indirect-mutation-offset.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-dataflow/indirect-mutation-offset.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
error: rustc_peek: bit not set
   |
   |
LL |     rustc_peek(x); //~ ERROR rustc_peek: bit not set


error: stop_after_dataflow ended compilation
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/mir-dataflow/liveness-enum.rs stdout ----
diff of stderr:

+ WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
+ WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
1 error: rustc_peek: bit not set
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/liveness-enum/liveness-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mir-dataflow/liveness-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-dataflow/liveness-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/liveness-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/liveness-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
error: rustc_peek: bit not set
   |
   |
LL |     rustc_peek(x); //~ ERROR rustc_peek: bit not set


error: stop_after_dataflow ended compilation
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/mir-dataflow/liveness-projection.rs stdout ----
diff of stderr:

+ WARN rustc_mir_dataflow::rustc_peek peek_at: place=_2
1 error: rustc_peek: bit not set
3    |


4 LL |         unsafe { rustc_peek(x); }
6 
6 
+ WARN rustc_mir_dataflow::rustc_peek peek_at: place=_2
+ WARN rustc_mir_dataflow::rustc_peek peek_at: place=_27
+ WARN rustc_mir_dataflow::rustc_peek peek_at: place=_27
7 error: rustc_peek: bit not set
9    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/liveness-projection/liveness-projection.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mir-dataflow/liveness-projection.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-dataflow/liveness-projection.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/liveness-projection" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/liveness-projection/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
WARN rustc_mir_dataflow::rustc_peek peek_at: place=_2
error: rustc_peek: bit not set
   |
   |
LL |         unsafe { rustc_peek(x); } //~ ERROR bit not set


WARN rustc_mir_dataflow::rustc_peek peek_at: place=_2
WARN rustc_mir_dataflow::rustc_peek peek_at: place=_27
WARN rustc_mir_dataflow::rustc_peek peek_at: place=_27
error: rustc_peek: bit not set
   |
   |
LL |         unsafe { rustc_peek(&p); } //~ ERROR bit not set


error: stop_after_dataflow ended compilation
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/mir-dataflow/liveness-ptr.rs stdout ----
diff of stderr:

+ WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
+ WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
1 error: rustc_peek: bit not set
3    |

4 LL |     rustc_peek(x);
5    |     ^^^^^^^^^^^^^
5    |     ^^^^^^^^^^^^^
6 
+ WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
7 error: stop_after_dataflow ended compilation
9 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/liveness-ptr/liveness-ptr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mir-dataflow/liveness-ptr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-dataflow/liveness-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/liveness-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/liveness-ptr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
error: rustc_peek: bit not set
   |
   |
LL |     rustc_peek(x); //~ ERROR rustc_peek: bit not set


WARN rustc_mir_dataflow::rustc_peek peek_at: place=_1
error: stop_after_dataflow ended compilation
error: aborting due to 2 previous errors


------------------------------------------
---
test result: FAILED. 12137 passed; 5 failed; 115 ignored; 0 measured; 0 filtered out; finished in 125.53s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:11
