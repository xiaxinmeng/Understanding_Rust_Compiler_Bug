 rust
trait Placer<Data: ?Sized> {
    type Place: Place<Data>;
    fn make_place(&mut self) -> Self::Place;
}

unsafe trait Place<Data: ?Sized> {
    type Owner;
    fn pointer(&mut self) -> *mut Data;
    unsafe fn finalize(self) -> Self::Owner;
}

trait Boxer<Data: ?Sized>: Sized {
    type Place: Place<Data, Owner=Self>;
    fn make_place() -> Self::Place;
}

impl<T> Boxer<T> for Box<T> { /* ... */ }

impl<T> Place<T> for IntermediateBox<T> { /* ... */ }
