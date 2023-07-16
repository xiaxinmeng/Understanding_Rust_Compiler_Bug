plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 13288/14754
........................................................................................ 13376/14754
........................................................................................ 13464/14754
........................................................................................ 13552/14754
...............................................................................F.F...... 13640/14754
.....................................................................................i.. 13816/14754
........................................................................................ 13904/14754
........................................................................................ 13992/14754
........................................................................................ 14080/14754
---
failures:

---- [ui] tests/ui/issues/issue-11820.rs stdout ----
normalized stderr:
warning: using `clone` on a double-reference, which copies the reference of type `NoClone` instead of cloning the inner type
   |
   |
LL |   let _: &NoClone = rnc.clone();
   |
   |
   = note: `#[warn(clone_double_ref)]` on by default
help: try dereferencing it
   |
LL |   let _: &NoClone = &(*rnc).clone();
   |                     +++   ~~~~~~~~~
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |   let _: &NoClone = <&NoClone>::clone(rnc);
   |                     ++++++++++++++++++   ~

warning: using `clone` on a double-reference, which copies the reference of type `Option<NoClone>` instead of cloning the inner type
   |
   |
LL |   let _: &Option<NoClone> = rsnc.clone();
   |
help: try dereferencing it
   |
   |
LL |   let _: &Option<NoClone> = &(*rsnc).clone();
help: or try being explicit if you are sure, that you want to clone a reference
   |
   |
LL |   let _: &Option<NoClone> = <&Option<NoClone>>::clone(rsnc);

warning: 2 warnings emitted


---
To only update this specific test, also pass `--test-args issues/issue-11820.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-11820.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11820/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11820/auxiliary"
stdout: none
--- stderr -------------------------------
warning: using `clone` on a double-reference, which copies the reference of type `NoClone` instead of cloning the inner type
  --> fake-test-src-base/issues/issue-11820.rs:10:21
   |
LL |   let _: &NoClone = rnc.clone();
   |
   |
   = note: `#[warn(clone_double_ref)]` on by default
help: try dereferencing it
   |
LL |   let _: &NoClone = &(*rnc).clone();
   |                     +++   ~~~~~~~~~
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |   let _: &NoClone = <&NoClone>::clone(rnc);
   |                     ++++++++++++++++++   ~

warning: using `clone` on a double-reference, which copies the reference of type `Option<NoClone>` instead of cloning the inner type
  --> fake-test-src-base/issues/issue-11820.rs:11:29
   |
LL |   let _: &Option<NoClone> = rsnc.clone();
   |
help: try dereferencing it
   |
   |
LL |   let _: &Option<NoClone> = &(*rsnc).clone();
help: or try being explicit if you are sure, that you want to clone a reference
   |
   |
LL |   let _: &Option<NoClone> = <&Option<NoClone>>::clone(rsnc);

warning: 2 warnings emitted
------------------------------------------



---- [ui] tests/ui/lint/clone-double-ref.rs stdout ----
diff of stderr:

