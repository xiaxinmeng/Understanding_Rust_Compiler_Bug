rust
fn read_uninit<'a>(&mut self, buf: &'a mut [MaybeUninit<u8>]) -> Result<&'a mut [u8]> {
    for byte in &mut *buf {
        unsafe {
            *byte.as_mut_ptr() = 0;
        }
    }
    let initialized = unsafe { mem::transmute::<&mut [MaybeUninit<u8>], &mut [u8]>(buf) };
    let size = self.read(initialized)?;
    Ok(&mut initialized[..size])
}
