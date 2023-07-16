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
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9964284fed181676300aad693449f5b751e35ff2 and 7c583d1664d4daaa9081534f2004e9e5bf9fdbd1
Executing the job since clippy or rustfmt subtree was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
test test::verify_config_test_names ... ok
test utils::test::test_trim_left_preserve_layout ... ok
test test::verify_check_works ... ok

Mismatch at tests/source/type.rs:89:
 // #3060
 macro_rules! foo {
     ($foo_api: ty) => {
-        type Target = ($foo_api) + 'static;
-    };
+        type Target = ( $foo_api ) + 'static;
 }
 
 
 type Target = (FooAPI) + 'static;


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
 


Mismatch at tests/source/configs/format_macro_matchers/false.rs:1:
 // rustfmt-format_macro_matchers: false
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
 

Mismatch at tests/source/single-line-macro/v2.rs:4:
Mismatch at tests/source/single-line-macro/v2.rs:4:
 // Preserve trailing comma inside macro, even if it looks an array.
 macro_rules! bar {
     ($m:ident) => {
-        $m!([
-            a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z,
-        $m!([
-        $m!([
-            a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z
-        ]);
+        $m!([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z,]);
+        $m!([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z]);
 }
 


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