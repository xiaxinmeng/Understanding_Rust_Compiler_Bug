 rust
#[cfg(unix)]
fn lookup() -> Path {
    if cfg!(target_os = "android") {
        Path("/data/tmp")
    } else {
        getenv_nonempty("TMPDIR").unwrap_or_default(Path("/tmp"))
    }
}