- error: using `clone` on a double-reference, which copies the reference of type `Vec<i32>` instead of cloning the inner type
-   --> $DIR/clone-double-ref.rs:7:22
+ error[E0658]: use of unstable library feature 'lazy_cell'
3    |
3    |
- LL |     let z: &Vec<_> = y.clone();
+ LL | use std::sync::LazyLock;
+    |     ^^^^^^^^^^^^^^^^^^^
6    |
- note: the lint level is defined here
- note: the lint level is defined here
-   --> $DIR/clone-double-ref.rs:2:9
+    = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
+    = help: add `#![feature(lazy_cell)]` to the crate attributes to enable
+ 
+ error[E0658]: use of unstable library feature 'lazy_cell'
9    |
- LL | #![deny(clone_double_ref)]
-    |         ^^^^^^^^^^^^^^^^
- help: try dereferencing it
- help: try dereferencing it
+ LL | pub static STRS: LazyLock<&str> = LazyLock::new(|| "First");
13    |
13    |
- LL |     let z: &Vec<_> = &(*y).clone();
-    |                      +++ ~~~~~~~~~
- help: or try being explicit if you are sure, that you want to clone a reference
+    = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
+    = help: add `#![feature(lazy_cell)]` to the crate attributes to enable
+ 
+ error[E0658]: use of unstable library feature 'lazy_cell'
17    |
17    |
- LL |     let z: &Vec<_> = <&Vec<i32>>::clone(y);
-    |                      +++++++++++++++++++ ~
+ LL | pub static STRS: LazyLock<&str> = LazyLock::new(|| "First");
+    |
+    = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
+    = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
+    = help: add `#![feature(lazy_cell)]` to the crate attributes to enable
- error: aborting due to previous error
+ warning: the feature `once_cell` has been stable since 1.70.0-nightly and no longer requires an attribute to enable
+   --> $DIR/clone-double-ref.rs:1:12
+    |
+    |
+ LL | #![feature(once_cell)]
+    |
+    = note: `#[warn(stable_features)]` on by default
22 
22 
+ error[E0658]: use of unstable library feature 'lazy_cell'
+    |
+    |
+ LL | pub static STRS: LazyLock<&str> = LazyLock::new(|| "First");
+    |
+    = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
+    = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
+    = help: add `#![feature(lazy_cell)]` to the crate attributes to enable
+ error: aborting due to 4 previous errors; 1 warning emitted
+ 
+ For more information about this error, try `rustc --explain E0658`.
23 
---
To only update this specific test, also pass `--test-args lint/clone-double-ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/clone-double-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/clone-double-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/clone-double-ref/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: use of unstable library feature 'lazy_cell'
  --> fake-test-src-base/lint/clone-double-ref.rs:13:5
LL | use std::sync::LazyLock;
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
   = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
   = help: add `#![feature(lazy_cell)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'lazy_cell'
  --> fake-test-src-base/lint/clone-double-ref.rs:15:18
   |
LL | pub static STRS: LazyLock<&str> = LazyLock::new(|| "First");
   |
   = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
   = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
   = help: add `#![feature(lazy_cell)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'lazy_cell'
  --> fake-test-src-base/lint/clone-double-ref.rs:15:35
   |
LL | pub static STRS: LazyLock<&str> = LazyLock::new(|| "First");
   |
   = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
   = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
   = help: add `#![feature(lazy_cell)]` to the crate attributes to enable
warning: the feature `once_cell` has been stable since 1.70.0-nightly and no longer requires an attribute to enable
  --> fake-test-src-base/lint/clone-double-ref.rs:1:12
   |
LL | #![feature(once_cell)]
LL | #![feature(once_cell)]
   |            ^^^^^^^^^
   |
   = note: `#[warn(stable_features)]` on by default

error[E0658]: use of unstable library feature 'lazy_cell'
  --> fake-test-src-base/lint/clone-double-ref.rs:15:35
   |
LL | pub static STRS: LazyLock<&str> = LazyLock::new(|| "First");
   |
   = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
   = note: see issue #109736 <https://github.com/rust-lang/rust/issues/109736> for more information
   = help: add `#![feature(lazy_cell)]` to the crate attributes to enable
error: aborting due to 4 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/lint/noop-method-call.rs stdout ----
diff of stderr:

+ warning: using `clone` on a double-reference, which copies the reference of type `PlainType<u32>` instead of cloning the inner type
+   --> $DIR/noop-method-call.rs:16:53
+    |
+ LL |     let non_clone_type_ref_clone: &PlainType<u32> = non_clone_type_ref.clone();
+    |
+    |
+    = note: `#[warn(clone_double_ref)]` on by default
+ help: try dereferencing it
+    |
+ LL |     let non_clone_type_ref_clone: &PlainType<u32> = &(*non_clone_type_ref).clone();
+    |                                                     +++                  ~~~~~~~~~
+ help: or try being explicit if you are sure, that you want to clone a reference
+    |
+ LL |     let non_clone_type_ref_clone: &PlainType<u32> = <&PlainType<u32>>::clone(non_clone_type_ref);
+ 
+ 
1 warning: call to `.clone()` on a reference in this situation does nothing
2   --> $DIR/noop-method-call.rs:16:71

