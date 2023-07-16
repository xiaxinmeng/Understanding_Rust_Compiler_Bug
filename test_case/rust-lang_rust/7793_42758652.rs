
#![crate_type = "lib"]

pub struct WriteLock;

impl WriteLock {
    pub fn new() -> WriteLock { fail!() }
}

impl Drop for WriteLock {
    // Removing the inline attribute fixes the problem
    #[inline(always)]
    fn drop(&mut self) { }
}
