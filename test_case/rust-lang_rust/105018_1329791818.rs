rust
fn mutate_path_buf(path_buf: &mut PathBuf) {
    let tmp: PathBuf = mem::take(path_buf);
    let mut tmp: Box<Path> = tmp.into_boxed_path();
    mutate_path(&mut *tmp);
    *path_buf = tmp.to_path_buf();
}

fn mutate_path(path: &mut Path) {
    // ...
}
