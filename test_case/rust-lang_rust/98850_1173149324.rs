plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.....................................FF.F..F..F..F....F......FF.F......

---- [ui] src/test/ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
diff of stderr:


9 error: item is named 'lintme'
11    |
11    |
- LL | fn lintme() { }
-    | ^^^^^^^^^^^^^^^
+ LL | fn lintme() {}
14    |
15 note: the lint level is defined here
16   --> $DIR/lint-plugin-deny-attr.rs:7:9

---
To only update this specific test, also pass `--test-args lint-plugin-deny-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() {} //~ ERROR item is named 'lintme'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs:7:9
   |
   |
LL | #![deny(test_lint)]

error: aborting due to previous error; 1 warning emitted
------------------------------------------



---- [ui] src/test/ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
diff of stderr:

9 warning: item is named 'lintme'
11    |
11    |
- LL | fn lintme() { }
-    | ^^^^^^^^^^^^^^^
+ LL | fn lintme() {}
14    |
14    |
15    = note: `#[warn(test_lint)]` on by default


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/lint-plugin-cmdline-load.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/lint-plugin-cmdline-load.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint-plugin-cmdline-load.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-load.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "crate-attr=plugin(lint_plugin_test)" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> <crate attribute>:1:1
   |
LL | plugin(lint_plugin_test)
   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: item is named 'lintme'
   |
   |
LL | fn lintme() {} //~ WARNING item is named 'lintme'
   |
   |
   = note: `#[warn(test_lint)]` on by default
warning: 2 warnings emitted
------------------------------------------



---- [ui] src/test/ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
diff of stderr:

1 warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> $DIR/lint-group-plugin-deny-cmdline.rs:7:1
3    |
3    |
4 LL | #![plugin(lint_group_plugin_test)]
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
7    = note: `#[warn(deprecated)]` on by default
8 
8 
9 error: item is named 'lintme'
-   --> $DIR/lint-group-plugin-deny-cmdline.rs:10:1
11    |
11    |
- LL | fn lintme() { }
-    | ^^^^^^^^^^^^^^^
+ LL | fn lintme() {}
14    |
14    |
15    = note: `-D test-lint` implied by `-D lint-me`


17 error: item is named 'pleaselintme'
-   --> $DIR/lint-group-plugin-deny-cmdline.rs:12:1
19    |
19    |
- LL | fn pleaselintme() { }
-    | ^^^^^^^^^^^^^^^^^^^^^
+ LL | fn pleaselintme() {}
22    |
22    |
23    = note: `-D please-lint` implied by `-D lint-me`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/lint-group-plugin-deny-cmdline.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/lint-group-plugin-deny-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint-group-plugin-deny-cmdline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "lint-me" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_group_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() {} //~ ERROR item is named 'lintme'
   |
   |
   = note: `-D test-lint` implied by `-D lint-me`

error: item is named 'pleaselintme'
   |
   |
LL | fn pleaselintme() {} //~ ERROR item is named 'pleaselintme'
   |
   |
   = note: `-D please-lint` implied by `-D lint-me`
error: aborting due to 2 previous errors; 1 warning emitted
------------------------------------------



---- [ui] src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
diff of stderr:

9 error: item is named 'lintme'
11    |
11    |
- LL | fn lintme() { }
-    | ^^^^^^^^^^^^^^^
+ LL | fn lintme() {}
14    |
14    |
15    = note: requested on the command line with `-D test-lint`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/lint-plugin-deny-cmdline.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/lint-plugin-deny-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint-plugin-deny-cmdline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() {} //~ ERROR item is named 'lintme'
   |
   |
   = note: requested on the command line with `-D test-lint`
error: aborting due to previous error; 1 warning emitted
------------------------------------------



---- [ui] src/test/ui-fulldeps/lint-group-plugin.rs stdout ----
diff of stderr:

9 warning: item is named 'lintme'
11    |
11    |
- LL | fn lintme() { }
-    | ^^^^^^^^^^^^^^^
+ LL | fn lintme() {}
14    |
14    |
15    = note: `#[warn(test_lint)]` on by default


