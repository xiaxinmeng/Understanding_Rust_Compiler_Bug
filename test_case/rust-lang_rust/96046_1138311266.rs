plain

---- [ui] src/test/ui/asm/aarch64/bad-reg.rs stdout ----
diff of stderr:

Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
142 LL |         asm!("", in("v0") foo, out("q0") bar);
144 
- error: aborting due to 18 previous errors
+ error: type `i32` cannot be used with this register class
+   --> $DIR/bad-reg.rs:37:27
+   --> $DIR/bad-reg.rs:37:27
+    |
+ LL |         asm!("", in("p0") foo);
+    |
+    |
+    = note: register class `preg` supports these types: 
+ error: type `i32` cannot be used with this register class
+   --> $DIR/bad-reg.rs:40:29
+    |
+    |
+ LL |         asm!("{}", in(preg) foo);
+    |
+    |
+    = note: register class `preg` supports these types: 
+ error: aborting due to 20 previous errors
146 
147 

---
To only update this specific test, also pass `--test-args asm/aarch64/bad-reg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/bad-reg.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-reg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+neon" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-reg/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid register class `foo`: unknown register class
   |
   |
LL |         asm!("{}", in(foo) foo);


error: invalid register `foo`: unknown register
   |
   |
LL |         asm!("", in("foo") foo);

error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:18:15
   |
   |
LL |         asm!("{:z}", in(reg) foo);
   |               ^^^^   ----------- argument
   |               template modifier
   |
   |
   = note: the `reg` register class supports the following template modifiers: `w`, `x`
error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:20:15
   |
   |
LL |         asm!("{:r}", in(vreg) foo);
   |               ^^^^   ------------ argument
   |               template modifier
   |
   |
   = note: the `vreg` register class supports the following template modifiers: `b`, `h`, `s`, `d`, `q`, `v`
error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:22:15
   |
   |
LL |         asm!("{:r}", in(vreg_low16) foo);
   |               ^^^^   ------------------ argument
   |               template modifier
   |
   |
   = note: the `vreg_low16` register class supports the following template modifiers: `b`, `h`, `s`, `d`, `q`, `v`

error: asm template modifiers are not allowed for `const` arguments
   |
   |
LL |         asm!("{:a}", const 0);
   |               ^^^^   ------- argument
   |               template modifier


error: asm template modifiers are not allowed for `sym` arguments
   |
   |
LL |         asm!("{:a}", sym main);
   |               ^^^^   -------- argument
   |               template modifier


error: invalid register `x29`: the frame pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("x29") foo);


error: invalid register `sp`: the stack pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("sp") foo);


error: invalid register `xzr`: the zero register cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("xzr") foo);


error: invalid register `x19`: x19 is used internally by LLVM and cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("x19") foo);


error: register class `preg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("", in("p0") foo);


error: register class `preg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", in(preg) foo);


error: register class `preg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", out(preg) _);


error: register `x0` conflicts with register `x0`
   |
   |
LL |         asm!("", in("x0") foo, in("w0") bar);
   |                  ------------  ^^^^^^^^^^^^ register `x0`
   |                  register `x0`


error: register `x0` conflicts with register `x0`
   |
   |
LL |         asm!("", in("x0") foo, out("x0") bar);
   |                  ------------  ^^^^^^^^^^^^^ register `x0`
   |                  register `x0`
   |
   |
help: use `lateout` instead of `out` to avoid conflict
   |
   |
LL |         asm!("", in("x0") foo, out("x0") bar);


error: register `v0` conflicts with register `v0`
   |
   |
LL |         asm!("", in("v0") foo, in("q0") bar);
   |                  ------------  ^^^^^^^^^^^^ register `v0`
   |                  register `v0`


error: register `v0` conflicts with register `v0`
   |
   |
LL |         asm!("", in("v0") foo, out("q0") bar);
   |                  ------------  ^^^^^^^^^^^^^ register `v0`
   |                  register `v0`
   |
   |
help: use `lateout` instead of `out` to avoid conflict
   |
   |
