plain
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 223 tests
..........iii...ii.................................................................................. 100/223
..............i..................iiiiii......i.................iii...........ii..F.................. 200/223
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [run-make] run-make-fulldeps/simd-ffi stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-linux-androideabi --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-linux-androideabi
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-unknown-linux-gnueabihf --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-unknown-linux-gnueabihf
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-unknown-linux-gnueabi --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-unknown-linux-gnueabi
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=aarch64-unknown-linux-gnu --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-aarch64-unknown-linux-gnu
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=mips-unknown-linux-gnu --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-mips-unknown-linux-gnu
Makefile:47: recipe for target 'mips-unknown-linux-gnu' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
'+sse2' is not a recognized feature for this target (ignoring feature)
warning: type `f32x4` should have an upper camel case name
  --> simd.rs:11:12
   |
11 | pub struct f32x4(f32, f32, f32, f32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `F32x4`
   = note: `#[warn(non_camel_case_types)]` on by default


warning: type `i32x4` should have an upper camel case name
  --> simd.rs:24:12
24 | pub struct i32x4(i32, i32, i32, i32);
24 | pub struct i32x4(i32, i32, i32, i32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `I32x4`

warning: `extern` block uses type `f32x4`, which is not FFI-safe
  --> simd.rs:15:17
   |
15 |     fn vsqrt(x: f32x4) -> f32x4;
   |                 ^^^^^ not FFI-safe
   |
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:11:1
   |
11 | pub struct f32x4(f32, f32, f32, f32);


warning: `extern` block uses type `f32x4`, which is not FFI-safe
  --> simd.rs:15:27
   |
15 |     fn vsqrt(x: f32x4) -> f32x4;
   |                           ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:11:1
   |
11 | pub struct f32x4(f32, f32, f32, f32);


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:19
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                   ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:29
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                             ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:39
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                                       ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)


'+sse2' is not a recognized feature for this target (ignoring feature)
warning: type `f32x4` should have an upper camel case name
  --> simd.rs:11:12
   |
11 | pub struct f32x4(f32, f32, f32, f32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `F32x4`
   = note: `#[warn(non_camel_case_types)]` on by default


warning: type `i32x4` should have an upper camel case name
  --> simd.rs:24:12
24 | pub struct i32x4(i32, i32, i32, i32);
24 | pub struct i32x4(i32, i32, i32, i32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `I32x4`

warning: `extern` block uses type `f32x4`, which is not FFI-safe
  --> simd.rs:15:17
   |
15 |     fn vsqrt(x: f32x4) -> f32x4;
   |                 ^^^^^ not FFI-safe
   |
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:11:1
   |
11 | pub struct f32x4(f32, f32, f32, f32);


warning: `extern` block uses type `f32x4`, which is not FFI-safe
  --> simd.rs:15:27
   |
15 |     fn vsqrt(x: f32x4) -> f32x4;
   |                           ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:11:1
   |
11 | pub struct f32x4(f32, f32, f32, f32);


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:19
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                   ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:29
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                             ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:39
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                                       ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)


'+sse2' is not a recognized feature for this target (ignoring feature)
warning: type `f32x4` should have an upper camel case name
  --> simd.rs:11:12
   |
11 | pub struct f32x4(f32, f32, f32, f32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `F32x4`
   = note: `#[warn(non_camel_case_types)]` on by default


warning: type `i32x4` should have an upper camel case name
  --> simd.rs:24:12
24 | pub struct i32x4(i32, i32, i32, i32);
24 | pub struct i32x4(i32, i32, i32, i32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `I32x4`

warning: `extern` block uses type `f32x4`, which is not FFI-safe
  --> simd.rs:15:17
   |
15 |     fn vsqrt(x: f32x4) -> f32x4;
   |                 ^^^^^ not FFI-safe
   |
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:11:1
   |
11 | pub struct f32x4(f32, f32, f32, f32);


warning: `extern` block uses type `f32x4`, which is not FFI-safe
  --> simd.rs:15:27
   |
15 |     fn vsqrt(x: f32x4) -> f32x4;
   |                           ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:11:1
   |
11 | pub struct f32x4(f32, f32, f32, f32);


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:19
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                   ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:29
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                             ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:39
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                                       ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)


'+sse2' is not a recognized feature for this target (ignoring feature)
warning: type `f32x4` should have an upper camel case name
  --> simd.rs:11:12
   |
11 | pub struct f32x4(f32, f32, f32, f32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `F32x4`
   = note: `#[warn(non_camel_case_types)]` on by default


warning: type `i32x4` should have an upper camel case name
  --> simd.rs:24:12
24 | pub struct i32x4(i32, i32, i32, i32);
24 | pub struct i32x4(i32, i32, i32, i32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `I32x4`

warning: `extern` block uses type `f32x4`, which is not FFI-safe
  --> simd.rs:15:17
   |
15 |     fn vsqrt(x: f32x4) -> f32x4;
   |                 ^^^^^ not FFI-safe
   |
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:11:1
   |
11 | pub struct f32x4(f32, f32, f32, f32);


warning: `extern` block uses type `f32x4`, which is not FFI-safe
  --> simd.rs:15:27
   |
15 |     fn vsqrt(x: f32x4) -> f32x4;
   |                           ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:11:1
   |
11 | pub struct f32x4(f32, f32, f32, f32);


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:39:19
   |
39 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                   ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:39:29
   |
39 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                             ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:39:39
   |
39 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                                       ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
note: the type is defined here
  --> simd.rs:24:1
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)


'+neon' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+neon' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+neon' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+neon' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+neon' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+neon' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
'+neon' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)
warning: type `f32x4` should have an upper camel case name
  --> simd.rs:11:12
   |
11 | pub struct f32x4(f32, f32, f32, f32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `F32x4`
   = note: `#[warn(non_camel_case_types)]` on by default


warning: type `i32x4` should have an upper camel case name
  --> simd.rs:24:12
24 | pub struct i32x4(i32, i32, i32, i32);
24 | pub struct i32x4(i32, i32, i32, i32);
   |            ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `I32x4`

error: use of SIMD type `i32x4` in FFI requires `#[target_feature(enable = "msa")]`
  --> simd.rs:50:19
   |
50 |     fn integer(a: i32x4, b: i32x4) -> i32x4;


error: use of SIMD type `i32x4` in FFI requires `#[target_feature(enable = "msa")]`
  --> simd.rs:50:29
   |
50 |     fn integer(a: i32x4, b: i32x4) -> i32x4;


error: use of SIMD type `i32x4` in FFI requires `#[target_feature(enable = "msa")]`
  --> simd.rs:50:39
   |
50 |     fn integer(a: i32x4, b: i32x4) -> i32x4;

error: aborting due to 3 previous errors; 2 warnings emitted


make: *** [mips-unknown-linux-gnu] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/simd-ffi

test result: FAILED. 202 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out; finished in 19.64s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:24:37
