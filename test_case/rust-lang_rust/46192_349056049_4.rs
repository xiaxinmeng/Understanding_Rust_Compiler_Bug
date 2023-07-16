
impl<'a, T: ?Sized> From<&'a T> for ToSqlOutput<'a> where &'a T: Into<ValueRef<'a>>
impl<'a, T: Into<Value>> From<T> for ToSqlOutput<'a>