11 LL | #![warn(noop_method_call)]
12    |         ^^^^^^^^^^^^^^^^
13 
13 
+ warning: using `clone` on a double-reference, which copies the reference of type `CloneType<u32>` instead of cloning the inner type
+   --> $DIR/noop-method-call.rs:25:49
+    |
+ LL |     let clone_type_ref_clone: &CloneType<u32> = clone_type_ref.clone();
+    |
+ help: try dereferencing it
+    |
+    |
+ LL |     let clone_type_ref_clone: &CloneType<u32> = &(*clone_type_ref).clone();
+    |                                                 +++              ~~~~~~~~~
+ help: or try being explicit if you are sure, that you want to clone a reference
+    |
+ LL |     let clone_type_ref_clone: &CloneType<u32> = <&CloneType<u32>>::clone(clone_type_ref);
+ 
+ 
14 warning: call to `.deref()` on a reference in this situation does nothing
15   --> $DIR/noop-method-call.rs:28:63

27    |
27    |
28    = note: the type `&PlainType<u32>` which `borrow` is being called on is the same as the type returned from `borrow`, so the method call does not do anything and can be removed
29 
+ warning: using `clone` on a double-reference, which copies the reference of type `str` instead of cloning the inner type
+   --> $DIR/noop-method-call.rs:44:43
+    |
+ LL |     let _v: Vec<&str> = xs.iter().map(|x| x.clone()).collect(); // ok, but could use `*x` instead
+    |
+ help: try dereferencing it
+    |
+    |
+ LL |     let _v: Vec<&str> = xs.iter().map(|x| &(*x).clone()).collect(); // ok, but could use `*x` instead
+    |                                           +++ ~~~~~~~~~
+ help: or try being explicit if you are sure, that you want to clone a reference
+    |
+ LL |     let _v: Vec<&str> = xs.iter().map(|x| <&str>::clone(x)).collect(); // ok, but could use `*x` instead
+ 
+ 
+ warning: using `clone` on a double-reference, which copies the reference of type `PlainType<T>` instead of cloning the inner type
+   --> $DIR/noop-method-call.rs:48:5
+ LL |     non_clone_type.clone();
+    |     ^^^^^^^^^^^^^^^^^^^^^^
+    |
+ help: try dereferencing it
+ help: try dereferencing it
+    |
+ LL |     &(*non_clone_type).clone();
+    |     +++              ~~~~~~~~~
+ help: or try being explicit if you are sure, that you want to clone a reference
+    |
+ LL |     <&PlainType<T>>::clone(non_clone_type);
+ 
+ 
30 warning: call to `.clone()` on a reference in this situation does nothing
31   --> $DIR/noop-method-call.rs:48:19

35    |
35    |
36    = note: the type `&PlainType<T>` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed
37 
+ warning: using `clone` on a double-reference, which copies the reference of type `PlainType<u32>` instead of cloning the inner type
+   --> $DIR/noop-method-call.rs:53:5
+ LL |     non_clone_type.clone();
+    |     ^^^^^^^^^^^^^^^^^^^^^^
+    |
+ help: try dereferencing it
+ help: try dereferencing it
+    |
+ LL |     &(*non_clone_type).clone();
+    |     +++              ~~~~~~~~~
+ help: or try being explicit if you are sure, that you want to clone a reference
+    |
+ LL |     <&PlainType<u32>>::clone(non_clone_type);
+ 
+ 
38 warning: call to `.clone()` on a reference in this situation does nothing
39   --> $DIR/noop-method-call.rs:53:19

