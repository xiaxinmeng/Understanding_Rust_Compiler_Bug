rust
fn foo() -> Result<Vec<T>, E> {
  let mut result = vec![];
  for x in values() {
    // I forget how placers work but I want to use them for p e r f
    result.place_back() <- try_process(x)?;
  }
  Ok(result)
}
