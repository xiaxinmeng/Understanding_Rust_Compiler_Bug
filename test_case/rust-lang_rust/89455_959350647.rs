plain
failures:

---- [ui] ui/lint/lint-enum-intrinsics-non-enums.rs stdout ----

error: warning: diagnostic messages should not end with punctuations
  |
1 | the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `()`, which is not an enum.
  |                                                                                                                               - this is a punctuation
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: punctuated-endings` to top of the test file
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-enum-intrinsics-non-enums" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-enum-intrinsics-non-enums/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:26:5
   |
LL |     discriminant(&());
   |
   |
   = note: `#[deny(enum_intrinsics_non_enums)]` on by default
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `()`, which is not an enum.
   |
   |
LL |     discriminant(&());

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:29:5
   |
   |
LL |     discriminant(&&SomeEnum::B);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `&SomeEnum`, which is not an enum.
   |
   |
LL |     discriminant(&&SomeEnum::B);

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:32:5
   |
   |
LL |     discriminant(&SomeStruct);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `SomeStruct`, which is not an enum.
   |
   |
LL |     discriminant(&SomeStruct);

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:35:5
   |
   |
LL |     discriminant(&123u32);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `u32`, which is not an enum.
   |
   |
LL |     discriminant(&123u32);

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:38:5
   |
   |
LL |     discriminant(&&123i8);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `&i8`, which is not an enum.
   |
   |
LL |     discriminant(&&123i8);

error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:46:5
   |
   |
LL |     variant_count::<&str>();
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `&str`, which is not an enum.
error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:49:5
   |
   |
LL |     variant_count::<*const u8>();
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `*const u8`, which is not an enum.
error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:52:5
   |
LL |     variant_count::<()>();
LL |     variant_count::<()>();
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `()`, which is not an enum.
error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:55:5
   |
   |
LL |     variant_count::<&SomeEnum>();
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `&SomeEnum`, which is not an enum.
error: aborting due to 9 previous errors


------------------------------------------
---
test result: FAILED. 12260 passed; 1 failed; 110 ignored; 0 measured; 0 filtered out; finished in 137.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:06
