plain

---- [ui] ui/llvm-asm/asm-src-loc-codegen-units.rs stdout ----
diff of stderr:

- error: invalid instruction mnemonic 'nowayisthisavalidinstruction'
+ error: invalid instruction
3    |
3    |
4 LL |         llvm_asm!("nowayisthisavalidinstruction");

8   --> <inline asm>:1:2
9    |
10 LL |     nowayisthisavalidinstruction
+    |     ^
12 
13 error: aborting due to previous error
14 
14 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/asm-src-loc-codegen-units/asm-src-loc-codegen-units.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args llvm-asm/asm-src-loc-codegen-units.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/asm-src-loc-codegen-units.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/asm-src-loc-codegen-units" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-C" "codegen-units=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/asm-src-loc-codegen-units/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid instruction
  --> /checkout/src/test/ui/llvm-asm/asm-src-loc-codegen-units.rs:9:9
   |
LL |         llvm_asm!("nowayisthisavalidinstruction"); //~ ERROR instruction
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
   |
LL |     nowayisthisavalidinstruction

error: aborting due to previous error

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=arm-linux-androideabi
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=arm-linux-androideabi

------------------------------------------


---- [ui] ui/llvm-asm/asm-src-loc.rs stdout ----
diff of stderr:

- error: invalid instruction mnemonic 'nowayisthisavalidinstruction'
+ error: invalid instruction
3    |
3    |
4 LL |         llvm_asm!("nowayisthisavalidinstruction");

8   --> <inline asm>:1:2
9    |
10 LL |     nowayisthisavalidinstruction
+    |     ^
12 
13 error: aborting due to previous error
14 
14 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/asm-src-loc/asm-src-loc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args llvm-asm/asm-src-loc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/llvm-asm/asm-src-loc.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/asm-src-loc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-asm/asm-src-loc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid instruction
  --> /checkout/src/test/ui/llvm-asm/asm-src-loc.rs:8:9
   |
LL |         llvm_asm!("nowayisthisavalidinstruction"); //~ ERROR instruction
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
   |
LL |     nowayisthisavalidinstruction

error: aborting due to previous error


---
test result: FAILED. 11076 passed; 2 failed; 148 ignored; 0 measured; 0 filtered out; finished in 703.25s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-linux-androideabi" "--suite" "ui" "--mode" "ui" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--llvm-version" "11.0.0-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target arm-linux-androideabi
Build completed unsuccessfully in 0:38:45
