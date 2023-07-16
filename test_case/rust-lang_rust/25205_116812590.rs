 rust
use std::io;
use std::path::Path;
fn visit_dirs<F : FnMut(&Path), E : FnMut(&Path, io::Error)>(dir: &Path, fs : Option<u64>, mut cb: F, mut err: E) {
    visit_dirs(&dir, fs, |p| cb(p), |p, e| err(p, e));
}
fn main() {
    visit_dirs(Path::new("foo.txt"), None, |p|{}, |p,e|{});
}