LL |         asm!("", in("v0") foo, out("q0") bar);

error: type `i32` cannot be used with this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:37:27
   |
   |
LL |         asm!("", in("p0") foo);
   |
   |
   = note: register class `preg` supports these types: 
error: type `i32` cannot be used with this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:40:29
   |
   |
LL |         asm!("{}", in(preg) foo);
   |
   |
   = note: register class `preg` supports these types: 
error: aborting due to 20 previous errors
------------------------------------------



---- [ui] src/test/ui/asm/aarch64/type-check-2.rs stdout ----
diff of stderr:

+ error: invalid `sym` operand
+    |
+    |
+ LL | global_asm!("{}", sym C);
+    |                   ^^^^^ is an `i32`
+    |
+    = help: `sym` operands must refer to either a function or a static
+ 
+ error: invalid `sym` operand
+    |
+    |
+ LL |         asm!("{}", sym C);
+    |                    ^^^^^ is an `i32`
+    |
+    = help: `sym` operands must refer to either a function or a static
1 error: arguments for inline assembly must be copyable
2   --> $DIR/type-check-2.rs:46:31
3    |


55    |
56    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
57 
- error: invalid `sym` operand
-   --> $DIR/type-check-2.rs:41:20
-    |
- LL |         asm!("{}", sym C);
-    |                    ^^^^^ is an `i32`
-    |
-    = help: `sym` operands must refer to either a function or a static
+ error: aborting due to 9 previous errors
65 
- error: invalid `sym` operand
-   --> $DIR/type-check-2.rs:92:19
-    |
- LL | global_asm!("{}", sym C);
-    |                   ^^^^^ is an `i32`
-    |
-    = help: `sym` operands must refer to either a function or a static
- 
- error[E0381]: use of possibly-uninitialized variable: `x`
-   --> $DIR/type-check-2.rs:19:28
-    |
- LL |         asm!("{}", in(reg) x);
-    |                            ^ use of possibly-uninitialized `x`
- 
- error[E0381]: use of possibly-uninitialized variable: `y`
-   --> $DIR/type-check-2.rs:22:9
-    |
- LL |         asm!("{}", inout(reg) y);
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly-uninitialized `y`
- 
- error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
-   --> $DIR/type-check-2.rs:30:29
-    |
- LL |         let v: Vec<u64> = vec![0, 1, 2];
-    |             - help: consider changing this to be mutable: `mut v`
- LL |         asm!("{}", in(reg) v[0]);
- LL |         asm!("{}", out(reg) v[0]);
-    |                             ^ cannot borrow as mutable
- 
- error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
-   --> $DIR/type-check-2.rs:32:31
-    |
- LL |         let v: Vec<u64> = vec![0, 1, 2];
-    |             - help: consider changing this to be mutable: `mut v`
- ...
- LL |         asm!("{}", inout(reg) v[0]);
-    |                               ^ cannot borrow as mutable
- error: aborting due to 13 previous errors
- 
- Some errors have detailed explanations: E0381, E0596.
- For more information about an error, try `rustc --explain E0381`.
- For more information about an error, try `rustc --explain E0381`.
108 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2/type-check-2.stderr
To only update this specific test, also pass `--test-args asm/aarch64/type-check-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/type-check-2.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid `sym` operand
   |
   |
LL | global_asm!("{}", sym C);
   |                   ^^^^^ is an `i32`
   |
   = help: `sym` operands must refer to either a function or a static

error: invalid `sym` operand
   |
   |
LL |         asm!("{}", sym C);
   |                    ^^^^^ is an `i32`
   |
   = help: `sym` operands must refer to either a function or a static
error: arguments for inline assembly must be copyable
  --> /checkout/src/test/ui/asm/aarch64/type-check-2.rs:46:31
   |
   |
LL |         asm!("{:v}", in(vreg) SimdNonCopy(0.0, 0.0, 0.0, 0.0));
   |
   |
   = note: `SimdNonCopy` does not implement the Copy trait

error: cannot use value of type `[closure@/checkout/src/test/ui/asm/aarch64/type-check-2.rs:58:28: 58:38]` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) |x: i32| x);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `Vec<i32>` for inline assembly
  --> /checkout/src/test/ui/asm/aarch64/type-check-2.rs:60:28
   |
   |
LL |         asm!("{}", in(reg) vec![0]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot use value of type `(i32, i32, i32)` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) (1, 2, 3));
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `[i32; 3]` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) [1, 2, 3]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `fn() {main}` for inline assembly
   |
   |
LL |         asm!("{}", inout(reg) f);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `&mut i32` for inline assembly
  --> /checkout/src/test/ui/asm/aarch64/type-check-2.rs:75:31
   |
   |
LL |         asm!("{}", inout(reg) r);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: aborting due to 9 previous errors
------------------------------------------



---- [ui] src/test/ui/asm/aarch64/type-check-3.rs stdout ----
diff of stderr:

143    |
144    = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size
- error[E0013]: constants cannot refer to statics
-   --> $DIR/type-check-3.rs:108:25
-    |
-    |
- LL | global_asm!("{}", const S);
-    |
-    |
-    = help: consider extracting the value of the `static` to a `const`, and referring to that
+ error: aborting due to 6 previous errors; 10 warnings emitted
- error[E0013]: constants cannot refer to statics
-   --> $DIR/type-check-3.rs:111:35
-    |
-    |
- LL | global_asm!("{}", const const_foo(S));
-    |
-    |
-    = help: consider extracting the value of the `static` to a `const`, and referring to that
- error[E0013]: constants cannot refer to statics
-   --> $DIR/type-check-3.rs:114:35
-    |
-    |
- LL | global_asm!("{}", const const_bar(S));
-    |
-    |
-    = help: consider extracting the value of the `static` to a `const`, and referring to that
- error: aborting due to 9 previous errors; 10 warnings emitted
- 
- For more information about this error, try `rustc --explain E0013`.
173 
173 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-3/type-check-3.stderr
To only update this specific test, also pass `--test-args asm/aarch64/type-check-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/type-check-3.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+neon" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-3/auxiliary"
stdout: none
--- stderr -------------------------------
warning: formatting may not be suitable for sub-register argument
   |
   |
