
pub struct Align { abi: u8, pref: u8 };

impl Align {
    pub fn abi(self) -> u64 { 1 << self.abi }
}
