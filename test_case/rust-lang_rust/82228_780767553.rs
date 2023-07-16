plain
.................................................................................................... 9200/11455
.................................................................................................... 9300/11455
.................................................................................................... 9400/11455
..............i.......i............................................................................. 9500/11455
.....................................................iiiiiii.iiiiii.i............................... 9600/11455
.................................................................................................... 9800/11455
.................................................................................................... 9900/11455
.................................................................................................... 10000/11455
.................................................................................................... 10100/11455
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.064 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.697 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.50s

 finished in 2.581 seconds
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 20 tests
iiiiiiiiiiii........

 finished in 0.591 seconds
Build completed successfully in 0:33:03
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 11455 tests
......................i............................................................................. 100/11455
....................................iiiiiiii.iiiiiiiii.............................................. 200/11455
.................................................................................................... 400/11455
.................................................................................................... 500/11455
.................................................................................................... 600/11455
..............................................................................ii.................... 700/11455
---
.................................................................................................... 9100/11455
.................................................................................................... 9200/11455
.................................................................................................... 9300/11455
.................................................................................................... 9400/11455
..............i......i.....................................................iiiiiii.................. 9500/11455
....................................................iiiiiii..iiiiii.i............................... 9600/11455
.................................................................................................... 9800/11455
.................................................................................................... 9900/11455
.................................................................................................... 10000/11455
.................................................................................................... 10100/11455
---

---- [ui] ui/try-block/try-block-bad-type.rs stdout ----
diff of stderr:

8    = help: the following implementations were found:
9              <i32 as From<NonZeroI32>>
10              <i32 as From<NonZero_c_int>>
+              <i32 as From<NonZero_c_long>>
11              <i32 as From<bool>>
-              <i32 as From<i16>>
-            and 3 others
+            and 4 others
14    = note: required by `from`
15 
16 error[E0271]: type mismatch resolving `<Result<i32, i32> as Try>::Ok == &str`

The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-block/try-block-bad-type.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-bad-type.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `?` couldn't convert the error to `i32`
   |
   |
LL |         Err("")?; //~ ERROR `?` couldn't convert the error
   |                ^ the trait `From<&str>` is not implemented for `i32`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following implementations were found:
             <i32 as From<NonZeroI32>>
             <i32 as From<NonZero_c_int>>
             <i32 as From<NonZero_c_long>>
             <i32 as From<bool>>
           and 4 others
   = note: required by `from`

error[E0271]: type mismatch resolving `<Result<i32, i32> as Try>::Ok == &str`
   |
   |
LL |         "" //~ ERROR type mismatch
   |         ^^ expected `i32`, found `&str`

error[E0271]: type mismatch resolving `<Result<i32, i32> as Try>::Ok == ()`
   |
   |
LL |     let res: Result<i32, i32> = try { }; //~ ERROR type mismatch
   |                                       ^ expected `i32`, found `()`

error[E0277]: the trait bound `(): Try` is not satisfied
   |
   |
LL |     let res: () = try { };
   |                         ^ the trait `Try` is not implemented for `()`
   |
   = note: required by `from_ok`

error[E0277]: the trait bound `(): Try` is not satisfied
   |
   |
LL |     let res: () = try { };
   |                         ^ the trait `Try` is not implemented for `()`

error[E0277]: the trait bound `i32: Try` is not satisfied
   |
   |
LL |     let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: Try` is not satisfied
   |                          ^ the trait `Try` is not implemented for `i32`
   |
   = note: required by `from_ok`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0271, E0277.
For more information about an error, try `rustc --explain E0271`.
---
test result: FAILED. 11325 passed; 1 failed; 129 ignored; 0 measured; 0 filtered out; finished in 75.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/ui --pass=check --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:17
