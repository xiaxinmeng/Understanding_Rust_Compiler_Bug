 Rust
pub fn assure_config_dir_exists(dir: &str) -> Result<&Path, ArgumentError> {
    Ok(Path::new(dir))
}
