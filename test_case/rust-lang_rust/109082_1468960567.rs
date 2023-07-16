plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:067e5fb3939cbadf64f1a956c1ef134a6494dc9b)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 10384/14645
........................................................................................ 10472/14645
........................................................................................ 10560/14645
........................................................................................ 10648/14645
........................................F.....................................F......ii. 10736/14645
..............i.....iii......F.......................................................... 10824/14645
........................................................................................ 11000/14645
........................................................................................ 11088/14645
........................................................................................ 11176/14645
........................................................................................ 11264/14645
---

---- [ui] tests/ui/proc-macro/generate-mod.rs stdout ----
diff of stderr:

4 LL | generate_mod::check!();
5    | ^^^^^^^^^^^^^^^^^^^^^^ `FromOutside` not found in expanded code of this procedural macro
-    = help: consider importing this struct:
-            FromOutside
-            FromOutside
9    = note: this error originates in the macro `generate_mod::check` (in Nightly builds, run with -Z macro-backtrace for more info)
10 
11 error[E0412]: cannot find type `Outer` in the expanded code of procedural macro `generate_mod::check`

14 LL | generate_mod::check!();
15    | ^^^^^^^^^^^^^^^^^^^^^^ `Outer` not found in expanded code of this procedural macro
-    = help: consider importing this struct:
-            Outer
-            Outer
19    = note: this error originates in the macro `generate_mod::check` (in Nightly builds, run with -Z macro-backtrace for more info)
20 
21 error[E0412]: cannot find type `FromOutside` in the expanded code of procedural macro `generate_mod::check_attr`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod/generate-mod.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/generate-mod.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/generate-mod.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0412]: cannot find type `FromOutside` in the expanded code of procedural macro `generate_mod::check`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> fake-test-src-base/proc-macro/generate-mod.rs:9:1
   |
LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in
   | ^^^^^^^^^^^^^^^^^^^^^^ `FromOutside` not found in expanded code of this procedural macro
   |
   = note: this error originates in the macro `generate_mod::check` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0412]: cannot find type `Outer` in the expanded code of procedural macro `generate_mod::check`
  --> fake-test-src-base/proc-macro/generate-mod.rs:9:1
   |
LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in
   | ^^^^^^^^^^^^^^^^^^^^^^ `Outer` not found in expanded code of this procedural macro
   |
   = note: this error originates in the macro `generate_mod::check` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0412]: cannot find type `FromOutside` in the expanded code of procedural macro `generate_mod::check_attr`
  --> fake-test-src-base/proc-macro/generate-mod.rs:12:1
   |
LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `FromOutside` not found in expanded code of this procedural macro
   |
   = note: this error originates in the attribute macro `generate_mod::check_attr` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0412]: cannot find type `OuterAttr` in the expanded code of procedural macro `generate_mod::check_attr`
  --> fake-test-src-base/proc-macro/generate-mod.rs:12:1
   |
LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `OuterAttr` not found in expanded code of this procedural macro
   |
   = note: this error originates in the attribute macro `generate_mod::check_attr` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find type `FromOutside` in this scope
  --> fake-test-src-base/proc-macro/generate-mod.rs:16:10
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
error: cannot find type `OuterDerive` in this scope
  --> fake-test-src-base/proc-macro/generate-mod.rs:16:10
   |
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find type `FromOutside` in this scope
  --> fake-test-src-base/proc-macro/generate-mod.rs:23:14
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
error: cannot find type `OuterDerive` in this scope
  --> fake-test-src-base/proc-macro/generate-mod.rs:23:14
   |
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in
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
  --> fake-test-src-base/proc-macro/generate-mod.rs:16:10
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
error: cannot find type `OuterDerive` in this scope
  --> fake-test-src-base/proc-macro/generate-mod.rs:16:10
   |
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
error: cannot find type `FromOutside` in this scope
  --> fake-test-src-base/proc-macro/generate-mod.rs:23:14
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
error: cannot find type `OuterDerive` in this scope
  --> fake-test-src-base/proc-macro/generate-mod.rs:23:14
   |
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
warning: cannot find type `FromOutside` in this scope
  --> fake-test-src-base/proc-macro/generate-mod.rs:30:10
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
note: the lint level is defined here
  --> fake-test-src-base/proc-macro/generate-mod.rs:30:10
  --> fake-test-src-base/proc-macro/generate-mod.rs:30:10
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this warning originates in the derive macro `generate_mod::CheckDeriveLint` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
warning: cannot find type `OuterDeriveLint` in this scope
  --> fake-test-src-base/proc-macro/generate-mod.rs:30:10
   |
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
note: the lint level is defined here
  --> fake-test-src-base/proc-macro/generate-mod.rs:30:10
  --> fake-test-src-base/proc-macro/generate-mod.rs:30:10
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this warning originates in the derive macro `generate_mod::CheckDeriveLint` (in Nightly builds, run with -Z macro-backtrace for more info)


