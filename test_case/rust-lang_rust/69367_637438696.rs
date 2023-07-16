rust
if let Ok(_) = guard.file_handle.try_lock() {
    Ok(())
} else {
    Err(2)
}
