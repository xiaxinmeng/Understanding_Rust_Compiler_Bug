rust
trait Any {}

trait AsAny {
    fn as_any(self);
}

impl<T> AsAny for T where T: Any + AsAny {
    fn as_any(self) {}
}

impl dyn AsAny {
    fn borrow<'a>(&'a self) -> &'a dyn AsAny {
        self
    }
}

fn demo<'a>(cell: &'a (dyn AsAny + 'static)) {
    cell.borrow::<'a, _>().as_any();
}
