diff
--- library/core/src/marker.rs
+++ library/core/src/marker.rs
@@ -89,7 +90,7 @@ impl<T: ?Sized> !Send for *mut T {}
 )]
 #[fundamental] // for Default, for example, which requires that `[T]: !Default` be evaluatable
 #[rustc_specialization_trait]
-pub trait Sized {
+pub trait Sized: ptr::Thin {
     // Empty.
 }
