rust
struct SetOnDrop<'a, T: 'a> {
    borrow: &'a mut T,
    value: Option<T>,
}

impl<T> Drop for SetOnDrop<'_, T> { // No 'a
    fn drop(&mut self) {
        if let Some(x) = self.value.take() {
            *self.borrow = x;
        }
    }
}
