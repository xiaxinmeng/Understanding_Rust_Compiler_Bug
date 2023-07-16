plain
........................................................................................  2816/15051
........................................................................................  2904/15051
........................................................................................  2992/15051
........................................................................................  3080/15051
...................................................................................F.F..  3168/15051
.................................................F......................................  3344/15051
........................................................................................  3432/15051
........................................................................i...............  3520/15051
.................F.....................i........................................i.......  3608/15051
---

---- [ui] tests/ui/check-static-immutable-mut-slices.rs stdout ----
diff of stderr:

- error[E0764]: mutable references are not allowed in the final value of statics
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
3    |
3    |
4 LL | static TEST: &'static mut [isize] = &mut [];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-immutable-mut-slices/check-static-immutable-mut-slices.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-static-immutable-mut-slices.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/check-static-immutable-mut-slices.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-immutable-mut-slices" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-immutable-mut-slices/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/check-static-immutable-mut-slices.rs:3:37
   |
LL | static TEST: &'static mut [isize] = &mut [];

error: aborting due to previous error

For more information about this error, try `rustc --explain E0764`.
For more information about this error, try `rustc --explain E0764`.
------------------------------------------


---- [ui] tests/ui/consts/const-mut-refs/issue-76510.rs stdout ----
diff of 64bit.stderr:

- error[E0764]: mutable references are not allowed in the final value of constants
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
3    |
3    |
4 LL | const S: &'static mut str = &mut " hello ";

The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/issue-76510/issue-76510.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-mut-refs/issue-76510.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-mut-refs/issue-76510.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/issue-76510" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/issue-76510/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/consts/const-mut-refs/issue-76510.rs:5:29
   |
LL | const S: &'static mut str = &mut " hello ";

error[E0658]: mutation through a reference is not allowed in constants
  --> fake-test-src-base/consts/const-mut-refs/issue-76510.rs:5:29
   |
   |
LL | const S: &'static mut str = &mut " hello ";
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0596]: cannot borrow data in a `&` reference as mutable
  --> fake-test-src-base/consts/const-mut-refs/issue-76510.rs:5:29
   |
LL | const S: &'static mut str = &mut " hello ";
   |                             ^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to 3 previous errors

Build completed unsuccessfully in 0:13:06
Some errors have detailed explanations: E0596, E0658, E0764.
Some errors have detailed explanations: E0596, E0658, E0764.
For more information about an error, try `rustc --explain E0596`.
------------------------------------------


---- [ui] tests/ui/consts/const-mut-refs/mut_ref_in_final.rs stdout ----
diff of stderr:

- error[E0764]: mutable references are not allowed in the final value of constants
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
3    |
3    |
4 LL | const B: *mut i32 = &mut 4;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final/mut_ref_in_final.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-mut-refs/mut_ref_in_final.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-mut-refs/mut_ref_in_final.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/consts/const-mut-refs/mut_ref_in_final.rs:10:21
   |
LL | const B: *mut i32 = &mut 4; //~ ERROR mutable references are not allowed

error[E0716]: temporary value dropped while borrowed
  --> fake-test-src-base/consts/const-mut-refs/mut_ref_in_final.rs:16:40
   |
   |
LL | const B3: Option<&mut i32> = Some(&mut 42); //~ ERROR temporary value dropped while borrowed
   |                              ----------^^-
   |                              |         | temporary value is freed at the end of this statement
   |                              |         creates a temporary value which is freed while still in use
   |                              |         creates a temporary value which is freed while still in use
   |                              using this value as a constant requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> fake-test-src-base/consts/const-mut-refs/mut_ref_in_final.rs:19:42
   |
   |
LL | const B4: Option<&mut i32> = helper(&mut 42); //~ ERROR temporary value dropped while borrowed
   |                              ------------^^-
   |                              |           | temporary value is freed at the end of this statement
   |                              |           creates a temporary value which is freed while still in use
   |                              |           creates a temporary value which is freed while still in use
   |                              using this value as a constant requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> fake-test-src-base/consts/const-mut-refs/mut_ref_in_final.rs:34:65
   |
   |
LL | const FOO: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                  -------------------------------^^--
   |                                  |                              |  temporary value is freed at the end of this statement
   |                                  |                              creates a temporary value which is freed while still in use
   |                                  |                              creates a temporary value which is freed while still in use
   |                                  using this value as a constant requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> fake-test-src-base/consts/const-mut-refs/mut_ref_in_final.rs:37:67
   |
   |
LL | static FOO2: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                    -------------------------------^^--
   |                                    |                              |  temporary value is freed at the end of this statement
   |                                    |                              creates a temporary value which is freed while still in use
   |                                    |                              creates a temporary value which is freed while still in use
   |                                    using this value as a static requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> fake-test-src-base/consts/const-mut-refs/mut_ref_in_final.rs:40:71
   |
   |
