
// This blanket implementation conflicts with the following 6 implementations
/*
impl<T, U> AsMut<U> for T
where
    T: ?Sized + DerefMut,
    U: ?Sized,
    <T as Deref>::Target: AsMut<U>,
{
    fn as_mut(&mut self) -> &mut U {
        self.deref_mut().as_mut()
    }
}
*/

// Would be covered by blanket implementation
impl<'a, T, U> AsMut<U> for &'a mut T
where
    T: ?Sized + AsMut<U>,
    U: ?Sized,
{
    fn as_mut(&mut self) -> &mut U {
        self.deref_mut().as_mut()
    }
}

// Would be covered by blanket implementation
impl<T, U> AsMut<U> for Box<T>
where
    T: ?Sized + AsMut<U>,
    U: ?Sized,
{
    fn as_mut(&mut self) -> &mut U {
        self.deref_mut().as_mut()
    }
}

// Would NOT covered by blanket implementation
// because `<String as Deref>::Target` is `str` and not `String`
impl AsMut<String> for String {
    fn as_mut(&mut self) -> &mut String {
        self
    }
}

// Would be covered by blanket implementation
// because `<String as Deref>::Target` is `str`
// and `str` implements `AsMut<str>`
impl AsMut<str> for String {
    fn as_mut(&mut self) -> &mut str {
        self
    }
}

// Would NOT covered by blanket implementation
// because `<Vec<T> as Deref>::Target` is `[T]` and not `Vec<T>`
impl<T> AsMut<Vec<T>> for Vec<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        self
    }
}

// Would be covered by blanket implementation
// because `<Vec<T> as Deref>::Target` is `[T]`
// and `[T]` implements `AsMut<[T]>`
impl<T> AsMut<[T]> for Vec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}
