plain
26 
+ warning: any use of this value will cause an error
+   --> $DIR/erroneous-const.rs:6:22
+    |
+ LL |     const VOID: () = [()][2];
+    |     --------------   ^^^^^^^ index out of bounds: the length is 1 but the index is 2
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
27 error[E0080]: could not evaluate static initializer
---

+ warning: any use of this value will cause an error
+   --> $DIR/erroneous-const.rs:6:22
+    |
+ LL |     const VOID: () = [()][2];
+    |     --------------   ^^^^^^^ index out of bounds: the length is 1 but the index is 2
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ note: the lint level is defined here
+   --> $DIR/erroneous-const.rs:2:9
+    |
+    |
+ LL | #![warn(const_err, unconditional_panic)]
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
---
To only update this specific test, also pass `--test-args consts/const-eval/erroneous-const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/erroneous-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/erroneous-const" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/erroneous-const/auxiliary"
stdout: none
--- stderr -------------------------------
warning: this operation will panic at runtime
   |
   |
LL |     const VOID: () = [()][2]; //~WARN any use of this value will cause an error
   |                      ^^^^^^^ index out of bounds: the length is 1 but the index is 2
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:2:20
   |
LL | #![warn(const_err, unconditional_panic)]
LL | #![warn(const_err, unconditional_panic)]
   |                    ^^^^^^^^^^^^^^^^^^^

warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:6:22
   |
LL |     const VOID: () = [()][2]; //~WARN any use of this value will cause an error
   |     --------------   ^^^^^^^ index out of bounds: the length is 1 but the index is 2
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:2:9
   |
LL | #![warn(const_err, unconditional_panic)]
LL | #![warn(const_err, unconditional_panic)]
   |         ^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:6:22
   |
LL |     const VOID: () = [()][2]; //~WARN any use of this value will cause an error
   |     --------------   ^^^^^^^ index out of bounds: the length is 1 but the index is 2
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: could not evaluate static initializer
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:15:17
   |
LL |         let _ = PrintName::<T>::VOID; //~ERROR could not evaluate static initializer
   |                 |
   |                 referenced constant has errors
   |                 inside `no_codegen::<i32>` at /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:15:17
...
---
Future incompatibility report: Future breakage diagnostic:
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:6:22
   |
LL |     const VOID: () = [()][2]; //~WARN any use of this value will cause an error
   |     --------------   ^^^^^^^ index out of bounds: the length is 1 but the index is 2
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:2:9
   |
LL | #![warn(const_err, unconditional_panic)]
---
Future breakage diagnostic:
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:6:22
   |
LL |     const VOID: () = [()][2]; //~WARN any use of this value will cause an error
   |     --------------   ^^^^^^^ index out of bounds: the length is 1 but the index is 2
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/erroneous-const.rs:2:9
   |
LL | #![warn(const_err, unconditional_panic)]
---
diff of stderr:

18   --> $DIR/issue-50814.rs:15:21
19    |
20 LL |     const MAX: u8 = A::MAX + B::MAX;
-    |                     |
-    |                     |
-    |                     attempt to compute `u8::MAX + u8::MAX`, which would overflow
+    |     -------------   ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
25    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
26    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

41 
41 
42 For more information about this error, try `rustc --explain E0080`.
43 Future incompatibility report: Future breakage diagnostic:
+ error: any use of this value will cause an error
+   --> $DIR/issue-50814.rs:15:21
+    |
+ LL |     const MAX: u8 = A::MAX + B::MAX;
+    |     -------------   ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
+    = note: `#[deny(const_err)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
---
To only update this specific test, also pass `--test-args consts/const-eval/issue-50814.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-50814.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:15:21
   |
   |
LL |     const MAX: u8 = A::MAX + B::MAX;
   |     -------------   ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error[E0080]: evaluation of `foo::<T>` failed
   |
   |
LL |     &Sum::<U8,U8>::MAX

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:15:21
   |
   |
LL |     const MAX: u8 = A::MAX + B::MAX;
   |     -------------   ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of `foo::<i32>` failed
error[E0080]: evaluation of `foo::<i32>` failed
  --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:23:6
   |
LL |     &Sum::<U8,U8>::MAX


note: the above error was encountered while instantiating `fn foo::<i32>`
   |
LL |     foo(0);
   |     ^^^^^^


error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:15:21
   |
LL |     const MAX: u8 = A::MAX + B::MAX;
   |     -------------   ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:15:21
   |
LL |     const MAX: u8 = A::MAX + B::MAX;
   |     -------------   ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------
