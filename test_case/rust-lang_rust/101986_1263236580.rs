plain

---- [ui] src/test/ui/asm/aarch64/type-check-3.rs stdout ----
diff of stderr:

4 LL |         asm!("{}", in(reg) 0u8);
5    |               ^^           --- for this argument
6    |
-    = note: `#[warn(asm_sub_register)]` on by default
8    = help: use `{0:w}` to have the register formatted as `w0`
9    = help: or use `{0:x}` to keep the default formatting of `x0`
+    = note: `#[warn(asm_sub_register)]` on by default
11 warning: formatting may not be suitable for sub-register argument
12   --> $DIR/type-check-3.rs:50:15



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-3/type-check-3.stderr
To only update this specific test, also pass `--test-args asm/aarch64/type-check-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/type-check-3.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+neon" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-3/auxiliary"
stdout: none
--- stderr -------------------------------
warning: formatting may not be suitable for sub-register argument
   |
   |
LL |         asm!("{}", in(reg) 0u8);
   |               ^^           --- for this argument
   |
   = help: use `{0:w}` to have the register formatted as `w0`
   = help: or use `{0:x}` to keep the default formatting of `x0`
   = note: `#[warn(asm_sub_register)]` on by default
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:50:15
   |
   |
LL |         asm!("{}", in(reg) 0u16);
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
   |               ^^           ---- for this argument
   |
   = help: use `{0:w}` to have the register formatted as `w0`
   = help: or use `{0:x}` to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:52:15
   |
   |
LL |         asm!("{}", in(reg) 0i32);
   |               ^^           ---- for this argument
   |
   = help: use `{0:w}` to have the register formatted as `w0`
   = help: or use `{0:x}` to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:54:15
   |
   |
LL |         asm!("{}", in(reg) 0f32);
   |               ^^           ---- for this argument
   |
   = help: use `{0:w}` to have the register formatted as `w0`
   = help: or use `{0:x}` to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:57:15
   |
   |
LL |         asm!("{}", in(vreg) 0i16);
   |               ^^            ---- for this argument
   |
   = help: use `{0:h}` to have the register formatted as `h0`
   = help: or use `{0:v}` to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:59:15
   |
   |
LL |         asm!("{}", in(vreg) 0f32);
   |               ^^            ---- for this argument
   |
   = help: use `{0:s}` to have the register formatted as `s0`
   = help: or use `{0:v}` to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:61:15
   |
   |
LL |         asm!("{}", in(vreg) 0f64);
   |               ^^            ---- for this argument
   |
   = help: use `{0:d}` to have the register formatted as `d0`
   = help: or use `{0:v}` to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:63:15
   |
   |
LL |         asm!("{}", in(vreg_low16) 0f64);
   |               ^^                  ---- for this argument
   |
   = help: use `{0:d}` to have the register formatted as `d0`
   = help: or use `{0:v}` to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:66:15
   |
   |
LL |         asm!("{0} {0}", in(reg) 0i16);
   |               ^^^ ^^^           ---- for this argument
   |
   = help: use `{0:w}` to have the register formatted as `w0`
   = help: or use `{0:x}` to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:68:15
   |
   |
LL |         asm!("{0} {0:x}", in(reg) 0i16);
   |               ^^^                 ---- for this argument
   |
   = help: use `{0:w}` to have the register formatted as `w0`
   = help: or use `{0:x}` to keep the default formatting of `x0`
error: type `i128` cannot be used with this register class
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:73:28
   |
   |
LL |         asm!("{}", in(reg) 0i128);
   |
   |
   = note: register class `reg` supports these types: i8, i16, i32, i64, f32, f64

error: type `float64x2_t` cannot be used with this register class
   |
   |
LL |         asm!("{}", in(reg) f64x2);
   |
   |
   = note: register class `reg` supports these types: i8, i16, i32, i64, f32, f64

error: type `Simd256bit` cannot be used with this register class
   |
   |
LL |         asm!("{}", in(vreg) f64x4);
   |
   |
   = note: register class `vreg` supports these types: i8, i16, i32, i64, f32, f64, i8x8, i16x4, i32x2, i64x1, f32x2, f64x1, i8x16, i16x8, i32x4, i64x2, f32x4, f64x2

error: incompatible types for asm inout argument
   |
   |
LL |         asm!("{:x}", inout(reg) 0u32 => val_f32);
   |                                 ^^^^    ^^^^^^^ type `f32`
   |                                 type `u32`
   |
   |
   = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size

error: incompatible types for asm inout argument
   |
   |
LL |         asm!("{:x}", inout(reg) 0u32 => val_ptr);
   |                                 ^^^^    ^^^^^^^ type `*mut u8`
   |                                 type `u32`
   |
   |
   = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size

error: incompatible types for asm inout argument
   |
   |
LL |         asm!("{:x}", inout(reg) main => val_u32);
   |                                 |
   |                                 type `fn()`
   |
   |
   = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size
error: aborting due to 6 previous errors; 10 warnings emitted
------------------------------------------


