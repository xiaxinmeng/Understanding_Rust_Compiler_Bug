rust
#[link(name = "kernel32")]
extern "system" {
    pub fn CloseHandle(hobject: HANDLE) -> BOOL;
    pub fn GetLastError() -> WIN32_ERROR;
    // etc,
}
