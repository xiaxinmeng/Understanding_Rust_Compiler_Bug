rust
impl ToString for char {
    #[inline]
    fn to_string(&self) -> String {
        String::from(self.encode_utf8(&mut [0; 4]))
    }
}
impl ToString for str {
    #[inline]
    fn to_string(&self) -> String {
        String::from(self)
    }
}

impl ToString for Cow<'_, str> {
    #[inline]
    fn to_string(&self) -> String {
        self[..].to_owned()
    }
}

impl ToString for String {
    #[inline]
    fn to_string(&self) -> String {
        self.to_owned()
    }
}
