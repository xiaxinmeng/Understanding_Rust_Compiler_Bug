rust
pub struct Hz;

impl Hz {
    pub const fn num(&self) -> u32 {
        42
    }
    pub const fn normalized(&self) -> Hz {
        Hz
    }

    pub const fn as_u32(&self) -> u32 {
        self.normalized().num()
    }
}
