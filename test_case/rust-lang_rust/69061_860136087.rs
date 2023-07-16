rust
fn address(&self) -> AddressKind<'_> {
    let len = self.len as usize - sun_path_offset(&self.addr);
    let path = unsafe { mem::transmute::<&[libc::c_char], &[u8]>(&self.addr.sun_path) };

    // macOS seems to return a len of 16 and a zeroed sun_path for unnamed addresses
    if len == 0
        || (cfg!(not(any(target_os = "linux", target_os = "android")))
            && self.addr.sun_path[0] == 0)
    {
        AddressKind::Unnamed
    } else if self.addr.sun_path[0] == 0 {
        AddressKind::Abstract(&path[1..len])
    } else {
        cfg_if! {
            if #[cfg(any(target_os = "macos",
                         target_os = "ios",
                         target_os = "freebsd",
                         target_os = "dragonfly",
                         target_os = "openbsd",
                         target_os = "netbsd"))] {
                // BSD-like systems may not includes the last '\0' in length,
                // because they have a sun_len as the length of sun_path
                let sun_len = self.addr.sun_len;
            } else {
                // Trim the last '\0'
                let mut sun_len = len;
                while sun_len > 0 && path[sun_len] == 0 {
                    sun_len -= 1;
                }
            }
        }

        AddressKind::Pathname(OsStr::from_bytes(&path[..sun_len]).as_ref())
    }
}
