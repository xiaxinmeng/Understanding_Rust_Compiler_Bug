rust
let file = File::from(owned_fd);
file.sync_all()?;
