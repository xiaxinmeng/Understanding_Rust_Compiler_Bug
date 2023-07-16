plain
test [ui] ui/feature-gates/feature-gate-asm.rs ... ignored
test [ui] ui/feature-gates/feature-gate-asm2.rs ... ignored
test [ui] ui/feature-gates/feature-gate-asm_const.rs ... ignored
test [ui] ui/feature-gates/feature-gate-arbitrary_self_types-raw-pointer.rs ... ok
test [ui] ui/feature-gates/feature-gate-asm_sym.rs ... ignored
test [ui] ui/feature-gates/feature-gate-asm_experimental_arch.rs ... ok
test [ui] ui/feature-gates/feature-gate-auto-traits.rs ... ok
test [ui] ui/feature-gates/feature-gate-associated_type_bounds.rs ... ok
test [ui] ui/feature-gates/feature-gate-box-expr.rs ... ok
test [ui] ui/feature-gates/feature-gate-box_patterns.rs ... ok
---
failures:

---- [ui] ui/asm/aarch64/sym.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/sym.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/sym/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/sym/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
   |
   |
LL |             asm!("bl {}", sym $func,
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
...
...
LL |     assert_eq!(call!(f1), 111);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm)]` to the crate attributes to enable
   = note: this error originates in the macro `call` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
   |
   |
LL |             asm!("bl {}", sym $func,
...
...
LL |     assert_eq!(call!(f2), 222);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm)]` to the crate attributes to enable
   = note: this error originates in the macro `call` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
   |
LL |             asm!(
   |             ^^^
...
...
LL |     assert_eq!(static_addr!(S1), &S1 as *const u32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm)]` to the crate attributes to enable
   = note: this error originates in the macro `static_addr` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
   |
LL |             asm!(
   |             ^^^
...
...
LL |     assert_eq!(static_tls_addr!(S2), &S2 as *const u32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm)]` to the crate attributes to enable
   = note: this error originates in the macro `static_tls_addr` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
   |
LL |             asm!(
   |             ^^^
...
...
LL |         assert_eq!(static_addr!(S1), &S1 as *const u32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm)]` to the crate attributes to enable
   = note: this error originates in the macro `static_addr` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
   |
LL |             asm!(
   |             ^^^
...
...
LL |         assert_eq!(static_tls_addr!(S2), &S2 as *const u32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm)]` to the crate attributes to enable
   = note: this error originates in the macro `static_tls_addr` (in Nightly builds, run with -Z macro-backtrace for more info)

error: expected one of `(`, `,`, `::`, or `=`, found `sym`
   |
   |
LL | #![feature(asm, thread_local, asm sym)]
   |                                   ^^^ expected one of `(`, `,`, `::`, or `=`

error[E0658]: `#[thread_local]` is an experimental feature, and does not currently handle destructors
   |
   |
LL | #[thread_local]
   |
   = note: see issue #29594 <https://github.com/rust-lang/rust/issues/29594> for more information
   = note: see issue #29594 <https://github.com/rust-lang/rust/issues/29594> for more information
   = help: add `#![feature(thread_local)]` to the crate attributes to enable

error[E0658]: sym operands for inline assembly are unstable
   |
   |
