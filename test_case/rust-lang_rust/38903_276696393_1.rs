rust
match foo.try_wait() {
    Ok(status) => {
        ...
    }
    Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
        ...
    }
    Err(err) => return Err(err),
}
