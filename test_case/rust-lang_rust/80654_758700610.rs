plain
failures:

---- [incremental] incremental/issue-80336-invalid-span.rs stdout ----

error in revision `rpass1`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-80336-invalid-span.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "rpass1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-80336-invalid-span/issue-80336-invalid-span.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-80336-invalid-span/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-80336-invalid-span/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'stdsimd'
   |
   |
LL |     let _ = is_x86_feature_detected!("avx2");
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(stdsimd)]` to the crate attributes to enable
error: 
error: 
        is_x86_feature_detected can only be used on x86 and x86_64 targets.
        You can prevent it from being used in other architectures by
        guarding it behind a cfg(target_arch) as follows:

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                if is_x86_feature_detected(...) { ... }
        
  --> /checkout/src/test/incremental/issue-80336-invalid-span.rs:8:13
   |
   |
LL |     let _ = is_x86_feature_detected!("avx2");
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
---
test result: FAILED. 125 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 17.97s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-wasm32-unknown-emscripten" "--suite" "incremental" "--mode" "incremental" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/latest/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.0-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --host= --target wasm32-unknown-emscripten --exclude library/core --exclude library/alloc --exclude library/proc_macro --exclude library/std --exclude library/term --exclude library/test
Build completed unsuccessfully in 0:30:10
