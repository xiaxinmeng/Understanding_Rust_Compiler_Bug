rust
    Err(_) => Err(io::Error::new(io::ErrorKind::Other{}, "invalid UTF-8"))
