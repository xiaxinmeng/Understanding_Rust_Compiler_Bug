rust
fn from(never: !) -> TryFromIntError {
    match never {}
}
