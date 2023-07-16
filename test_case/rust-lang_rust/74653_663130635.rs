rust
pub fn tracked_env_var(var: impl AsRef<OsStr> + AsRef<str> + Copy) -> Result<String, VarError> {
    let value = env::var(var); // Need `Copy` to avoid consuming `var` here
    bridge::client::FreeFunctions::track_env_var(var, value.as_deref().ok());
    value
}
