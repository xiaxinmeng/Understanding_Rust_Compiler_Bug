rust
fn take_fd(object: impl IntoRawFd) {
    let fd = object.into_raw_fd();

    // I'm now responsible for closing fd so I'd better do that
}
