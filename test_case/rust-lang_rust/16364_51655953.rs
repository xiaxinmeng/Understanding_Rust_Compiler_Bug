
/// Performs division that's checked against failure.
///
/// # Failure
///
/// If dividing by zero is attempted, this will return `None`.
///
/// If the result would overflow or underflow, this will wrap around.
