plain
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........ii................
failures:

---- [run-make] run-make-fulldeps/simd-ffi stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-linux-androideabi --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-linux-androideabi
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-unknown-linux-gnueabihf --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-unknown-linux-gnueabihf
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-unknown-linux-gnueabi --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-unknown-linux-gnueabi
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=aarch64-unknown-linux-gnu --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-aarch64-unknown-linux-gnu
--- stderr -------------------------------
--- stderr -------------------------------
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
   = note: `#[warn(improper_ctypes)]` on by default
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:11:1
   |
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
  --> simd.rs:11:1
   |
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
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:29
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                             ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:39
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                                       ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unknown feature specified for `-Ctarget-feature`: `sse2`
  = note: it is still passed through to the codegen backend
  = note: consider filing a feature request


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
   = note: `#[warn(improper_ctypes)]` on by default
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:11:1
   |
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
  --> simd.rs:11:1
   |
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
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:29
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                             ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:39
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                                       ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unknown feature specified for `-Ctarget-feature`: `sse2`
  = note: it is still passed through to the codegen backend
  = note: consider filing a feature request


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
   = note: `#[warn(improper_ctypes)]` on by default
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:11:1
   |
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
  --> simd.rs:11:1
   |
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
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:29
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                             ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:35:39
   |
35 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                                       ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unknown feature specified for `-Ctarget-feature`: `sse2`
  = note: it is still passed through to the codegen backend
  = note: consider filing a feature request


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
   = note: `#[warn(improper_ctypes)]` on by default
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:11:1
   |
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
  --> simd.rs:11:1
   |
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
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:39:29
   |
39 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                             ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `extern` block uses type `i32x4`, which is not FFI-safe
  --> simd.rs:39:39
   |
39 |     fn integer(a: i32x4, b: i32x4) -> i32x4;
   |                                       ^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
   = note: this struct has unspecified layout
  --> simd.rs:24:1
   |
24 | pub struct i32x4(i32, i32, i32, i32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unknown feature specified for `-Ctarget-feature`: `sse2`
  = note: it is still passed through to the codegen backend
  = note: consider filing a feature request


error: target features fp, neon must all be enabled or disabled together

'+sse2' is not a recognized feature for this target (ignoring feature)
'+sse2' is not a recognized feature for this target (ignoring feature)


make: *** [Makefile:47: aarch64-unknown-linux-gnu] Error 1



failures:
