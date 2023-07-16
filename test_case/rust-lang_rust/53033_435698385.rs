rust
impl<T, U> CoerceUnsized<ManuallyDrop<U>> for ManuallyDrop<T>
	where T: CoerceUnsized<U>
{}
