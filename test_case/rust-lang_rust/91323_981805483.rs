plain

- error: any use of this value will cause an error
-   --> $DIR/assert-type-intrinsics.rs:14:9
-    |
- LL | /     const _BAD1: () = unsafe {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL | |         MaybeUninit::<!>::uninit().assume_init();
-    | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ aborted execution: attempted to instantiate uninhabited type `!`
- LL | |     };
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
- 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error: `assert_uninit_valid` is not yet stable as a const fn
16    |
16    |
- LL | /     const _BAD2: () = unsafe {
- LL | |         intrinsics::assert_uninit_valid::<bool>();
-    | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ aborted execution: attempted to leave type `bool` uninitialized, which is invalid
- LL | |     };
-    | |______-
+ LL |         intrinsics::assert_uninit_valid::<bool>();
22    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = help: add `#![feature(const_assert_type2)]` to the crate attributes to enable
+    = help: add `#![feature(const_assert_type2)]` to the crate attributes to enable
25 
- error: any use of this value will cause an error
+ error: `assert_zero_valid` is not yet stable as a const fn
28    |
28    |
- LL | /     const _BAD3: () = unsafe {
- LL | |         intrinsics::assert_zero_valid::<&'static i32>();
-    | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ aborted execution: attempted to zero-initialize type `&i32`, which is invalid
- LL | |     };
-    | |______-
+ LL |         intrinsics::assert_zero_valid::<&'static i32>();
34    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = help: add `#![feature(const_assert_type2)]` to the crate attributes to enable
---
To only update this specific test, also pass `--test-args consts/assert-type-intrinsics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/assert-type-intrinsics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/assert-type-intrinsics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/assert-type-intrinsics/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `assert_uninit_valid` is not yet stable as a const fn
   |
   |
LL |         intrinsics::assert_uninit_valid::<bool>();
   |
   = help: add `#![feature(const_assert_type2)]` to the crate attributes to enable


error: `assert_zero_valid` is not yet stable as a const fn
   |
   |
LL |         intrinsics::assert_zero_valid::<&'static i32>();
   |
   = help: add `#![feature(const_assert_type2)]` to the crate attributes to enable

error: aborting due to 2 previous errors
---
test result: FAILED. 12306 passed; 1 failed; 115 ignored; 0 measured; 0 filtered out; finished in 135.90s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:16
