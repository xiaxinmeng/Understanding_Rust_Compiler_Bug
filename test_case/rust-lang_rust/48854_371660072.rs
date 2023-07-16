rust
#[test]
#[should_panic]
fn not_a_num() -> Result<(), ParseIntError> {
    let _: u32 = "abc".parse()?;
    Ok(())
}
