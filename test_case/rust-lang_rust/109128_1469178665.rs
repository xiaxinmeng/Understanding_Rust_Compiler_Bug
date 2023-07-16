plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
  |     ^ mismatched closing delimiter

test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at tests/source/configs/format_macro_bodies/true.rs:1:
 // rustfmt-format_macro_bodies: true
 macro_rules! foo {
 macro_rules! foo {
-    ($a: ident : $b: ty) => {
-        $a(42): $b;
-    };
-    ($a: ident $b: ident $c: ident) => {
-        $a = $b + $c;
-    };
+    ($a: ident : $b: ty) => { $a(42): $b; };
+    ($a: ident $b: ident $c: ident) => { $a=$b+$c; };
 

Mismatch at tests/source/configs/format_macro_matchers/true.rs:1:
 // rustfmt-format_macro_matchers: true
 // rustfmt-format_macro_matchers: true
 
 macro_rules! foo {
-    ($a:ident : $b:ty) => {
-        $a(42): $b;
-    };
-    ($a:ident $b:ident $c:ident) => {
-        $a = $b + $c;
-    };
+    ($a: ident : $b: ty) => { $a(42): $b; };
+    ($a: ident $b: ident $c: ident) => { $a=$b+$c; };
 

Mismatch at tests/source/configs/format_macro_matchers/false.rs:1:
 // rustfmt-format_macro_matchers: false
 // rustfmt-format_macro_matchers: false
 
 macro_rules! foo {
-    ($a: ident : $b: ty) => {
-        $a(42): $b;
-    };
-    ($a: ident $b: ident $c: ident) => {
-        $a = $b + $c;
-    };
+    ($a: ident : $b: ty) => { $a(42): $b; };
+    ($a: ident $b: ident $c: ident) => { $a=$b+$c; };
 

Mismatch at tests/source/macros.rs:122:
         20, 21, 22);
         20, 21, 22);
 
     // #1092
-    chain!(input, a: take!(max_size), || []);
+    chain!(input, a:take!(max_size), || []);
     // #2727
     // #2727
     foo!("bar");
Mismatch at tests/source/macros.rs:156:
 }
 
 fn issue1739() {
 fn issue1739() {
-    sql_function!(
-        add_rss_item,
-        add_rss_item_t,
-            a: types::Integer,
-            a: types::Integer,
-            b: types::Timestamptz,
-            c: types::Text,
-            d: types::Text,
-            e: types::Text
-    );
-    );
+    sql_function!(add_rss_item,
+                  add_rss_item_t,
+                  (a: types::Integer,
+                   b: types::Timestamptz,
+                   c: types::Text,
+                   d: types::Text,
+                   e: types::Text));
 
     w.slice_mut(s![
         ..,
Mismatch at tests/source/macros.rs:232:
Mismatch at tests/source/macros.rs:232:
             "debugMessage": debug.message,
     } else {
     } else {
-        json!({ "errorKind": format!("{:?}", error.err_kind()) })
+        json!({"errorKind": format!("{:?}", error.err_kind())})
 }
 

Mismatch at tests/source/type.rs:129:
Mismatch at tests/source/type.rs:129:
 fn issue3139() {
     assert_eq!(
         to_json_value(&None::<i32>).unwrap(),
-        json!({ "test": None::<i32> })
+        json!(  { "test": None  ::  <i32> }  )
 }
 
test test::self_tests ... ok
test test::system_tests ... FAILED
test test::system_tests ... FAILED
test test::idempotence_tests ... ok

failures:

---- test::system_tests stdout ----
Warning: the `fn_args_layout` option is deprecated. Use `fn_params_layout`. instead
Warning: the `merge_imports` option is deprecated. Use `imports_granularity="Crate"` instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `0`: 5 system tests failed', src/tools/rustfmt/src/test/mod.rs:189:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'test::system_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:76:10

failures:
    test::system_tests

