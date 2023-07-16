rust
// Without `Copy`
pub fn tracked_env_var(var: impl AsRef<OsStr> + AsRef<str>) -> Result<String, VarError> {
    let var = &var.to_string(); // Have to convert into an owned string
    let value = env::var(var);
    bridge::client::FreeFunctions::track_env_var(var, value.as_deref().ok());
    value
}
