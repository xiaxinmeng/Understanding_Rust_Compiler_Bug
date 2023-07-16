rust
trait ExactSizeIterator : IsEmpty {
    fn len(&self) -> usize;
}

trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl<T> IsEmpty for T where T: ExactSizeIterator {
    default fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
