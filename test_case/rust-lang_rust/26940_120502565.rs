 rust
pub struct MaybeCrashOnDrop { c: bool }
impl Drop for MaybeCrashOnDrop {
    fn drop(&mut self) {
        if self.c {
            unsafe { *(1 as *mut u8) = 0 }
        }
    }
}
pub struct InteriorUnsafe { pub m: MaybeCrashOnDrop }
impl InteriorUnsafe {
    pub fn new() -> InteriorUnsafe {
        InteriorUnsafe { m: MaybeCrashOnDrop{ c: true } }
    }
}
impl Drop for InteriorUnsafe {
    fn drop(&mut self) {
        self.m.c = false;
    }
}
