Rust
fn chdir(p: &path::Path) -> io::Result<()> {
    let p: &OsStr = p.as_ref();
    
    // Skip the `\\?\` verbatim prefix, if any.
    static PREFIX: &[u8] = br"\\?\";
    let bytes = os_str_as_u8_slice(p);
    let prefix_len = if bytes.starts_with(PREFIX) {
        PREFIX.len()
    } else {
        0
    };
    
    let mut p = p.encode_wide().skip(prefix_len).collect::<Vec<_>>();
    p.push(0);
    
    cvt(unsafe { c::SetCurrentDirectoryW(p.as_ptr()) }).map(drop)
}
