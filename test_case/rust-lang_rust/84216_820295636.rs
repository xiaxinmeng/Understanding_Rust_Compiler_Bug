plain
.................................................................................................... 9400/11760
.................................................................................................... 9500/11760
..............................................................................................i..... 9600/11760
.i.................................................................................................. 9700/11760
........................................iiiiiii..iiiiii.i........................................... 9800/11760
.................................................................................................... 10000/11760
.................................................................................................... 10100/11760
.................................................................................................... 10200/11760
.................................................................................................... 10300/11760
---
failures:

---- [ui] ui/consts/cast-discriminant-zst-enum.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/cast-discriminant-zst-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/cast-discriminant-zst-enum/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/cast-discriminant-zst-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'bench_black_box'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
LL |     assert_eq!(-1i8, black_box(kind) as i8);
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL |     assert_eq!(-1i16, black_box(kind) as i16);
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL |     assert_eq!(-1i32, black_box(kind) as i32);
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL |     assert_eq!(-1i64, black_box(kind) as i64);
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL |     assert_eq!(-1i128, black_box(kind) as i128);
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/consts/const_discriminant.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_discriminant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_discriminant/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_discriminant/auxiliary"
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
LL |     assert_eq!(TEST_A, discriminant(black_box(&Test::A(17))));
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL |     assert_eq!(TEST_B, discriminant(black_box(&Test::B)));
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL |     assert_ne!(TEST_B, discriminant(black_box(&Test::C { a: 42, b: 7 })));
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'bench_black_box'
   |
   |
LL |     assert_eq!(TEST_V, discriminant(black_box(&SingleVariant::V)));
   |
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = note: see issue #64102 <https://github.com/rust-lang/rust/issues/64102> for more information
   = help: add `#![feature(bench_black_box)]` to the crate attributes to enable
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.

---
test result: FAILED. 11661 passed; 2 failed; 97 ignored; 0 measured; 0 filtered out; finished in 122.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:29
