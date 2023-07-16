
impl<'a, T: ?Sized + SetValue> SetValue for &'a T {
impl<T: IsA<Object>> SetValue for T {
