plain
failures:

---- [ui] ui/sanitize/address.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/sanitize/address.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a" "-A" "unused" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "sanitizer=address" "-O" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL | use std::hint::black_box;
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL |     let xs = black_box(xs.as_ptr());
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/sanitize/leak.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/sanitize/leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/leak/a" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "sanitizer=leak" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/leak/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL | use std::hint::black_box;
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL |         let xs = black_box(xs);
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/sanitize/memory.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/sanitize/memory.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/memory/a" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "sanitizer=memory" "-Zsanitizer-memory-track-origins" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/memory/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL | use std::hint::black_box;
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   |
LL |     black_box(r)
   |     ^^^^^^^^^
   |
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.

---
test result: FAILED. 11703 passed; 3 failed; 78 ignored; 0 measured; 0 filtered out; finished in 133.38s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:19:10
