rust
use std::any::Any;
pub trait CoalesceAny {
    fn coalesce_any<'a>(&mut self, other: Box<dyn Any + 'a>) -> Option<Box<dyn Any>>;
}
impl<'a, T: 'a> CoalesceAny for T {
    fn coalesce_any(&mut self, other: Box<dyn Any + 'a>) -> Option<Box<dyn Any>> {
        todo!()
    }
}
