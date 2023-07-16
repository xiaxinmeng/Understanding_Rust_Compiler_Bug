plain
.................................................................................................... 9300/11520
.................................................................................................... 9400/11520
.......................................................................i......i..................... 9500/11520
.................................................................................................... 9600/11520
..........iiiiiii..iiiiii.i......................................................................... 9700/11520
............................................................F..F..................F............F.... 9800/11520
.................................................................................................... 10000/11520
.................................................................................................... 10100/11520
.................................................................................................... 10200/11520
.................................................................................................... 10300/11520
---
failures:

---- [ui] ui/simd/issue-17170.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/issue-17170.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-17170/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-17170/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error[E0075]: SIMD vector length must be a power of two
  --> /checkout/src/test/ui/simd/issue-17170.rs:5:1
   |
LL | struct T(f64, f64, f64);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0075`.
For more information about this error, try `rustc --explain E0075`.

------------------------------------------


---- [ui] ui/simd/issue-39720.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/issue-39720.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-39720/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/issue-39720/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0075]: SIMD vector length must be a power of two
  --> /checkout/src/test/ui/simd/issue-39720.rs:8:1
   |
LL | pub struct Char3(pub i8, pub i8, pub i8);

error[E0075]: SIMD vector length must be a power of two
  --> /checkout/src/test/ui/simd/issue-39720.rs:12:1
   |
   |
LL | pub struct Short3(pub i16, pub i16, pub i16);

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0075`.
For more information about this error, try `rustc --explain E0075`.

------------------------------------------


---- [ui] ui/simd/simd-type.rs stdout ----
diff of stderr:

10 LL | struct empty2([f32; 0]);
12 
+ error[E0075]: SIMD vector length must be a power of two
+   --> $DIR/simd-type.rs:13:1
+    |
+    |
+ LL | struct pow2([f32; 7]);
+ 
13 error[E0076]: SIMD vector should be homogeneous
14   --> $DIR/simd-type.rs:16:1
15    |
15    |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/simd-type/simd-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args simd/simd-type.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/simd-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/simd-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/simd-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0075]: SIMD vector cannot be empty
  --> /checkout/src/test/ui/simd/simd-type.rs:7:1
   |
LL | struct empty; //~ ERROR SIMD vector cannot be empty

error[E0075]: SIMD vector cannot be empty
  --> /checkout/src/test/ui/simd/simd-type.rs:10:1
   |
   |
LL | struct empty2([f32; 0]); //~ ERROR SIMD vector cannot be empty

error[E0075]: SIMD vector length must be a power of two
  --> /checkout/src/test/ui/simd/simd-type.rs:13:1
   |
   |
LL | struct pow2([f32; 7]); //~ ERROR SIMD vector length must be a power of two

error[E0076]: SIMD vector should be homogeneous
  --> /checkout/src/test/ui/simd/simd-type.rs:16:1
   |
   |
LL | struct i64f64(i64, f64); //~ ERROR SIMD vector should be homogeneous
   | ^^^^^^^^^^^^^^^^^^^^^^^^ SIMD elements must have the same type

error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
   |
   |
LL | struct FooV(Foo, Foo); //~ ERROR SIMD vector element type should be a primitive scalar (integer/float/pointer) type


error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
   |
   |
LL | struct FooV2([Foo; 2]); //~ ERROR SIMD vector element type should be a primitive scalar (integer/float/pointer) type

error[E0075]: SIMD vector cannot have more than 32768 elements
  --> /checkout/src/test/ui/simd/simd-type.rs:27:1
   |
   |
LL | struct TooBig([f32; 65536]); //~ ERROR SIMD vector cannot have more than 32768 elements

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0075, E0076, E0077.
---
---- [ui] ui/simd/simd-type-generic-monomorphisation-power-of-two.rs stdout ----

error: ui test compiled successfully!
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/simd-type-generic-monomorphisation-power-of-two.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/simd-type-generic-monomorphisation-power-of-two" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/simd-type-generic-monomorphisation-power-of-two/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 11423 passed; 4 failed; 93 ignored; 0 measured; 0 filtered out; finished in 130.71s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:38
