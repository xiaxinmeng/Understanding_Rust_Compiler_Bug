
impl<'a, T> From<Cow<'a, T>> for T::Owned
impl<'a, T> From<&'a T> for Cow<'a, T> 
impl<'a, T> From<T::Owned> for Cow<'a, T> 
impl<'a, T> From<&'a T::Owned> for Cow<'a, T>
