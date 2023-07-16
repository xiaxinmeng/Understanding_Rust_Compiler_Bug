diff
diff --git a/src/test/ui/lint/lint-unused-imports.stderr b/src/test/ui/lint/lint-unused-imports.stderr
index f9a54f477f..bf194696f6 100644
--- a/src/test/ui/lint/lint-unused-imports.stderr
+++ b/src/test/ui/lint/lint-unused-imports.stderr
@@ -34,12 +34,35 @@ error: unused import: `foo::Square`
 LL |         use foo::Square;
    |             ^^^^^^^^^^^
 
+warning: the item `g` is imported redundantly
+  --> $DIR/lint-unused-imports.rs:68:9
+   |
+LL | / fn g() {
+LL | |     use self::g;
+   | |         ^^^^^^^
+LL | |     fn f() {
+LL | |         self::g();
+LL | |     }
+LL | | }
+   | |_- the item `g` was already imported here
+   |
+   = note: #[warn(redundant_import)] on by default
+
 error: unused import: `self::g`
   --> $DIR/lint-unused-imports.rs:68:9
    |
 LL |     use self::g;
    |         ^^^^^^^
 
+warning: the item `foo` is imported redundantly
+  --> $DIR/lint-unused-imports.rs:77:9
+   |
+LL | use test2::{foo, bar};
+   |             --- the item `foo` was already imported here
+...
+LL |     use test2::foo;
+   |         ^^^^^^^^^^
+
 error: unused import: `test2::foo`
   --> $DIR/lint-unused-imports.rs:77:9
    |
diff --git a/src/test/ui/rust-2018/future-proofing-locals.stderr b/src/test/ui/rust-2018/future-proofing-locals.stderr
index 4d666d22af..fa8333b5d2 100644
--- a/src/test/ui/rust-2018/future-proofing-locals.stderr
+++ b/src/test/ui/rust-2018/future-proofing-locals.stderr
@@ -52,5 +52,51 @@ error: imports cannot refer to local variables
 LL |     use {T as _, x};
    |                  ^
 
+warning: the item `T` is imported redundantly
+  --> $DIR/future-proofing-locals.rs:19:9
+   |
+LL | / mod T {
+LL | |     pub struct U;
+LL | | }
+   | |_- the item `T` was already imported here
+...
+LL |       use T;
+   |           ^
+   |
+   = note: #[warn(redundant_import)] on by default
+
+warning: the item `x` is imported redundantly
+  --> $DIR/future-proofing-locals.rs:31:9
+   |
+LL | / mod x {
+LL | |     pub struct y;
+LL | | }
+   | |_- the item `x` was already imported here
+...
+LL |       use x;
+   |           ^
+
+warning: the item `x` is imported redundantly
+  --> $DIR/future-proofing-locals.rs:37:17
+   |
+LL | / mod x {
+LL | |     pub struct y;
+LL | | }
+   | |_- the item `x` was already imported here
+...
+LL |               use x;
+   |                   ^
+
+warning: the item `x` is imported redundantly
+  --> $DIR/future-proofing-locals.rs:45:18
+   |
+LL | / mod x {
+LL | |     pub struct y;
+LL | | }
+   | |_- the item `x` was already imported here
+...
+LL |       use {T as _, x};
+   |                    ^
+
 error: aborting due to 9 previous errors
