rust
use self::internal::Internal;

trait Error {
    /// Since you can never import the `Internal` type, you can never override this method.
    fn __type_id(&self, _: Internal) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }
}

impl dyn Error + 'static {
    fn type_id(&self) -> TypeId {
        self.__type_id(Internal)
    }
}

mod internal {
    pub struct Internal;
}
