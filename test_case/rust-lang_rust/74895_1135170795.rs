rust
/// Checks whether a path to an existing file can be written to.
/// If the file doesn't exist, this function will return false.
pub(crate) fn is_writeable(path: impl AsRef<Path>) -> bool {
    File::options()
        .write(true)
        // Make sure we don't accidentally truncate the file.
        .truncate(false)
        .open(path.as_ref())
        .is_ok()
}
