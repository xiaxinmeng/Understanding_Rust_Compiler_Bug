plain
.................................................................................................... 9400/11715
.................................................................................................... 9500/11715
........................................................i......i.................................... 9600/11715
.................................................................................................... 9700/11715
..iiiiiii..iiiiii.i................................................................................. 9800/11715
.................................................................................................... 10000/11715
.................................................................................................... 10100/11715
.................................................................................................... 10200/11715
.................................................................................................... 10300/11715
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.113 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i....i......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.30s

 finished in 2.377 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiii..........

 finished in 0.587 seconds
Build completed successfully in 0:30:10
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 160 tests
..................................i.............................i..............i.................... 100/160
F..............F..........Fi................................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu

---- [mir-opt] mir-opt/issue-72181.rs stdout ----
---- [mir-opt] mir-opt/issue-72181.rs stdout ----
38         _2 = [move _3, move _4];         // scope 1 at $DIR/issue-72181.rs:26:13: 26:43
39         StorageDead(_4);                 // scope 1 at $DIR/issue-72181.rs:26:42: 26:43
40         StorageDead(_3);                 // scope 1 at $DIR/issue-72181.rs:26:42: 26:43
-         FakeRead(ForLet, _2);            // scope 1 at $DIR/issue-72181.rs:26:9: 26:10
+         FakeRead(ForLet(None), _2);      // scope 1 at $DIR/issue-72181.rs:26:9: 26:10
42         StorageLive(_5);                 // scope 2 at $DIR/issue-72181.rs:27:13: 27:30
43         StorageLive(_6);                 // scope 4 at $DIR/issue-72181.rs:27:24: 27:25
44         _6 = const 0_usize;              // scope 4 at $DIR/issue-72181.rs:27:24: 27:25

thread '[mir-opt] mir-opt/issue-72181.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_72181.main.mir_map.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3517:25

---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
46     bb0: {
46     bb0: {
47         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
48         _1 = [const Const(Value(Scalar(0x00000001)): usize), const Const(Value(Scalar(0x00000002)): usize), const Const(Value(Scalar(0x00000003)): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:17:17: 17:26
-         FakeRead(ForLet, _1);            // bb0[2]: scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
+         FakeRead(ForLet(None), _1);      // bb0[2]: scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
50         StorageLive(_2);                 // bb0[3]: scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
51         StorageLive(_3);                 // bb0[4]: scope 1 at $DIR/region-subtyping-basic.rs:18:16: 18:17
52         _3 = const Const(Value(Scalar(0x00000000)): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:18:16: 18:17
57 
58     bb1: {
58     bb1: {
59         _2 = &'_#3r _1[_3];              // bb1[0]: scope 1 at $DIR/region-subtyping-basic.rs:18:13: 18:18
-         FakeRead(ForLet, _2);            // bb1[1]: scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
+         FakeRead(ForLet(None), _2);      // bb1[1]: scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
61         StorageLive(_6);                 // bb1[2]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
62         _6 = _2;                         // bb1[3]: scope 2 at $DIR/region-subtyping-basic.rs:19:13: 19:14
-         FakeRead(ForLet, _6);            // bb1[4]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
+         FakeRead(ForLet(None), _6);      // bb1[4]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
64         StorageLive(_7);                 // bb1[5]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
65         _7 = const Const(Value(Scalar(0x01)): bool); // bb1[6]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
66         switchInt(move _7) -> [Const(Value(Scalar(0x00)): bool): bb3, otherwise: bb2]; // bb1[7]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6

thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3517:25
---- [mir-opt] mir-opt/simple-match.rs stdout ----
---- [mir-opt] mir-opt/simple-match.rs stdout ----
5     let mut _0: usize;                   // return place in scope 0 at $DIR/simple-match.rs:5:27: 5:32
7     bb0: {
7     bb0: {
-         FakeRead(ForMatchedPlace, _1);   // scope 0 at $DIR/simple-match.rs:6:11: 6:12
+         FakeRead(ForMatchedPlace(None), _1); // scope 0 at $DIR/simple-match.rs:6:11: 6:12
9         switchInt(_1) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/simple-match.rs:7:9: 7:13
11 


thread '[mir-opt] mir-opt/simple-match.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simple_match.match_bool.mir_map.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3517:25

failures:
    [mir-opt] mir-opt/issue-72181.rs
    [mir-opt] mir-opt/nll/region-subtyping-basic.rs
    [mir-opt] mir-opt/nll/region-subtyping-basic.rs
    [mir-opt] mir-opt/simple-match.rs

test result: FAILED. 153 passed; 3 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.02s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:49
