plain
........................................................................................ 6600/13644
..................................................i..................................... 6688/13644
........................................................................................ 6776/13644
...........................i........................................................ii.i 6864/13644
i........i....iF..............................................................i......... 6952/13644
.........................................................i....i......................... 7128/13644
................i..................i.............i...................................... 7216/13644
......................i................................................................. 7304/13644
...........................................i............................................ 7392/13644
---

8    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
9    = note: `#[deny(const_err)]` on by default
10 
+ error[E0080]: evaluation of `foo::<T>` failed
+    |
+    |
+ LL |     &Sum::<U8,U8>::MAX
+ 
+ error: any use of this value will cause an error
+   --> $DIR/issue-50814.rs:15:21
+    |
+    |
+ LL |     const MAX: u8 = A::MAX + B::MAX;
+    |     -------------   ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
11 error[E0080]: evaluation of `foo::<i32>` failed
---

+ error: any use of this value will cause an error
+   --> $DIR/issue-50814.rs:15:21
+    |
+ LL |     const MAX: u8 = A::MAX + B::MAX;
+    |     -------------   ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: `#[deny(const_err)]` on by default
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
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: `#[deny(const_err)]` on by default


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
  --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:21:6
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
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: `#[deny(const_err)]` on by default


Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:15:21
   |
LL |     const MAX: u8 = A::MAX + B::MAX;
   |     -------------   ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: `#[deny(const_err)]` on by default
------------------------------------------
