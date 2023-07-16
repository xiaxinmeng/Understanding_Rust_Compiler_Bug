plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.......................i.............F...F.F.FF..F......F.......F......

---- [ui] src/test/ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
diff of stderr:


- warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> $DIR/lint-plugin-deny-attr.rs:5:1
-    |
- LL | #![plugin(lint_plugin_test)]
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
-    = note: `#[warn(deprecated)]` on by default
- 
- 
9 error: item is named 'lintme'
11    |

17    |
18 LL | #![deny(test_lint)]
18 LL | #![deny(test_lint)]
19    |         ^^^^^^^^^
+ 
+ warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
+    |
+    |
+ LL | #![plugin(lint_plugin_test)]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
+    = note: `#[warn(deprecated)]` on by default
20 
21 error: aborting due to previous error; 1 warning emitted
22 
---
To only update this specific test, also pass `--test-args lint-plugin-deny-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
stdout: none
--- stderr -------------------------------
error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs:7:9
   |
   |
LL | #![deny(test_lint)]
   |         ^^^^^^^^^

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
diff of stderr:

- warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> <crate attribute>:1:1
-    |
- LL | plugin(lint_plugin_test)
-    | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
-    = note: `#[warn(deprecated)]` on by default
- 
- 
9 warning: item is named 'lintme'
11    |

13    | ^^^^^^^^^^^^^^^
14    |
14    |
15    = note: `#[warn(test_lint)]` on by default
+ 
+ warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
+   --> <crate attribute>:1:1
+    |
+ LL | plugin(lint_plugin_test)
+    | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
+    = note: `#[warn(deprecated)]` on by default
16 
17 warning: 2 warnings emitted
18 
---
To only update this specific test, also pass `--test-args lint-plugin-cmdline-load.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-load.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Z" "crate-attr=plugin(lint_plugin_test)"
stdout: none
--- stderr -------------------------------
warning: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ WARNING item is named 'lintme'
   |
   |
   = note: `#[warn(test_lint)]` on by default

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> <crate attribute>:1:1
   |
LL | plugin(lint_plugin_test)
   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 2 warnings emitted
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
diff of stderr:

6    |
7    = note: `forbid` lint level was set on command line
8 
- error[E0453]: allow(test_lint) incompatible with previous forbid
-   --> $DIR/lint-plugin-forbid-cmdline.rs:10:9
-    |
- LL | #[allow(test_lint)]
-    |         ^^^^^^^^^ overruled by previous forbid
-    |
-    = note: `forbid` lint level was set on command line
- 
- warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> $DIR/lint-plugin-forbid-cmdline.rs:6:1
-    |
- LL | #![plugin(lint_plugin_test)]
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
-    = note: `#[warn(deprecated)]` on by default
- 
- 
25 error: item is named 'lintme'
27    |

38    |
38    |
39    = note: `forbid` lint level was set on command line
- error: aborting due to 4 previous errors; 1 warning emitted
- error: aborting due to 4 previous errors; 1 warning emitted
+ warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
+    |
+    |
+ LL | #![plugin(lint_plugin_test)]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
+    = note: `#[warn(deprecated)]` on by default
+ 
+ error: aborting due to 3 previous errors; 1 warning emitted
42 
---
To only update this specific test, also pass `--test-args lint-plugin-forbid-cmdline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-F" "test-lint"
stdout: none
--- stderr -------------------------------
error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)] //~ ERROR allow(test_lint) incompatible
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line

error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
   |
   = note: requested on the command line with `-F test-lint`

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)] //~ ERROR allow(test_lint) incompatible
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

error: aborting due to 3 previous errors; 1 warning emitted


For more information about this error, try `rustc --explain E0453`.
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
diff of stderr:

- warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> $DIR/lint-plugin-deny-cmdline.rs:6:1
-    |
- LL | #![plugin(lint_plugin_test)]
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
-    = note: `#[warn(deprecated)]` on by default
- 
- 
9 error: item is named 'lintme'
11    |

13    | ^^^^^^^^^^^^^^^
14    |
14    |
15    = note: requested on the command line with `-D test-lint`
+ 
+ warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
+    |
+    |
+ LL | #![plugin(lint_plugin_test)]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
+    = note: `#[warn(deprecated)]` on by default
16 
17 error: aborting due to previous error; 1 warning emitted
18 
---
To only update this specific test, also pass `--test-args lint-plugin-deny-cmdline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-D" "test-lint"
stdout: none
--- stderr -------------------------------
error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
   |
   = note: requested on the command line with `-D test-lint`

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
diff of stderr:

7 LL | #[allow(test_lint)]
8    |         ^^^^^^^^^ overruled by previous forbid
9 
- error[E0453]: allow(test_lint) incompatible with previous forbid
-   --> $DIR/lint-plugin-forbid-attrs.rs:11:9
-    |
- LL | #![forbid(test_lint)]
-    |           --------- `forbid` level set here
- ...
- LL | #[allow(test_lint)]
-    |         ^^^^^^^^^ overruled by previous forbid
- 
- warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> $DIR/lint-plugin-forbid-attrs.rs:5:1
-    |
- LL | #![plugin(lint_plugin_test)]
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
-    = note: `#[warn(deprecated)]` on by default
- 
- 
27 error: item is named 'lintme'
29    |


