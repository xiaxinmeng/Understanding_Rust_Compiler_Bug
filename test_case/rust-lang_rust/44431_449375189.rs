rust
#[stable(feature = "path_from_str", since = "1.26.0")]
impl FromStr for PathBuf {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PathBuf::from(s))
    }
}
