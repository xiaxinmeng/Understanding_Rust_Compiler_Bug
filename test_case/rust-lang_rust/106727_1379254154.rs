rust
impl TryFrom<u8> for Foo {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Foo::Zero),
            1 => Ok(Foo::One),
            2 => Ok(Foo::Two),
            3 => Ok(Foo::Three),
            4 => Ok(Foo::Four),
            _ => loop {},
        }
    }
}
