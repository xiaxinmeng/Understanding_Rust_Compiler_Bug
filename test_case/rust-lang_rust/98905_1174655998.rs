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

fn func(_: impl AsRef<Path>) {}

fn main() {
    let s: &str = ".";
    let path: &Path = Path::new(&s);
    func(path);
    func(&path);
    func(Cow::Borrowed(path));
    func(CurDir);
    func(&CurDir);
    func(Cow::Borrowed(&CurDir));
    func(s);
    func(&s);
    func(Cow::Borrowed(s));
}
