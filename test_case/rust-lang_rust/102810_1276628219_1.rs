rust
struct NopDrop;
impl Drop for NopDrop {
	fn drop(&mut self) {}
}

pub struct UBCheck<T: ?Sized>(NopDrop, PhantomData<T>);