45 LL | #[allow(test_lint)]
46    |         ^^^^^^^^^ overruled by previous forbid
- error: aborting due to 4 previous errors; 1 warning emitted
- error: aborting due to 4 previous errors; 1 warning emitted
+ warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
+    |
+    |
+ LL | #![plugin(lint_plugin_test)]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
+    = note: `#[warn(deprecated)]` on by default
+ 
+ error: aborting due to 3 previous errors; 1 warning emitted
49 
---
To only update this specific test, also pass `--test-args lint-plugin-forbid-attrs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

error: item is named 'lintme'
   |
   |
LL | fn lintme() {} //~ ERROR item is named 'lintme'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs:7:11
   |
   |
LL | #![forbid(test_lint)]


error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

error: aborting due to 3 previous errors; 1 warning emitted


For more information about this error, try `rustc --explain E0453`.
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin.rs stdout ----
diff of stderr:

- warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> $DIR/lint-plugin.rs:5:1
-    |
- LL | #![plugin(lint_plugin_test)]
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
-    = note: `#[warn(deprecated)]` on by default
- 
- 
9 warning: item is named 'lintme'
11    |

13    | ^^^^^^^^^^^^^^^
14    |
14    |
15    = note: `#[warn(test_lint)]` on by default
+ 
+ warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
+    |
+    |
+ LL | #![plugin(lint_plugin_test)]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
+    = note: `#[warn(deprecated)]` on by default
16 
17 warning: 2 warnings emitted
18 
---
To only update this specific test, also pass `--test-args lint-plugin.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
warning: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ WARNING item is named 'lintme'
   |
   |
   = note: `#[warn(test_lint)]` on by default

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)] //~ WARNING use of deprecated attribute
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 2 warnings emitted
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----
diff of stderr:

6    |
7    = note: requested on the command line with `-A test_lint`
8 
+ warning: item is named 'lintme'
+    |
+    |
+ LL | fn lintme() {}
+    |
+    |
+    = note: `#[warn(clippy::test_lint)]` on by default
+ 
9 warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
11    |

18    |
18    |
19    = note: requested on the command line with `-A test_lint`
20 
- warning: item is named 'lintme'
-   --> $DIR/lint-tool-cmdline-allow.rs:9:1
-    |
- LL | fn lintme() {}
-    |
-    |
-    = note: `#[warn(clippy::test_lint)]` on by default
- 
- warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
-    |
-    = note: requested on the command line with `-A test_lint`
- warning: 6 warnings emitted
+ warning: 5 warnings emitted
34 
35 
---
To only update this specific test, also pass `--test-args lint-tool-cmdline-allow.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-A" "test-lint"
stdout: none
--- stderr -------------------------------
warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`

warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`

warning: item is named 'lintme'
   |
   |
LL | fn lintme() {}
   |
   |
   = note: `#[warn(clippy::test_lint)]` on by default

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_tool_test)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`
warning: 5 warnings emitted
------------------------------------------



---- [ui] src/test/ui-fulldeps/lint-tool-test.rs stdout ----
diff of stderr:

30 LL | #![deny(clippy_group)]
31    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
32 
- warning: lint name `test_group` is deprecated and may not have an effect in the future.
-   --> $DIR/lint-tool-test.rs:31:9
-    |
- LL | #[allow(test_group)]
-    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
- warning: unknown lint: `this_lint_does_not_exist`
-   --> $DIR/lint-tool-test.rs:36:8
-    |
-    |
- LL | #[deny(this_lint_does_not_exist)]
-    |
-    = note: `#[warn(unknown_lints)]` on by default
- 
- 
- warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> $DIR/lint-tool-test.rs:6:1
-    |
- LL | #![plugin(lint_tool_test)]
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
-    = note: `#[warn(deprecated)]` on by default
- 
- warning: lint name `test_lint` is deprecated and may not have an effect in the future.
-   --> $DIR/lint-tool-test.rs:9:23
-   --> $DIR/lint-tool-test.rs:9:23
-    |
- LL | #![cfg_attr(foo, warn(test_lint))]
-    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
- 
- warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
-   --> $DIR/lint-tool-test.rs:14:9
-    |
- LL | #![deny(clippy_group)]
-    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
- 
67 error: item is named 'lintme'
69    |


91 LL | #[allow(test_group)]
92    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
+ warning: unknown lint: `this_lint_does_not_exist`
+   --> $DIR/lint-tool-test.rs:36:8
+    |
+    |
+ LL | #[deny(this_lint_does_not_exist)]
+    |
+    = note: `#[warn(unknown_lints)]` on by default
+ 
+ 
+ warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
+    |
+    |
+ LL | #![plugin(lint_tool_test)]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
+    = note: `#[warn(deprecated)]` on by default
+ 
94 warning: lint name `test_lint` is deprecated and may not have an effect in the future.
95   --> $DIR/lint-tool-test.rs:9:23
95   --> $DIR/lint-tool-test.rs:9:23
96    |

109 LL | #[allow(test_group)]
110    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
- error: aborting due to 2 previous errors; 14 warnings emitted
+ error: aborting due to 2 previous errors; 11 warnings emitted
113 
114 
---
To only update this specific test, also pass `--test-args lint-tool-test.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "--cfg" "foo"
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

error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:14:9
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^
   = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`

error: item is named 'lintmetoo'
   |
   |
LL |     fn lintmetoo() { } //~ ERROR item is named 'lintmetoo'
   |
   |
   = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`

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
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
error: aborting due to 2 previous errors; 11 warnings emitted
------------------------------------------


