`diff

--- src/test/ui/mir/ssa-analysis-regression-50041.rs	2022-07-17 12:25:49.238091488 +0200
+++ /tmp/im/8e9c14a03c386623fed51bea2843c68ff16ca8d1.rs	2022-09-18 05:59:43.438071009 +0200
@@ -5,7 +5,7 @@
 #![feature(lang_items)]
 #![no_std]

-struct NonNull<T: ?Sized>(*const T);
+struct NonNull<T: ?Sized>(*mut T);

 struct Unique<T: ?Sized>(NonNull<T>);

@@ -23,7 +23,7 @@
 }

 #[inline(never)]
-fn dealloc<T: ?Sized>(_: *const T) {}
+fn dealloc<T: ?Sized>(_: *mut T) {}

 pub struct Foo<T>(T);
