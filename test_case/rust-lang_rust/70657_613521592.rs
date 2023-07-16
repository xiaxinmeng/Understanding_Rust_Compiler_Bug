rust
// This now compiles
match try {} {
    Ok(()) | Err(()) => (),
}
