plain
........................................................................................ 4488/13651
........................................................................................ 4576/13651
.......................................................i................................ 4664/13651
........................................................................................ 4752/13651
...................................F...............................F.................... 4840/13651
........................................................................................ 5016/13651
........................................................................................ 5104/13651
........................................................................................ 5192/13651
.....................................................................................i.. 5280/13651
---
......................................................................ii................ 7920/13651
........................................................................................ 8008/13651
........................................................................................ 8096/13651
........................................................................................ 8184/13651
...ii................i..F...i..ii....................................................... 8272/13651
........................................................................................ 8448/13651
........................................................................................ 8536/13651
........................................................................................ 8624/13651
..................................................i...ii................................ 8712/13651
---
........................................................................................ 9680/13651
........................................................................................ 9768/13651
........................................................................................ 9856/13651
........................................................................................ 9944/13651
......................................................F......................F.......... 10032/13651
........................................................................................ 10208/13651
........................................................................................ 10296/13651
........................................................................................ 10384/13651
........................................................................................ 10472/13651
---

---- [ui] src/test/ui/empty/empty-macro-use.rs stdout ----
diff of stderr:

4 LL |     macro_two!();
6    |
-    = note: consider importing this macro:
-            two_macros::macro_two
+    = help: consider importing this macro:
---
To only update this specific test, also pass `--test-args empty/empty-macro-use.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-macro-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-macro-use" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-macro-use/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot find macro `macro_two` in this scope
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     macro_two!();
   |
   = help: consider importing this macro:
           use two_macros::macro_two;


error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/hygiene/globs.rs stdout ----
diff of stderr:

37 LL |         n!(f);
39    |
-    = note: consider importing this function:
-            foo::f
+    = help: consider importing this function:
---
To only update this specific test, also pass `--test-args hygiene/globs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/globs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/globs" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/globs/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find function `f` in this scope
   |
   |
LL |         f(); //~ ERROR cannot find function `f` in this scope
   |
help: consider importing this function
   |
LL | use foo::f;
LL | use foo::f;
   |

error[E0425]: cannot find function `g` in this scope
  --> /checkout/src/test/ui/hygiene/globs.rs:15:5
   |
LL |       g(); //~ ERROR cannot find function `g` in this scope
...
LL | /     m! {
LL | /     m! {
LL | |         use bar::*;
LL | |         g();
LL | |         f(); //~ ERROR cannot find function `f` in this scope
LL | |     }
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this function
   |
   |
LL | use bar::g;

error[E0425]: cannot find function `f` in this scope
  --> /checkout/src/test/ui/hygiene/globs.rs:61:12
   |
   |
LL | n!(f);
...
...
LL |         n!(f); //~ ERROR cannot find function `f` in this scope
   |
   = help: consider importing this function:
           use foo::f;
   = note: this error originates in the macro `n` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the macro `n` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0425]: cannot find function `f` in this scope
  --> /checkout/src/test/ui/hygiene/globs.rs:65:17
   |
LL | n!(f);
...
...
LL |                 f //~ ERROR cannot find function `f` in this scope
   |
   = help: consider importing this function:
           use foo::f;
   = note: this error originates in the macro `n` (in Nightly builds, run with -Z macro-backtrace for more info)
---
To only update this specific test, also pass `--test-args hygiene/no_implicit_prelude-2018.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/no_implicit_prelude-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude-2018" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude-2018/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/hygiene/no_implicit_prelude-2018.rs:7:9
   |
   |
LL |         print!(); //~ ERROR cannot find macro `print` in this scope
   |
   = help: consider importing this macro:
           use std::print;

---
error: /checkout/src/test/ui/macros/issue-102601.rs:12: expected help message not found: consider importing this macro:

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-102601.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-102601" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-102601/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Help,
---

---- [ui] src/test/ui/macros/issue-88228.rs stdout ----
diff of stderr:

4 LL |     bla!();
6    |
-    = note: consider importing this macro:
-            crate::hey::bla
+    = help: consider importing this macro:
+    = help: consider importing this macro:
+            use crate::hey::bla;
10 error: cannot find derive macro `println` in this scope
11   --> $DIR/issue-88228.rs:14:10


21 LL | #[derive(Bla)]
23    |
-    = note: consider importing this derive macro:
-            crate::hey::Bla
+    = help: consider importing this derive macro:
+    = help: consider importing this derive macro:
+            use crate::hey::Bla;
27 error: aborting due to 3 previous errors
28 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-88228/issue-88228.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/issue-88228.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-88228.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-88228" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "deduplicate-diagnostics=yes" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-88228/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot find macro `bla` in this scope
   |
   |
LL |     bla!();
   |
   = help: consider importing this macro:
   = help: consider importing this macro:
           use crate::hey::bla;
error: cannot find derive macro `println` in this scope
  --> /checkout/src/test/ui/macros/issue-88228.rs:14:10
   |
   |
LL | #[derive(println)]
   |
   = note: `println` is in scope, but it is a function-like macro


error: cannot find derive macro `Bla` in this scope
   |
   |
LL | #[derive(Bla)]
   |
   = help: consider importing this derive macro:
   = help: consider importing this derive macro:
           use crate::hey::Bla;
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/macros/macro-use-wrong-name.rs stdout ----
diff of stderr:

9 LL | macro_rules! macro_one { () => ("one") }
10    | ---------------------- similarly named macro `macro_one` defined here
-    = note: consider importing this macro:
-            two_macros::macro_two
+    = help: consider importing this macro:
+            use two_macros::macro_two;
---
To only update this specific test, also pass `--test-args macros/macro-use-wrong-name.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-use-wrong-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-use-wrong-name" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-use-wrong-name/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot find macro `macro_two` in this scope
   |
   |
