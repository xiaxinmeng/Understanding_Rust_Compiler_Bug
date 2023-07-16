rust
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "linux", // android?
    target_os = "netbsd",
    target_os = "openbsd",
    // target_os = "freebsd", ??
    // TODO: verify
))]
