plain

---- [ui] src/test/ui/feature-gates/feature-gate-abi.rs stdout ----
diff of stderr:

238 LL |     extern "platform-intrinsic" fn m2();
240 
240 
+ error[E0570]: `"efiapi"` is not a supported ABI for the current target
+    |
+    |
+ LL | extern "efiapi" {}
+ 
+ 
241 error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
243    |


250 LL | extern "platform-intrinsic" fn f2() {}
252 
252 
+ error[E0570]: `"efiapi"` is not a supported ABI for the current target
+    |
+    |
+ LL | extern "efiapi" fn f10() {}
+ 
+ 
+ error[E0570]: `"efiapi"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "efiapi" fn dm10() {}
+ 
+ 
253 error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
255    |


262 LL |     extern "platform-intrinsic" fn m2() {}
264 
264 
+ error[E0570]: `"efiapi"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "efiapi" fn m10() {}
+ 
+ 
265 error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
267    |


274 LL |     extern "platform-intrinsic" fn im2() {}
276 
- error: aborting due to 34 previous errors
- error: aborting due to 34 previous errors
+ error[E0570]: `"efiapi"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "efiapi" fn im10() {}
278 
- For more information about this error, try `rustc --explain E0658`.
+ error: aborting due to 39 previous errors
+ 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/auxiliary" "--crate-type=rlib"
stdout: none
--- stderr -------------------------------
error[E0658]: intrinsics are subject to change
   |
   |
LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:15:8
   |
LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:17:8
   |
   |
LL | extern "rust-call" fn f4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown

error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL | extern "efiapi" fn f10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:22:12
   |
   |
LL |     extern "rust-intrinsic" fn m1(); //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:24:12
   |
LL |     extern "platform-intrinsic" fn m2(); //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:26:12
   |
   |
LL |     extern "rust-call" fn m4(_: ()); //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn m10(); //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:29:12
   |
   |
LL |     extern "rust-call" fn dm4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn dm10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:37:12
   |
   |
LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:39:12
   |
LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:41:12
   |
   |
LL |     extern "rust-call" fn m4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn m10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:47:12
   |
   |
LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:49:12
   |
LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:51:12
   |
   |
LL |     extern "rust-call" fn im4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn im10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:56:18
   |
   |
LL | type A1 = extern "rust-intrinsic" fn(); //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:57:18
   |
LL | type A2 = extern "platform-intrinsic" fn(); //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:58:18
   |
   |
LL | type A4 = extern "rust-call" fn(_: ()); //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL | type A10 = extern "efiapi" fn(); //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:62:8
   |
   |
LL | extern "rust-intrinsic" {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:63:8
   |
LL | extern "platform-intrinsic" {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: rust-call ABI is subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:64:8
   |
   |
LL | extern "rust-call" {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL | extern "efiapi" {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable

error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "rust-intrinsic" fn m1(); //~ ERROR intrinsics are subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "platform-intrinsic" fn m2(); //~ ERROR platform intrinsics are experimental


error[E0570]: `"efiapi"` is not a supported ABI for the current target
   |
   |
LL | extern "efiapi" {} //~ ERROR efiapi ABI is experimental and subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental


error[E0570]: `"efiapi"` is not a supported ABI for the current target
   |
   |
LL | extern "efiapi" fn f10() {} //~ ERROR efiapi ABI is experimental and subject to change


error[E0570]: `"efiapi"` is not a supported ABI for the current target
   |
   |
LL |     extern "efiapi" fn dm10() {} //~ ERROR efiapi ABI is experimental and subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental


error[E0570]: `"efiapi"` is not a supported ABI for the current target
   |
   |
LL |     extern "efiapi" fn m10() {} //~ ERROR efiapi ABI is experimental and subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental


error[E0570]: `"efiapi"` is not a supported ABI for the current target
   |
   |
LL |     extern "efiapi" fn im10() {} //~ ERROR efiapi ABI is experimental and subject to change

error: aborting due to 39 previous errors

Some errors have detailed explanations: E0570, E0658.
