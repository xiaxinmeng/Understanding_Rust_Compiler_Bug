rust
impl<T: 'static> IsEqual for T {
    const TO<U>: bool = TypeId::of::<T>() == TypeId::of::<U>()
    where
        U: 'static;
}
