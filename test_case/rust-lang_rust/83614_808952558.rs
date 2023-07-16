plain
..........................................ii................i..i..ii................................ 7300/11718
.................................................................................................... 7400/11718
.................................................................................................... 7500/11718
.................................................................................................... 7600/11718
.............................................................................................i..ii.. 7700/11718
.............FFF.F.F..F.....................................ii...................................... 7800/11718
.............................................................i...................................... 8000/11718
......................................................i............................................. 8100/11718
...........................................................................................i........ 8200/11718
.................................................................................................... 8300/11718
---
.................................................................................................... 9400/11718
.................................................................................................... 9500/11718
............................................................i......i................................ 9600/11718
.................................................................................................... 9700/11718
......iiiiiii..iiiiii.i............................................................................. 9800/11718
.................................................................................................... 10000/11718
.................................................................................................... 10100/11718
.................................................................................................... 10200/11718
.................................................................................................... 10300/11718
---

---- [ui] ui/interior-mutability/interior-mutability.rs stdout ----
diff of stderr:

- error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+ error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
3    |
3    |
4 LL |     catch_unwind(|| { x.set(23); });

-    |     ^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |     ^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
6    | 
7   ::: $SRC_DIR/std/src/panic.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/interior-mutability.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/interior-mutability.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args interior-mutability/interior-mutability.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/interior-mutability/interior-mutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stderr:
------------------------------------------
error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   |
LL |     catch_unwind(|| { x.set(23); });
   |     ^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
  ::: /checkout/library/std/src/panic.rs:430:40
   |
   |
