rust
struct Ptr<T: ?Sized + Pointee, U = <T as Pointee>::Meta> {
    addr: usize,
    meta: U,
    t: std::marker::PhantomData<T>,
}