43    |
43    |
44    = note: the type `&PlainType<u32>` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed
- warning: 5 warnings emitted
+ warning: 10 warnings emitted
47 
48 
---
To only update this specific test, also pass `--test-args lint/noop-method-call.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/noop-method-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/noop-method-call" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/noop-method-call/auxiliary"
stdout: none
--- stderr -------------------------------
warning: using `clone` on a double-reference, which copies the reference of type `PlainType<u32>` instead of cloning the inner type
  --> fake-test-src-base/lint/noop-method-call.rs:16:53
   |
LL |     let non_clone_type_ref_clone: &PlainType<u32> = non_clone_type_ref.clone();
   |
   |
   = note: `#[warn(clone_double_ref)]` on by default
help: try dereferencing it
   |
LL |     let non_clone_type_ref_clone: &PlainType<u32> = &(*non_clone_type_ref).clone();
help: or try being explicit if you are sure, that you want to clone a reference
   |
   |
LL |     let non_clone_type_ref_clone: &PlainType<u32> = <&PlainType<u32>>::clone(non_clone_type_ref);


warning: call to `.clone()` on a reference in this situation does nothing
  --> fake-test-src-base/lint/noop-method-call.rs:16:71
   |
LL |     let non_clone_type_ref_clone: &PlainType<u32> = non_clone_type_ref.clone();
   |                                                                       ^^^^^^^^ unnecessary method call
   |
   = note: the type `&PlainType<u32>` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed
  --> fake-test-src-base/lint/noop-method-call.rs:4:9
   |
LL | #![warn(noop_method_call)]
   |         ^^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^^

warning: using `clone` on a double-reference, which copies the reference of type `CloneType<u32>` instead of cloning the inner type
  --> fake-test-src-base/lint/noop-method-call.rs:25:49
   |
LL |     let clone_type_ref_clone: &CloneType<u32> = clone_type_ref.clone();
   |
help: try dereferencing it
   |
   |
LL |     let clone_type_ref_clone: &CloneType<u32> = &(*clone_type_ref).clone();
help: or try being explicit if you are sure, that you want to clone a reference
   |
   |
LL |     let clone_type_ref_clone: &CloneType<u32> = <&CloneType<u32>>::clone(clone_type_ref);


warning: call to `.deref()` on a reference in this situation does nothing
  --> fake-test-src-base/lint/noop-method-call.rs:28:63
   |
LL |     let non_deref_type_deref: &PlainType<u32> = non_deref_type.deref();
   |                                                               ^^^^^^^^ unnecessary method call
   |
   = note: the type `&PlainType<u32>` which `deref` is being called on is the same as the type returned from `deref`, so the method call does not do anything and can be removed

warning: call to `.borrow()` on a reference in this situation does nothing
  --> fake-test-src-base/lint/noop-method-call.rs:36:66
   |
LL |     let non_borrow_type_borrow: &PlainType<u32> = non_borrow_type.borrow();
   |                                                                  ^^^^^^^^^ unnecessary method call
   |
   = note: the type `&PlainType<u32>` which `borrow` is being called on is the same as the type returned from `borrow`, so the method call does not do anything and can be removed

warning: using `clone` on a double-reference, which copies the reference of type `str` instead of cloning the inner type
  --> fake-test-src-base/lint/noop-method-call.rs:44:43
   |
LL |     let _v: Vec<&str> = xs.iter().map(|x| x.clone()).collect(); // ok, but could use `*x` instead
   |
help: try dereferencing it
   |
   |
LL |     let _v: Vec<&str> = xs.iter().map(|x| &(*x).clone()).collect(); // ok, but could use `*x` instead
   |                                           +++ ~~~~~~~~~
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |     let _v: Vec<&str> = xs.iter().map(|x| <&str>::clone(x)).collect(); // ok, but could use `*x` instead


warning: using `clone` on a double-reference, which copies the reference of type `PlainType<T>` instead of cloning the inner type
  --> fake-test-src-base/lint/noop-method-call.rs:48:5
LL |     non_clone_type.clone();
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
help: try dereferencing it
help: try dereferencing it
   |
LL |     &(*non_clone_type).clone();
help: or try being explicit if you are sure, that you want to clone a reference
   |
   |
LL |     <&PlainType<T>>::clone(non_clone_type);


