plain
........................................................................................ 6600/13657
.....................................................i.................................. 6688/13657
........................................................................................ 6776/13657
............................i........................................................ii. 6864/13657
ii........i...Fi...............................................................i........ 6952/13657
...........................................................i....i....................... 7128/13657
..................i..................i.............i.................................... 7216/13657
..........................i............................................................. 7304/13657
..............................................i......................................... 7392/13657
---

---- [ui] src/test/ui/consts/const-eval/issue-50814.rs stdout ----
diff of stderr:

4 LL |     const MAX: u8 = A::MAX + B::MAX;
5    |                     ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
6 
+ error[E0080]: evaluation of `foo::<T>` failed
+    |
+    |
+ LL |     &Sum::<U8,U8>::MAX
+ 
+ 
+ error[E0080]: evaluation of `<Sum<U8, U8> as Unsigned>::MAX` failed
+    |
+    |
+ LL |     const MAX: u8 = A::MAX + B::MAX;
+    |                     ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
7 error[E0080]: evaluation of `foo::<i32>` failed
8   --> $DIR/issue-50814.rs:20:6
9    |

---
To only update this specific test, also pass `--test-args consts/const-eval/issue-50814.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-50814.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `<Sum<U8, U8> as Unsigned>::MAX` failed
   |
   |
LL |     const MAX: u8 = A::MAX + B::MAX;
   |                     ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow

error[E0080]: evaluation of `foo::<T>` failed
   |
   |
LL |     &Sum::<U8,U8>::MAX


error[E0080]: evaluation of `<Sum<U8, U8> as Unsigned>::MAX` failed
   |
   |
LL |     const MAX: u8 = A::MAX + B::MAX;
   |                     ^^^^^^^^^^^^^^^ attempt to compute `u8::MAX + u8::MAX`, which would overflow
error[E0080]: evaluation of `foo::<i32>` failed
  --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:20:6
   |
   |
LL |     &Sum::<U8,U8>::MAX


note: the above error was encountered while instantiating `fn foo::<i32>`
   |
LL |     foo(0);
   |     ^^^^^^

