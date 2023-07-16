diff
src/test/ui/nll/user-annotations/patterns.rs
--- INDEX/src/test/ui/nll/user-annotations/patterns.rs
+++ WORKDIR/src/test/ui/nll/user-annotations/patterns.rs
@@ -9,11 +9,11 @@ fn variable_no_initializer() {
 }

 fn tuple_no_initializer() {
-    // FIXME(#47187): We are not propagating ascribed type through tuples.
+

     let x = 22;
     let (y, z): (&'static u32, &'static u32);
-    y = &x;
+    y = &x; //~ ERROR
 }

 fn ref_with_ascribed_static_type() -> u32 {
@@ -34,11 +34,11 @@ fn ref_with_ascribed_any_type() -> u32 {
 struct Single<T> { value: T }

 fn struct_no_initializer() {
-    // FIXME(#47187): We are not propagating ascribed type through patterns.
+

     let x = 22;
     let Single { value: y }: Single<&'static u32>;
-    y = &x;
+    y = &x; //~ ERROR
 }

 fn variable_with_initializer() {
@@ -91,26 +91,26 @@ fn struct_double_field_underscore_with_initializer() {
 }

 fn static_to_a_to_static_through_variable<'a>(x: &'a u32) -> &'static u32 {
-    // The error in this test is inconsistency with
-    // `static_to_a_to_static_through_tuple`, but "feels right" to
-    // me. It occurs because we special case the single binding case
-    // and force the type of `y` to be `&'a u32`, even though the
-    // right-hand side has type `&'static u32`.
+
+
+
+
+

     let y: &'a u32 = &22;
     y //~ ERROR
 }
 fn static_to_a_to_static_through_tuple<'a>(x: &'a u32) -> &'static u32 {
-    // FIXME(#47187): The fact that this type-checks is perhaps surprising.
-    // What happens is that the right-hand side is constrained to have
-    // type `&'a u32`, which is possible, because it has type
-    // `&'static u32`. The variable `y` is then forced to have type
-    // `&'static u32`, but it is constrained only by the right-hand
-    // side, not the ascribed type, and hence it passes.
+
+
+
+
+
+

     let (y, _z): (&'a u32, u32) = (&22, 44);
-    y
+    y //~ ERROR
 }

 fn a_to_static_then_static<'a>(x: &'a u32) -> &'static u32 {
