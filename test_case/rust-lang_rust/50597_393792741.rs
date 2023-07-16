rust
trait TryConvert<T> {
    fn try_convert(self) -> Result<T, ()>;
}

impl<T, U> TryConvert<T> for U where U: TryInto<T> {
    fn try_convert(self) -> Result<T, ()> { self.try_into().map_err(|_| ()) }
}

impl TryConvert<bool> for u32 {
    // ...
}
