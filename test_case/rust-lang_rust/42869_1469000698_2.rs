rust
#[cfg(target_os = "windows")]
let path: PathBuf = match path.strip_prefix(r"\\?\") {
    Ok(path) => path.to_path_buf(),
    Err(_) => path,
};
