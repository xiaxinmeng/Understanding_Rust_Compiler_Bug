rust
pub struct ProtectedSlice<'s, T> {
    data: &'s [T], // Note that the data itself is kept private.
}

impl<'s, T> ProtectedSlice<'s, T> {
    pub const fn new(data: &'s [T]) -> Self {
        Self { data }
    }

    pub const fn len(&self) -> usize {
        self.data.len()
    }
}


pub struct ProtectedVec<T> {
    data: Vec<T>,
}

impl<T> ProtectedVec<T> {
    pub const fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub /*const*/ fn len(&self) -> usize {
        self.data.len()
    }
}
