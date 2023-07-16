rust
/// Type of `UNICODE_VERSION` value:
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct UnicodeVersion {
    /// Major version.
    pub major: u16,

    /// Minor version.
    pub minor: u16,

    /// Micro (or Update) version.
    pub micro: u16,
}

impl<T: Into<u16>> From<(T, T, T)> for UnicodeVersion {
    fn from(t: (T, T, T)) -> UnicodeVersion {
        UnicodeVersion {
            major: t.0.into(),
            minor: t.1.into(),
            micro: t.2.into(),
        }
    }
}

impl<T: From<u16>> Into<(T, T, T)> for UnicodeVersion {
    fn into(self) -> (T, T, T) {
        (self.major.into(), self.minor.into(), self.micro.into())
    }
}
