 rust
fn unwatch(&mut self, path: &Path) -> Result<(), Error> {
  // FIXME:
  // once https://github.com/rust-lang/rust/pull/22351 gets merged,
  // just use a &Path
  match self.watches.remove(&path.to_path_buf()) {
    None => Err(Error::WatchNotFound),