---- [ui] tests/ui/proc-macro/lints_in_proc_macros.rs stdout ----
diff of stderr:
diff of stderr:

2   --> $DIR/lints_in_proc_macros.rs:9:5
3    |
4 LL |     bang_proc_macro2!();
-    |     ^^^^^^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `foobar`
+    |     ^^^^^^^^^^^^^^^^^^^ `foobar2` not found in expanded code of this procedural macro
7    = note: this error originates in the macro `bang_proc_macro2` (in Nightly builds, run with -Z macro-backtrace for more info)
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/lints_in_proc_macros/lints_in_proc_macros.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/lints_in_proc_macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/lints_in_proc_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/lints_in_proc_macros" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/lints_in_proc_macros/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `foobar2` in the expanded code of procedural macro `bang_proc_macro2`
  --> fake-test-src-base/proc-macro/lints_in_proc_macros.rs:9:5
   |
LL |     bang_proc_macro2!();
   |     ^^^^^^^^^^^^^^^^^^^ `foobar2` not found in expanded code of this procedural macro
   = note: this error originates in the macro `bang_proc_macro2` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

140   --> $DIR/parent-source-spans.rs:29:5
141    |
142 LL |     parent_source_spans!($($tokens)*);
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a tuple variant with a similar name exists: `Ok`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ok` not found in expanded code of this procedural macro
144 ...
145 LL |     one!("hello", "world");

-   --> $SRC_DIR/core/src/result.rs:LL:COL
148    |
148    |
-    = note: similarly named tuple variant `Ok` defined here
-    |
151    = note: this error originates in the macro `parent_source_spans` which comes from the expansion of the macro `one` (in Nightly builds, run with -Z macro-backtrace for more info)
152 
153 error[E0425]: cannot find value `ok` in the expanded code of procedural macro `parent_source_spans`
154   --> $DIR/parent-source-spans.rs:29:5
155    |
156 LL |     parent_source_spans!($($tokens)*);
156 LL |     parent_source_spans!($($tokens)*);
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a tuple variant with a similar name exists: `Ok`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ok` not found in expanded code of this procedural macro
158 ...
159 LL |     two!("yay", "rust");

