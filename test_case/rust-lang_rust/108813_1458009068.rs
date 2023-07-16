plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:46007752205b5430f5cabe1357251ea7621a9e98)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/consts/issue-17718-const-bad-values.rs stdout ----
diff of stderr:

27    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered mutable reference in a `const`
28    |
29    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 8, align: 4) {
-                ╾─alloc4──╼ 00 00 00 00                         │ ╾──╼....
+    = note: the raw bytes of the constant (size: 16, align: 8) {
32            }
33 
34 error: aborting due to 4 previous errors

---
To only update this specific test, also pass `--test-args consts/issue-17718-const-bad-values.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/issue-17718-const-bad-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-17718-const-bad-values" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-17718-const-bad-values/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: mutable references are not allowed in the final value of constants
  --> fake-test-src-base/consts/issue-17718-const-bad-values.rs:1:34
   |
LL | const C1: &'static mut [usize] = &mut [];

error[E0013]: constants cannot refer to statics
  --> fake-test-src-base/consts/issue-17718-const-bad-values.rs:6:46
   |
   |
LL | const C2: &'static mut usize = unsafe { &mut S };
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error[E0013]: constants cannot refer to statics
  --> fake-test-src-base/consts/issue-17718-const-bad-values.rs:6:46
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL | const C2: &'static mut usize = unsafe { &mut S };
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/issue-17718-const-bad-values.rs:1:1
   |
   |
LL | const C1: &'static mut [usize] = &mut [];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered mutable reference in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error: aborting due to 4 previous errors

---

---- [ui] tests/ui/error-codes/E0017.rs stdout ----
diff of stderr:

73    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered mutable reference in a `const`
74    |
75    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-                ╾─alloc7──╼                                     │ ╾──╼
+    = note: the raw bytes of the constant (size: 8, align: 8) {
78            }
79 
80 error: aborting due to 7 previous errors; 2 warnings emitted

---
To only update this specific test, also pass `--test-args error-codes/E0017.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/error-codes/E0017.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/auxiliary"
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


error[E0764]: mutable references are not allowed in the final value of constants
  --> fake-test-src-base/error-codes/E0017.rs:5:30
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error[E0658]: mutation through a reference is not allowed in statics
  --> fake-test-src-base/error-codes/E0017.rs:8:39
   |
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0658
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0764]: mutable references are not allowed in the final value of statics
  --> fake-test-src-base/error-codes/E0017.rs:8:39
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0658


error[E0596]: cannot borrow immutable static item `X` as mutable
  --> fake-test-src-base/error-codes/E0017.rs:8:39
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0658
   |                                       ^^^^^^ cannot borrow as mutable

warning: taking a mutable reference to a `const` item
  --> fake-test-src-base/error-codes/E0017.rs:12:38
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

error[E0764]: mutable references are not allowed in the final value of statics
error[E0764]: mutable references are not allowed in the final value of statics
  --> fake-test-src-base/error-codes/E0017.rs:12:38
   |
LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error[E0764]: mutable references are not allowed in the final value of statics
  --> fake-test-src-base/error-codes/E0017.rs:14:52
   |
   |
LL | static STATIC_MUT_REF: &'static mut i32 = unsafe { &mut M }; //~ ERROR mutable references are not

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/error-codes/E0017.rs:5:1
   |
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered mutable reference in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: aborting due to 7 previous errors; 2 warnings emitted

---

---- [ui] tests/ui/error-codes/E0388.rs stdout ----
diff of stderr:

67    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered mutable reference in a `const`
68    |
69    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 4, align: 4) {
-                ╾─alloc5──╼                                     │ ╾──╼
+    = note: the raw bytes of the constant (size: 8, align: 8) {
72            }
73 
74 error: aborting due to 6 previous errors; 2 warnings emitted

---
To only update this specific test, also pass `--test-args error-codes/E0388.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/error-codes/E0388.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/auxiliary"
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


error[E0764]: mutable references are not allowed in the final value of constants
  --> fake-test-src-base/error-codes/E0388.rs:4:30
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error[E0658]: mutation through a reference is not allowed in statics
  --> fake-test-src-base/error-codes/E0388.rs:7:39
   |
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR cannot borrow
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0764]: mutable references are not allowed in the final value of statics
  --> fake-test-src-base/error-codes/E0388.rs:7:39
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR cannot borrow


error[E0596]: cannot borrow immutable static item `X` as mutable
  --> fake-test-src-base/error-codes/E0388.rs:7:39
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR cannot borrow
   |                                       ^^^^^^ cannot borrow as mutable

warning: taking a mutable reference to a `const` item
  --> fake-test-src-base/error-codes/E0388.rs:11:38
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

error[E0764]: mutable references are not allowed in the final value of statics
error[E0764]: mutable references are not allowed in the final value of statics
  --> fake-test-src-base/error-codes/E0388.rs:11:38
   |
LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/error-codes/E0388.rs:4:1
   |
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered mutable reference in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: aborting due to 6 previous errors; 2 warnings emitted

