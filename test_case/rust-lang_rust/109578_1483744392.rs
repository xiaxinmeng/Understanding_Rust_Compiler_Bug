rust
/* `x?` turns into */
match x {
  Ok(ok) => ok,
  Err(err) => return Err(err.into()),
}
