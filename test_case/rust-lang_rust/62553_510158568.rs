rust
fn into_inner (self) -> Box<[u8]>
{
    use ::core::{mem::MaybeUninit, ptr};
    let this = MaybeUninit::new(self);
    unsafe {
        ptr::read(&mut (*this.as_mut_ptr()).inner)
    }
}