warning: call to `.clone()` on a reference in this situation does nothing
  --> fake-test-src-base/lint/noop-method-call.rs:48:19
LL |     non_clone_type.clone();
   |                   ^^^^^^^^ unnecessary method call
   |
   |
   = note: the type `&PlainType<T>` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed

warning: using `clone` on a double-reference, which copies the reference of type `PlainType<u32>` instead of cloning the inner type
  --> fake-test-src-base/lint/noop-method-call.rs:53:5
LL |     non_clone_type.clone();
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
help: try dereferencing it
help: try dereferencing it
   |
LL |     &(*non_clone_type).clone();
help: or try being explicit if you are sure, that you want to clone a reference
   |
   |
LL |     <&PlainType<u32>>::clone(non_clone_type);


warning: call to `.clone()` on a reference in this situation does nothing
  --> fake-test-src-base/lint/noop-method-call.rs:53:19
LL |     non_clone_type.clone();
   |                   ^^^^^^^^ unnecessary method call
   |
   |
   = note: the type `&PlainType<u32>` which `clone` is being called on is the same as the type returned from `clone`, so the method call does not do anything and can be removed
warning: 10 warnings emitted
------------------------------------------



---- [ui] tests/ui/trait-bounds/issue-94680.rs stdout ----
normalized stderr:
warning: using `clone` on a double-reference, which copies the reference of type `()` instead of cloning the inner type
   |
   |
LL |             (it.clone(), it)
   |
   |
   = note: `#[warn(clone_double_ref)]` on by default
help: try dereferencing it
   |
LL |             (&(*it).clone(), it)
   |              +++  ~~~~~~~~~
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |             (<&()>::clone(it), it)
   |              +++++++++++++  ~
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args trait-bounds/issue-94680.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/trait-bounds/issue-94680.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/issue-94680" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/issue-94680/auxiliary"
stdout: none
--- stderr -------------------------------
warning: using `clone` on a double-reference, which copies the reference of type `()` instead of cloning the inner type
  --> fake-test-src-base/trait-bounds/issue-94680.rs:11:14
   |
LL |             (it.clone(), it)
   |
   |
   = note: `#[warn(clone_double_ref)]` on by default
help: try dereferencing it
   |
LL |             (&(*it).clone(), it)
   |              +++  ~~~~~~~~~
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |             (<&()>::clone(it), it)
   |              +++++++++++++  ~
warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/trivial-bounds/issue-73021-impossible-inline.rs#inline stdout ----
diff of stderr:

6    |
7    = note: `#[warn(trivial_bounds)]` on by default
8 
+ warning: using `clone` on a double-reference, which copies the reference of type `()` instead of cloning the inner type
+    |
+ LL |     it.clone()
+    |     ^^^^^^^^^^
+    |
+    |
+    = note: `#[warn(clone_double_ref)]` on by default
+ help: try dereferencing it
+    |
+ LL |     &(*it).clone()
+    |     +++  ~~~~~~~~~
+ help: or try being explicit if you are sure, that you want to clone a reference
+    |
+ LL |     <&()>::clone(it)
+    |     +++++++++++++  ~
+ 
9 warning: trait bound i32: Foo does not depend on any type or lifetime parameters
11    |

42 LL |     i32: Iterator,
43    |          ^^^^^^^^
---
47 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/issue-73021-impossible-inline.inline/issue-73021-impossible-inline.inline.stderr
To only update this specific test, also pass `--test-args trivial-bounds/issue-73021-impossible-inline.rs`


error in revision `inline`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/trivial-bounds/issue-73021-impossible-inline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "inline" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/issue-73021-impossible-inline.inline" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/issue-73021-impossible-inline.inline/auxiliary" "-Zmir-opt-level=3" "--emit=mir"
stdout: none
--- stderr -------------------------------
warning: trait bound for<'any> &'any mut (): Clone does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:20:29
   |
LL |     for<'any> &'any mut (): Clone,
   |
   |
   = note: `#[warn(trivial_bounds)]` on by default

