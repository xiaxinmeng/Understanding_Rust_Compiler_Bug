plain
failures:

---- [codegen] codegen/sparc-struct-abi.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sparc-struct-abi/sparc-struct-abi.ll" "/checkout/src/test/codegen/sparc-struct-abi.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/sparc-struct-abi.rs:85:11: error: CHECK: expected string not found in input
// CHECK: define void @structfloatlongfloat_input(%FloatLongFloat* noalias nocapture dereferenceable(24) %a)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sparc-struct-abi/sparc-struct-abi.ll:41:123: note: scanning from here
 ret { float, i32, i64, float, i32 } { float 0x3FB99999A0000000, i32 undef, i64 123, float 0x40091EB860000000, i32 undef }
                                                                                                                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sparc-struct-abi/sparc-struct-abi.ll:45:1: note: possible intended match here
define void @structfloatlongfloat_input(%FloatLongFloat* noalias nocapture readnone dereferenceable(24) %a) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sparc-struct-abi/sparc-struct-abi.ll
Check file: /checkout/src/test/codegen/sparc-struct-abi.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           36: } 
           37:  
           38: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
           39: define inreg { float, i32, i64, float, i32 } @structfloatlongfloat() unnamed_addr #0 { 
           40: start: 
           41:  ret { float, i32, i64, float, i32 } { float 0x3FB99999A0000000, i32 undef, i64 123, float 0x40091EB860000000, i32 undef } 
check:85'0                                                                                                                               X error: no match found
           42: } 
check:85'0     ~~
           43:  
check:85'0     ~
           44: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
check:85'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45: define void @structfloatlongfloat_input(%FloatLongFloat* noalias nocapture readnone dereferenceable(24) %a) unnamed_addr #0 { 
check:85'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:85'1     ?                                                                                                                              possible intended match
           46: start: 
check:85'0     ~~~~~~~
           47:  ret void 
check:85'0     ~~~~~~~~~~
           48: } 
check:85'0     ~~
           49:  
check:85'0     ~
           50: attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "target-cpu"="v9" } 
check:85'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>

---
test result: FAILED. 265 passed; 1 failed; 39 ignored; 0 measured; 0 filtered out; finished in 4.50s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-stable" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "stable" "--color" "always"


Build completed unsuccessfully in 0:19:34
