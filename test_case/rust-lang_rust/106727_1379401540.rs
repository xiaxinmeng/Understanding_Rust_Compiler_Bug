rust
impl Foo {
    fn convert(val: u8) -> Result<Self, u8> {
        match val {
            0 => Ok(Foo::Zero),
            1 => Ok(Foo::One),
            2 => Ok(Foo::Two),
            3 => Ok(Foo::Three),
            4 => Ok(Foo::Four),
            _ => Err(255),
        }        
    }
}

impl TryFrom<u8> for Foo {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        Foo::convert(val).map_err(|_| "nope")
    }
}
