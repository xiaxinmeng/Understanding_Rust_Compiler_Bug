 rust
impl<T: Trace> Trace for Gc<T> {
    fn trace(&self, gc_info: &mut GcInfo) {
        match *self {
            X(a, _) => { a.trace(gc_info) }
            Y(_, _) => {}
            Z(_, b) => { b.trace(gc_info) }
        }
    }
}
