rust
#[stable(feature = "rust1", since = "1.0.0")]
impl ops::Deref for PathBuf {
    type Target = Path;

    fn deref(&self) -> &Path {
        Path::new(&self.inner)
    }
}
