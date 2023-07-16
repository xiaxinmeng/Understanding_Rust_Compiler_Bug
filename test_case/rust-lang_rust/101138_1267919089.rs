plain
Successfully built 855708bd0243
Successfully tagged rust-ci:latest
Built container sha256:855708bd02433eea7797c9d8fcd1f2f899b9aac0df3b9b95121bb05b8432df7e
Uploading finished image to https://ci-caches.rust-lang.org/docker/3ba9d538a45014cf9c069a7f7b39b17975213bb4ad3cb92953bcf42e2feac7246274891423ce6ec10459580375996bc5323b9b2e1eaa3f9fe83d7d3a2f2335cf
upload failed: - to s3://rust-lang-ci-sccache2/docker/3ba9d538a45014cf9c069a7f7b39b17975213bb4ad3cb92953bcf42e2feac7246274891423ce6ec10459580375996bc5323b9b2e1eaa3f9fe83d7d3a2f2335cf Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
diff of stderr:

2   --> $DIR/lint-tool-test.rs:9:23
3    |
4 LL | #![cfg_attr(foo, warn(test_lint))]
-    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
6    |
7    = note: `#[warn(renamed_and_removed_lints)]` on by default
7    = note: `#[warn(renamed_and_removed_lints)]` on by default
+ help: lint name `test_lint` is deprecated and may not have an effect in the future.
+    |
+ LL | #![cfg_attr(foo, warn(clippy::test_lint))]
8 
8 
9 warning: lint name `clippy_group` is deprecated and may not have an effect in the future.

11    |
11    |
12 LL | #![deny(clippy_group)]
-    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
+    |
+    |
+ help: lint name `clippy_group` is deprecated and may not have an effect in the future.
+    |
+ LL | #![deny(clippy::group)]
14 
14 
15 warning: lint name `test_group` is deprecated and may not have an effect in the future.

17    |
17    |
18 LL | #[allow(test_group)]
-    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
+    |
+    |
+ help: lint name `test_group` is deprecated and may not have an effect in the future.
+    |
+ LL | #[allow(clippy::test_group)]
20 
21 warning: lint name `test_lint` is deprecated and may not have an effect in the future.
22   --> $DIR/lint-tool-test.rs:9:23


23    |
24 LL | #![cfg_attr(foo, warn(test_lint))]
-    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
+    |
+    |
+ help: lint name `test_lint` is deprecated and may not have an effect in the future.
+    |
+ LL | #![cfg_attr(foo, warn(clippy::test_lint))]
26 
26 
27 warning: lint name `clippy_group` is deprecated and may not have an effect in the future.

29    |
29    |
30 LL | #![deny(clippy_group)]
-    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
+    |
+    |
+ help: lint name `clippy_group` is deprecated and may not have an effect in the future.
+    |
+ LL | #![deny(clippy::group)]
32 
32 
33 warning: lint name `test_group` is deprecated and may not have an effect in the future.

35    |
35    |
36 LL | #[allow(test_group)]
-    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
+    |
+    |
+ help: lint name `test_group` is deprecated and may not have an effect in the future.
+    |
+ LL | #[allow(clippy::test_group)]
38 
39 warning: unknown lint: `this_lint_does_not_exist`
40   --> $DIR/lint-tool-test.rs:36:8


56   --> $DIR/lint-tool-test.rs:9:23
57    |
58 LL | #![cfg_attr(foo, warn(test_lint))]
-    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
+    |
+    |
+ help: lint name `test_lint` is deprecated and may not have an effect in the future.
+    |
+ LL | #![cfg_attr(foo, warn(clippy::test_lint))]
60 
60 
61 warning: lint name `clippy_group` is deprecated and may not have an effect in the future.

63    |
63    |
64 LL | #![deny(clippy_group)]
-    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
+    |
+    |
+ help: lint name `clippy_group` is deprecated and may not have an effect in the future.
+    |
+ LL | #![deny(clippy::group)]
66 
66 
67 error: item is named 'lintme'

89   --> $DIR/lint-tool-test.rs:31:9
90    |
90    |
91 LL | #[allow(test_group)]
-    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
+    |
+    |
+ help: lint name `test_group` is deprecated and may not have an effect in the future.
+    |
+ LL | #[allow(clippy::test_group)]
93 
94 warning: lint name `test_lint` is deprecated and may not have an effect in the future.
95   --> $DIR/lint-tool-test.rs:9:23


96    |
97 LL | #![cfg_attr(foo, warn(test_lint))]
-    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
+    |
+    |
+ help: lint name `test_lint` is deprecated and may not have an effect in the future.
+    |
+ LL | #![cfg_attr(foo, warn(clippy::test_lint))]
99 
99 
100 warning: lint name `clippy_group` is deprecated and may not have an effect in the future.

102    |
102    |
103 LL | #![deny(clippy_group)]
-    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
+    |
+    |
+ help: lint name `clippy_group` is deprecated and may not have an effect in the future.
+    |
+ LL | #![deny(clippy::group)]
105 
105 
106 warning: lint name `test_group` is deprecated and may not have an effect in the future.

108    |
108    |
109 LL | #[allow(test_group)]
-    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
+    |
+    |
+ help: lint name `test_group` is deprecated and may not have an effect in the future.
+    |
+ LL | #[allow(clippy::test_group)]
111 
112 error: aborting due to 2 previous errors; 14 warnings emitted
113 

---
To only update this specific test, also pass `--test-args lint-tool-test.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
stdout: none
--- stderr -------------------------------
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
   |
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |
   = note: `#[warn(renamed_and_removed_lints)]` on by default
help: lint name `test_lint` is deprecated and may not have an effect in the future.
   |
   |
LL | #![cfg_attr(foo, warn(clippy::test_lint))]


warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |
   |
help: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
LL | #![deny(clippy::group)]


warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |
   |
help: lint name `test_group` is deprecated and may not have an effect in the future.
   |
LL | #[allow(clippy::test_group)]

warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |
help: lint name `test_lint` is deprecated and may not have an effect in the future.
   |
   |
LL | #![cfg_attr(foo, warn(clippy::test_lint))]


warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |
   |
help: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
LL | #![deny(clippy::group)]


warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |
   |
help: lint name `test_group` is deprecated and may not have an effect in the future.
   |
LL | #[allow(clippy::test_group)]

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
   |
help: lint name `test_lint` is deprecated and may not have an effect in the future.
   |
   |
LL | #![cfg_attr(foo, warn(clippy::test_lint))]


warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |
   |
help: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
LL | #![deny(clippy::group)]


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
   |
   |
help: lint name `test_group` is deprecated and may not have an effect in the future.
   |
LL | #[allow(clippy::test_group)]

warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |
help: lint name `test_lint` is deprecated and may not have an effect in the future.
   |
   |
LL | #![cfg_attr(foo, warn(clippy::test_lint))]


warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |
   |
help: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
LL | #![deny(clippy::group)]


warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |
   |
help: lint name `test_group` is deprecated and may not have an effect in the future.
   |
LL | #[allow(clippy::test_group)]

error: aborting due to 2 previous errors; 14 warnings emitted
------------------------------------------

