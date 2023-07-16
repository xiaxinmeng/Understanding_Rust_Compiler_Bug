diff
-pub const fn null<T>() -> *const T {
+pub const fn null<T: ?Sized + Thin>() -> *const T {

-pub const fn null_mut<T>() -> *mut T {
+pub const fn null_mut<T: ?Sized + Thin>() -> *mut T {
 