-   --> $SRC_DIR/core/src/result.rs:LL:COL
162    |
162    |
-    = note: similarly named tuple variant `Ok` defined here
-    |
165    = note: this error originates in the macro `parent_source_spans` which comes from the expansion of the macro `two` (in Nightly builds, run with -Z macro-backtrace for more info)
166 
167 error[E0425]: cannot find value `ok` in the expanded code of procedural macro `parent_source_spans`
168   --> $DIR/parent-source-spans.rs:29:5
169    |
170 LL |     parent_source_spans!($($tokens)*);
170 LL |     parent_source_spans!($($tokens)*);
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a tuple variant with a similar name exists: `Ok`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ok` not found in expanded code of this procedural macro
172 ...
173 LL |     three!("hip", "hop");

-   --> $SRC_DIR/core/src/result.rs:LL:COL
-    |
-    |
-    = note: similarly named tuple variant `Ok` defined here
178    |
179    = note: this error originates in the macro `parent_source_spans` which comes from the expansion of the macro `three` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/parent-source-spans/parent-source-spans.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/parent-source-spans/parent-source-spans.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/parent-source-spans.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/parent-source-spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/parent-source-spans" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/parent-source-spans/auxiliary"
stdout: none
--- stderr -------------------------------
error: first final: "hello"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:16:12
   |
LL |     three!($a, $b);
...
...
LL |     one!("hello", "world");
   |
   |
   = note: this error originates in the macro `two` which comes from the expansion of the macro `one` (in Nightly builds, run with -Z macro-backtrace for more info)

error: second final: "world"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:16:16
   |
LL |     three!($a, $b);
...
...
LL |     one!("hello", "world");
   |
   |
   = note: this error originates in the macro `two` which comes from the expansion of the macro `one` (in Nightly builds, run with -Z macro-backtrace for more info)
error: first parent: "hello"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:10:5
   |
   |
LL |     two!($a, $b);
...
...
LL |     one!("hello", "world");
   |
   = note: this error originates in the macro `one` (in Nightly builds, run with -Z macro-backtrace for more info)

error: second parent: "world"
error: second parent: "world"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:10:5
   |
LL |     two!($a, $b);
...
...
LL |     one!("hello", "world");
   |
   = note: this error originates in the macro `one` (in Nightly builds, run with -Z macro-backtrace for more info)


error: first grandparent: "hello"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:36:5
   |
LL |     one!("hello", "world");


error: second grandparent: "world"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:36:5
   |
LL |     one!("hello", "world");

error: first source: "hello"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:36:5
   |
   |
LL |     one!("hello", "world");

error: second source: "world"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:36:5
   |
   |
LL |     one!("hello", "world");


error: first final: "yay"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:16:12
   |
LL |     three!($a, $b);
...
...
LL |     two!("yay", "rust");
   |
   |
   = note: this error originates in the macro `two` (in Nightly builds, run with -Z macro-backtrace for more info)
error: second final: "rust"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:16:16
   |
   |
LL |     three!($a, $b);
...
...
LL |     two!("yay", "rust");
   |
   |
   = note: this error originates in the macro `two` (in Nightly builds, run with -Z macro-backtrace for more info)

error: first parent: "yay"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:42:5
   |
LL |     two!("yay", "rust");

error: second parent: "rust"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:42:5
   |
   |
LL |     two!("yay", "rust");


error: first source: "yay"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:42:5
   |
LL |     two!("yay", "rust");

error: second source: "rust"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:42:5
   |
   |
LL |     two!("yay", "rust");


error: first final: "hip"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:48:12
   |
LL |     three!("hip", "hop");


error: second final: "hop"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:48:19
   |
LL |     three!("hip", "hop");


error: first source: "hip"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:48:12
   |
LL |     three!("hip", "hop");


error: second source: "hop"
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:48:19
   |
LL |     three!("hip", "hop");


error[E0425]: cannot find value `ok` in the expanded code of procedural macro `parent_source_spans`
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:29:5
LL |     parent_source_spans!($($tokens)*);
LL |     parent_source_spans!($($tokens)*);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ok` not found in expanded code of this procedural macro
...
LL |     one!("hello", "world");
   |
   |
   = note: this error originates in the macro `parent_source_spans` which comes from the expansion of the macro `one` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0425]: cannot find value `ok` in the expanded code of procedural macro `parent_source_spans`
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:29:5
LL |     parent_source_spans!($($tokens)*);
LL |     parent_source_spans!($($tokens)*);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ok` not found in expanded code of this procedural macro
...
LL |     two!("yay", "rust");
   |
   |
   = note: this error originates in the macro `parent_source_spans` which comes from the expansion of the macro `two` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0425]: cannot find value `ok` in the expanded code of procedural macro `parent_source_spans`
  --> fake-test-src-base/proc-macro/parent-source-spans.rs:29:5
LL |     parent_source_spans!($($tokens)*);
LL |     parent_source_spans!($($tokens)*);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ok` not found in expanded code of this procedural macro
...
LL |     three!("hip", "hop");
   |
   |
   = note: this error originates in the macro `parent_source_spans` which comes from the expansion of the macro `three` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 21 previous errors

For more information about this error, try `rustc --explain E0425`.
------------------------------------------