LL |         asm!("{}", in(reg) 0u8);
   |               ^^           --- for this argument
   |
   = note: `#[warn(asm_sub_register)]` on by default
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:50:15
   |
   |
LL |         asm!("{}", in(reg) 0u16);
   |               ^^           ---- for this argument
   |
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:52:15
   |
   |
LL |         asm!("{}", in(reg) 0i32);
   |               ^^           ---- for this argument
   |
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:54:15
   |
   |
LL |         asm!("{}", in(reg) 0f32);
   |               ^^           ---- for this argument
   |
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:57:15
   |
   |
LL |         asm!("{}", in(vreg) 0i16);
   |               ^^            ---- for this argument
   |
   = help: use the `h` modifier to have the register formatted as `h0`
   = help: or use the `v` modifier to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:59:15
   |
   |
LL |         asm!("{}", in(vreg) 0f32);
   |               ^^            ---- for this argument
   |
   = help: use the `s` modifier to have the register formatted as `s0`
   = help: or use the `v` modifier to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:61:15
   |
   |
LL |         asm!("{}", in(vreg) 0f64);
   |               ^^            ---- for this argument
   |
   = help: use the `d` modifier to have the register formatted as `d0`
   = help: or use the `v` modifier to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:63:15
   |
   |
LL |         asm!("{}", in(vreg_low16) 0f64);
   |               ^^                  ---- for this argument
   |
   = help: use the `d` modifier to have the register formatted as `d0`
   = help: or use the `v` modifier to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:66:15
   |
   |
LL |         asm!("{0} {0}", in(reg) 0i16);
   |               ^^^ ^^^           ---- for this argument
   |
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:68:15
   |
   |
LL |         asm!("{0} {0:x}", in(reg) 0i16);
   |               ^^^                 ---- for this argument
   |
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
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

