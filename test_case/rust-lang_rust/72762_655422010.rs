rust
fn narrowing_rem(&self, den: u8) -> u8 { (*self % Self::from(den)) as u8 }