warning: using `clone` on a double-reference, which copies the reference of type `()` instead of cloning the inner type
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:23:5
LL |     it.clone()
   |     ^^^^^^^^^^
   |
   |
   = note: `#[warn(clone_double_ref)]` on by default
help: try dereferencing it
   |
LL |     &(*it).clone()
   |     +++  ~~~~~~~~~
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |     <&()>::clone(it)
   |     +++++++++++++  ~

warning: trait bound i32: Foo does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:28:21
   |
LL | struct S where i32: Foo;


warning: trait bound i32: Foo does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:31:28
   |
LL | impl Foo for () where i32: Foo {


warning: trait bound i32: Foo does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:40:19
   |
LL | fn f() where i32: Foo {


warning: trait bound &'static str: Foo does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:48:28
   |
LL | fn g() where &'static str: Foo {


warning: trait bound String: Neg does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:57:13
   |
LL |     String: ::std::ops::Neg<Output = String>,


warning: trait bound i32: Iterator does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:65:10
LL |     i32: Iterator,
   |          ^^^^^^^^

warning: 8 warnings emitted
warning: 8 warnings emitted
------------------------------------------


---- [ui] tests/ui/trivial-bounds/issue-73021-impossible-inline.rs#no-opt stdout ----
diff of stderr:

6    |
7    = note: `#[warn(trivial_bounds)]` on by default
8 
+ warning: using `clone` on a double-reference, which copies the reference of type `()` instead of cloning the inner type
+    |
+ LL |     it.clone()
+    |     ^^^^^^^^^^
+    |
+    |
+    = note: `#[warn(clone_double_ref)]` on by default
+ help: try dereferencing it
+    |
+ LL |     &(*it).clone()
+    |     +++  ~~~~~~~~~
+ help: or try being explicit if you are sure, that you want to clone a reference
+    |
+ LL |     <&()>::clone(it)
+    |     +++++++++++++  ~
+ 
9 warning: trait bound i32: Foo does not depend on any type or lifetime parameters
11    |

42 LL |     i32: Iterator,
43    |          ^^^^^^^^
---
47 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/issue-73021-impossible-inline.no-opt/issue-73021-impossible-inline.no-opt.stderr
To only update this specific test, also pass `--test-args trivial-bounds/issue-73021-impossible-inline.rs`


error in revision `no-opt`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/trivial-bounds/issue-73021-impossible-inline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "no_opt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/issue-73021-impossible-inline.no-opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/issue-73021-impossible-inline.no-opt/auxiliary"
stdout: none
--- stderr -------------------------------
warning: trait bound for<'any> &'any mut (): Clone does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:20:29
   |
LL |     for<'any> &'any mut (): Clone,
   |
   |
   = note: `#[warn(trivial_bounds)]` on by default

warning: using `clone` on a double-reference, which copies the reference of type `()` instead of cloning the inner type
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:23:5
LL |     it.clone()
   |     ^^^^^^^^^^
   |
   |
   = note: `#[warn(clone_double_ref)]` on by default
help: try dereferencing it
   |
LL |     &(*it).clone()
   |     +++  ~~~~~~~~~
help: or try being explicit if you are sure, that you want to clone a reference
   |
LL |     <&()>::clone(it)
   |     +++++++++++++  ~

warning: trait bound i32: Foo does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:28:21
   |
LL | struct S where i32: Foo;


warning: trait bound i32: Foo does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:31:28
   |
LL | impl Foo for () where i32: Foo {


warning: trait bound i32: Foo does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:40:19
   |
LL | fn f() where i32: Foo {


warning: trait bound &'static str: Foo does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:48:28
   |
LL | fn g() where &'static str: Foo {


warning: trait bound String: Neg does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:57:13
   |
LL |     String: ::std::ops::Neg<Output = String>,


warning: trait bound i32: Iterator does not depend on any type or lifetime parameters
  --> fake-test-src-base/trivial-bounds/issue-73021-impossible-inline.rs:65:10
LL |     i32: Iterator,
   |          ^^^^^^^^

warning: 8 warnings emitted
