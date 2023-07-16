rust
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct TypeId {
    id: usize,
}

impl TypeId {
    pub fn of<T: ?Sized>() -> Self {
        TypeId {
            id: TypeId::of::<T> as usize,
        }
    }
}