17 warning: item is named 'pleaselintme'
19    |
19    |
- LL | fn pleaselintme() { }
-    | ^^^^^^^^^^^^^^^^^^^^^
+ LL | fn pleaselintme() {}
22    |
22    |
23    = note: `#[warn(please_lint)]` on by default


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/lint-group-plugin.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/lint-group-plugin.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint-group-plugin.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_group_plugin_test)] //~ WARNING use of deprecated attribute
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: item is named 'lintme'
   |
   |
LL | fn lintme() {} //~ WARNING item is named 'lintme'
   |
   |
   = note: `#[warn(test_lint)]` on by default

warning: item is named 'pleaselintme'
   |
   |
LL | fn pleaselintme() {} //~ WARNING item is named 'pleaselintme'
   |
   |
   = note: `#[warn(please_lint)]` on by default
warning: 3 warnings emitted
------------------------------------------



---- [ui] src/test/ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
diff of stderr:

1 error[E0453]: allow(test_lint) incompatible with previous forbid
-   --> $DIR/lint-plugin-forbid-cmdline.rs:10:9
3    |
3    |
4 LL | #[allow(test_lint)]
5    |         ^^^^^^^^^ overruled by previous forbid

7    = note: `forbid` lint level was set on command line
8 
9 error[E0453]: allow(test_lint) incompatible with previous forbid
-   --> $DIR/lint-plugin-forbid-cmdline.rs:10:9
11    |
11    |
12 LL | #[allow(test_lint)]
13    |         ^^^^^^^^^ overruled by previous forbid
23    = note: `#[warn(deprecated)]` on by default
24 
24 
25 error: item is named 'lintme'
-   --> $DIR/lint-plugin-forbid-cmdline.rs:8:1
27    |
27    |
- LL | fn lintme() { }
-    | ^^^^^^^^^^^^^^^
+ LL | fn lintme() {}
30    |
30    |
31    = note: requested on the command line with `-F test-lint`


33 error[E0453]: allow(test_lint) incompatible with previous forbid
-   --> $DIR/lint-plugin-forbid-cmdline.rs:10:9
35    |
35    |
36 LL | #[allow(test_lint)]
37    |         ^^^^^^^^^ overruled by previous forbid

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/lint-plugin-forbid-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint-plugin-forbid-cmdline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() {}
   |
   |
   = note: requested on the command line with `-F test-lint`

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line
error: aborting due to 4 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0453`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin.rs stdout ----
diff of stderr:

9 warning: item is named 'lintme'
11    |
11    |
- LL | fn lintme() { }
-    | ^^^^^^^^^^^^^^^
+ LL | fn lintme() {}
14    |
14    |
15    = note: `#[warn(test_lint)]` on by default


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/lint-plugin.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/lint-plugin.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint-plugin.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)] //~ WARNING use of deprecated attribute
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: item is named 'lintme'
   |
   |
LL | fn lintme() {} //~ WARNING item is named 'lintme'
   |
   |
   = note: `#[warn(test_lint)]` on by default
warning: 2 warnings emitted
------------------------------------------



---- [ui] src/test/ui-fulldeps/lint-tool-test.rs stdout ----
diff of stderr:

67 error: item is named 'lintme'
69    |
69    |
- LL | fn lintme() { }
-    | ^^^^^^^^^^^^^^^
+ LL | fn lintme() {}
72    |
73 note: the lint level is defined here
74   --> $DIR/lint-tool-test.rs:14:9


80 error: item is named 'lintmetoo'
82    |
82    |
- LL |     fn lintmetoo() { }
-    |     ^^^^^^^^^^^^^^^^^^
+ LL |     fn lintmetoo() {}
85    |
85    |
86    = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/lint-tool-test.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/lint-tool-test.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint-tool-test.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
stdout: none
--- stderr -------------------------------
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
   |
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
   = note: `#[warn(renamed_and_removed_lints)]` on by default


warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`

warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
warning: unknown lint: `this_lint_does_not_exist`
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:36:8
   |
   |
LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
   |
   = note: `#[warn(unknown_lints)]` on by default


warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_tool_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`

warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
