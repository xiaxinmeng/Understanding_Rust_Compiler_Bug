rust
pub struct Separator;

impl Separator {
    pub const STR: &str = crate::sys::path::MAIN_SEP_STR;
    pub const CHAR: char = crate::sys::path::MAIN_SEP;
}

impl Deref for Separator {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        Self::STR
    }
}

impl AsRef<str> for Separator {
    fn as_ref(&self) -> &str {
        self
    }
}

impl AsRef<char> for Separator {
    fn as_ref(&self) -> &char {
        &Self::CHAR
    }
}
