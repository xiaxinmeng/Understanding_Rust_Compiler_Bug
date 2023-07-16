plain
failures:

---- [ui] ui/consts/issue-79137-monomorphic.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-79137-monomorphic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-monomorphic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-monomorphic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
   |
LL |     pub const VALUE: usize = std::mem::variant_count::<T>();
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[deny(enum_intrinsics_non_enums)]` on by default
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `T`, which is not an enum.
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/lint/lint-enum-intrinsics-non-enums.rs stdout ----


1 error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
+   --> $DIR/lint-enum-intrinsics-non-enums.rs:15:5
+    |
+ LL |     discriminant::<T>(v);
+    |
+    |
+    = note: `#[deny(enum_intrinsics_non_enums)]` on by default
+ note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `T`, which is not an enum.
+   --> $DIR/lint-enum-intrinsics-non-enums.rs:15:23
+    |
+ LL |     discriminant::<T>(v);
+ 
+ 
+ error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
+   --> $DIR/lint-enum-intrinsics-non-enums.rs:19:5
+ LL |     variant_count::<T>()
+    |     ^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+    = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `T`, which is not an enum.
+ 
+ error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
2   --> $DIR/lint-enum-intrinsics-non-enums.rs:26:5
3    |
4 LL |     discriminant(&());
5    |     ^^^^^^^^^^^^^^^^^
6    |
6    |
-    = note: `#[deny(enum_intrinsics_non_enums)]` on by default
8 note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `()`, which is not an enum.
9   --> $DIR/lint-enum-intrinsics-non-enums.rs:26:18

91    |
91    |
92    = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `&SomeEnum`, which is not an enum.
- error: aborting due to 9 previous errors
+ error: aborting due to 11 previous errors
95 
96 
96 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-enum-intrinsics-non-enums/lint-enum-intrinsics-non-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-enum-intrinsics-non-enums.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-enum-intrinsics-non-enums" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-enum-intrinsics-non-enums/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:15:5
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL |     discriminant::<T>(v);
   |
   |
   = note: `#[deny(enum_intrinsics_non_enums)]` on by default
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `T`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:15:23
   |
LL |     discriminant::<T>(v);


error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:19:5
LL |     variant_count::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `T`, which is not an enum.

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:26:5
   |
LL |     discriminant(&());
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `()`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:26:18
   |
LL |     discriminant(&());


error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:29:5
   |
LL |     discriminant(&&SomeEnum::B);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `&SomeEnum`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:29:18
   |
LL |     discriminant(&&SomeEnum::B);


error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:32:5
   |
LL |     discriminant(&SomeStruct);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `SomeStruct`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:32:18
   |
LL |     discriminant(&SomeStruct);


error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:35:5
   |
LL |     discriminant(&123u32);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `u32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:35:18
   |
LL |     discriminant(&123u32);


error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:38:5
   |
LL |     discriminant(&&123i8);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `&i8`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:38:18
   |
LL |     discriminant(&&123i8);


error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:46:5
   |
LL |     variant_count::<&str>();
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `&str`, which is not an enum.

error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:49:5
   |
LL |     variant_count::<*const u8>();
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `*const u8`, which is not an enum.

error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:52:5
LL |     variant_count::<()>();
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `()`, which is not an enum.

error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:55:5
   |
LL |     variant_count::<&SomeEnum>();
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `&SomeEnum`, which is not an enum.
error: aborting due to 11 previous errors


------------------------------------------
---
test result: FAILED. 11639 passed; 2 failed; 96 ignored; 0 measured; 0 filtered out; finished in 145.90s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:40
