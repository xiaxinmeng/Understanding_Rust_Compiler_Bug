rust
#[cfg(target_arch = "x86_64")]
pub type CCharRepr = i8;

#[cfg(target_arch = "aarch64")]
pub type CCharRepr = u8;

#[repr(transparent)]
pub struct CChar {
    repr: CCharRepr
}

impl CChar {
    pub fn new(repr: CCharRepr) -> Self {
        Self { repr }
    }

    pub unsafe fn get_inner(&self) -> CCharRepr {
        self.repr
    }
}
