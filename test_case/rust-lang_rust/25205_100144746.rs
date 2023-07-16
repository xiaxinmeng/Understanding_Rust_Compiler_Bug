
fn visit_dirs<F : FnMut(&Path), E : FnMut(&Path, io::Error)>(dir: &Path, fs : Option<u64>, mut cb: F, mut err: E) {
...
}
