rust
async fn foo() -> Result<(), ()> {
    Ok(())
}

async fn bar() -> Result<(), ()> {
    foo()?;
    Ok(())
}
