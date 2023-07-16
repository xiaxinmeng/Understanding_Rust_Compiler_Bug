plain
---- [ui] ui/c-unwind/feature-gate-stdcall-unwind.rs stdout ----
diff of stderr:

7    = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=arm-linux-androideabi
8    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0570]: The ABI `"stdcall-unwind"` is not supported for the current target
+   --> $DIR/feature-gate-stdcall-unwind.rs:4:1
+    |
+ LL | extern "stdcall-unwind" fn f() {}
11 
- For more information about this error, try `rustc --explain E0658`.
+ error: aborting due to 2 previous errors
+ 
+ 
+ Some errors have detailed explanations: E0570, E0658.
+ For more information about an error, try `rustc --explain E0570`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-unwind/feature-gate-stdcall-unwind/feature-gate-stdcall-unwind.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args c-unwind/feature-gate-stdcall-unwind.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-unwind/feature-gate-stdcall-unwind.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-unwind/feature-gate-stdcall-unwind" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-unwind/feature-gate-stdcall-unwind/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/c-unwind/feature-gate-stdcall-unwind.rs:4:8
   |
LL | extern "stdcall-unwind" fn f() {}
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable

error[E0570]: The ABI `"stdcall-unwind"` is not supported for the current target
  --> /checkout/src/test/ui/c-unwind/feature-gate-stdcall-unwind.rs:4:1
   |
LL | extern "stdcall-unwind" fn f() {}

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0570, E0658.
---
---- [ui] ui/c-unwind/feature-gate-thiscall-unwind.rs stdout ----
diff of stderr:

7    = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
8    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0570]: The ABI `"thiscall-unwind"` is not supported for the current target
+    |
+    |
+ LL | extern "thiscall-unwind" fn f() {}
11 
- For more information about this error, try `rustc --explain E0658`.
+ error: aborting due to 2 previous errors
+ 
+ 
+ Some errors have detailed explanations: E0570, E0658.
+ For more information about an error, try `rustc --explain E0570`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-unwind/feature-gate-thiscall-unwind/feature-gate-thiscall-unwind.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args c-unwind/feature-gate-thiscall-unwind.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-unwind/feature-gate-thiscall-unwind.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-unwind/feature-gate-thiscall-unwind" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-unwind/feature-gate-thiscall-unwind/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/c-unwind/feature-gate-thiscall-unwind.rs:4:8
   |
LL | extern "thiscall-unwind" fn f() {}
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable

error[E0570]: The ABI `"thiscall-unwind"` is not supported for the current target
   |
   |
LL | extern "thiscall-unwind" fn f() {}

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0570, E0658.
---
test result: FAILED. 11162 passed; 2 failed; 151 ignored; 0 measured; 0 filtered out; finished in 541.04s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-linux-androideabi" "--suite" "ui" "--mode" "ui" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--llvm-version" "11.0.1-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target arm-linux-androideabi
Build completed unsuccessfully in 0:28:05
