 rust
match recover(|| state.stream.read(buf)) {
    Ok(Ok(len)) => len as c_int,
    Ok(Err(err)) => {
        if retriable_error(&err) {
            BIO_set_retry_write(bio);
        }
        state.error = Some(err);
        -1
    }
    Err(err) => {
        state.panic = Some(err);
        -1
    }
}
