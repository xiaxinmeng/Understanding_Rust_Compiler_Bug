rust
use std::borrow::Cow;
use std::path::Path;

#[derive(Clone)]
struct CurDir;

impl AsRef<Path> for CurDir {
    fn as_ref(&self) -> &Path {
        ".".as_ref()
    }
}

impl<'a> AsRef<Path> for Cow<'a, CurDir> {
    fn as_ref(&self) -> &Path {
        self.deref().as_ref()
    }
}
