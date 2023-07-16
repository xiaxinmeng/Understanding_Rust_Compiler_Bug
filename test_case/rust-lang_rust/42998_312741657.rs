rust
/// Type for Unicode Version.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UnicodeVersion(
    pub u16, // Major version
    pub u16, // Minor version
    pub u16 // Micro (or Update) version
);

impl UnicodeVersion {
    /// Major version
    pub fn major(&self) -> u16 {
        self.0
    }

    /// Minor version
    pub fn minor(&self) -> u16 {
        self.1
    }

    /// Micro (or Update) version
    pub fn micro(&self) -> u16 {
        self.2
    }
}
