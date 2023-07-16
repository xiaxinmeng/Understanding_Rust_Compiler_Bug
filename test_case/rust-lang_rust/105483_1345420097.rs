diff
use std::any::Any;

pub trait Coalesce {
    /// Merge `other` into `self`.
    fn coalesce(&mut self, other: Self)
    where
        Self: Sized;
}
/// This trait allows the merging of a type with an arbitrary trait object.
///
/// If the merger is unsuccessful (they are not of the same type) the trait object is returned.
///
/// This trait has a blanket implementation on any [Sized] type implementing [Coalesce].
pub trait CoalesceAny: Coalesce {
    /// Merge `other` into `self`. Trait object is returned if merging was unsuccessful.
    fn coalesce_any<'a>(&mut self, other: Box<dyn Any + 'a>) -> Option<Box<dyn Any>>;
}
impl<T: Coalesce> CoalesceAny for T {
    fn coalesce_any<'a>(&mut self, other: Box<dyn Any + 'a>) -> Option<Box<dyn Any>> {
+       assert_static(&self); // <~ :(
        let other: Self = match other.downcast() {
            Ok(downcasted) => *downcasted,
            Err(not_downcasted) => return Some(not_downcasted),
        };
        self.coalesce(other);
        None
    }
}

fn assert_static<T: 'static>(_: &T) {}
