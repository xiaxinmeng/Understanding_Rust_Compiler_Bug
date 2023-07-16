plain
 finished in 0.180 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 146 tests
...............................................F......F..........FF................................. 100/146
................F..F..........................
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [incremental] incremental/hashes/panic_exprs.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/panic_exprs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs/panic_exprs.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `typeck(indexing)` should be clean but is not
   |
   |
LL | / pub fn indexing(slice: &[u8]) -> u8 {
LL | |     #[cfg(cfail1)]
LL | |     {
LL | |         slice[100]
LL | |     }
LL | | }
   | |_^

---


---- [incremental] incremental/hashes/for_loops.rs stdout ----

error in revision `cfail5`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail5" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-relative-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `typeck(change_loop_body)` should be clean but is not
   |
   |
LL | / pub fn change_loop_body() {
LL | |     let mut _x = 0;
LL | |     for _ in 0..1 {
LL | |         _x = 2;
LL | |         break;
LL | |     }
LL | | }


error: `typeck(change_iteration_variable_name)` should be clean but is not
   |
LL | / pub fn change_iteration_variable_name() {
LL | |     let mut _x = 0;
LL | |     let mut _x = 0;
LL | |     for _a in 0..1 {
LL | |         _x = 1;
LL | |         break;
LL | |     }
LL | | }


error: `typeck(change_iterable)` should be clean but is not
   |
LL | / pub fn change_iterable() {
LL | |     let mut _x = 0;
LL | |     let mut _x = 0;
LL | |     for _ in &[0, 1, 3] {
LL | |         _x = 1;
LL | |         break;
LL | |     }
LL | | }


error: `typeck(add_loop_label)` should be clean but is not
   |
   |
LL | / pub fn add_loop_label() {
LL | |     let mut _x = 0;
LL | |     'label: for _ in 0..1 {
LL | |         _x = 1;
LL | |         break;
LL | |     }
LL | | }


error: `typeck(add_loop_label_to_break)` should be clean but is not
   |
   |
LL | / pub fn add_loop_label_to_break() {
LL | |     let mut _x = 0;
LL | |     'label: for _ in 0..1 {
LL | |         _x = 1;
LL | |         break 'label;
LL | |     }
LL | | }


error: `typeck(change_break_label)` should be clean but is not
   |
   |
LL | / pub fn change_break_label() {
LL | |     let mut _x = 0;
LL | |     'outer: for _ in 0..1 {
LL | |         'inner: for _ in 0..1 {
LL | |     }
LL | | }
   | |_^


error: `typeck(add_loop_label_to_continue)` should be clean but is not
   |
   |
LL | / pub fn add_loop_label_to_continue() {
LL | |     let mut _x = 0;
LL | |     'label: for _ in 0..1 {
LL | |         _x = 1;
LL | |         continue 'label;
LL | |     }
LL | | }


error: `typeck(change_continue_label)` should be clean but is not
   |
   |
LL | / pub fn change_continue_label() {
LL | |     let mut _x = 0;
LL | |     'outer: for _ in 0..1 {
LL | |         'inner: for _ in 0..1 {
LL | |     }
LL | | }
   | |_^


error: `typeck(change_continue_to_break)` should be clean but is not
   |
   |
LL | / pub fn change_continue_to_break() {
LL | |     let mut _x = 0;
LL | |     for _ in 0..1 {
LL | |         _x = 1;
LL | |         break   ;
LL | |     }
LL | | }

error: aborting due to 9 previous errors



------------------------------------------


---- [incremental] incremental/hashes/while_loops.rs stdout ----

error in revision `cfail5`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail5" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/while_loops.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-relative-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `typeck(add_loop_label)` should be clean but is not
   |
   |
LL | / pub fn add_loop_label() {
LL | |     let mut _x = 0;
LL | |     'label: while true {
LL | |         _x = 1;
LL | |         break;
LL | |     }
LL | | }

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/hashes/while_let_loops.rs stdout ----

error in revision `cfail5`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_let_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail5" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/while_let_loops.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-relative-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `typeck(add_loop_label)` should be clean but is not
   |
   |
LL | / pub fn add_loop_label() {
LL | |     let mut _x = 0;
LL | |     'label: while let Some(0u32) = None {
LL | |         _x = 1;
LL | |         break;
LL | |     }
LL | | }

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/spans_significant_w_panic.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_significant_w_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/spans_significant_w_panic.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `typeck(main)` should be clean but is not
   |
LL | / pub fn main() {
LL | / pub fn main() {
LL | |     if std::hint::black_box(false) {
LL | |         panic!()
LL | |     }
LL | | }

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/string_constant.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/string_constant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant/string_constant.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `typeck(x)` should be clean but is not
   |
   |
LL | /     pub fn x() {
LL | |         println!("{}", "2");
LL | |     }

error: aborting due to previous error


---
test result: FAILED. 140 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.93s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "incremental" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:57
