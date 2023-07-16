rust
pub fn greet<'a, N>(name: N) -> Result<(), ModError>
    where
        N: TryInto<Name<'a>>,
        ModError: From<N::Error>,
{
    let name: Name<'a> = name.try_into()?;
    println!("Hello, {}!", name.string());
    Ok(())
}
