rust
impl<T> ManuallyDrop<T> {
    pub unsafe fn take(slot: &mut ManuallyDrop<T>) -> T;
}
