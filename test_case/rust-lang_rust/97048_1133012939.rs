rust
pub struct Struct {
    pub bar: u8,
    pub security_bit: u8
}

pub fn func(Struct { bar, .. }: Struct) {}
