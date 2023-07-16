rust
fn some_failable() -> Result<(), MyError> {
  let _stuff = do_failable()
    .track_error()?;
  Ok(())
}
