rust
use core::ops::Deref;

pub trait HasId {
    fn id(&self) -> usize;
}

// Notice the lack of bound `<T as Deref>::Target : HasId`
impl<T:Deref> HasId for T {
    fn id(&self) -> usize {
        let target:&<T as Deref>::Target = self.deref();
        target.id()
    }
}
