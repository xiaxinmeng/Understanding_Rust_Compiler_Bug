 diff
diff --git a/src/test/compile-fail/issue-16538.rs b/src/test/compile-fail/issue-16538.rs
index 707b72b..0e02283 100644
--- a/src/test/compile-fail/issue-16538.rs
+++ b/src/test/compile-fail/issue-16538.rs
@@ -20,5 +20,6 @@ mod Y {

 static foo: *const Y::X = Y::foo(Y::x as *const Y::X);
 //~^ ERROR cannot refer to other statics by value
+//~| ERROR: the trait `core::kinds::Sync` is not implemented for the type

 fn main() {}
diff --git a/src/test/compile-fail/issue-7364.rs b/src/test/compile-fail/issue-7364.rs
index 2646edd..ab5ba29 100644
--- a/src/test/compile-fail/issue-7364.rs
+++ b/src/test/compile-fail/issue-7364.rs
@@ -14,8 +14,8 @@ use std::cell::RefCell;
 // Regresion test for issue 7364
 static boxed: Box<RefCell<int>> = box RefCell::new(0);
 //~^ ERROR statics are not allowed to have custom pointers
-//~^^ ERROR: the trait `core::kinds::Sync` is not implemented for the type
-//~^^^ ERROR: the trait `core::kinds::Sync` is not implemented for the type
-//~^^^^ ERROR: the trait `core::kinds::Sync` is not implemented for the type
+//~| ERROR: the trait `core::kinds::Sync` is not implemented for the type
+//~| ERROR: the trait `core::kinds::Sync` is not implemented for the type
+//~| ERROR: the trait `core::kinds::Sync` is not implemented for the type

 fn main() { }