LL |     macro_two!();
   |     ^^^^^^^^^ help: a macro with a similar name exists: `macro_one`
  ::: /checkout/src/test/ui/macros/auxiliary/two_macros.rs:2:1
   |
   |
LL | macro_rules! macro_one { () => ("one") }
   | ---------------------- similarly named macro `macro_one` defined here
   = help: consider importing this macro:
           use two_macros::macro_two;

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/missing/missing-macro-use.rs stdout ----
diff of stderr:

4 LL |     macro_two!();
6    |
-    = note: consider importing this macro:
-            two_macros::macro_two
+    = help: consider importing this macro:
---
To only update this specific test, also pass `--test-args missing/missing-macro-use.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing/missing-macro-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-macro-use" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-macro-use/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot find macro `macro_two` in this scope
   |
   |
LL |     macro_two!();
   |
   = help: consider importing this macro:
           use two_macros::macro_two;


error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/proc-macro/derive-helper-shadowing.rs stdout ----
diff of stderr:

16 LL |             #[derive(GenHelperUse)]
18    |
-    = note: consider importing this attribute macro:
-            empty_helper
+    = help: consider importing this attribute macro:
+    = help: consider importing this attribute macro:
+            use empty_helper;
21    = note: this error originates in the derive macro `GenHelperUse` (in Nightly builds, run with -Z macro-backtrace for more info)
22 
23 error: cannot find attribute `empty_helper` in this scope

29 LL |             gen_helper_use!();
31    |
-    = note: consider importing this attribute macro:
-            crate::empty_helper
+    = help: consider importing this attribute macro:
+    = help: consider importing this attribute macro:
+            use crate::empty_helper;
34    = note: this error originates in the macro `gen_helper_use` (in Nightly builds, run with -Z macro-backtrace for more info)
35 
36 error[E0659]: `empty_helper` is ambiguous

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-helper-shadowing/derive-helper-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/derive-helper-shadowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-helper-shadowing" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-helper-shadowing/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs:42:15
   |
   |
LL |             #[renamed] //~ ERROR cannot use a derive helper attribute through an import
   |
note: the derive helper attribute imported here
  --> /checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs:41:17
   |
   |
LL |             use empty_helper as renamed;
   |                 ^^^^^^^^^^^^^^^^^^^^^^^

error: cannot find attribute `empty_helper` in this scope
   |
   |
LL |             #[derive(GenHelperUse)] //~ ERROR cannot find attribute `empty_helper` in this scope
   |
   = help: consider importing this attribute macro:
           use empty_helper;
           use empty_helper;
   = note: this error originates in the derive macro `GenHelperUse` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find attribute `empty_helper` in this scope
   |
   |
LL |         #[empty_helper] //~ ERROR cannot find attribute `empty_helper` in this scope
...
...
LL |             gen_helper_use!();
   |
   = help: consider importing this attribute macro:
           use crate::empty_helper;
           use crate::empty_helper;
   = note: this error originates in the macro `gen_helper_use` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `empty_helper` is ambiguous
   |
   |
LL |         use empty_helper; //~ ERROR `empty_helper` is ambiguous
   |             ^^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
note: `empty_helper` could refer to the derive helper attribute defined here
   |
   |
LL | #[derive(Empty)]
   |          ^^^^^
note: `empty_helper` could also refer to the attribute macro imported here
   |
   |
LL | use test_macros::empty_attr as empty_helper;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `crate::empty_helper` to refer to this attribute macro unambiguously

error[E0659]: `empty_helper` is ambiguous
   |
   |
LL | #[empty_helper] //~ ERROR `empty_helper` is ambiguous
   |   ^^^^^^^^^^^^ ambiguous name
   = note: ambiguous because of a name conflict with a derive helper attribute
   = note: ambiguous because of a name conflict with a derive helper attribute
note: `empty_helper` could refer to the derive helper attribute defined here
   |
   |
LL | #[derive(Empty)]
   |          ^^^^^
note: `empty_helper` could also refer to the attribute macro imported here
   |
   |
LL | use test_macros::empty_attr as empty_helper;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `crate::empty_helper` to refer to this attribute macro unambiguously
warning: derive helper attribute is used before it is introduced
  --> /checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs:19:3
   |
   |
LL | #[empty_helper] //~ ERROR `empty_helper` is ambiguous
...
...
LL | #[derive(Empty)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79202 <https://github.com/rust-lang/rust/issues/79202>
   = note: `#[warn(legacy_derive_helpers)]` on by default
