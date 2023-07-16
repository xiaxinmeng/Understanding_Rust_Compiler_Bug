plain
test [codegen] codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs ... ok
test [codegen] codegen/simd-intrinsic/simd-intrinsic-generic-scatter.rs ... ok
test [codegen] codegen/simd-intrinsic/simd-intrinsic-generic-select.rs ... ok
test [codegen] codegen/simd-intrinsic/simd-intrinsic-float-sin.rs ... ok
test [codegen] codegen/sse42-implies-crc32.rs ... ignored
test [codegen] codegen/slice-init.rs ... ok
test [codegen] codegen/sparc-struct-abi.rs ... ok
test [codegen] codegen/swap-small-types.rs ... ignored
test [codegen] codegen/slice-iter-len-eq-zero.rs ... ok
---
test [assembly] assembly/x86_64-fortanix-unknown-sgx-lvi-inline-assembly.rs ... ignored
test [assembly] assembly/asm/riscv-types.rs#riscv64 ... ok
test [assembly] assembly/asm/wasm-types.rs ... ok
test [assembly] assembly/static-relocation-model.rs#A64 ... ok
test [assembly] assembly/x86_64-sse_crc.rs ... FAILED
test [assembly] assembly/target-feature-multiple.rs#TWOFLAGS ... ok
test [assembly] assembly/target-feature-multiple.rs#SINGLEFLAG ... ok
test [assembly] assembly/static-relocation-model.rs#x64 ... ok
test [assembly] assembly/asm/x86-modifiers.rs#x86_64 ... ok
---
test [assembly] assembly/asm/x86-types.rs#i686 ... ok

failures:

---- [assembly] assembly/x86_64-sse_crc.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/x86_64-sse_crc.rs" "-Zthreads=1" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/assembly/x86_64-sse_crc/x86_64-sse_crc.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target=x86_64-unknown-linux-gnu" "--crate-type" "staticlib" "-Ctarget-feature=+sse4.2" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/assembly/x86_64-sse_crc/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.


------------------------------------------



failures:
    [assembly] assembly/x86_64-sse_crc.rs
test result: FAILED. 24 passed; 1 failed; 16 ignored; 0 measured; 0 filtered out; finished in 0.43s




command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "assembly" "--mode" "assembly" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:20:03
