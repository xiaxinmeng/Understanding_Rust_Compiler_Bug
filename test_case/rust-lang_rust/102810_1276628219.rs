rust
pub struct UBCheck<T: ?Sized>(PhantomData<T>);

unsafe impl<#[may_dangle] T: ?Sized> Drop for UBCheck<T> {
    fn drop(&mut self) {}
}
