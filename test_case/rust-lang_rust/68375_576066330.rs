rust
pub struct ArrayBase<D: Data<Elem = Elem>, Elem = <D as Data>::Elem> {
    ptr: *mut Elem,
    d: D,
}
