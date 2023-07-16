plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 134 tests
iiiiF.iiiiiiiiii..........F......................................................................... 100/134
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [assembly] assembly/asm/aarch64-modifiers.rs stdout ----
---- [assembly] assembly/asm/aarch64-modifiers.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/aarch64-modifiers.rs" "-Zthreads=1" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-modifiers/aarch64-modifiers.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "--target" "aarch64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-modifiers/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: register class `vreg` requires the `fp` target feature
   |
   |
39 |             asm!($code, out($reg) y);
...
...
67 | check!(vreg vreg "add {0}.4s, {0}.4s, {0}.4s");
   |
   = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
   |
   |
39 |             asm!($code, out($reg) y);
...
...
73 | check!(vreg_b vreg "ldr {:b}, [x0]");
   |
   = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
   |
   |
39 |             asm!($code, out($reg) y);
...
...
79 | check!(vreg_h vreg "ldr {:h}, [x0]");
   |
   = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
   |
   |
39 |             asm!($code, out($reg) y);
...
...
85 | check!(vreg_s vreg "ldr {:s}, [x0]");
   |
   = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
   |
   |
39 |             asm!($code, out($reg) y);
...
...
91 | check!(vreg_d vreg "ldr {:d}, [x0]");
   |
   = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
   |
   |
39 |             asm!($code, out($reg) y);
...
...
97 | check!(vreg_q vreg "ldr {:q}, [x0]");
   |
   = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
39  |             asm!($code, out($reg) y);
...
...
103 | check!(vreg_v vreg "add {0:v}.4s, {0:v}.4s, {0:v}.4s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
39  |             asm!($code, out($reg) y);
...
...
109 | check!(vreg_low16 vreg_low16 "add {0}.4s, {0}.4s, {0}.4s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
39  |             asm!($code, out($reg) y);
...
...
115 | check!(vreg_low16_b vreg_low16 "ldr {:b}, [x0]");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
39  |             asm!($code, out($reg) y);
...
...
121 | check!(vreg_low16_h vreg_low16 "ldr {:h}, [x0]");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
39  |             asm!($code, out($reg) y);
...
...
127 | check!(vreg_low16_s vreg_low16 "ldr {:s}, [x0]");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
39  |             asm!($code, out($reg) y);
...
...
133 | check!(vreg_low16_d vreg_low16 "ldr {:d}, [x0]");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
39  |             asm!($code, out($reg) y);
...
...
139 | check!(vreg_low16_q vreg_low16 "ldr {:q}, [x0]");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
39  |             asm!($code, out($reg) y);
...
...
145 | check!(vreg_low16_v vreg_low16 "add {0:v}.4s, {0:v}.4s, {0:v}.4s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 14 previous errors
---
---- [assembly] assembly/asm/aarch64-types.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/aarch64-types.rs" "-Zthreads=1" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "aarch64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
193 | check!(vreg_i8 i8 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
193 | check!(vreg_i8 i8 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
199 | check!(vreg_i16 i16 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
199 | check!(vreg_i16 i16 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
205 | check!(vreg_i32 i32 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
205 | check!(vreg_i32 i32 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
211 | check!(vreg_f32 f32 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
211 | check!(vreg_f32 f32 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
217 | check!(vreg_i64 i64 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
217 | check!(vreg_i64 i64 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
223 | check!(vreg_f64 f64 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
223 | check!(vreg_f64 f64 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
229 | check!(vreg_ptr ptr vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
229 | check!(vreg_ptr ptr vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
235 | check!(vreg_i8x8 i8x8 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
235 | check!(vreg_i8x8 i8x8 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
241 | check!(vreg_i16x4 i16x4 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
241 | check!(vreg_i16x4 i16x4 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
247 | check!(vreg_i32x2 i32x2 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
247 | check!(vreg_i32x2 i32x2 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
253 | check!(vreg_i64x1 i64x1 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
253 | check!(vreg_i64x1 i64x1 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
259 | check!(vreg_f32x2 f32x2 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
259 | check!(vreg_f32x2 f32x2 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
265 | check!(vreg_f64x1 f64x1 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
265 | check!(vreg_f64x1 f64x1 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
271 | check!(vreg_i8x16 i8x16 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
271 | check!(vreg_i8x16 i8x16 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
277 | check!(vreg_i16x8 i16x8 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
277 | check!(vreg_i16x8 i16x8 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
283 | check!(vreg_i32x4 i32x4 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
283 | check!(vreg_i32x4 i32x4 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
289 | check!(vreg_i64x2 i64x2 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
289 | check!(vreg_i64x2 i64x2 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
295 | check!(vreg_f32x4 f32x4 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
295 | check!(vreg_f32x4 f32x4 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
301 | check!(vreg_f64x2 f64x2 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
301 | check!(vreg_f64x2 f64x2 vreg "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
307 | check!(vreg_low16_i8 i8 vreg_low16 "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
307 | check!(vreg_low16_i8 i8 vreg_low16 "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
313 | check!(vreg_low16_i16 i16 vreg_low16 "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
313 | check!(vreg_low16_i16 i16 vreg_low16 "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
...
319 | check!(vreg_low16_f32 f32 vreg_low16 "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
123 |                 in($class) x
...
...
319 | check!(vreg_low16_f32 f32 vreg_low16 "fmov" "s");
    |
    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


error: register class `vreg_low16` requires the `fp` target feature
    |
    |
122 |                 out($class) y,
...
---
test result: FAILED. 115 passed; 2 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.55s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "assembly" "--mode" "assembly" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:00
