rust
impl<T> Option<T> {
    pub fn as_deref(&self) -> Option<&T::Target> where T: Deref {…}
    pub fn as_deref_mut(&mut self) -> Option<&mut T::Target> where T: DerefMut {…}
}

impl<T, E> Result<T, E> {
    pub fn as_deref_ok(&self) -> Result<&T::Target, &E> where T: Deref {…}
    pub fn as_deref_err(&self) -> Result<&T, &E::Target> where E: Deref {…}
    pub fn as_deref(&self) -> Result<&T::Target, &E::Target> where T: Deref, E: Deref {…}

    pub fn as_deref_mut_ok(&mut self) -> Result<&mut T::Target, &mut E> where T: DerefMut {…}
    pub fn as_deref_mut_err(&mut self) -> Result<&mut T, &mut E::Target> where E: DerefMut {…}
    pub fn as_deref_mut(&mut self) -> Result<&mut T::Target, &mut E::Target> where T: DerefMut, E: DerefMut {…}
}
