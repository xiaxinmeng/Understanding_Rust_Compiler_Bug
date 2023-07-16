plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at tests/source/macros.rs:146:
 
 fn issue1178() {
     macro_rules! foo {
-        (#[$attr:meta] $name:ident) => {};
+        (#[$attr:meta] $name:ident) => {}
 
     foo!(

Mismatch at tests/source/macros.rs:264:
Mismatch at tests/source/macros.rs:264:
 
 // #878
 macro_rules! try_opt {
-    ($expr:expr) => {
-        match $expr {
-            Some(val) => val,
+    ($expr:expr) => (match $expr {
+        Some(val) => val,
-            None => {
-                return None;
-            }
-        }
---
 
-macro foo() {}
+macro foo() {
 
-pub macro bar($x:ident + $y:expr;) {
+}
+
+
+pub macro bar($x:ident+$y:expr; ) {
     fn foo($x: Foo) {
-        long_function(
-            a_long_argument_to_a_long_function_is_what_this_is(AAAAAAAAAAAAAAAAAAAAAAAAAAAA),
-            $x.bar($y),
-        );
+    long_function(a_long_argument_to_a_long_function_is_what_this_is(AAAAAAAAAAAAAAAAAAAAAAAAAAAA),
+                  $x.bar($y));
 }
 

Mismatch at tests/source/macros.rs:921:
---
-    () => {{}};
+    () => {{}}
 }
 
 macro lex_err($kind: ident $(, $body: expr)*) {

Mismatch at tests/source/itemized-blocks/wrap.rs:69:
 ///    type
 ///  * `msg` is the (serialized) message
-/// let x = 42;
-/// let x = 42;
+/// let x =     42;
 fn func2() {}
 


Mismatch at tests/source/itemized-blocks/wrap.rs:76:
 /// Look:
 /// 