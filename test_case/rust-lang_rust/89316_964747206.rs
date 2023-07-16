plain
downloading https://static.rust-lang.org/dist/2021-10-22/rustc-beta-x86_64-unknown-linux-gnu.tar.xz
#=#=#                                                                         
##O#- #                                                                       
curl: (22) The requested URL returned error: 503 
Warning: Transient problem: HTTP error Will retry in 1 seconds. 3 retries 
##O=#  #                                                                      
#=#=-#  #                                                                     

                                                                           0.1%
---
test [ui] ui/asm/x86_64/duplicate-options.rs ... ok
test [ui] ui/assign-imm-local-twice.rs ... ok
test [ui] ui/asm/x86_64/type-check-3.rs ... ok
test [ui] ui/asm/x86_64/type-check-2.rs ... ok
test [ui] ui/asm/x86_64/multiple-clobber-abi.rs ... FAILED
test [ui] ui/assignment-operator-unimplemented.rs ... ok
test [ui] ui/assoc-inherent.rs ... ok
test [ui] ui/assoc-lang-items.rs ... ok
test [ui] ui/asm/x86_64/issue-89875.rs ... ok
---
test [ui] ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] ui/asm/x86_64/multiple-clobber-abi.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/multiple-clobber-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/multiple-clobber-abi/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/multiple-clobber-abi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: sym operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/multiple-clobber-abi.rs:25:13
LL |             sym foo,
   |             ^^^^^^^
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable

error[E0658]: sym operands for inline assembly are unstable
  --> /checkout/src/test/ui/asm/x86_64/multiple-clobber-abi.rs:26:13
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |             sym bar,
   |             ^^^^^^^
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------



failures:
    [ui] ui/asm/x86_64/multiple-clobber-abi.rs
test result: FAILED. 12279 passed; 1 failed; 111 ignored; 0 measured; 0 filtered out; finished in 149.55s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:53
