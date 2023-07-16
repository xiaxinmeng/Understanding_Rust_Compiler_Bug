rust
use std::any::TypeId;
use std::fmt::{Debug, Display};

trait TypeInfo {
    fn type_id(&self) -> TypeId
    where
        Self: 'static;
}

impl<T: ?Sized> TypeInfo for T {
    fn type_id(&self) -> TypeId
    where
        T: 'static,
    {
        TypeId::of::<Self>()
    }
}

trait Error: Debug + Display + TypeInfo {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
