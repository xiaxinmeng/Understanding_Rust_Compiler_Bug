rust
pub trait Write {
    ...

    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result {
        write(&mut self, args)
    }
}

