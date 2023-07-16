 rust
pub trait Headers {
    // Theoretical
    pub type HeaderEntries<'a>: Iterator<(&'a str, Vec<&'a str>)>;

    fn find<'a>(&'a self, key: &str) -> Option<Vec<&'a str>>;
    fn has(&self, key: &str) -> bool;
    fn iter<'a>(&'a self) -> HeaderEntries<'a>;
}
