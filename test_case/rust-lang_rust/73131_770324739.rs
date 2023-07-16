rust
fn make_exit_status_a_result<E: Error>(self, err: E) -> Result<(), E>
