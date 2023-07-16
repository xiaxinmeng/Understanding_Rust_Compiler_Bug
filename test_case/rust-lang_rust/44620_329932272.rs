diff
diff --git a/src/test/ui/lint/fn_must_use.rs b/src/test/ui/rfc_1940-must_use_on_functions/fn_must_use.rs
similarity index 47%
rename from src/test/ui/lint/fn_must_use.rs
rename to src/test/ui/rfc_1940-must_use_on_functions/fn_must_use.rs
index c549ded..7eb4c329 100644
--- a/src/test/ui/lint/fn_must_use.rs
+++ b/src/test/ui/rfc_1940-must_use_on_functions/fn_must_use.rs
@@ -12,7 +12,7 @@
 #![warn(unused_must_use)]
 
 struct MyStruct {
-    n: usize
+    n: usize,
 }
 
 impl MyStruct {
@@ -22,7 +22,34 @@ impl MyStruct {
     }
 }
 
-#[must_use="it's important"]
+trait EvenNature {
+    #[must_use = "no side effects"]
+    fn is_even(&self) -> bool;
+}
+
+impl EvenNature for MyStruct {
+    fn is_even(&self) -> bool {
+        self.n % 2 == 0
+    }
+}
+
+trait Replaceable {
+    fn replace(&mut self, substitute: usize) -> usize;
+}
+
+impl Replaceable for MyStruct {
+    // ↓ N.b.: `#[must_use]` attribute on a particular trait implementation
+    // method won't work; the attribute should be on the method signature in
+    // the trait's definition.
+    #[must_use]
+    fn replace(&mut self, substitute: usize) -> usize {
+        let previously = self.n;
+        self.n = substitute;
+        previously
+    }
+}
+
+#[must_use = "it's important"]
 fn need_to_use_this_value() -> bool {
     false
 }
@@ -30,6 +57,14 @@ fn need_to_use_this_value() -> bool {
 fn main() {
     need_to_use_this_value();
 
-    let m = MyStruct { n: 2 };
+    let mut m = MyStruct { n: 2 };
     m.need_to_use_this_method_value();
+    m.is_even(); // trait method!
+
+    m.replace(3);
+
+    2.eq(&3);
+
+    // FIXME: operators should probably be `must_use` if underlying method is
+    2 == 3;
 }
diff --git a/src/test/ui/lint/fn_must_use.stderr b/src/test/ui/rfc_1940-must_use_on_functions/fn_must_use.stderr
similarity index 45%
rename from src/test/ui/lint/fn_must_use.stderr
rename to src/test/ui/rfc_1940-must_use_on_functions/fn_must_use.stderr
index 2428377..69755c8 100644
--- a/src/test/ui/lint/fn_must_use.stderr
+++ b/src/test/ui/rfc_1940-must_use_on_functions/fn_must_use.stderr
@@ -1,7 +1,7 @@
 warning: unused return value of `need_to_use_this_value` which must be used: it's important
-  --> $DIR/fn_must_use.rs:31:5
+  --> $DIR/fn_must_use.rs:58:5
    |
-31 |     need_to_use_this_value();
+58 |     need_to_use_this_value();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
 note: lint level defined here
@@ -11,8 +11,20 @@ note: lint level defined here
    |         ^^^^^^^^^^^^^^^
 
 warning: unused return value of `MyStruct::need_to_use_this_method_value` which must be used
-  --> $DIR/fn_must_use.rs:34:5
+  --> $DIR/fn_must_use.rs:61:5
    |
-34 |     m.need_to_use_this_method_value();
+61 |     m.need_to_use_this_method_value();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 
+warning: unused return value of `EvenNature::is_even` which must be used: no side effects
+  --> $DIR/fn_must_use.rs:62:5
+   |
+62 |     m.is_even(); // trait method!
+   |     ^^^^^^^^^^^^
+
+warning: unused return value of `std::cmp::PartialEq::eq` which must be used
+  --> $DIR/fn_must_use.rs:66:5
+   |
+66 |     2.eq(&3);
+   |     ^^^^^^^^^
+
