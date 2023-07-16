plain
test [assembly] assembly/asm/x86-types.rs#i686 ... ok

failures:

---- [assembly] assembly/asm/powerpc-types.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/powerpc-types/powerpc-types.s" "/checkout/src/test/assembly/asm/powerpc-types.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/assembly/asm/powerpc-types.rs:73:11: error: CHECK: expected string not found in input
// CHECK: mr {{[0-31]+}}, {{[0-31]+}}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/powerpc-types/powerpc-types.s:36:2: note: scanning from here
 mr 3, 4
 ^
/checkout/src/test/assembly/asm/powerpc-types.rs:79:11: error: CHECK: expected string not found in input
// CHECK: mr {{[0-31]+}}, {{[0-31]+}}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/powerpc-types/powerpc-types.s:80:2: note: scanning from here
 mr 3, 4
 ^
/checkout/src/test/assembly/asm/powerpc-types.rs:85:11: error: CHECK: expected string not found in input
// CHECK: mr {{[0-31]+}}, {{[0-31]+}}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/powerpc-types/powerpc-types.s:124:2: note: scanning from here
 mr 3, 4
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/powerpc-types/powerpc-types.s:126:3: note: possible intended match here
 stw 3, 20(1)
  ^
/checkout/src/test/assembly/asm/powerpc-types.rs:91:11: error: CHECK: expected string not found in input
// CHECK: mr {{[1-31]+}}, {{[1-31]+}}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/powerpc-types/powerpc-types.s:168:2: note: scanning from here
 mr 3, 4
 ^
/checkout/src/test/assembly/asm/powerpc-types.rs:97:11: error: CHECK: expected string not found in input
// CHECK: mr {{[1-31]+}}, {{[1-31]+}}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/powerpc-types/powerpc-types.s:212:2: note: scanning from here
 mr 3, 4
 ^
/checkout/src/test/assembly/asm/powerpc-types.rs:103:11: error: CHECK: expected string not found in input
// CHECK: mr {{[1-31]+}}, {{[1-31]+}}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/powerpc-types/powerpc-types.s:256:2: note: scanning from here
 mr 3, 4

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/powerpc-types/powerpc-types.s
Check file: /checkout/src/test/assembly/asm/powerpc-types.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           31:  bl dont_merge@PLT
           32:  b .LBB0_1
           33: .LBB0_1:
           34:  lwz 4, 16(1)
           35:  #APP
           36:  mr 3, 4
check:73        X~~~~~~ error: no match found
           37:  #NO_APP
check:73       ~~~~~~~~
           38:  stb 3, 23(1)
check:73       ~~~~~~~~~~~~~
           39:  b .LBB0_2
check:73       ~~~~~~~~~~
           40: .LBB0_2:
check:73       ~~~~~~~~
           41:  lbz 3, 23(1)
            .
            .
            .
            .
           75:  bl dont_merge@PLT
           76:  b .LBB1_1
           77: .LBB1_1:
           78:  lwz 4, 16(1)
           79:  #APP
           80:  mr 3, 4
check:79        X~~~~~~ error: no match found
           81:  #NO_APP
check:79       ~~~~~~~~
           82:  sth 3, 22(1)
check:79       ~~~~~~~~~~~~~
           83:  b .LBB1_2
check:79       ~~~~~~~~~~
           84: .LBB1_2:
check:79       ~~~~~~~~
           85:  lhz 3, 22(1)
            .
            .
            .
            .
          119:  bl dont_merge@PLT
          120:  b .LBB2_1
          121: .LBB2_1:
          122:  lwz 4, 16(1)
          123:  #APP
          124:  mr 3, 4
check:85'0      X~~~~~~ error: no match found
          125:  #NO_APP
check:85'0     ~~~~~~~~
          126:  stw 3, 20(1)
check:85'0     ~~~~~~~~~~~~~
check:85'1       ?           possible intended match
          127:  b .LBB2_2
check:85'0     ~~~~~~~~~~
          128: .LBB2_2:
check:85'0     ~~~~~~~~
          129:  lwz 3, 20(1)
check:85'0     ~~~~~~~~~~~~~
          130:  lwz 0, 36(1)
check:85'0     ~~~~~~~~~~~~~
          131:  lwz 30, 24(1)
check:85'0     ~~~~~~~~~~~~~~
            .
            .
            .
          163:  bl dont_merge@PLT
          164:  b .LBB3_1
          165: .LBB3_1:
          166:  lwz 4, 16(1)
          167:  #APP
          168:  mr 3, 4
check:91        X~~~~~~ error: no match found
          169:  #NO_APP
check:91       ~~~~~~~~
          170:  stb 3, 23(1)
check:91       ~~~~~~~~~~~~~
          171:  b .LBB3_2
check:91       ~~~~~~~~~~
          172: .LBB3_2:
check:91       ~~~~~~~~
          173:  lbz 3, 23(1)
            .
            .
            .
            .
          207:  bl dont_merge@PLT
          208:  b .LBB4_1
          209: .LBB4_1:
          210:  lwz 4, 16(1)
          211:  #APP
          212:  mr 3, 4
check:97        X~~~~~~ error: no match found
          213:  #NO_APP
check:97       ~~~~~~~~
          214:  sth 3, 22(1)
check:97       ~~~~~~~~~~~~~
          215:  b .LBB4_2
check:97       ~~~~~~~~~~
          216: .LBB4_2:
check:97       ~~~~~~~~
          217:  lhz 3, 22(1)
            .
            .
            .
            .
          251:  bl dont_merge@PLT
          252:  b .LBB5_1
          253: .LBB5_1:
          254:  lwz 4, 16(1)
          255:  #APP
          256:  mr 3, 4
check:103       X~~~~~~ error: no match found
          257:  #NO_APP
check:103      ~~~~~~~~
          258:  stw 3, 20(1)
check:103      ~~~~~~~~~~~~~
          259:  b .LBB5_2
check:103      ~~~~~~~~~~
          260: .LBB5_2:
check:103      ~~~~~~~~
          261:  lwz 3, 20(1)
            .
            .
            .
>>>>>>
---
test result: FAILED. 21 passed; 1 failed; 12 ignored; 0 measured; 0 filtered out; finished in 0.22s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "assembly" "--mode" "assembly" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:20:00
