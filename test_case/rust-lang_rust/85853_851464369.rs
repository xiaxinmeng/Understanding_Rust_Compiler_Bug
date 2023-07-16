plain

---- [ui] ui/lint/lint-ctypes-fn.rs stdout ----
diff of stderr:

21    = help: consider using `*const u8` and a length instead
22    = note: string slices have no C equivalent
23 
+ error: `extern` fn uses type `Box<u32>`, which is not FFI-safe
+    |
+    |
+ LL | pub extern "C" fn box_type(p: Box<u32>) { }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |                               ^^^^^^^^ not FFI-safe
+    |
+    = note: box cannot be represented as a single pointer
+ 
+ error: `extern` fn uses type `Box<u32>`, which is not FFI-safe
+    |
+    |
+ LL | pub extern "C" fn opt_box_type(p: Option<Box<u32>>) { }
+    |                                   ^^^^^^^^^^^^^^^^ not FFI-safe
+    |
+    = note: box cannot be represented as a single pointer
+ 
24 error: `extern` fn uses type `char`, which is not FFI-safe
26    |


117    = help: consider using an `extern fn(...) -> ...` function pointer instead
118    = note: this function pointer has Rust-specific calling convention
119 
+ error: `extern` fn uses type `Box<u32>`, which is not FFI-safe
+    |
+    |
+ LL | pub extern "C" fn fn_contained(p: RustBadRet) { }
+    |                                   ^^^^^^^^^^ not FFI-safe
+    |
+    = note: box cannot be represented as a single pointer
+ 
120 error: `extern` fn uses type `i128`, which is not FFI-safe
122    |


134    = help: consider using `*const u8` and a length instead
135    = note: string slices have no C equivalent
136 
+ error: `extern` fn uses type `Box<u32>`, which is not FFI-safe
+    |
+    |
+ LL | pub extern "C" fn transparent_fn(p: TransparentBadFn) { }
+    |                                     ^^^^^^^^^^^^^^^^ not FFI-safe
+    |
+    = note: box cannot be represented as a single pointer
+ 
137 error: `extern` fn uses type `PhantomData<bool>`, which is not FFI-safe
139    |


160    = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
161    = note: this struct has unspecified layout
- error: aborting due to 17 previous errors
+ error: aborting due to 21 previous errors
164 
165 
---
To only update this specific test, also pass `--test-args lint/lint-ctypes-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `extern` fn uses type `[u32]`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn slice_type(p: &[u32]) { }
   |                                 ^^^^^^ not FFI-safe
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-ctypes-fn.rs:4:9
   |
   |
LL | #![deny(improper_ctypes_definitions)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: consider using a raw pointer instead
   = note: slices have no C equivalent

error: `extern` fn uses type `str`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn str_type(p: &str) { }
   |                               ^^^^ not FFI-safe
   |
   = help: consider using `*const u8` and a length instead
   = note: string slices have no C equivalent

error: `extern` fn uses type `Box<u32>`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn box_type(p: Box<u32>) { }
   |                               ^^^^^^^^ not FFI-safe
   |
   = note: box cannot be represented as a single pointer

error: `extern` fn uses type `Box<u32>`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn opt_box_type(p: Option<Box<u32>>) { }
   |                                   ^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = note: box cannot be represented as a single pointer

error: `extern` fn uses type `char`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn char_type(p: char) { }
   |                                ^^^^ not FFI-safe
   |
   = help: consider using `u32` or `libc::wchar_t` instead
   = note: the `char` type has no C equivalent

error: `extern` fn uses type `i128`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn i128_type(p: i128) { }
   |                                ^^^^ not FFI-safe
   |
   = note: 128-bit integers don't currently have a known stable ABI

error: `extern` fn uses type `u128`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn u128_type(p: u128) { }
   |                                ^^^^ not FFI-safe
   |
   = note: 128-bit integers don't currently have a known stable ABI

error: `extern` fn uses type `(i32, i32)`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn tuple_type(p: (i32, i32)) { }
   |                                 ^^^^^^^^^^ not FFI-safe
   = help: consider using a struct instead
   = help: consider using a struct instead
   = note: tuples have unspecified layout

error: `extern` fn uses type `(i32, i32)`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn tuple_type2(p: I32Pair) { }
   |                                  ^^^^^^^ not FFI-safe
   = help: consider using a struct instead
   = help: consider using a struct instead
   = note: tuples have unspecified layout

error: `extern` fn uses type `ZeroSize`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn zero_size(p: ZeroSize) { }
   |                                ^^^^^^^^ not FFI-safe
   = help: consider adding a member to this struct
   = note: this struct has no fields
note: the type is defined here
  --> /checkout/src/test/ui/lint/lint-ctypes-fn.rs:26:1
  --> /checkout/src/test/ui/lint/lint-ctypes-fn.rs:26:1
   |
LL | pub struct ZeroSize;


error: `extern` fn uses type `ZeroSizeWithPhantomData`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn zero_size_phantom(p: ZeroSizeWithPhantomData) { }
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   = note: composed only of `PhantomData`
note: the type is defined here
  --> /checkout/src/test/ui/lint/lint-ctypes-fn.rs:61:1
   |
   |
LL | pub struct ZeroSizeWithPhantomData(PhantomData<i32>);


error: `extern` fn uses type `PhantomData<bool>`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn zero_size_phantom_toplevel() -> PhantomData<bool> {
   |                                                   ^^^^^^^^^^^^^^^^^ not FFI-safe
   = note: composed only of `PhantomData`


error: `extern` fn uses type `fn()`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn fn_type(p: RustFn) { }
   |                              ^^^^^^ not FFI-safe
   |
   = help: consider using an `extern fn(...) -> ...` function pointer instead
   = note: this function pointer has Rust-specific calling convention

error: `extern` fn uses type `fn()`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn fn_type2(p: fn()) { }
   |                               ^^^^ not FFI-safe
   |
   = help: consider using an `extern fn(...) -> ...` function pointer instead
   = note: this function pointer has Rust-specific calling convention

error: `extern` fn uses type `Box<u32>`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn fn_contained(p: RustBadRet) { }
   |                                   ^^^^^^^^^^ not FFI-safe
   |
   = note: box cannot be represented as a single pointer

error: `extern` fn uses type `i128`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn transparent_i128(p: TransparentI128) { }
   |                                       ^^^^^^^^^^^^^^^ not FFI-safe
   |
   = note: 128-bit integers don't currently have a known stable ABI

error: `extern` fn uses type `str`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn transparent_str(p: TransparentStr) { }
   |                                      ^^^^^^^^^^^^^^ not FFI-safe
   |
   = help: consider using `*const u8` and a length instead
   = note: string slices have no C equivalent

error: `extern` fn uses type `Box<u32>`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn transparent_fn(p: TransparentBadFn) { }
   |                                     ^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = note: box cannot be represented as a single pointer

error: `extern` fn uses type `PhantomData<bool>`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn unused_generic2<T>() -> PhantomData<bool> {
   |                                           ^^^^^^^^^^^^^^^^^ not FFI-safe
   = note: composed only of `PhantomData`


error: `extern` fn uses type `Vec<T>`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn used_generic4<T>(x: Vec<T>) { }
   |                                       ^^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout

error: `extern` fn uses type `Vec<T>`, which is not FFI-safe
   |
   |
LL | pub extern "C" fn used_generic5<T>() -> Vec<T> {
   |                                         ^^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
error: aborting due to 21 previous errors


------------------------------------------
---
test result: FAILED. 11841 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 118.37s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:07
