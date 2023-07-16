 rust
match state.stream.read(buf) {
    Ok(len) => len as c_int,
    Err(err) => {
        if retriable_error(&err) {
            BIO_set_retry_write(bio);
        }
        state.error = Some(err);
        -1
    }
}