LL | static mut FOO3: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                        -------------------------------^^--
   |                                        |                              |  temporary value is freed at the end of this statement
   |                                        |                              creates a temporary value which is freed while still in use
   |                                        |                              creates a temporary value which is freed while still in use
   |                                        using this value as a static requires that borrow lasts for `'static`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0716, E0764.
For more information about an error, try `rustc --explain E0716`.
For more information about an error, try `rustc --explain E0716`.
------------------------------------------


---- [ui] tests/ui/consts/issue-17718-const-bad-values.rs stdout ----
diff of stderr:

- error[E0764]: mutable references are not allowed in the final value of constants
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
3    |
3    |
4 LL | const C1: &'static mut [usize] = &mut [];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-17718-const-bad-values/issue-17718-const-bad-values.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-17718-const-bad-values.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/issue-17718-const-bad-values.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-17718-const-bad-values" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-17718-const-bad-values/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/consts/issue-17718-const-bad-values.rs:1:34
   |
LL | const C1: &'static mut [usize] = &mut [];

error[E0013]: constants cannot refer to statics
  --> fake-test-src-base/consts/issue-17718-const-bad-values.rs:5:46
   |
   |
LL | const C2: &'static mut usize = unsafe { &mut S };
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error[E0013]: constants cannot refer to statics
  --> fake-test-src-base/consts/issue-17718-const-bad-values.rs:5:46
   |
   |
LL | const C2: &'static mut usize = unsafe { &mut S };
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0013, E0764.
For more information about an error, try `rustc --explain E0013`.
For more information about an error, try `rustc --explain E0013`.
------------------------------------------


---- [ui] tests/ui/consts/write_to_static_via_mut_ref.rs stdout ----
diff of stderr:

- error[E0764]: mutable references are not allowed in the final value of statics
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
3    |
3    |
4 LL | static OH_NO: &mut i32 = &mut 42;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/write_to_static_via_mut_ref/write_to_static_via_mut_ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/write_to_static_via_mut_ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/write_to_static_via_mut_ref.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/write_to_static_via_mut_ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/write_to_static_via_mut_ref/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/consts/write_to_static_via_mut_ref.rs:3:26
   |
LL | static OH_NO: &mut i32 = &mut 42; //~ ERROR mutable references are not allowed


error[E0594]: cannot assign to `*OH_NO`, as `OH_NO` is an immutable static item
  --> fake-test-src-base/consts/write_to_static_via_mut_ref.rs:6:5
   |
LL |     *OH_NO = 43; //~ ERROR cannot assign to `*OH_NO`, as `OH_NO` is an immutable static

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0594, E0764.
---

13    | ^^^^^^^^^^^^
14    = note: `#[warn(const_item_mutation)]` on by default
15 
- error[E0764]: mutable references are not allowed in the final value of constants
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
17   --> $DIR/E0017.rs:5:30
18    |
19 LL | const CR: &'static mut i32 = &mut C;
28    = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
29    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
30 
- error[E0764]: mutable references are not allowed in the final value of statics
- error[E0764]: mutable references are not allowed in the final value of statics
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
32   --> $DIR/E0017.rs:7:39
33    |
34 LL | static STATIC_REF: &'static mut i32 = &mut X;
54 LL | const C: i32 = 2;
55    | ^^^^^^^^^^^^
56 
- error[E0764]: mutable references are not allowed in the final value of statics
- error[E0764]: mutable references are not allowed in the final value of statics
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
58   --> $DIR/E0017.rs:11:38
59    |
60 LL | static CONST_REF: &'static mut i32 = &mut C;
61    |                                      ^^^^^^
62 
- error[E0764]: mutable references are not allowed in the final value of statics
- error[E0764]: mutable references are not allowed in the final value of statics
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
64   --> $DIR/E0017.rs:13:52
65    |
66 LL | static STATIC_MUT_REF: &'static mut i32 = unsafe { &mut M };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/E0017.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0017.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/error-codes/E0017.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/auxiliary"
stdout: none
--- stderr -------------------------------
warning: taking a mutable reference to a `const` item
  --> fake-test-src-base/error-codes/E0017.rs:5:30
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed
   |
   |
   = note: each usage of a `const` item creates a new temporary
   = note: the mutable reference will refer to this temporary, not the original `const` item
note: `const` item defined here
  --> fake-test-src-base/error-codes/E0017.rs:2:1
LL | const C: i32 = 2;
   | ^^^^^^^^^^^^
   = note: `#[warn(const_item_mutation)]` on by default


error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/error-codes/E0017.rs:5:30
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error[E0658]: mutation through a reference is not allowed in statics
  --> fake-test-src-base/error-codes/E0017.rs:7:39
   |
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0658
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/error-codes/E0017.rs:7:39
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0658


