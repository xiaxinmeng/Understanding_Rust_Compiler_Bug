diff
diff --git a/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.stderr b/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.stderr
index 3cb8abcdcfd..cd637056c94 100644
--- a/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.stderr
+++ b/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.stderr
@@ -1,11 +1,12 @@
-error[E0271]: type mismatch resolving `for<'r> <[closure@$DIR/issue-57611-trait-alias.rs:21:9: 21:14] as std::ops::FnOnce<(&'r X,)>>::Output == &'r X`
+error[E0308]: mismatched types
   --> $DIR/issue-57611-trait-alias.rs:17:16
    |
 LL |     type Bar = impl Baz<Self, Self>;
-   |                ^^^^^^^^^^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
+   |                ^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
    |
-   = note: the return type of a function must have a statically known size
+   = note: expected type `std::ops::FnOnce<(&X,)>`
+              found type `std::ops::FnOnce<(&X,)>`
 
 error: aborting due to previous error
 
-For more information about this error, try `rustc --explain E0271`.
+For more information about this error, try `rustc --explain E0308`.
