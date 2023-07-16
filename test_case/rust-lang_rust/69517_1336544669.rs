rust
#[test]
fn wrapped() {
  fn wrapper() -> Result<(), std::io::Error> {
    let f = std::fs::File::open("/root/nonexistantfile")?;
    assert_eq!(example_method(&f), 0);
    Ok(())
  }
  wrapper().unwrap();
}
