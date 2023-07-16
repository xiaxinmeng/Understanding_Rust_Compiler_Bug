
fn need_dir(s: &Path) {
    if os::path_is_dir(s) { return; }
    if !os::make_dir(s, 493_i32 /* oct: 755 */) {
        fail fmt!("can't make_dir %s", s.to_str());
    }
}
