plain
.................................................................................................... 8500/11717
.................................................................................................... 8600/11717
.................................................................................................... 8700/11717
.................................................................................................... 8800/11717
...........................................iiiiiiiii................................................ 8900/11717
.................................................................................................... 9100/11717
.................................................................................................... 9200/11717
.................................................................................................... 9300/11717
.................................................................................................... 9400/11717
.................................................................................................... 9400/11717
.................................................................................................... 9500/11717
..........................................................i......i.................................. 9600/11717
.................................................................................................... 9700/11717
....iiiiiii..iiiiii.i............................................................................... 9800/11717
.................................................................................................... 10000/11717
.................................................................................................... 10100/11717
.................................................................................................... 10200/11717
.................................................................................................... 10300/11717
---
 finished in 0.404 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.125 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.28s

 finished in 2.352 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiii..........

 finished in 0.596 seconds
Build completed successfully in 0:30:10
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 11717 tests
......................i............................................................................. 100/11717
.....................................iiiii..iii.iiiiiiiii........................................... 200/11717
.................................................................................................... 400/11717
.................................................................................................... 500/11717
.................................................................................................... 600/11717
.............................F...................................................................... 700/11717
---
.................................................................................................... 9300/11717
.................................................................................................... 9400/11717
.................................................................................................... 9500/11717
..........................................................i......i.................................. 9600/11717
...................iiiiiii.......................................................................... 9700/11717
....iiiiiii..iiiiii.i............................................................................... 9800/11717
.................................................................................................... 10000/11717
.................................................................................................... 10100/11717
.................................................................................................... 10200/11717
.................................................................................................... 10300/11717
---
........i.i......................................................................................... 11700/11717
.................
failures:

---- [ui] ui/async-await/large_moves.rs stdout ----

- error: moving 10024 bytes
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
+ error: moving 10012 bytes
+ error: moving 10012 bytes
2   --> $DIR/large_moves.rs:9:13
3    |
4 LL |       let x = async {

16 LL | #![deny(large_assignments)]
18 
- error: moving 10024 bytes
+ error: moving 10012 bytes
20   --> $DIR/large_moves.rs:15:14
20   --> $DIR/large_moves.rs:15:14
21    |
22 LL |     let z = (x, 42);
23    |              ^ value moved from here
24 
- error: moving 10024 bytes
+ error: moving 10012 bytes
+ error: moving 10012 bytes
26   --> $DIR/large_moves.rs:15:13
27    |
28 LL |     let z = (x, 42);
29    |             ^^^^^^^ value moved from here
30 
- error: moving 10024 bytes
+ error: moving 10012 bytes
+ error: moving 10012 bytes
32   --> $DIR/large_moves.rs:17:13
33    |
34 LL |     let a = z.0;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves/large_moves.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/large_moves.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/large_moves.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: moving 10012 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:9:13
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

error: moving 10012 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:15:14
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |              ^ value moved from here
error: moving 10012 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:15:13
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |             ^^^^^^^ value moved from here
error: moving 10012 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:17:13
   |
   |
LL |     let a = z.0; //~ ERROR large_assignments
   |             ^^^ value moved from here
error: aborting due to 4 previous errors


------------------------------------------
---
test result: FAILED. 11583 passed; 1 failed; 133 ignored; 0 measured; 0 filtered out; finished in 71.53s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/ui --pass=check --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:13