error[E0596]: cannot borrow immutable static item `X` as mutable
  --> fake-test-src-base/error-codes/E0017.rs:7:39
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0658
   |                                       ^^^^^^ cannot borrow as mutable

warning: taking a mutable reference to a `const` item
  --> fake-test-src-base/error-codes/E0017.rs:11:38
   |
LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed
   |
   |
   = note: each usage of a `const` item creates a new temporary
   = note: the mutable reference will refer to this temporary, not the original `const` item
note: `const` item defined here
  --> fake-test-src-base/error-codes/E0017.rs:2:1
LL | const C: i32 = 2;
   | ^^^^^^^^^^^^


error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/error-codes/E0017.rs:11:38
   |
LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed


error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/error-codes/E0017.rs:13:52
   |
LL | static STATIC_MUT_REF: &'static mut i32 = unsafe { &mut M }; //~ ERROR mutable references are not

error: aborting due to 6 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0596, E0658, E0764.
---

13    | ^^^^^^^^^^^^
14    = note: `#[warn(const_item_mutation)]` on by default
15 
- error[E0764]: mutable references are not allowed in the final value of constants
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
17   --> $DIR/E0388.rs:4:30
18    |
19 LL | const CR: &'static mut i32 = &mut C;
28    = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
29    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
30 
- error[E0764]: mutable references are not allowed in the final value of statics
- error[E0764]: mutable references are not allowed in the final value of statics
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
32   --> $DIR/E0388.rs:6:39
33    |
34 LL | static STATIC_REF: &'static mut i32 = &mut X;
54 LL | const C: i32 = 2;
55    | ^^^^^^^^^^^^
56 
- error[E0764]: mutable references are not allowed in the final value of statics
- error[E0764]: mutable references are not allowed in the final value of statics
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
58   --> $DIR/E0388.rs:10:38
59    |
60 LL | static CONST_REF: &'static mut i32 = &mut C;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/E0388.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0388.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/error-codes/E0388.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/auxiliary"
stdout: none
--- stderr -------------------------------
warning: taking a mutable reference to a `const` item
  --> fake-test-src-base/error-codes/E0388.rs:4:30
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed
   |
   |
   = note: each usage of a `const` item creates a new temporary
   = note: the mutable reference will refer to this temporary, not the original `const` item
note: `const` item defined here
  --> fake-test-src-base/error-codes/E0388.rs:2:1
LL | const C: i32 = 2;
   | ^^^^^^^^^^^^
   = note: `#[warn(const_item_mutation)]` on by default


error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/error-codes/E0388.rs:4:30
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error[E0658]: mutation through a reference is not allowed in statics
  --> fake-test-src-base/error-codes/E0388.rs:6:39
   |
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR cannot borrow
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/error-codes/E0388.rs:6:39
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR cannot borrow


error[E0596]: cannot borrow immutable static item `X` as mutable
  --> fake-test-src-base/error-codes/E0388.rs:6:39
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR cannot borrow
   |                                       ^^^^^^ cannot borrow as mutable

warning: taking a mutable reference to a `const` item
  --> fake-test-src-base/error-codes/E0388.rs:10:38
   |
LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed
   |
   |
   = note: each usage of a `const` item creates a new temporary
   = note: the mutable reference will refer to this temporary, not the original `const` item
note: `const` item defined here
  --> fake-test-src-base/error-codes/E0388.rs:2:1
LL | const C: i32 = 2;
   | ^^^^^^^^^^^^


error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/error-codes/E0388.rs:10:38
   |
LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error: aborting due to 5 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0596, E0658, E0764.
Some errors have detailed explanations: E0596, E0658, E0764.
For more information about an error, try `rustc --explain E0596`.
------------------------------------------


---- [ui] tests/ui/issues/issue-46604.rs stdout ----
diff of stderr:

- error[E0764]: mutable references are not allowed in the final value of statics
+ error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
+               safety since holding multiple mutable references to shared data is not allowed.
+               
+               
+               If you really want global mutable state, try using static mut or a global UnsafeCell.
3    |
3    |
4 LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604/issue-46604.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-46604.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-46604.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: Statics are shared everywhere, and if they refer to mutable data one might violate memory
              safety since holding multiple mutable references to shared data is not allowed.
              
              
              If you really want global mutable state, try using static mut or a global UnsafeCell.
  --> fake-test-src-base/issues/issue-46604.rs:1:25
   |
LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];   //~ ERROR mutable references are not allowed


error[E0594]: cannot assign to `buf[_]`, as `buf` is an immutable static item
  --> fake-test-src-base/issues/issue-46604.rs:6:5
   |
LL |     buf[0]=2;                                   //~ ERROR E0594

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0594, E0764.
