plain
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/simd-type.rs:11:1
+   --> $DIR/type-len.rs:11:1
9    |
10 LL | struct empty2([f32; 0]);

12 
13 error[E0076]: SIMD vector should be homogeneous
-   --> $DIR/simd-type.rs:17:1
-   --> $DIR/simd-type.rs:17:1
+   --> $DIR/type-len.rs:17:1
15    |
16 LL | struct i64f64(i64, f64);
17    | ^^^^^^^^^^^^^^^^^^^^^^^^ SIMD elements must have the same type
18 
18 
19 error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
-   --> $DIR/simd-type.rs:22:1
21    |
21    |
22 LL | struct FooV(Foo, Foo);

24 
24 
25 error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
-   --> $DIR/simd-type.rs:25:1
27    |
27    |
28 LL | struct FooV2([Foo; 2]);

30 
31 error[E0075]: SIMD vector cannot have more than 32768 elements
-   --> $DIR/simd-type.rs:28:1
-   --> $DIR/simd-type.rs:28:1
+   --> $DIR/type-len.rs:28:1
33    |
34 LL | struct TooBig([f32; 65536]);


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-len/type-len.stderr
To only update this specific test, also pass `--test-args simd/type-len.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/type-len.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-len" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-len/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0075]: SIMD vector cannot be empty
  --> /checkout/src/test/ui/simd/type-len.rs:8:1
   |
LL | struct empty; //~ ERROR SIMD vector cannot be empty

error[E0075]: SIMD vector cannot be empty
  --> /checkout/src/test/ui/simd/type-len.rs:11:1
   |
   |
LL | struct empty2([f32; 0]); //~ ERROR SIMD vector cannot be empty

error[E0076]: SIMD vector should be homogeneous
  --> /checkout/src/test/ui/simd/type-len.rs:17:1
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
  --> /checkout/src/test/ui/simd/type-len.rs:28:1
   |
   |
LL | struct TooBig([f32; 65536]); //~ ERROR SIMD vector cannot have more than 32768 elements

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0075, E0076, E0077.
---
test result: FAILED. 12152 passed; 1 failed; 117 ignored; 0 measured; 0 filtered out; finished in 109.52s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:14
