rust
fn try_thing() -> Result<(), std::io::Error> { ... }
fn try_other_thing() -> Result<(), failure::Error> { ... }

existential type X:;
fn foo() -> X {
    try_thing()?;
    try_other_thing()?;
    Ok::<(), failure::Error>(())
}