LL | pub fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R> {
   |                                        ---------- required by this bound in `catch_unwind`
   |
   = help: within `Cell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
   = note: required because it appears within the type `Cell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `&Cell<i32>`
   = note: required because it appears within the type `[closure@/checkout/src/test/ui/interior-mutability/interior-mutability.rs:5:18: 5:35]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/not-panic/not-panic-safe-2.rs stdout ----
diff of stderr:

- error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+ error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
3    |
3    |
4 LL | fn assert<T: UnwindSafe + ?Sized>() {}

5    |              ---------- required by this bound in `assert`
6 ...
7 LL |     assert::<Rc<RefCell<i32>>>();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
9    |
10    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
11    = note: required because it appears within the type `RefCell<i32>`

12    = note: required because of the requirements on the impl of `UnwindSafe` for `Rc<RefCell<i32>>`
13 
- error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+ error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
16    |
16    |
17 LL | fn assert<T: UnwindSafe + ?Sized>() {}

18    |              ---------- required by this bound in `assert`
19 ...
20 LL |     assert::<Rc<RefCell<i32>>>();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
22    |
23    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
24    = note: required because it appears within the type `Cell<isize>`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-2/not-panic-safe-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args not-panic/not-panic-safe-2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/not-panic/not-panic-safe-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<Rc<RefCell<i32>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `Rc<RefCell<i32>>`

error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<Rc<RefCell<i32>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
   = note: required because it appears within the type `Cell<isize>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `Rc<RefCell<i32>>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/not-panic/not-panic-safe-3.rs stdout ----
diff of stderr:

- error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+ error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
3    |
3    |
4 LL | fn assert<T: UnwindSafe + ?Sized>() {}

5    |              ---------- required by this bound in `assert`
6 ...
7 LL |     assert::<Arc<RefCell<i32>>>();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
9    |
10    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
11    = note: required because it appears within the type `RefCell<i32>`

12    = note: required because of the requirements on the impl of `UnwindSafe` for `Arc<RefCell<i32>>`
13 
- error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+ error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
16    |
16    |
17 LL | fn assert<T: UnwindSafe + ?Sized>() {}

18    |              ---------- required by this bound in `assert`
19 ...
20 LL |     assert::<Arc<RefCell<i32>>>();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
22    |
23    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
24    = note: required because it appears within the type `Cell<isize>`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-3/not-panic-safe-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args not-panic/not-panic-safe-3.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/not-panic/not-panic-safe-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<Arc<RefCell<i32>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `Arc<RefCell<i32>>`

error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<Arc<RefCell<i32>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
   = note: required because it appears within the type `Cell<isize>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `Arc<RefCell<i32>>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/not-panic/not-panic-safe-4.rs stdout ----
diff of stderr:

- error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+ error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
3    |
3    |
4 LL | fn assert<T: UnwindSafe + ?Sized>() {}

5    |              ---------- required by this bound in `assert`
6 ...
7 LL |     assert::<&RefCell<i32>>();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |     ^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
9    |
10    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
11    = note: required because it appears within the type `RefCell<i32>`

12    = note: required because of the requirements on the impl of `UnwindSafe` for `&RefCell<i32>`
13 
- error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+ error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
16    |
16    |
17 LL | fn assert<T: UnwindSafe + ?Sized>() {}

18    |              ---------- required by this bound in `assert`
19 ...
20 LL |     assert::<&RefCell<i32>>();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |     ^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
22    |
23    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
24    = note: required because it appears within the type `Cell<isize>`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-4/not-panic-safe-4.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args not-panic/not-panic-safe-4.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/not-panic/not-panic-safe-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<&RefCell<i32>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `&RefCell<i32>`

error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<&RefCell<i32>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
   = note: required because it appears within the type `Cell<isize>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `&RefCell<i32>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/not-panic/not-panic-safe-5.rs stdout ----
diff of stderr:

- error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+ error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
3    |
3    |
4 LL | fn assert<T: UnwindSafe + ?Sized>() {}

5    |              ---------- required by this bound in `assert`
6 ...
7 LL |     assert::<*const UnsafeCell<i32>>();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
9    |
10    = help: the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
11    = note: required because of the requirements on the impl of `UnwindSafe` for `*const UnsafeCell<i32>`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-5/not-panic-safe-5.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args not-panic/not-panic-safe-5.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/not-panic/not-panic-safe-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-5" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-5/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<*const UnsafeCell<i32>>(); //~ ERROR E0277
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   = help: the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `*const UnsafeCell<i32>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/not-panic/not-panic-safe-6.rs stdout ----
diff of stderr:

- error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+ error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
3    |
3    |
4 LL | fn assert<T: UnwindSafe + ?Sized>() {}

5    |              ---------- required by this bound in `assert`
6 ...
7 LL |     assert::<*mut RefCell<i32>>();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
9    |
10    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
11    = note: required because it appears within the type `RefCell<i32>`

12    = note: required because of the requirements on the impl of `UnwindSafe` for `*mut RefCell<i32>`
13 
- error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+ error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
16    |
16    |
17 LL | fn assert<T: UnwindSafe + ?Sized>() {}

18    |              ---------- required by this bound in `assert`
19 ...
20 LL |     assert::<*mut RefCell<i32>>();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
22    |
23    = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
24    = note: required because it appears within the type `Cell<isize>`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-6/not-panic-safe-6.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args not-panic/not-panic-safe-6.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/not-panic/not-panic-safe-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-6" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe-6/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the type `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<*mut RefCell<i32>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<i32>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<i32>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `*mut RefCell<i32>`

error[E0277]: the type `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<*mut RefCell<i32>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsafeCell<isize>` may contain interior mutability and a reference might not be safely transferrable across a catch_unwind boundary
   |
   = help: within `RefCell<i32>`, the trait `RefUnwindSafe` is not implemented for `UnsafeCell<isize>`
   = note: required because it appears within the type `Cell<isize>`
   = note: required because it appears within the type `RefCell<i32>`
   = note: required because of the requirements on the impl of `UnwindSafe` for `*mut RefCell<i32>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/not-panic/not-panic-safe.rs stdout ----
diff of stderr:

- error[E0277]: the type `&mut i32` may not be safely transferred across an unwind boundary
+ error[E0277]: the type `&mut i32` might not be safely transferred across an unwind boundary
3    |
3    |
4 LL | fn assert<T: UnwindSafe + ?Sized>() {}

5    |              ---------- required by this bound in `assert`
6 ...
7 LL |     assert::<&mut i32>();
-    |     ^^^^^^^^^^^^^^^^^^ `&mut i32` may not be safely transferred across an unwind boundary
+    |     ^^^^^^^^^^^^^^^^^^ `&mut i32` might not be safely transferred across an unwind boundary
9    |
10    = help: the trait `UnwindSafe` is not implemented for `&mut i32`
11    = note: `UnwindSafe` is implemented for `&i32`, but not for `&mut i32`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe/not-panic-safe.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe/not-panic-safe.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args not-panic/not-panic-safe.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/not-panic/not-panic-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/not-panic/not-panic-safe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the type `&mut i32` might not be safely transferred across an unwind boundary
   |
   |
LL | fn assert<T: UnwindSafe + ?Sized>() {}
   |              ---------- required by this bound in `assert`
...
LL |     assert::<&mut i32>();
   |     ^^^^^^^^^^^^^^^^^^ `&mut i32` might not be safely transferred across an unwind boundary
   |
   = help: the trait `UnwindSafe` is not implemented for `&mut i32`
   = note: `UnwindSafe` is implemented for `&i32`, but not for `&mut i32`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 11615 passed; 7 failed; 96 ignored; 0 measured; 0 filtered out; finished in 140.49s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:52