---

---- [ui] src/test/ui/proc-macro/generate-mod.rs stdout ----
diff of stderr:

4 LL | generate_mod::check!();
6    |
-    = note: consider importing this struct:
-            FromOutside
+    = help: consider importing this struct:
+    = help: consider importing this struct:
+            use FromOutside;
9    = note: this error originates in the macro `generate_mod::check` (in Nightly builds, run with -Z macro-backtrace for more info)
11 error[E0412]: cannot find type `Outer` in this scope


14 LL | generate_mod::check!();
16    |
-    = note: consider importing this struct:
-            Outer
+    = help: consider importing this struct:
+    = help: consider importing this struct:
+            use Outer;
19    = note: this error originates in the macro `generate_mod::check` (in Nightly builds, run with -Z macro-backtrace for more info)
20 
21 error[E0412]: cannot find type `FromOutside` in this scope

24 LL | #[generate_mod::check_attr]
26    |
-    = note: consider importing this struct:
-            FromOutside
+    = help: consider importing this struct:
+    = help: consider importing this struct:
+            use FromOutside;
29    = note: this error originates in the attribute macro `generate_mod::check_attr` (in Nightly builds, run with -Z macro-backtrace for more info)
31 error[E0412]: cannot find type `OuterAttr` in this scope


34 LL | #[generate_mod::check_attr]
36    |
-    = note: consider importing this struct:
-            OuterAttr
+    = help: consider importing this struct:
+    = help: consider importing this struct:
+            use OuterAttr;
39    = note: this error originates in the attribute macro `generate_mod::check_attr` (in Nightly builds, run with -Z macro-backtrace for more info)
40 
41 error: cannot find type `FromOutside` in this scope

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod/generate-mod.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/generate-mod.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/generate-mod.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0412]: cannot find type `FromOutside` in this scope
   |
   |
LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
   |
   = help: consider importing this struct:
           use FromOutside;
           use FromOutside;
   = note: this error originates in the macro `generate_mod::check` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0412]: cannot find type `Outer` in this scope
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:9:1
   |
   |
LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
   |
   = help: consider importing this struct:
           use Outer;
           use Outer;
   = note: this error originates in the macro `generate_mod::check` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0412]: cannot find type `FromOutside` in this scope
   |
   |
LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
   |
   = help: consider importing this struct:
           use FromOutside;
           use FromOutside;
   = note: this error originates in the attribute macro `generate_mod::check_attr` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0412]: cannot find type `OuterAttr` in this scope
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:12:1
   |
   |
LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
   |
   = help: consider importing this struct:
           use OuterAttr;
           use OuterAttr;
   = note: this error originates in the attribute macro `generate_mod::check_attr` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find type `FromOutside` in this scope
   |
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
error: cannot find type `OuterDerive` in this scope
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:16:10
   |
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find type `FromOutside` in this scope
   |
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
error: cannot find type `OuterDerive` in this scope
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:23:14
   |
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0412`.
Future incompatibility report: Future breakage diagnostic:
Future incompatibility report: Future breakage diagnostic:
error: cannot find type `FromOutside` in this scope
   |
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
error: cannot find type `OuterDerive` in this scope
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:16:10
   |
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
error: cannot find type `FromOutside` in this scope
   |
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
error: cannot find type `OuterDerive` in this scope
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:23:14
   |
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
warning: cannot find type `FromOutside` in this scope
   |
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
note: the lint level is defined here
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:30:10
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:30:10
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this warning originates in the derive macro `generate_mod::CheckDeriveLint` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
warning: cannot find type `OuterDeriveLint` in this scope
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:30:10
   |
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
note: the lint level is defined here
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:30:10
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:30:10
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this warning originates in the derive macro `generate_mod::CheckDeriveLint` (in Nightly builds, run with -Z macro-backtrace for more info)



failures:
