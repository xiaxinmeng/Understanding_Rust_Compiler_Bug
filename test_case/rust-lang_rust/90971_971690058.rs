rust
pub trait Type {
    type Ref;
}

pub struct TypeRef<T: Type> {
    r: T::Ref,
}

impl<T: Type> From<T::Ref> for TypeRef<T> {
    fn from(_: T::Ref) -> Self {
        todo!()
    }
}
