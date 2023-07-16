rust
#![forbid(unreachable_code)]

#[allow(dead_code)] // Remove warning.
fn foo() -> Result<(), ()> {
    Ok(0)?;
    Err(()) // Just so it doesn't emit error here.
}
