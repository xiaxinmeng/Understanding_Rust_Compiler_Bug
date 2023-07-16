plain
test [assembly] assembly/asm/x86-modifiers.rs#i686 ... ok
test [assembly] assembly/asm/riscv-types.rs#riscv64 ... ok
test [assembly] assembly/asm/x86-modifiers.rs#x86_64 ... ok
test [assembly] assembly/asm/arm-modifiers.rs ... ok
test [assembly] assembly/target-feature-multiple.rs#TWOFLAGS ... FAILED
test [assembly] assembly/target-feature-multiple.rs#SINGLEFLAG ... FAILED
test [assembly] assembly/asm/arm-types.rs ... ok
test [assembly] assembly/asm/aarch64-types.rs ... ok
test [assembly] assembly/asm/x86-types.rs#x86_64 ... ok
test [assembly] assembly/asm/x86-types.rs#x86_64 ... ok
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=nvptx64-nvidia-cuda

failures:


---- [assembly] assembly/target-feature-multiple.rs#TWOFLAGS stdout ----

error in revision `TWOFLAGS`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/target-feature-multiple.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "--cfg" "twoflags" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/target-feature-multiple.TWOFLAGS/target-feature-multiple.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-C" "target-feature=+rdrnd" "-C" "target-feature=+rdseed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/target-feature-multiple.TWOFLAGS/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
warning: `extern` block uses type `(u32, i32)`, which is not FFI-safe
   |
   |
29 |     fn x86_rdrand32_step() -> (u32, i32);
   |                               ^^^^^^^^^^ not FFI-safe
   |
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider using a struct instead
   = note: tuples have unspecified layout

warning: `extern` block uses type `(u32, i32)`, which is not FFI-safe
   |
   |
31 |     fn x86_rdseed32_step() -> (u32, i32);
   |                               ^^^^^^^^^^ not FFI-safe
   = help: consider using a struct instead
   = help: consider using a struct instead
   = note: tuples have unspecified layout

'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
LLVM ERROR: Cannot select: intrinsic %llvm.x86.rdseed.32
------------------------------------------



---- [assembly] assembly/target-feature-multiple.rs#SINGLEFLAG stdout ----

error in revision `SINGLEFLAG`: compilation failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/target-feature-multiple.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "--cfg" "singleflag" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/target-feature-multiple.SINGLEFLAG/target-feature-multiple.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-C" "target-feature=+rdrnd,+rdseed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/target-feature-multiple.SINGLEFLAG/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
warning: `extern` block uses type `(u32, i32)`, which is not FFI-safe
   |
   |
29 |     fn x86_rdrand32_step() -> (u32, i32);
   |                               ^^^^^^^^^^ not FFI-safe
   |
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider using a struct instead
   = note: tuples have unspecified layout

warning: `extern` block uses type `(u32, i32)`, which is not FFI-safe
   |
   |
31 |     fn x86_rdseed32_step() -> (u32, i32);
   |                               ^^^^^^^^^^ not FFI-safe
   = help: consider using a struct instead
   = help: consider using a struct instead
   = note: tuples have unspecified layout

'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
'+rdrnd' is not a recognized feature for this target (ignoring feature)
'+rdseed' is not a recognized feature for this target (ignoring feature)
LLVM ERROR: Cannot select: intrinsic %llvm.x86.rdseed.32
------------------------------------------




failures:
    [assembly] assembly/target-feature-multiple.rs#SINGLEFLAG
    [assembly] assembly/target-feature-multiple.rs#TWOFLAGS
test result: FAILED. 17 passed; 2 failed; 12 ignored; 0 measured; 0 filtered out; finished in 0.39s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-nvptx64-nvidia-cuda" "--suite" "assembly" "--mode" "assembly" "--target" "nvptx64-nvidia-cuda" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v14.4.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target nvptx64-nvidia-cuda src/test/run-make src/test/assembly
Build completed unsuccessfully in 0:00:30
