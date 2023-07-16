rust
struct Ptr<T: ?Sized + Pointee<Meta = Meta>, Meta: 'static + Copy = <T as Pointee>::Meta> {
    addr: usize,
    meta: Meta,
    _marker: std::marker::PhantomData<T>,
}

trait Pointee {
    type Meta: 'static + Copy;
}
impl<T> Pointee for T {
    type Meta = ();
}
impl<T> Pointee for [T] {
    type Meta = usize;
}

fn covariant<'a>(p: Ptr<&'static ()>) -> Ptr<&'a ()> { p }
