rust
#[rustc_const_unstable(feature = "const_box", issue = "92521")]
unsafe impl<#[may_dangle] T: ?Sized, A: Allocator> const Drop for Box<T, A> {
    fn drop(&mut self) {
        // FIXME: Do nothing, drop is currently performed by compiler.
    }
}