LL |             asm!("bl {}", sym $func,
...
...
LL |     assert_eq!(call!(f1), 111);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable
   = note: this error originates in the macro `call` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: sym operands for inline assembly are unstable
   |
   |
LL |             asm!("bl {}", sym $func,
...
...
LL |     assert_eq!(call!(f2), 222);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable
   = note: this error originates in the macro `call` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: sym operands for inline assembly are unstable
   |
   |
LL |                 sym = sym $s);
...
...
LL |     assert_eq!(static_addr!(S1), &S1 as *const u32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable
   = note: this error originates in the macro `static_addr` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: sym operands for inline assembly are unstable
   |
LL |                 sym = sym $s
   |                 ^^^^^^^^^^^^
...
...
LL |     assert_eq!(static_tls_addr!(S2), &S2 as *const u32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable
   = note: this error originates in the macro `static_tls_addr` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: sym operands for inline assembly are unstable
   |
   |
LL |                 sym = sym $s);
...
...
LL |         assert_eq!(static_addr!(S1), &S1 as *const u32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable
   = note: this error originates in the macro `static_addr` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: sym operands for inline assembly are unstable
   |
LL |                 sym = sym $s
   |                 ^^^^^^^^^^^^
...
...
LL |         assert_eq!(static_tls_addr!(S2), &S2 as *const u32);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_sym)]` to the crate attributes to enable
   = note: this error originates in the macro `static_tls_addr` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 14 previous errors

For more information about this error, try `rustc --explain E0658`.

---
1 warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:48:15
+   --> $DIR/type-check-3.rs:47:15
3    |
4 LL |         asm!("{}", in(reg) 0u8);
5    |               ^^           --- for this argument

9    = help: or use the `x` modifier to keep the default formatting of `x0`
11 warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:50:15
+   --> $DIR/type-check-3.rs:49:15
13    |
13    |
14 LL |         asm!("{}", in(reg) 0u16);
15    |               ^^           ---- for this argument

18    = help: or use the `x` modifier to keep the default formatting of `x0`
20 warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:52:15
+   --> $DIR/type-check-3.rs:51:15
22    |
22    |
23 LL |         asm!("{}", in(reg) 0i32);
24    |               ^^           ---- for this argument

27    = help: or use the `x` modifier to keep the default formatting of `x0`
29 warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:54:15
+   --> $DIR/type-check-3.rs:53:15
31    |
31    |
32 LL |         asm!("{}", in(reg) 0f32);
33    |               ^^           ---- for this argument

36    = help: or use the `x` modifier to keep the default formatting of `x0`
38 warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:57:15
+   --> $DIR/type-check-3.rs:56:15
40    |
40    |
41 LL |         asm!("{}", in(vreg) 0i16);
42    |               ^^            ---- for this argument

45    = help: or use the `v` modifier to keep the default formatting of `v0`
47 warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:59:15
+   --> $DIR/type-check-3.rs:58:15
49    |
49    |
50 LL |         asm!("{}", in(vreg) 0f32);
51    |               ^^            ---- for this argument

54    = help: or use the `v` modifier to keep the default formatting of `v0`
56 warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:61:15
+   --> $DIR/type-check-3.rs:60:15
58    |
58    |
59 LL |         asm!("{}", in(vreg) 0f64);
60    |               ^^            ---- for this argument

63    = help: or use the `v` modifier to keep the default formatting of `v0`
65 warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:63:15
+   --> $DIR/type-check-3.rs:62:15
67    |
67    |
68 LL |         asm!("{}", in(vreg_low16) 0f64);
69    |               ^^                  ---- for this argument

72    = help: or use the `v` modifier to keep the default formatting of `v0`
74 warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:66:15
+   --> $DIR/type-check-3.rs:65:15
76    |
76    |
77 LL |         asm!("{0} {0}", in(reg) 0i16);
78    |               ^^^ ^^^           ---- for this argument

81    = help: or use the `x` modifier to keep the default formatting of `x0`
83 warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-3.rs:68:15
+   --> $DIR/type-check-3.rs:67:15
85    |
85    |
86 LL |         asm!("{0} {0:x}", in(reg) 0i16);
87    |               ^^^                 ---- for this argument

90    = help: or use the `x` modifier to keep the default formatting of `x0`
92 error: type `i128` cannot be used with this register class
-   --> $DIR/type-check-3.rs:73:28
+   --> $DIR/type-check-3.rs:72:28
94    |
94    |
95 LL |         asm!("{}", in(reg) 0i128);


98    = note: register class `reg` supports these types: i8, i16, i32, i64, f32, f64
99 
100 error: type `float64x2_t` cannot be used with this register class
-   --> $DIR/type-check-3.rs:75:28
102    |
102    |
103 LL |         asm!("{}", in(reg) f64x2);


106    = note: register class `reg` supports these types: i8, i16, i32, i64, f32, f64
107 
108 error: type `Simd256bit` cannot be used with this register class
-   --> $DIR/type-check-3.rs:77:29
110    |
110    |
111 LL |         asm!("{}", in(vreg) f64x4);


114    = note: register class `vreg` supports these types: i8, i16, i32, i64, f32, f64, i8x8, i16x4, i32x2, i64x1, f32x2, f64x1, i8x16, i16x8, i32x4, i64x2, f32x4, f64x2
115 
116 error: incompatible types for asm inout argument
-   --> $DIR/type-check-3.rs:88:33
118    |
118    |
119 LL |         asm!("{:x}", inout(reg) 0u32 => val_f32);
120    |                                 ^^^^    ^^^^^^^ type `f32`

124    = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size
125 
126 error: incompatible types for asm inout argument
-   --> $DIR/type-check-3.rs:90:33
128    |
128    |
129 LL |         asm!("{:x}", inout(reg) 0u32 => val_ptr);
130    |                                 ^^^^    ^^^^^^^ type `*mut u8`

134    = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size
135 
136 error: incompatible types for asm inout argument
-   --> $DIR/type-check-3.rs:92:33
138    |
138    |
139 LL |         asm!("{:x}", inout(reg) main => val_u32);
140    |                                 ^^^^    ^^^^^^^ type `u32`

144    = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size
146 error[E0013]: constants cannot refer to statics
-   --> $DIR/type-check-3.rs:108:25
+   --> $DIR/type-check-3.rs:107:25
148    |
148    |
149 LL | global_asm!("{}", const S);


152    = help: consider extracting the value of the `static` to a `const`, and referring to that
154 error[E0013]: constants cannot refer to statics
-   --> $DIR/type-check-3.rs:111:35
+   --> $DIR/type-check-3.rs:110:35
156    |
156    |
157 LL | global_asm!("{}", const const_foo(S));


160    = help: consider extracting the value of the `static` to a `const`, and referring to that
162 error[E0013]: constants cannot refer to statics
-   --> $DIR/type-check-3.rs:114:35
+   --> $DIR/type-check-3.rs:113:35
164    |
164    |
165 LL | global_asm!("{}", const const_bar(S));


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-3/type-check-3.stderr
To only update this specific test, also pass `--test-args asm/aarch64/type-check-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/type-check-3.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+neon" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:47:15
   |
LL |         asm!("{}", in(reg) 0u8);
   |               ^^           --- for this argument
   |
   = note: `#[warn(asm_sub_register)]` on by default
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:49:15
   |
   |
LL |         asm!("{}", in(reg) 0u16);
   |               ^^           ---- for this argument
   |
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:51:15
   |
   |
LL |         asm!("{}", in(reg) 0i32);
   |               ^^           ---- for this argument
   |
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:53:15
   |
   |
LL |         asm!("{}", in(reg) 0f32);
   |               ^^           ---- for this argument
   |
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:56:15
   |
   |
LL |         asm!("{}", in(vreg) 0i16);
   |               ^^            ---- for this argument
   |
   = help: use the `h` modifier to have the register formatted as `h0`
   = help: or use the `v` modifier to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:58:15
   |
   |
LL |         asm!("{}", in(vreg) 0f32);
   |               ^^            ---- for this argument
   |
   = help: use the `s` modifier to have the register formatted as `s0`
   = help: or use the `v` modifier to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:60:15
   |
   |
LL |         asm!("{}", in(vreg) 0f64);
   |               ^^            ---- for this argument
   |
   = help: use the `d` modifier to have the register formatted as `d0`
   = help: or use the `v` modifier to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:62:15
   |
   |
LL |         asm!("{}", in(vreg_low16) 0f64);
   |               ^^                  ---- for this argument
   |
   = help: use the `d` modifier to have the register formatted as `d0`
   = help: or use the `v` modifier to keep the default formatting of `v0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:65:15
   |
   |
LL |         asm!("{0} {0}", in(reg) 0i16);
   |               ^^^ ^^^           ---- for this argument
   |
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:67:15
   |
   |
LL |         asm!("{0} {0:x}", in(reg) 0i16);
   |               ^^^                 ---- for this argument
   |
   = help: use the `w` modifier to have the register formatted as `w0`
   = help: or use the `x` modifier to keep the default formatting of `x0`
error: type `i128` cannot be used with this register class
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:72:28
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
   |                                 ^^^^    ^^^^^^^ type `u32`
   |                                 type `fn()`
   |
   |
   = note: asm inout arguments must have the same type, unless they are both pointers or integers of the same size
error[E0013]: constants cannot refer to statics
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:107:25
   |
   |
LL | global_asm!("{}", const S);
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error[E0013]: constants cannot refer to statics
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:110:35
   |
   |
LL | global_asm!("{}", const const_foo(S));
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error[E0013]: constants cannot refer to statics
  --> /checkout/src/test/ui/asm/aarch64/type-check-3.rs:113:35
   |
   |
LL | global_asm!("{}", const const_bar(S));
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error: aborting due to 9 previous errors; 10 warnings emitted

For more information about this error, try `rustc --explain E0013`.

---
test result: FAILED. 12226 passed; 2 failed; 147 ignored; 0 measured; 0 filtered out; finished in 149.48s



command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:20:23
