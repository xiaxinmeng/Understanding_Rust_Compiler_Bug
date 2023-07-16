plain

---- [ui] ui/consts/const-eval/ub-enum.rs stdout ----
diff of 64bit.stderr:

- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:24:1
+ error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
+   --> $DIR/ub-enum.rs:60:54
3    |
- LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered 0x0000000000000001, but expected a valid enum tag
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-            }
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:27:1
-   --> $DIR/ub-enum.rs:27:1
-    |
- LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc9, but expected initialized plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-            }
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:30:1
-   --> $DIR/ub-enum.rs:30:1
-    |
- LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0.<enum-tag>: encountered pointer to alloc13, but expected initialized plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-            }
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:42:1
-   --> $DIR/ub-enum.rs:42:1
-    |
- LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered 0x0000000000000000, but expected a valid enum tag
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-            }
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:44:1
-   --> $DIR/ub-enum.rs:44:1
-    |
- LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc19, but expected initialized plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-            }
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:47:1
-   --> $DIR/ub-enum.rs:47:1
-    |
- LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0.<enum-tag>: encountered pointer to alloc23, but expected initialized plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-            }
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:56:1
-   --> $DIR/ub-enum.rs:56:1
-    |
- LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-                __ __ __ __ __ __ __ __                         │ ░░░░░░░░
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:60:1
-    |
-    |
81 LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered pointer to alloc29, but expected initialized plain (non-pointer) bytes
83    |
83    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 8) {
-            }
-            }
+    = note: source type: `&i32` (64 bits)
+    = note: target type: `Option<Enum2>` (128 bits)
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:77:1
-    |
-    |
- LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(B)>.0: encountered a value of the never type `!`
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 1, align: 1) {
-            }
+ error: aborting due to previous error
99 
- error[E0080]: it is undefined behavior to use this value
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:79:1
-    |
- LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 1, align: 1) {
-            }
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:87:1
-   --> $DIR/ub-enum.rs:87:1
-    |
- LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 4) {
-                78 00 00 00 ff ff ff ff                         │ x.......
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:92:1
-    |
-    |
- LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(Ok)>.0.1: encountered a value of uninhabited type Never
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 4) {
-            }
- 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:94:1
-   --> $DIR/ub-enum.rs:94:1
-    |
- LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(Ok)>.0.1: encountered a value of the never type `!`
-    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 4) {
-            }
- 
- error: aborting due to 13 previous errors
- 
- 
- For more information about this error, try `rustc --explain E0080`.
+ For more information about this error, try `rustc --explain E0512`.
147 


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   |
   |
   = note: source type: `&i32` (64 bits)
   = note: target type: `Option<Enum2>` (128 bits)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0512`.

---
test result: FAILED. 12017 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 106.72s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:34
