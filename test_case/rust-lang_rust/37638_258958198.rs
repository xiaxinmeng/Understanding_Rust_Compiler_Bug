 rust
#[cfg(windows)]
fn get_windows_version() -> u32 {
    extern crate kernel32;
    kernel32::GetVersion() as u32 // probably want to parse that instead
}

#[cfg(not(windows))]
fn get_windows_version() -> u32 {
    panic!();
}
