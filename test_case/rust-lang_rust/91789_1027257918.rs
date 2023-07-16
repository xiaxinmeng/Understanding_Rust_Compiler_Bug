rust
fn os_string_push_str(buf: &mut OsString, data: &str) -> Result<(), TryReserveError> {
    buf.try_reserve(data.len())?;
    buf.push(data);
    Ok(())
}
