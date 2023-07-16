diff
--- a/src/test/ui/resolve/use_suggestion_placement.rs
+++ b/src/test/ui/resolve/use_suggestion_placement.rs
@@ -10,11 +10,7 @@ mod m {
 }

 mod foo {
-    // FIXME: UsePlacementFinder is broken because active attributes are
-    // removed, and thus the `derive` attribute here is not in the AST.
-    // An inert attribute should work, though.
-    // #[derive(Debug)]
-    #[allow(warnings)]
+    #[derive(Debug)]
     pub struct Foo;

     // test whether the use suggestion isn't
