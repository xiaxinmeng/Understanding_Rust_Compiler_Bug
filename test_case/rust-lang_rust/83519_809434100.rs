plain
.................................................................................................... 9400/11721
.................................................................................................... 9500/11721
...............................................................i......i............................. 9600/11721
.................................................................................................... 9700/11721
.........iiiiiii..iiiiii.i.......................................................................... 9800/11721
.................................................................................................... 10000/11721
.................................................................................................... 10100/11721
.................................................................................................... 10200/11721
.................................................................................................... 10300/11721
---
............i.i..................................................................................... 11700/11721
.....................
failures:

---- [ui] ui/async-await/large_moves.rs stdout ----

1 error: moving 10024 bytes
-   --> $DIR/large_moves.rs:9:13
+   --> $DIR/large_moves.rs:10:13
---
19 error: moving 10024 bytes
-   --> $DIR/large_moves.rs:15:14
+   --> $DIR/large_moves.rs:16:14
21    |
22 LL |     let z = (x, 42);
23    |              ^ value moved from here
24 
25 error: moving 10024 bytes
-   --> $DIR/large_moves.rs:15:13
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/large_moves.rs:16:13
27    |
28 LL |     let z = (x, 42);
29    |             ^^^^^^^ value moved from here
30 
31 error: moving 10024 bytes
-   --> $DIR/large_moves.rs:17:13
+   --> $DIR/large_moves.rs:18:13
+   --> $DIR/large_moves.rs:18:13
33    |
34 LL |     let a = z.0;
35    |             ^^^ value moved from here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves/large_moves.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/large_moves.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/large_moves.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:10:13
   |
LL |       let x = async { //~ ERROR large_assignments
   |  _____________^
LL | |         let y = [0; 9999];
LL | |         dbg!(y);
LL | |         thing(&y).await;
LL | |         dbg!(y);
LL | |     };
   | |_____^ value moved from here
note: the lint level is defined here
  --> /checkout/src/test/ui/async-await/large_moves.rs:1:9
   |
   |
LL | #![deny(large_assignments)]

error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:16:14
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |              ^ value moved from here
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:16:13
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |             ^^^^^^^ value moved from here
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:18:13
   |
   |
LL |     let a = z.0; //~ ERROR large_assignments
   |             ^^^ value moved from here
error: aborting due to 4 previous errors


------------------------------------------
---
test result: FAILED. 11624 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 138.70s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:29
