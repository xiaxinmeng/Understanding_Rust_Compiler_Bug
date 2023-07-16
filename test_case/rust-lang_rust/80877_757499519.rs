rust
impl<T: AsRef<Header2>> From<T> for [u8; 20] {
    fn from(header2: T) -> [u8; 20] {
        let mut buf = [0u8; 20];
        unimplemented!();
    }
}
